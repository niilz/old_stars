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
  isUserLogin: boolean;
  setSessionUser?: (user: User) => void;
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
    AuthService.loginUser({
      name: !props.isUserLogin ? 'master' : userName,
      pwd: pwd,
    })
      .then((loggedInUser) => {
        if (!props.onLogin || !props.setSessionUser)
          throw 'login- and setSessionUser callback must be defined';
        const loginState = evalLoginState(props.isUserLogin);
        setUserName('');
        setPwd('');
        props.onLogin(loginState);
        props.setSessionUser(loggedInUser);
      })
      .catch((e) => console.error(e));
  };
  return (
    <>
      <form
        onSubmit={preventFormSubmission}
        className={`${styles.RegisterLoginForm} ${
          props.isUserLogin ? props.styles : ''
        }`}
      >
        {props.isUserLogin ? (
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
        {props.isUserLogin || props.isAdminView ? (
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

function evalLoginState(isUserLogin: boolean) {
  if (isUserLogin) {
    return LoginState.LoggedInUser;
  } else {
    return LoginState.LoggedInMaster;
  }
}
