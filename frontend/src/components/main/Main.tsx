import { useContext } from 'react';
import { ContextType, LoginContext } from '../../App';
import { User } from '../../model/User';
import { AdminConsole } from '../admin/AdminConsole';
import { Button } from '../button/Button';
import { Header } from '../header/Header';
import { Login, LoginState, LoginType } from '../login/Login';
import { AppLogo } from '../logo/Logo';
import { Playground } from '../playground/Playground';
import styles from './Main.module.css';

interface MainProps {
  onLogin: (state: LoginState) => void;
  sessionUser: User | undefined;
  setSessionUser: (user: User) => void;
  onRegister: (user: User) => void;
  users: User[];
  deleteUser: (id: Number) => void;
  openAdminLogin: (flag: boolean) => void;
  isAdminView: boolean;
  setAdminView: (flag: boolean) => void;
  onLogout: () => void;
  onUserUpdate: (user: User) => void;
}

export function Main(props: MainProps) {
  const { setLoginType, loginState, setLoginState } = useContext(LoginContext);
  const handleAdminHomeClick = () => {
    props.setAdminView(false);
    setLoginType(props.sessionUser ? LoginType.None : LoginType.User);
    setLoginState(
      props.sessionUser ? LoginState.LoggedInUser : LoginState.LoggedInClub
    );
  };
  return (
    <div className={styles.Main}>
      {!props.isAdminView ? (
        <>
          {showBigHeaderAndStar(props.isAdminView, loginState) && (
            <>
              <Header
                showLogo={false}
                styles={{
                  headerStripes: styles.headerStripes,
                  title: styles.title,
                }}
              />
              <AppLogo styles={styles.logo} />
            </>
          )}
          {props.sessionUser &&
          showPlayground(loginState, props.sessionUser) ? (
            <Playground
              user={props.sessionUser}
              users={props.users}
              logout={props.onLogout}
              onUserUpdate={props.onUserUpdate}
            />
          ) : (
            <Login {...props} />
          )}
          {!props.isAdminView && (
            <Button
              text="admin"
              styles={styles.Btn}
              callback={() => props.openAdminLogin(true)}
            />
          )}
        </>
      ) : (
        <AdminConsole
          navToHome={handleAdminHomeClick}
          users={props.users}
          onDelete={props.deleteUser}
        />
      )}
    </div>
  );
}

function showBigHeaderAndStar(isAdminView: boolean, ls: LoginState) {
  return !isAdminView && ls !== LoginState.LoggedInUser;
}

function showPlayground(ls: LoginState, sessionUser: User) {
  return ls === LoginState.LoggedInUser && sessionUser;
}
