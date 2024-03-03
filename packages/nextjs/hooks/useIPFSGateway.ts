import { useEffect, useState } from "react";
import { IPFSHTTPClient, create } from "kubo-rpc-client";
import { CID } from "multiformats/cid";
import { useInterval } from "usehooks-ts";

const IPFS_POLLING_INTERVAL = 5000;
const IPFS_GATEWAY = "ipfs.cf-ipfs.com";

function convertIPFSUrlToGatwayURL(ipfsUrl: string): string {
  // Extracting the hash part from the IPFS URL
  let hash: string = ipfsUrl.split("://")[1].split("/")[0];
  // Converto to CIDv1
  hash = CID.parse(hash).toV1().toString();
  // Building the converted URL
  const convertedUrl = `https://${hash}.${IPFS_GATEWAY}${ipfsUrl.substring(ipfsUrl.lastIndexOf("/"))}`;
  return convertedUrl;
}

export const useIPFSGateway = () => {
  const [ipfsHttpClient, setIpfsHttpClient] = useState<IPFSHTTPClient>();
  const [isOnline, setIsOnline] = useState<boolean>(false);
  const [isConnecting, setIsConnecting] = useState<boolean>(true);

  useEffect(() => {
    const init = async () => {
      if (ipfsHttpClient) return;

      const client = create({ url: "http://127.0.0.1:5001" });

      setIpfsHttpClient(client);
      try {
        // isOnline return a Promise, not typed correctly
        setIsOnline(await client.isOnline());
        setIsConnecting(false);
      } catch (e) {
        console.error(e);
        setIsOnline(false);
        setIsConnecting(false);
      }
    };

    init();
  }, [ipfsHttpClient]);

  useInterval(async () => {
    if (ipfsHttpClient) {
      try {
        setIsOnline(await ipfsHttpClient.isOnline());
      } catch (e) {}
    }
  }, IPFS_POLLING_INTERVAL);

  const uploadFile = async (file: Uint8Array) => {
    try {
      if (!ipfsHttpClient) throw Error("IPFS not yet initialized");
      const result = await ipfsHttpClient.add(file, { cidVersion: 1, hashAlg: "sha2-256" });
      return result.cid;
    } catch (e) {
      if (e instanceof Error && (e.message == "Failed to fetch" || e.message == "IPFS not yet initialized")) {
        console.error("Failed to connect to IPFS");
        setIsOnline(false);
        return undefined;
      }
      throw e;
    }
  };

  const getFile = async (ipfsPath: string) => {
    try {
      if (!ipfsPath) throw Error("Path not provided");
      if (!ipfsHttpClient || !(await ipfsHttpClient.isOnline())) {
        const url = convertIPFSUrlToGatwayURL(ipfsPath);
        const response = await fetch(url);
        const data = await response.blob();
        return new Uint8Array(await data.arrayBuffer());
      }
      if (ipfsPath.startsWith("ipfs://")) {
        ipfsPath = ipfsPath.substring(7);
      }
      const resp = ipfsHttpClient.cat(ipfsPath);
      let content: Uint8Array = new Uint8Array();
      for await (const chunk of resp) {
        content = Uint8Array.from([...content, ...chunk]);
      }
      return content;
    } catch (e) {
      if (e instanceof Error && (e.message == "Failed to fetch" || e.message == "IPFS not yet initialized")) {
        console.error("Failed to connect to IPFS");
        setIsOnline(false);
        return undefined;
      }
      if (e instanceof Error && e.message == "Path not provided") {
        console.error(e.message);
        return undefined;
      }
      throw e;
    }
  };
  return {
    isOnline,
    isConnecting,
    uploadFile,
    getFile,
  };
};
