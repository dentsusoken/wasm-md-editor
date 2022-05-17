import React from "react";
import { render } from "react-dom";
import TextArea from "./components/Textarea";
import "./index.css";

const App: React.FC = (): JSX.Element => {
  return (<>
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
