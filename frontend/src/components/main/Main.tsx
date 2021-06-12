import { Header } from '../header/Header';
import Login, { LoginState } from '../login/Login';
import styles from './Main.module.css';

interface MainProps {
  setLoginState: (state: LoginState) => void;
  styles: string;
}

export function Main(props: MainProps) {
  {
    /*<img src={logo} className="App-logo" alt="logo" />*/
  }
  return (
    <div className={styles.Main}>
      <Header />
      <Login styles={props.styles} login={props.setLoginState} />
    </div>
  );
}
