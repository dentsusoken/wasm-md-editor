import React, { useEffect, useState } from "react";
import { render } from "react-dom";
import TextArea from "./components/Textarea";
import "./index.css";
import { wasm_main } from "wasm-md-editor/wasm_md_editor";

export type IWasmMain = {
  wasm_main: typeof wasm_main;
};

export type AppType = {
  text?: string;
};
export const useMarkdownParser = () => {
  const [state, setState] = useState<IWasmMain | null>(null);
  useEffect(() => {
    import("wasm-md-editor/wasm_md_editor").then((res) => {
      return setState(res);
    });
  }, []);
  return state;
};
const App: React.FC<AppType> = ({ text }): JSX.Element => {
  const instance = useMarkdownParser();
  return (
    <>
      <div
        dangerouslySetInnerHTML={{
          __html: instance?.wasm_main(text as string) ?? "",
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
