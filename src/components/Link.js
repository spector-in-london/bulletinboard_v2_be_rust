import styled from 'styled-components';

import { Link as RouterLink } from 'react-router-dom';

const Link = styled(RouterLink)`
  color: unset;
  text-decoration: none;
  position: relative;

  &:hover {
    color: #ffbc05;
  }

  &:not(:last-child) {
    margin-right: 8px;
  }

  :focus {
    outline: none;
    border-bottom: 2px solid #ffbc05;

    &:before {
      content: '';
      position: absolute;
      top: 2px;
      left: -4px;
      right: -4px;
      bottom: -4px;
      background: white;
      opacity: 0.5;
      z-index: -1
    }
  }
`;

export default Link;
