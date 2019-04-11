import React from 'react';

const Background = () => {
  return (
    <svg style={{ position: 'fixed', bottom: 0, zIndex: -2 }} xmlns='http://www.w3.org/2000/svg' viewBox='0 0 1920 725'>
      <path fill='#fff' d='M0 0h1920v725H0z'/>
      <path d='M1920.001 727h-244.233L.001 309.184V0l1920 478.71V727z' fill='#ffbc05'/>
      <path d='M1824.242 726.835H.001V272l1824.24 454.834z'/>
    </svg>
  );
};

export default Background;
