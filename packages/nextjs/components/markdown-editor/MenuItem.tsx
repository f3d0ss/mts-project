import React, { FunctionComponent } from "react";

type MenuItemProps = {
  icon: FunctionComponent<React.SVGProps<SVGSVGElement>>;
  title: string;
  action: () => void;
  isActive: () => boolean;
};

export default function MenuItem({ icon: Icon, title, action, isActive }: MenuItemProps) {
  return (
    <button
      className={isActive() ? "rounded-base border border-gray-400" : "border  border-transparent"}
      onClick={action}
      title={title}
    >
      <Icon />
    </button>
  );
}
