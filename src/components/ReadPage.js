import React, { Component } from 'react';

import Comments from './Comments';
import LoadMoreButton from './LoadMoreButton';
import Message from './Message';
import SortButton from './SortButton';

class ReadPage extends Component {
  render() {
    const { comments, hasError, onChangeSort, onLoadMore, sortOrder } = this.props;

    return (
      <>
        <SortButton onClick={onChangeSort}>
          {sortOrder === 'desc'? 'sort: newest first' : 'sort: oldest first'}
        </SortButton>
        <Comments comments={comments} />
        <LoadMoreButton onClick={onLoadMore}>load more...</LoadMoreButton>
        {hasError && <Message error />}
      </>
    );
  }
}

export default ReadPage;
