import React from 'react';

import styled from 'styled-components';

const MessageWrapper = styled.div`
  margin-top: 1em;
  padding: 0.5em;
  background-color: #eee;
`;

const Message = (props) => {
  const { error } = props;

  if (error) {
    return (
      <MessageWrapper>
        Potentially, things are broken. Yell at your screen and try again.
      </MessageWrapper>
    );
  }

  return null;
};

export default Message;
