import { TextField } from "@mui/material";
import React from "react";
export type TextAreaProps = {
  autoFocus?: boolean;
  defaultValue?: string;
  disabled?: boolean;
  maxLength?: number;
  onBlur?: (s: string) => void;
};

const TextArea: React.FC<TextAreaProps> = ({
  autoFocus,
  defaultValue,
  disabled,
  maxLength = 1024,
  onBlur,
}): JSX.Element => {
  const inputProps = {
    maxLength: maxLength,
  };

  const handleOnBlur = (
    e: React.FocusEvent<HTMLInputElement | HTMLTextAreaElement>
  ): void => {
    if (disabled) {
      return;
    }
    if (onBlur) {
      const input = e.currentTarget.value;
      onBlur(input);
    }
  };

  return (
    <>
      <TextField
        className="class"
        id="outlined-multiline-flexible"
        autoFocus={autoFocus}
        label="Leave a comment"
        multiline
        rows="4"
        defaultValue={defaultValue}
        inputProps={inputProps}
        disabled={disabled}
        onBlur={handleOnBlur}
        variant="outlined"
      />
    </>
  );
};


export default TextArea;
