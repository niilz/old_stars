import { User } from '../../model/User';
import { ApiResponse, handleResponse } from '../../services/fetch-service';
import { deleteUser } from '../../services/user-service';
import { Button } from '../button/Button';
import { Header } from '../header/Header';
import { UserList } from '../user-list/UserList';
import styles from './AdminConsole.module.css';

interface AdminConsoleProps {
  navToHome: () => void;
  users: User[];
  onDelete: (voidResult: Promise<ApiResponse>) => void;
}

export function AdminConsole(props: AdminConsoleProps) {
  const deleteUserFromList = (id: number) => {
    const voidResult = deleteUser(id);
    props.onDelete(voidResult);
  };

  return (
    <div className={styles.AdminConsole}>
      <Header showLogo={true} />
      <UserList
        users={props.users}
        isEditable={true}
        onDelete={deleteUserFromList}
      />
      <Button text="Home" styles={styles.Btn} callback={props.navToHome} />
    </div>
  );
}
