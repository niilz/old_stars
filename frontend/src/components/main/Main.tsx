import { Header } from '../header/Header';
import Login, { LoginState } from '../login/Login';
import './Main.css';

interface MainProps {
  setLoginState: (state: LoginState) => void;
}

export function Main(props: MainProps) {
  {
    /*<img src={logo} className="App-logo" alt="logo" />*/
  }
  return (
    <div className="Main">
      <Header />
      <Login login={props.setLoginState} />
    </div>
  );
}
