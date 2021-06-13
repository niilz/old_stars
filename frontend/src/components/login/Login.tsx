import { User } from '../../model/User';
import { RegisterLoginForm } from '../register-login-form/RegisterLoginForm';

type LoginProps = {
  isMasterLogin: boolean;
  onRegister: (user: User) => void;
  onLogin: (loginState: LoginState) => void;
  isAdminView: boolean;
};

export enum LoginState {
  LoggedInMaster,
  LoggedInUser,
  LoggedOut,
  LoginError,
}

export function Login(props: LoginProps) {
  return (
    <RegisterLoginForm
      isMasterLogin={props.isMasterLogin}
      onRegister={props.onRegister}
      onLogin={props.onLogin}
      isAdminView={props.isAdminView}
    />
  );
}
