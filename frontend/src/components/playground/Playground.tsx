import { Header } from '../header/Header';
import styles from './Playground.module.css';
import { User } from '../../model/User';
import { UserList } from '../user-list/UserList';
import { HomeButton } from '../home-button/HomeButton';

interface PlaygroundProps {
  logout: () => void;
  users: User[];
}

export function Playground(props: PlaygroundProps) {
  return (
    <div className={styles.Playground}>
      <Header showLogo={true} />
      <UserList users={props.users} onDelete={(id) => null} />
      <HomeButton callback={props.logout} />
    </div>
  );
}

export default Playground;
