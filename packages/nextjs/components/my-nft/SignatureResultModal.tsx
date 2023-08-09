import React from "react";
import { QRCodeSVG } from "qrcode.react";
import { notification } from "~~/utils/scaffold-eth";

type SignatureResultProps = {
  signature: `0x${string}`;
  tokenId: bigint;
};

export default function SignatureResult({ signature, tokenId }: SignatureResultProps) {
  const ticket = JSON.stringify({ signature, tokenId }, (_, v) => (typeof v === "bigint" ? v.toString() : v));

  const copy = async () => {
    await navigator.clipboard.writeText(ticket);
    notification.success(<span>Copied!</span>);
  };

  return (
    <div>
      <button onClick={copy}>
        <span className="font-bold">Here is your QR</span>
        <QRCodeSVG value={ticket} size={256} />
        Copy on the Clipboard
      </button>
    </div>
  );
}
