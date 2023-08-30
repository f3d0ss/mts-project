import React, { ReactNode } from "react";

type MarkdownDisplayProps = {
  children?: ReactNode;
};

export default function MarkdownDisplay({ children }: MarkdownDisplayProps) {
  const rootProps = typeof children === "string" ? { dangerouslySetInnerHTML: { __html: children } } : {};
  return (
    <div className="prose border rounded-2xl p-1" {...rootProps}>
      {typeof children !== "string" ? children : null}
    </div>
  );
}
