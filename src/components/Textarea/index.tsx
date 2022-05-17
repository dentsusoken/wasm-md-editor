import { TextField } from "@mui/material";
import React from "react";
export type TextAreaProps = {
  autoFocus?: boolean;
  defaultValue?: string;
  disabled?: boolean;
  maxLength?: number;
};

const TextArea: React.FC<TextAreaProps> = ({
  autoFocus,
  defaultValue,
  disabled,
  maxLength = 1024000,
}): JSX.Element => {
  const inputProps = {
    maxLength: maxLength,
  };


  return (
    <>
      <TextField
        className="class"
        id="outlined-multiline-flexible"
        autoFocus={autoFocus}
        label="Write down your document!"
        multiline
        rows="40"
        defaultValue={defaultValue}
        inputProps={inputProps}
        disabled={disabled}
        variant="outlined"
      />
    </>
  );
};


export default TextArea;
