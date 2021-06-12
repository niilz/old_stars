import { useState } from 'react';
import { LoginState } from './components/login/Login';
import Playground from './components/playground/Playground';
import { AdminConsole } from './components/admin/AdminConsole';
import { Main } from './components/main/Main';
import styles from './App.module.css';
import './global.css';

function App() {
  const [loginState, setLoginState] = useState(LoginState.LoggedOut);
  const [showAdmin, setShowAdmin] = useState(false);
  return (
    <>
      {getMain(loginState, setLoginState, showAdmin, setShowAdmin)}
      <button onClick={() => setShowAdmin(true)}>admin</button>
    </>
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
      return <Main styles={styles.AppLogo} setLoginState={setLoginState} />;
    case LoginState.LoginError:
      return <div>Das war total falsch!</div>;
    default:
      throw `Unreachabale state: ${loginState}`;
  }
}
