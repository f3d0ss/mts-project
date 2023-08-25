// Inspired by InputBase of Scaffold
import { ChangeEvent, ReactNode, useCallback } from "react";
import { CommonInputProps } from "~~/components/scaffold-eth";

type TextAreaInputProps<T> = CommonInputProps<T> & {
  error?: boolean;
  disabled?: boolean;
  prefix?: ReactNode;
  suffix?: ReactNode;
  rows?: number;
};

export const TextAreaInput = <T extends { toString: () => string } | undefined = string>({
  name,
  value,
  onChange,
  placeholder,
  error,
  disabled,
  prefix,
  suffix,
  rows,
}: TextAreaInputProps<T>) => {
  let modifier = "";
  if (error) {
    modifier = "border-error";
  } else if (disabled) {
    modifier = "border-disabled bg-base-300";
  }

  const handleChange = useCallback(
    (e: ChangeEvent<HTMLTextAreaElement>) => {
      onChange(e.target.value as unknown as T);
    },
    [onChange],
  );

  return (
    <div className={`flex border-2 border-base-300 bg-base-200 rounded-2xl text-accent ${modifier}`}>
      {prefix}
      <textarea
        className="input input-ghost focus:outline-none focus:bg-transparent focus:text-gray-400 h-[4.4rem] min-h-[4.4rem] px-4 border w-full font-medium placeholder:text-accent/50 text-gray-400"
        placeholder={placeholder}
        name={name}
        value={value?.toString()}
        onChange={handleChange}
        disabled={disabled}
        autoComplete="off"
        rows={rows}
      />
      {suffix}
    </div>
  );
};
