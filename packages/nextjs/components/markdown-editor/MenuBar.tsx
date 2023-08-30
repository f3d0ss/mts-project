import React, { useMemo } from "react";
import MenuItem from "./MenuItem";
import { Editor } from "@tiptap/react";
import { LuBold, LuCode, LuHeading1, LuHeading2, LuItalic, LuList, LuListOrdered, LuQuote } from "react-icons/lu";

type MenuBarProps = {
  editor: Editor | null;
};

export default function MenuBar({ editor }: MenuBarProps) {
  const items = useMemo(
    () => [
      {
        icon: LuHeading1,
        title: "Heading 1",
        action: () => editor?.chain().focus().toggleHeading({ level: 1 }).run(),
        isActive: () => !!editor?.isActive("heading", { level: 1 }),
      },
      {
        icon: LuHeading2,
        title: "Heading 2",
        action: () => editor?.chain().focus().toggleHeading({ level: 2 }).run(),
        isActive: () => !!editor?.isActive("heading", { level: 2 }),
      },
      {
        icon: LuBold,
        title: "Bold",
        action: () => editor?.chain().focus().toggleBold().run(),
        isActive: () => !!editor?.isActive("bold"),
      },
      {
        icon: LuItalic,
        title: "Italic",
        action: () => editor?.chain().focus().toggleItalic().run(),
        isActive: () => !!editor?.isActive("italic"),
      },
      {
        icon: LuCode,
        title: "Code",
        action: () => editor?.chain().focus().toggleCodeBlock().run(),
        isActive: () => !!editor?.isActive("code"),
      },
      {
        icon: LuQuote,
        title: "Blockquote",
        action: () => editor?.chain().focus().toggleBlockquote().run(),
        isActive: () => !!editor?.isActive("blockquote"),
      },
      {
        icon: LuListOrdered,
        title: "Ordered List",
        action: () => editor?.chain().focus().toggleOrderedList().run(),
        isActive: () => !!editor?.isActive("orderedList"),
      },
      {
        icon: LuList,
        title: "Bullet List",
        action: () => editor?.chain().focus().toggleBulletList().run(),
        isActive: () => !!editor?.isActive("bulletList"),
      },
    ],
    [editor],
  );

  return (
    <div className="flex flex-wrap content-center gap-2.5 border-b p-2">
      {items.map((item, index) => (
        <MenuItem key={index} {...item} />
      ))}
    </div>
  );
}
