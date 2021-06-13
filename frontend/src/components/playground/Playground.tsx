import { Header } from '../header/Header';
import styles from './Playground.module.css';
import { User } from '../../model/User';
import { UserList } from '../user-list/UserList';
import { Button } from '../button/Button';

interface PlaygroundProps {
  logout: () => void;
  users: User[];
}

export function Playground(props: PlaygroundProps) {
  return (
    <div className={styles.Playground}>
      <Header showLogo={true} />
      <UserList users={props.users} isEditable={false} />
      <Button text="logout" styles={styles.Btn} callback={props.logout} />
    </div>
  );
}
