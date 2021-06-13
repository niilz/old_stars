import React, { useState } from 'react';
import AuthService from '../../services/auth-service';
import { AppLogo } from '../logo/Logo';
import styles from './Login.module.css';

type LoginProps = {
  login: (loginGranted: LoginState) => void;
  styles: string;
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
    AuthService.checkPassword({
      name: 'master',
      pwd: password,
    }).then((loginResponse) => {
      console.log(loginResponse);
      props.login(loginResponse ? LoginState.LoggedIn : LoginState.LoginError);
    });
  };

  return (
    <>
      <AppLogo styles={`${styles.AppLogo} ${props.styles}`} />
      <form className={styles.Login}>
        <input
          //type="password"
          type="text"
          placeholder="Master Passwort"
          onChange={(e) => setPassword(e.target.value)}
        />
        <button onClick={handleLogin}>Login</button>
      </form>
    </>
  );
}

export default Login;
