import { useBalance } from "wagmi";

type ControllerBalanceInTokenProps = {
  controllerAddress: string;
  tokenAddress: string;
};

export default function ControllerBalanceInToken({ controllerAddress, tokenAddress }: ControllerBalanceInTokenProps) {
  const { data, isLoading } = useBalance({
    address: controllerAddress,
    token: tokenAddress,
  });

  if (!data || isLoading) {
    return (
      <div className="animate-pulse flex space-x-4">
        <div className="rounded-md bg-slate-300 h-6 w-6"></div>
        <div className="flex items-center space-y-6">
          <div className="h-2 w-28 bg-slate-300 rounded"></div>
        </div>
      </div>
    );
  }

  return (
    <>
      <span className="font-normal">{Number(data.formatted).toFixed(4)} </span>
      <span className="text-[0.8em] font-bold ml-1">{data.symbol} </span>
    </>
  );
}
