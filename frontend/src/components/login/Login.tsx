import React, { useState } from 'react';
import AuthService from '../../services/auth-service';

type LoginProps = {
  login: (loginGranted: LoginState) => void;
};

export enum LoginState {
  LoggedIn,
  LoggedOut,
  LoginError,
}

function Login(props: LoginProps) {
  const [password, setPassword] = useState('');

  const handleLogin = (e: React.MouseEvent) => {
    e.preventDefault();
    AuthService.checkPassword(password).then((passwordOk) =>
      props.login(passwordOk ? LoginState.LoggedIn : LoginState.LoginError)
    );
  };

  return (
    <form>
      <input
        //type="password"
        type="text"
        placeholder="Master Passwort"
        onChange={(e) => setPassword(e.target.value)}
      />
      <button onClick={handleLogin}>Login</button>
    </form>
  );
}

export default Login;
