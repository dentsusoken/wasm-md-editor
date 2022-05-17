import React, { useState } from "react";
import { render } from "react-dom";
import TextArea from "./components/Textarea";
import "./index.css";

const App: React.FC = (): JSX.Element => {
  return (
    <TextArea
      autoFocus={true}
      disabled={false}
      defaultValue={"# default value"}
    />
  );
};

render(<App />, document.getElementById("root"));
