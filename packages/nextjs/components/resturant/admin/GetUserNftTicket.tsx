import React, { useEffect, useState } from "react";
import { useContractRead } from "wagmi";
import Html5QrcodePlugin from "~~/components/Html5QrcodePlugin";
import { InputBase } from "~~/components/scaffold-eth";
import contracts from "~~/generated/usefulAbis";
import { FetchedNft } from "~~/types/fetchedNft";

type GetUserNftTicketProps = {
  resturantAddress: string;
  onRead: (id: bigint, nft: FetchedNft, signature: `0x${string}`) => void;
};

export default function GetUserNftTicket({ resturantAddress, onRead }: GetUserNftTicketProps) {
  const [tokenId, setTokenId] = useState<bigint | string>("");
  const [signature, setSignature] = useState<`0x${string}` | undefined>();
  const [state, setState] = useState<"Init" | "Scan" | "Manual">("Init");
  const [ticket, setTicket] = useState<string>("");

  const { data: nft } = useContractRead({
    address: resturantAddress,
    abi: contracts.ResturantToken.abi,
    functionName: "getNft",
    args: tokenId ? [BigInt(tokenId)] : undefined,
  });

  useEffect(() => {
    if (nft && signature) {
      onRead(BigInt(tokenId), nft, signature);
    }
  }, [nft, signature, onRead, tokenId]);

  const decodeTicket = (jsonTicket: string) => {
    console.log({ jsonTicket });
    try {
      const ticket = JSON.parse(jsonTicket);
      setTokenId(ticket.tokenId);
      setSignature(ticket.signature);
    } catch {}
    setTicket(jsonTicket);
  };

  const scanButton = (
    <button className="btn btn-secondary btn-sm" onClick={() => setState("Scan")}>
      Scan
    </button>
  );

  const manualButton = (
    <button className="btn btn-secondary btn-sm" onClick={() => setState("Manual")}>
      Manual
    </button>
  );

  switch (state) {
    case "Init":
      return (
        <>
          {scanButton} {manualButton}
        </>
      );

    case "Scan":
      return (
        <>
          {manualButton}
          <Html5QrcodePlugin
            fps={10}
            qrbox={250}
            disableFlip={false}
            qrCodeSuccessCallback={decodeTicket}
            verbose={true}
          />
        </>
      );
    case "Manual":
      return (
        <>
          {scanButton}
          <InputBase value={ticket} onChange={decodeTicket} name="Ticket" placeholder="Ticket" />
        </>
      );
  }
}
