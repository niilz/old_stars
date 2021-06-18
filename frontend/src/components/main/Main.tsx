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
  loginType: LoginType;
  setLoginType: (loginType: LoginType) => void;
  loginState: LoginState;
  setLoginState: (loginState: LoginState) => void;
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
  const handleAdminHomeClick = () => {
    props.setAdminView(false);
    props.setLoginType(props.sessionUser ? LoginType.None : LoginType.User);
    props.setLoginState(
      props.sessionUser ? LoginState.LoggedInUser : LoginState.LoggedInClub
    );
  };
  return (
    <div className={styles.Main}>
      {!props.isAdminView ? (
        <>
          {showBigHeaderAndStar(props) && (
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
          {showPlayground(props) && props.sessionUser ? (
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

function showBigHeaderAndStar(props: MainProps) {
  return !props.isAdminView && props.loginState !== LoginState.LoggedInUser;
}

function showPlayground(props: MainProps) {
  return props.loginState === LoginState.LoggedInUser && props.sessionUser;
}
