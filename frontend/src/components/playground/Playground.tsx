import { Header } from '../header/Header';
import styles from './Playground.module.css';
import { User } from '../../model/User';
import { UserList } from '../user-list/UserList';

interface PlaygroundProps {
  users: User[];
}

export function Playground(props: PlaygroundProps) {
  return (
    <div className={styles.Playground}>
      <Header showLogo={true} />
      <UserList users={props.users} onDelete={(id) => null} />
    </div>
  );
}

export default Playground;
