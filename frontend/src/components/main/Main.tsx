import { Header } from '../header/Header';
import Login, { LoginState } from '../login/Login';
import { AppLogo } from '../logo/Logo';
import styles from './Main.module.css';

interface MainProps {
  setLoginState: (state: LoginState) => void;
  showAdmin: boolean;
  setShowAdmin: (flag: boolean) => void;
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
      <Login login={props.setLoginState} />
      {!props.showAdmin ? (
        <button onClick={() => props.setShowAdmin(true)}>admin</button>
      ) : null}
    </div>
  );
}
