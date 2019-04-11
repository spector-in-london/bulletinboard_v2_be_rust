import React from 'react';
import styled from 'styled-components';

import FormGroup from './FormGroup';

const Textarea = styled.textarea`
  border: unset;
  border-bottom: 1px solid black;
  padding: 0.5em;
  resize: none;

  :focus {
    outline: 2px solid #ffbc05;
    box-shadow: 0 0 0 3px rgba(255, 188, 5, 0.265);
  }
`;

const TextareaField = (props) => {
  const { label, autofocus, onChange, name } = props;

  return (
    <FormGroup label={label}>
      <Textarea
        autoFocus={autofocus}
        maxLength='255'
        name={name}
        onChange={onChange}
        placeholder='Max 255 characters'
        rows='6' />
    </FormGroup>
  );
};

export default TextareaField;
