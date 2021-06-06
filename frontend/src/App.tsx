import React, { useState } from 'react';
import logo from './logo.svg';
import './App.css';
import Login, { LoginState } from './components/login/Login';
import Playground from './components/playground/Playground';

function App() {
  const [loginState, setLoginState] = useState(LoginState.LoggedOut);
  return (
    <div className="App">
      <header className="App-header">Old-Stars App</header>
      <img src={logo} className="App-logo" alt="logo" />
      {getMain(loginState, setLoginState)}
    </div>
  );
}

export default App;

function getMain(
  loginState: LoginState,
  setLoginState: (lg: LoginState) => void
): JSX.Element {
  switch (loginState) {
    case LoginState.LoggedIn:
      return <Playground />;
    case LoginState.LoggedOut:
      return <Login login={setLoginState} />;
    case LoginState.LoginError:
      return <div>Das war total falsch!</div>;
    default:
      throw `Unreachabale state: ${loginState}`;
  }
}
