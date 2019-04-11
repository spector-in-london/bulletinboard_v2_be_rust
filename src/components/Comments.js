import React from 'react';
import styled from 'styled-components';

import Comment from './Comment';

const NoCommentsMessage = styled.div`
  margin-bottom: 1em;
`;

const Comments = (props) => {
  const { comments } = props;

  if (comments.length === 0) {
    return (
      <NoCommentsMessage>Nothing yet, be the first!</NoCommentsMessage>
    );
  }

  return (
    <div>
      {comments.map(comment =>
        <Comment key={comment.id} {...comment} />
      )}
    </div>
  );
};

export default Comments;
