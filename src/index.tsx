import React from "react";
import { render } from "react-dom";
import "./index.css";

const App: React.FC = (): JSX.Element => {
  return <h1>REACT</h1>;
};

render(<App />, document.getElementById("root"));
