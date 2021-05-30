import React, { useState } from 'react';
import logo from './logo.svg';
import './App.css';
import Login from './components/login/Login';
import Playground from './components/playground/Playground';

function App() {
  const [isLoggedIn, setLoggedIn] = useState(false);
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        {isLoggedIn ? <Playground /> : <Login login={setLoggedIn} />}
      </header>
    </div>
  );
}

export default App;
