import { useEffect, useState } from "react";
import { IPFSHTTPClient, create } from "kubo-rpc-client";
import { useInterval } from "usehooks-ts";

const IPFS_POLLING_INTERVAL = 3000;

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
      const result = await ipfsHttpClient.add(file);
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
      if (!ipfsHttpClient) throw Error("IPFS not yet initialized");
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
