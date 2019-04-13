import React, { Component } from 'react';
import { createGlobalStyle } from 'styled-components';

import Background from '../components/Background';
import ContentWrapper from '../components/ContentWrapper';
import Decoration from '../components/Decoration';
import Header from '../components/Header';
import Link from '../components/Link';
import Main from '../components/Main';
import Navigation from '../components/Navigation';
import SiteTitle from '../components/SiteTitle';

const GlobalStyle = createGlobalStyle`
  body {
    font-family: 'Oxygen', 'Helvetica Neue', sans-serif;
  }
`;

class App extends Component {
  render() {
    const { children } = this.props;

    return (
      <React.Fragment>
        <ContentWrapper>
          <Header>
            <SiteTitle>bulletin board</SiteTitle>
            <Navigation>
              <Link to='/read'>read</Link>
              <Link to='/write'>write</Link>
            </Navigation>
            <Decoration />
          </Header>
          <Main>
            {children}
          </Main>
          <GlobalStyle />
        </ContentWrapper>
        <Background />
      </React.Fragment>
    );
  }
}

export default App;
