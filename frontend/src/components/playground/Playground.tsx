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
}

export function Playground(props: PlaygroundProps) {
  console.log('playground', props.user);
  return (
    <div className={styles.Playground}>
      <Header showLogo={true} />
      <UserView user={props.user} />
      <UserList users={props.users} isEditable={false} />
      <Button text="logout" styles={styles.Btn} callback={props.logout} />
    </div>
  );
}
