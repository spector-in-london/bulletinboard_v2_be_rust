import React, { useState } from 'react';
import styled, { css } from 'styled-components';

const ButtonWrapper = styled.button`
  display: block;
  width: 100%;
  border: none;
  padding: 0.5em;
  background-color: rgba(255, 188, 5, 0.265);
  outline: none;

  :disabled {
    cursor: not-allowed;
    background-color: #eee;
  }

  ${props => props.focusWithKeyboard && css`
    :focus {
      outline: none;
      background-color: #ffbc05;
      box-shadow: 0 0 0 3px rgba(255, 188, 5, 0.265);
    }
  `}
`;

const Button = (props) => {
  const [canFocusWithKeyBoard, toggleCanFocus] = useState(false);
  const toggleFocusStyle = ({ type }) => toggleCanFocus(type === 'keyup');

  return (
    <ButtonWrapper
      {...props}
      focusWithKeyboard={canFocusWithKeyBoard}
      onKeyUp={toggleFocusStyle}
      onMouseDown={toggleFocusStyle}
    />
  );
};

export default Button;
