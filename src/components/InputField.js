import React from 'react';
import styled from 'styled-components';

import FormGroup from './FormGroup';

const Input = styled.input`
  border: unset;
  border-bottom: 1px solid black;
  padding: 0.5em;

  :focus {
    outline: 2px solid #ffbc05;
    box-shadow: 0 0 0 3px rgba(255, 188, 5, 0.265);
  }
`;

const InputField = (props) => {
  const { label, autofocus, onChange, name } = props;

  return (
    <FormGroup label={label}>
      <Input
        autoFocus={autofocus}
        maxLength='50'
        name={name}
        onChange={onChange}
        placeholder='Max 50 characters'
        type='text' />
    </FormGroup>
  );
};

export default InputField;
