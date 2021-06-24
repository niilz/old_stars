import { useContext, useState } from 'react';
import { LoginState } from '../../Constants';
import { User } from '../../model/User';
import { UserContext } from '../main/Main';
import { Message, MsgType } from '../message/Message';
import { RegisterLoginForm } from '../register-login-form/RegisterLoginForm';
import styles from './Login.module.css';

type LoginProps = {
  onLogin: (ls: LoginState) => void;
};

export function Login(props: LoginProps) {
  const [message, setMessage] = useState('');
  const [msgType, setMsgType] = useState(MsgType.NONE);

  const { addUser } = useContext(UserContext);

  const handleError = (msgType: MsgType, msg: string) => {
    setMessage(msg);
    setMsgType(msgType);
  };

  const handleLogin = (loginState: LoginState) => {
    if (loginState !== LoginState.LoginError) {
      setMessage('');
      setMsgType(MsgType.NONE);
    }
  };

  const handleRegister = (user: User) => {
    setMessage('Registration was successful');
    setMsgType(MsgType.INFO);
    addUser(user);
  };

  return (
    <>
      <Message msg={message} type={msgType} />
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
