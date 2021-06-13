import { User } from '../../model/User';
import { deleteUser } from '../../services/user-service';
import { Header } from '../header/Header';
import { AppLogo } from '../logo/Logo';
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
      <div className={styles.adminHeader}>
        <AppLogo styles={styles.AppLogo} />
        <Header />
      </div>
      <UserList users={props.users} onDelete={deleteUserFromList} />
      <RegistrationForm
        onNewUser={(newUser: User) => props.onUsers([...props.users, newUser])}
      />
      <button onClick={props.navToHome}>Home</button>
    </div>
  );
}
