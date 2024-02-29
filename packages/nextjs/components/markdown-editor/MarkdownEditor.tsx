import React, { useEffect, useState } from "react";
import MenuBar from "./MenuBar";
// import TextStyle from "@tiptap/extension-text-style";
import Placeholder from "@tiptap/extension-placeholder";
import { EditorContent, useEditor } from "@tiptap/react";
import StarterKit from "@tiptap/starter-kit";
import htmlToMarkdown from "@wcj/html-to-markdown";

type MarkdownEditorProps = {
  value?: string;
  placeholder?: string;
  onChange: (value: string) => void;
};

export default function MarkdownEditor({ value, placeholder, onChange }: MarkdownEditorProps) {
  const [localValue, setLocalValue] = useState(value);
  const editor = useEditor(
    {
      content: value,
      extensions: [
        StarterKit.configure({}),
        Placeholder.configure({
          placeholder,
          emptyEditorClass:
            "before:content-[attr(data-placeholder)] before:absolute before:opacity-30 dark:before:opacity-20 text-sm font-normal",
        }),
      ],
      editorProps: {
        attributes: {
          class: "prose prose-sm focus:outline-none ",
        },
      },
      onUpdate: ({ editor }) => {
        setLocalValue(editor.getHTML());
      },
    },
    [],
  );

  useEffect(() => {
    if (!onChange || typeof localValue !== "string" || localValue === value) return;
    const init = async () => {
      onChange(await htmlToMarkdown({ html: localValue }));
    };
    init();

    onChange(localValue as string);
  }, [localValue, onChange, value]);

  return (
    <div className="prose rounded-xl border-2 border-base-300 bg-base-200">
      <MenuBar editor={editor} />
      <EditorContent className="p-3 cursor-text" editor={editor} />
    </div>
  );
}
