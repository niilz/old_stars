import { useContext } from 'react';
import { Button } from '../components/button/Button';
import { Header } from '../components/header/Header';
import { UserList } from '../components/user-list/UserList';
import { User } from '../model/User';
import styles from './OneHistoryView.module.css';
import { ViewContext } from '../context/Contexts';
import { View } from './View';
import { DateAndTime } from './ArchiveView';

interface OneHistoryViewProps {
  dateAndTime: DateAndTime;
  users: User[];
}
export function OneHistoryView(props: OneHistoryViewProps) {
  const { setActiveView } = useContext(ViewContext);

  const { date, time } = props.dateAndTime;

  return (
    <div className={styles.OneHistory}>
      <Header showLogo={true} />
      <h1>Historisches Ergebnis</h1>
      <h2>{`${date} / ${time}`}</h2>
      <UserList users={props.users} isEditable={false} />
      <Button
        text="back"
        styles={styles.Btn}
        callback={() => setActiveView(View.Histories)}
      />
      <Button
        text="Home"
        styles={styles.Btn}
        callback={() => setActiveView(View.Playground)}
      />
    </div>
  );
}
