import React, { useEffect, useState } from "react";
// import { render } from "react-dom";
import { createRoot } from "react-dom/client";
import TextArea from "./components/Textarea";
import "./index.css";
import {wasm_main} from "wasm-md-editor";

export type IWasmMain = {
  wasm_main: typeof wasm_main;
};

export type AppType = {
  text: string;
};
export const useMarkdownParser = () => {
  const [state, setState] = useState<IWasmMain | null>(null);
  useEffect(() => {
    (async () => {
      await import("wasm-md-editor").then((module) => {
        setState(module);
        
      });
    })();
  }, []);
  return state;
};
const App: React.FC<AppType> = ({ text }): JSX.Element => {
  const instance = useMarkdownParser();
  return (
    <>
      <div
        dangerouslySetInnerHTML={{
          __html: instance?.wasm_main(text) ?? "",
        }}
      />
      <TextArea
        autoFocus={false}
        disabled={false}
        defaultValue={"# default value"}
      />
      <TextArea
        autoFocus={false}
        disabled={false}
        defaultValue={"# defdault value"}
      />
    </>
  );
};
const root = createRoot(document.getElementById("root")!);
root.render(<App text={"# aiuro"} />);
