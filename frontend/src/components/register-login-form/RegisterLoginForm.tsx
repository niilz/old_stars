import React, { useState } from 'react';
import { User } from '../../model/User';
import AuthService from '../../services/auth-service';
import { insertUser } from '../../services/user-service';
import { Button } from '../button/Button';
import { LoginState, LoginType } from '../login/Login';
import { MsgType } from '../message/Message';
import styles from './RegisterLoginForm.module.css';

interface RegisterLoginFormProps {
  loginType: LoginType;
  onRegister: (user: User) => void;
  onLogin: (loginGranted: LoginState) => void;
  setSessionUser?: (user: User) => void;
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
      name: evalLoginName(props.loginType, userName),
      pwd: pwd,
    })
      .then((loggedInUser) => {
        const loginState = evalLoginState(props.loginType);
        setUserName('');
        setPwd('');
        props.onLogin(loginState);
        if (props.loginType !== LoginType.User) return;
        if (!props.setSessionUser)
          throw 'login- and setSessionUser callback must be defined';
        props.setSessionUser(loggedInUser as User);
      })
      .catch((e) => props.onError(MsgType.ERR, e));
  };
  console.log(props.loginType);
  return (
    <>
      <form
        onSubmit={preventFormSubmission}
        className={`${styles.RegisterLoginForm} ${
          props.loginType == LoginType.User ? props.styles : ''
        }`}
      >
        {props.loginType === LoginType.User && (
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
        <Button
          text="Login"
          styles={styles.registerBtn}
          callback={handleLogin}
        />
        {props.loginType === LoginType.User && (
          <Button
            text="Register"
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

function evalLoginState(loginType: LoginType) {
  switch (loginType) {
    case LoginType.User:
      return LoginState.LoggedInUser;
    case LoginType.Master:
      return LoginState.LoggedInMaster;
    case LoginType.Admin:
      return LoginState.LoggedInAdmin;
    default:
      throw `Unhandled loginTyp: ${loginType}. Cannot evaluate a LoginState`;
  }
}

function evalLoginName(loginType: LoginType, userName: string) {
  switch (loginType) {
    case LoginType.User:
      return userName;
    case LoginType.Master:
      return 'master';
    case LoginType.Admin:
      return 'admin';
    default:
      throw 'Cannot evaluate the Login Name if the userName is undefined';
  }
}
