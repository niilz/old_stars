import { useState } from 'react';
import './App.css';
import { LoginState } from './components/login/Login';
import Playground from './components/playground/Playground';
import { AdminConsole } from './components/admin/AdminConsole';
import { Main } from './components/main/Main';

function App() {
  const [loginState, setLoginState] = useState(LoginState.LoggedOut);
  const [showAdmin, setShowAdmin] = useState(false);
  return (
    <div className="App">
      {getMain(loginState, setLoginState, showAdmin, setShowAdmin)}
      <button onClick={() => setShowAdmin(true)}>admin</button>
    </div>
  );
}

export default App;

function getMain(
  loginState: LoginState,
  setLoginState: (lg: LoginState) => void,
  showAdmin: boolean,
  setShowAdmin: (ssa: boolean) => void
): JSX.Element {
  if (showAdmin) {
    return <AdminConsole navToHome={() => setShowAdmin(false)} />;
  }

  switch (loginState) {
    case LoginState.LoggedIn:
      return <Playground />;
    case LoginState.LoggedOut:
      return <Main setLoginState={setLoginState} />;
    case LoginState.LoginError:
      return <div>Das war total falsch!</div>;
    default:
      throw `Unreachabale state: ${loginState}`;
  }
}
