import { useState } from "react";
import Image from "next/image";

type InputImageProps = {
  onChange: (image: Uint8Array) => void;
};

export const InputImage = ({ onChange }: InputImageProps) => {
  const [image, setImage] = useState<string | undefined>();

  const retrieveFile = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!e.target.files) return;
    const data = e.target.files[0];
    const reader = new window.FileReader();
    reader.readAsArrayBuffer(data);
    reader.onloadend = async () => {
      if (reader.result) {
        console.log("Buffer data: ", reader.result);
        const base64Image = Buffer.from(reader.result as ArrayBuffer).toString("base64");
        const imageUrl = `data:image/jpeg;base64,${base64Image}`;
        setImage(imageUrl);
        onChange(new Uint8Array(reader.result as ArrayBuffer));
      }
    };
    e.preventDefault();
  };

  return (
    <div className="flex flex-col items-center">
      <label
        htmlFor="picture"
        className={`btn btn-primary rounded-full capitalize font-normal font-white flex items-center gap-1 hover:gap-2 transition-all tracking-widest ${
          image && "text-xs min-h-0 h-8"
        }`}
      >
        {image ? "Change picture" : "Choose a picture"}
      </label>

      <input
        type="file"
        id="picture"
        name="picture"
        accept="image/png, image/jpeg"
        onChange={retrieveFile}
        hidden={true}
      />
      {image && <Image alt="Selected Image" src={image} width={100} height={100} />}
    </div>
  );
};
