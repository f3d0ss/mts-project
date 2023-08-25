import { useEffect, useState } from "react";
import type { Helia } from "@helia/interface";
import { unixfs } from "@helia/unixfs";
import { createHelia } from "helia";

export const useIPFS = () => {
  const [id, setId] = useState<string | undefined>();
  const [helia, setHelia] = useState<Helia | undefined>();
  const [isOnline, setIsOnline] = useState<boolean>(false);

  useEffect(() => {
    const init = async () => {
      if (helia) return;

      // application-specific data lives in the datastore
      // const datastore = new MemoryDatastore();

      // libp2p is the networking layer that underpins Helia
      // const libp2p = await createLibp2p({
      //   datastore,
      //   addresses: {
      //     listen: ["/ip4/127.0.0.1/tcp/0/ws"],
      //   },
      //   transports: [webSockets()],
      //   connectionEncryption: [noise()],
      //   streamMuxers: [yamux()],
      //   peerDiscovery: [
      //     bootstrap({
      //       list: [
      //         "/dnsaddr/bootstrap.libp2p.io/p2p/QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN",
      //         "/dnsaddr/bootstrap.libp2p.io/p2p/QmQCU2EcMqAqQPR2i9bChDtGNJchTbq5TbXJJ16u19uLTa",
      //         "/dnsaddr/bootstrap.libp2p.io/p2p/QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb",
      //         "/dnsaddr/bootstrap.libp2p.io/p2p/QmcZf59bWwK5XFi76CZX8cbJ4BhTzzA3gU1ZjYZcYW3dwt",
      //       ],
      //     }),
      //   ],
      //   services: {
      //     identify: identifyService(),
      //   },
      // });

      const heliaNode = await createHelia({
        // libp2p
      });

      const nodeId = heliaNode.libp2p.peerId.toString();
      const nodeIsOnline = heliaNode.libp2p.isStarted();

      setHelia(heliaNode);
      setId(nodeId);
      setIsOnline(nodeIsOnline);
    };

    init();
  }, [helia]);

  const uploadFile = helia
    ? async (file: Uint8Array) => {
        const fs = unixfs(helia);

        // add the bytes to your node and receive a unique content identifier
        const cid = await fs.addBytes(file, {
          onProgress: evt => {
            console.info("add event", evt.type, evt.detail);
          },
        });
        return cid;
      }
    : undefined;

  return { helia, id, isOnline, uploadFile };
};
