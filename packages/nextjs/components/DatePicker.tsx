import React, { ChangeEvent, useCallback } from "react";

type DatePickerProps = {
  value: Date;
  onChange: (date: Date) => void;
};

export default function DatePicker({ value, onChange }: DatePickerProps) {
  const handleChange = useCallback(
    (e: ChangeEvent<HTMLInputElement>) => {
      console.log({ e });
      const date = Date.parse(e.target.value);
      if (!isNaN(date)) {
        onChange(new Date(date));
        console.log(new Date(date).toLocaleString());
      }
    },
    [onChange],
  );

  return (
    <div className="relative flex border-2 border-base-300 bg-base-200 rounded-full text-accent " id="myDatetimepicker">
      <input
        type="datetime-local"
        value={value.toISOString().slice(0, 16)}
        className="input input-ghost focus:outline-none focus:bg-transparent focus:text-gray-400 h-[2.2rem] min-h-[2.2rem] px-4 border w-full font-medium placeholder:text-accent/50 text-gray-400"
        placeholder="Select a date"
        onChange={handleChange}
      />
    </div>
  );
}
