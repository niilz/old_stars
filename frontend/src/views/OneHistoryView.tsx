import { useContext } from 'react';
import { Button } from '../components/button/Button';
import { Header } from '../components/header/Header';
import { UserList } from '../components/user-list/UserList';
import { User } from '../model/User';
import styles from './OneHistoryView.module.css';
import { ViewContext } from '../context/Contexts';
import { View } from './View';

interface OneHistoryViewProps {
  users: User[];
}
export function OneHistoryView(props: OneHistoryViewProps) {
  const { setActiveView } = useContext(ViewContext);

  return (
    <div className={styles.OneHistory}>
      <Header showLogo={true} />
      <UserList users={props.users} isEditable={false} />
      <Button
        text="Home"
        styles={styles.Btn}
        callback={() => setActiveView(View.Playground)}
      />
    </div>
  );
}
