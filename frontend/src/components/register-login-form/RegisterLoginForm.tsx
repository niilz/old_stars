import React, { useState } from 'react';
import { User } from '../../model/User';
import AuthService from '../../services/auth-service';
import { insertUser } from '../../services/user-service';
import { Button } from '../button/Button';
import { LoginState } from '../login/Login';
import { MsgType } from '../message/Message';
import styles from './RegisterLoginForm.module.css';

interface RegisterLoginFormProps {
  onRegister: (user: User) => void;
  onLogin?: (loginGranted: LoginState) => void;
  isUserLogin: boolean;
  setSessionUser?: (user: User) => void;
  btnCallback?: () => void;
  isAdminView: boolean;
  styles?: string;
  onError: (type: MsgType, msg: string) => void;
}

export function RegisterLoginForm(props: RegisterLoginFormProps) {
  const [userName, setUserName] = useState('');
  const [pwd, setPwd] = useState('');

  const handleRegister = async () => {
    try {
      const newUser = await insertUser({ name: userName, pwd });
      props.onRegister(newUser as User);
      setUserName('');
      setPwd('');
    } catch (e) {
      props.onError(MsgType.ERR, e);
    }
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
        props.setSessionUser(loggedInUser as User);
      })
      .catch((e) => props.onError(MsgType.ERR, e));
  };
  return (
    <>
      <form
        onSubmit={preventFormSubmission}
        className={`${styles.RegisterLoginForm} ${
          props.isUserLogin ? props.styles : ''
        }`}
      >
        {(props.isUserLogin || props.isAdminView) && (
          <input
            type="text"
            placeholder="user-name"
            value={userName}
            onChange={(e) => setUserName(e.target.value)}
          />
        )}
        <input
          //type="password"
          type="text"
          value={pwd}
          placeholder="password"
          onChange={(e) => setPwd(e.target.value)}
        />
        {!props.isAdminView && (
          <Button
            text="Login"
            styles={styles.registerBtn}
            callback={handleLogin}
          />
        )}
        {(props.isUserLogin || props.isAdminView) && (
          <Button
            text={`${props.isUserLogin ? 'Register' : 'Save'}`}
            styles={styles.registerBtn}
            callback={handleRegister}
          />
        )}
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
