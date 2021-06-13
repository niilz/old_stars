import { User } from '../../model/User';
import { deleteUser } from '../../services/user-service';
import { Header } from '../header/Header';
import { RegistrationForm } from '../registration-form/RegistrationForm';
import { UserList } from '../user-list/UserList';
import styles from './AdminConsole.module.css';

interface AdminConsoleProps {
  navToHome: () => void;
  users: User[];
  onUsers: (users: User[]) => void;
}

export function AdminConsole(props: AdminConsoleProps) {
  const deleteUserFromList = (id: number) => {
    deleteUser(id);
    const updatedUsers = props.users.filter((user) => user['id'] !== id);
    props.onUsers(updatedUsers);
  };
  return (
    <div className={styles.AdminConsole}>
      <Header showLogo={true} />
      <UserList users={props.users} onDelete={deleteUserFromList} />
      <RegistrationForm
        onNewUser={(newUser: User) => props.onUsers([...props.users, newUser])}
      />
      <button className={styles.homeBtn} onClick={props.navToHome}>
        Home
      </button>
    </div>
  );
}
