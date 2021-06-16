import { useState } from 'react';
import { User } from '../../model/User';
import { deleteUser } from '../../services/user-service';
import { Button } from '../button/Button';
import { Header } from '../header/Header';
import { Message, MsgType } from '../message/Message';
import { UserList } from '../user-list/UserList';
import styles from './AdminConsole.module.css';

interface AdminConsoleProps {
  navToHome: () => void;
  users: User[];
  onDelete: (id: Number) => void;
}

export function AdminConsole(props: AdminConsoleProps) {
  const [message, setMessage] = useState('');
  const [type, setType] = useState(MsgType.NONE);

  const handleError = (msgType: MsgType, msg: string) => {
    setMessage(msg);
    setType(msgType);
  };

  const deleteUserFromList = (id: number) => {
    deleteUser(id);
    props.onDelete(id);
  };

  return (
    <div className={styles.AdminConsole}>
      <Header showLogo={true} />
      <UserList
        users={props.users}
        isEditable={true}
        onDelete={deleteUserFromList}
      />
      <Message msg={message} type={type} />
      <Button text="Home" styles={styles.Btn} callback={props.navToHome} />
    </div>
  );
}
