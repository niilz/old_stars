import { Header } from '../header/Header';
import styles from './Playground.module.css';
import { OldStar, User } from '../../model/User';
import { UserList } from '../user-list/UserList';
import { Button } from '../button/Button';
import { UserView } from '../user/UserView';
import { Modal } from '../modal/Modal';
import { WaterRoundWarning } from '../waterround-warning/WaterRoundWarning';

interface PlaygroundProps {
  logout: () => void;
  openAdminLogin: () => void;
  user: User;
  users: User[];
  onUserUpdate: (user: User) => void;
  onRefresh: () => void;
}

export function Playground(props: PlaygroundProps) {
  const oldstar = new OldStar(props.user);
  return (
    <>
      {oldstar.needsWaterRound() && (
        <Modal
          children={
            <WaterRoundWarning
              userId={props.user.id}
              onWaterConsumed={props.onUserUpdate}
            />
          }
        />
      )}
      <div className={styles.Playground}>
        <Header showLogo={true} />
        <UserView user={props.user} onUserUpdate={props.onUserUpdate} />
        <UserList users={props.users} isEditable={false} />
        <div className={styles.buttons}>
          <Button text="logout" styles={styles.Btn} callback={props.logout} />
          <Button
            text="🔄"
            styles={styles.refresh}
            callback={props.onRefresh}
          />
        </div>

        <Button
          text="admin"
          styles={styles.Btn}
          callback={props.openAdminLogin}
        />
      </div>
    </>
  );
}
