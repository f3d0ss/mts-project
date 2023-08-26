import { useEffect, useState } from "react";
import { IPFSHTTPClient, create } from "kubo-rpc-client";

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

  const uploadFile = ipfsHttpClient
    ? async (file: Uint8Array) => {
        console.log({ isOnline });
        const result = await ipfsHttpClient.add(file);
        return result.cid;
      }
    : undefined;

  const getFile = ipfsHttpClient
    ? async (ipfsPath: string) => {
        if (ipfsPath.startsWith("ipfs://")) {
          ipfsPath = ipfsPath.substring(7);
        }
        const resp = ipfsHttpClient.cat(ipfsPath);
        let content: Uint8Array = new Uint8Array();
        for await (const chunk of resp) {
          content = Uint8Array.from([...content, ...chunk]);
        }
        return content;
      }
    : undefined;
  return {
    isOnline,
    isConnecting,
    uploadFile,
    getFile,
  };
};
