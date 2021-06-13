import { User } from '../../model/User';
import { Button } from '../button/Button';
import { Header } from '../header/Header';
import { Login, LoginState } from '../login/Login';
import { AppLogo } from '../logo/Logo';
import styles from './Main.module.css';

interface MainProps {
  onLogin: (state: LoginState) => void;
  isAdminView: boolean;
  setAdminView: (flag: boolean) => void;
  isUserLogin: boolean;
  onRegister: (user: User) => void;
}

export function Main(props: MainProps) {
  {
    /*<img src={logo} className="App-logo" alt="logo" />*/
  }
  return (
    <div className={styles.Main}>
      <Header
        showLogo={false}
        styles={{ headerStripes: styles.headerStripes, title: styles.title }}
      />
      <AppLogo styles={styles.logo} />
      <Login
        isUserLogin={props.isUserLogin}
        onRegister={props.onRegister}
        onLogin={props.onLogin}
        isAdminView={props.isAdminView}
      />
      {!props.isAdminView ? (
        <Button
          text="admin"
          styles={styles.Btn}
          callback={() => props.setAdminView(true)}
        />
      ) : null}
    </div>
  );
}
