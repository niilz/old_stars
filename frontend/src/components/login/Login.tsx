import { useContext, useState } from 'react';
import { LoginState } from '../../Constants';
import { User } from '../../model/User';
import { LoginContext, UserContext } from '../main/Main';
import { Message, MsgType } from '../message/Message';
import { RegisterLoginForm } from '../register-login-form/RegisterLoginForm';
import styles from './Login.module.css';

type LoginProps = {
  onLogin: (ls: LoginState) => void;
};

export function Login(props: LoginProps) {
  const [message, setMessage] = useState('');
  const [type, setType] = useState(MsgType.NONE);

  const { setLoginState } = useContext(LoginContext);
  const { addUser } = useContext(UserContext);

  const handleError = (msgType: MsgType, msg: string) => {
    setMessage(msg);
    setType(msgType);
  };

  const handleLogin = (loginState: LoginState) => {
    if (loginState !== LoginState.LoginError) {
      setMessage('');
      setType(MsgType.NONE);
    }
    setLoginState(loginState);
  };

  const handleRegister = (user: User) => {
    setMessage('Registration was successful');
    setType(MsgType.INFO);
    addUser(user);
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
