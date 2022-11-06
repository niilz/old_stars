import React, { useContext, useState } from 'react';
import { AppCtx } from '../../App';
import { LoginState, LoginType } from '../../Constants';
import { User } from '../../model/User';
import AuthService from '../../services/auth-service';
import { insertUser } from '../../services/user-service';
import { Button } from '../button/Button';
import { UserContext } from '../main/Main';
import { MsgType } from '../message/Message';
import styles from './RegisterLoginForm.module.css';

interface RegisterLoginFormProps {
  onRegister: (user: User) => void;
  onLogin: (loginGranted: LoginState) => void;
  styles?: string;
  onError: (type: MsgType, msg: string) => void;
}

export function RegisterLoginForm(props: RegisterLoginFormProps) {
  const { loginType, setLoginType } = useContext(AppCtx);
  const { setSessionUser } = useContext(UserContext);

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
      name: userName || evalLoginName(loginType),
      pwd: pwd,
    })
      .then((loggedInUser) => {
        const loginState = evalLoginState(loginType);
        props.onLogin(loginState);
        setUserName('');
        setPwd('');
        setLoginType(evalLoginType(loginType));
        if (loginType !== LoginType.User) return;
        setSessionUser(loggedInUser as User);
      })
      .catch((e) => props.onError(MsgType.ERR, e));
  };

  return (
    <>
      <form
        onSubmit={preventFormSubmission}
        className={`${styles.RegisterLoginForm} ${
          loginType === LoginType.User ? props.styles : ''
        }`}
      >
        {loginType === LoginType.User && (
          <input
            type="text"
            placeholder="user-name"
            value={userName}
            onChange={(e) => setUserName(e.target.value)}
          />
        )}
        <input
          type="password"
          value={pwd}
          placeholder={getPwdPlaceholder(loginType)}
          onChange={(e) => setPwd(e.target.value)}
        />
        <Button
          text="Login"
          styles={styles.registerBtn}
          callback={handleLogin}
        />
        {loginType === LoginType.User && (
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
    case LoginType.Club:
      return LoginState.LoggedInClub;
    case LoginType.Admin:
      return LoginState.LoggedInAdmin;
    default:
      throw `Unhandled loginTyp: ${loginType}. Cannot evaluate a LoginState`;
  }
}

function evalLoginName(loginType: LoginType) {
  switch (loginType) {
    case LoginType.Club:
      return 'club';
    case LoginType.Admin:
      return 'admin!';
    default:
      throw 'Cannot evaluate the Login Name if the userName is undefined';
  }
}

function evalLoginType(prevLoginType: LoginType) {
  switch (prevLoginType) {
    case LoginType.Club:
      return LoginType.User;
    case LoginType.User:
      return LoginType.None;
    case LoginType.Admin:
      return LoginType.None;
    default:
      throw 'Unsupported LoginType-State';
  }
}

function getPwdPlaceholder(loginType: LoginType) {
  return `${LoginType[loginType]} Password`;
}
