import React, { Component } from 'react';

import SubmitButton from '../components/SubmitButton';
import InputField from '../components/InputField';
import Message from '../components/Message';
import TextareaField from '../components/TextareaField';

class WritePage extends Component {
  render() {
    const { onChange, onSubmit, isBusy, hasError } = this.props;

    return (
      <form onSubmit={onSubmit}>
        <InputField
          autofocus
          label='Your Name'
          name='name'
          onChange={onChange} />
        <InputField
          label='A Title for Your Comment'
          name='title'
          onChange={onChange} />
        <TextareaField
          label='Your Comment'
          name='body'
          onChange={onChange} />
        <SubmitButton
          disabled={isBusy}
          type='submit'>
          Submit
        </SubmitButton>
        {hasError && <Message error/>}
      </form>
    );
  }
}

export default WritePage;
