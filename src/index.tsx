import React, { useEffect, useState } from "react";
import { render } from "react-dom";
import TextArea from "./components/Textarea";
import "./index.css";
import { wasm_main } from "wasm-md-editor";
// import { wasm_main } from "wasm-md-editor";

export type IWasmMain = {
  wasmMain: typeof wasm_main;
};

export type AppType = {
  text?: string;
};
const useMarkdownParser = () => {
  const [state, setState] = useState<IWasmMain | null>(null);
  useEffect(() => {
    (async () => {
      const wasmContainer = await import("wasm-md-editor");
      setState(wasmContainer);
    })();
  }, []);
  return state;
};
const App: React.FC<AppType> = (text): JSX.Element => {
  const instance = useMarkdownParser();
  return (
    <>
      <div
        dangerouslySetInnerHTML={{
          __html: instance?.wasmMain(text) ?? "",
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

render(<App />, document.getElementById("root"));
