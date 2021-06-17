import { useState } from 'react';
import { User } from '../../model/User';
import { Message, MsgType } from '../message/Message';
import { RegisterLoginForm } from '../register-login-form/RegisterLoginForm';
import styles from './Login.module.css';

type LoginProps = {
  loginType: LoginType;
  setLoginType: (loginType: LoginType) => void;
  onLogin: (loginState: LoginState) => void;
  onRegister?: (user: User) => void;
  setSessionUser?: (user: User) => void;
};

export enum LoginState {
  LoggedInMaster,
  LoggedInAdmin,
  LoggedInUser,
  LoggedOut,
  LoginError,
}

export enum LoginType {
  Master,
  User,
  Admin,
  None,
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
    console.log('handle Login in Login got called');
    props.onLogin(loginState);
  };

  const handleRegister = (user: User) => {
    if (!props.onRegister)
      throw 'onRegister must be defined to register a User';
    setMessage('Registration was successful');
    setType(MsgType.INFO);
    props.onRegister(user);
  };

  return (
    <>
      <Message msg={message} type={type} />
      <RegisterLoginForm
        {...props}
        onRegister={handleRegister}
        onLogin={handleLogin}
        onError={handleError}
        styles={styles.LoginForm}
      />
    </>
  );
}
