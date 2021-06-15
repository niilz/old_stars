import { useState } from 'react';
import { User } from '../../model/User';
import { Message, MsgType } from '../message/Message';
import { RegisterLoginForm } from '../register-login-form/RegisterLoginForm';
import styles from './Login.module.css';

type LoginProps = {
  isUserLogin: boolean;
  onRegister: (user: User) => void;
  onLogin: (loginState: LoginState) => void;
  setSessionUser: (user: User) => void;
  isAdminView: boolean;
};

export enum LoginState {
  LoggedInMaster,
  LoggedInUser,
  LoggedOut,
  LoginError,
}

export function Login(props: LoginProps) {
  const [message, setMessage] = useState('');
  const [type, setType] = useState(MsgType.NONE);

  const handleError = (msgType: MsgType, msg: string) => {
    setMessage(msg);
    setType(msgType);
  };

  const handleLogin = (loginState: LoginState) => {
    if (loginState !== LoginState.LoginError) {
      setMessage('');
      setType(MsgType.NONE);
    }
    props.onLogin(loginState);
  };

  const handleRegister = (user: User) => {
    setMessage('Registration was successful');
    setType(MsgType.INFO);
    props.onRegister(user);
  };

  return (
    <>
      <Message msg={message} type={type} />
      <RegisterLoginForm
        isUserLogin={props.isUserLogin}
        onRegister={handleRegister}
        onLogin={handleLogin}
        onError={handleError}
        setSessionUser={props.setSessionUser}
        isAdminView={props.isAdminView}
        styles={styles.LoginForm}
      />
    </>
  );
}
