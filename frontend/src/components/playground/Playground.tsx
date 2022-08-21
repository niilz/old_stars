import { Header } from '../header/Header';
import styles from './Playground.module.css';
import { User } from '../../model/User';
import { UserList } from '../user-list/UserList';
import { Button } from '../button/Button';
import { UserView } from '../user/UserView';

interface PlaygroundProps {
  user: User;
  logout: () => void;
  users: User[];
  onUserUpdate: (user: User) => void;
  onRefresh: () => void;
}

export function Playground(props: PlaygroundProps) {
  return (
    <div className={styles.Playground}>
      <Header showLogo={true} />
      <UserView user={props.user} onUserUpdate={props.onUserUpdate} />
      <UserList users={props.users} isEditable={false} />
      <div className={styles.buttons}>
        <Button text="logout" styles={styles.Btn} callback={props.logout} />
        <Button text="🔄" styles={styles.refresh} callback={props.onRefresh} />
      </div>
    </div>
  );
}
