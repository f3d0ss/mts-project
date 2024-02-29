import React from "react";
import Markdown from "react-markdown";

type MarkdownDisplayProps = {
  children?: string;
};

export default function MarkdownDisplay({ children }: MarkdownDisplayProps) {
  return (
    <div className="prose border rounded-2xl p-1">
      <Markdown>{children}</Markdown>
    </div>
  );
}
