import React, { useState } from "react";
import SignatureResult from "./SignatureResultModal";
import { encodePacked, keccak256 } from "viem";
import { useWalletClient } from "wagmi";
import { ResturantNft } from "~~/types/resturantNft";
import { notification } from "~~/utils/scaffold-eth";

type GenerateTicketButtonProps = {
  nft: ResturantNft;
};

export default function GenerateTicketButton({ nft }: GenerateTicketButtonProps) {
  const [isLoading, setLoading] = useState<boolean>(false);

  //   *When wagmi will add signMessage with raw data
  //   const { data, isLoading, signMessage: signMessageWagmi } = useSignMessage();

  const wallet = useWalletClient();
  return (
    <button
      className="btn btn-primary rounded-full capitalize font-normal font-white w-24 flex items-center gap-1 hover:gap-2 transition-all tracking-widest"
      onClick={async () => {
        const messageHash = keccak256(encodePacked(["uint256", "address"], [nft.id, nft.resturant]));
        console.log({ messageHash });
        setLoading(true);
        const signature = await wallet.data?.signMessage({ message: { raw: messageHash } });
        setLoading(false);
        if (signature) {
          notification.success(<SignatureResult signature={signature} tokenId={nft.id} />, {
            duration: Infinity,
          });
        }
        //   *When wagmi will add signMessage with raw data
        //   signMessageWagmi({
        //     message: messageHash,
        //   });
      }}
      disabled={isLoading}
    >
      {isLoading ? "Check Wallet" : "Generate Ticket"}
    </button>
  );
}
