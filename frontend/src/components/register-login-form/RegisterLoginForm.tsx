import React, { useState } from 'react';
import { User } from '../../model/User';
import AuthService from '../../services/auth-service';
import { insertUser } from '../../services/user-service';
import { Button } from '../button/Button';
import { LoginState } from '../login/Login';
import styles from './RegisterLoginForm.module.css';

interface RegisterLoginFormProps {
  onRegister: (user: User) => void;
  onLogin?: (loginGranted: LoginState) => void;
  isMasterLogin: boolean;
  btnCallback?: () => void;
  isAdminView: boolean;
  styles?: string;
}

export function RegisterLoginForm(props: RegisterLoginFormProps) {
  const [userName, setUserName] = useState('');
  const [pwd, setPwd] = useState('');

  const handleRegister = async () => {
    const newUser = await insertUser({ name: userName, pwd });
    console.log('registerd The User:', newUser);
    props.onRegister(newUser);
    setUserName('');
    setPwd('');
  };

  const handleLogin = () => {
    AuthService.checkPassword({
      name: props.isMasterLogin ? 'master' : userName,
      pwd: pwd,
    }).then((wasLoginSuccessful) => {
      if (props.onLogin == undefined) {
        throw 'Trying to login without having a login callback defined';
      }
      console.log('THE LOGIN-STATE (wasLoginSuccessful): ', wasLoginSuccessful);
      const loginState = evalLoginState(
        wasLoginSuccessful,
        props.isMasterLogin
      );
      props.onLogin(loginState);
    });
  };
  return (
    <>
      <form
        onSubmit={preventFormSubmission}
        className={`${styles.RegisterLoginForm} ${
          !props.isAdminView ? props.styles : ''
        }`}
      >
        {!props.isMasterLogin ? (
          <input
            type="text"
            placeholder="user-name"
            value={userName}
            onChange={(e) => setUserName(e.target.value)}
          />
        ) : null}
        <input
          //type="password"
          type="text"
          value={pwd}
          placeholder="password"
          onChange={(e) => setPwd(e.target.value)}
        />
        {!props.isAdminView ? (
          <Button
            text="Login"
            styles={styles.registerBtn}
            callback={handleLogin}
          />
        ) : null}
        {!props.isMasterLogin ? (
          <Button
            text="Register"
            styles={styles.registerBtn}
            callback={handleRegister}
          />
        ) : null}
      </form>
    </>
  );
}

function preventFormSubmission(e: React.FormEvent) {
  e.preventDefault();
}

function evalLoginState(wasLoginSuccessful: boolean, isMasterLogin: boolean) {
  if (wasLoginSuccessful) {
    if (isMasterLogin) {
      return LoginState.LoggedInMaster;
    } else {
      return LoginState.LoggedInUser;
    }
  } else {
    return LoginState.LoginError;
  }
}
