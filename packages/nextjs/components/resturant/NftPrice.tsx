import React from "react";
import { formatUnits } from "viem";
import { useToken } from "wagmi";

type NftPriceProps = {
  price?: bigint;
  tokenAddress?: string;
};

export default function NftPrice({ price, tokenAddress }: NftPriceProps) {
  const { data: paymentToken } = useToken({
    address: tokenAddress,
  });
  return (
    <div>
      <span>{formatUnits(price || 0n, 18)}</span> <span className="font-bold">{paymentToken?.symbol}</span>
    </div>
  );
}
