import { useEffect, useState } from 'react';
import { User } from '../../model/User';
import { deleteUser, getAllUsers } from '../../services/user-service';
import { Header } from '../header/Header';
import { AppLogo } from '../logo/Logo';
import { RegistrationForm } from '../registration-form/RegistrationForm';
import { UserList } from '../user-list/UserList';
import styles from './AdminConsole.module.css';

type AdminConsoleProps = {
  navToHome: () => void;
};

export function AdminConsole(props: AdminConsoleProps) {
  const [users, setUsers] = useState(new Array<User>());
  useEffect(() => {
    const fetchUsers = async () => {
      const fetchedUsers = await getAllUsers();
      setUsers(fetchedUsers);
    };
    fetchUsers();
  }, []);

  const deleteUserFromList = (id: number) => {
    deleteUser(id);
    const updatedUsers = users.filter((user) => user['id'] !== id);
    setUsers(updatedUsers);
  };

  return (
    <div className={styles.AdminConsole}>
      <div className={styles.adminHeader}>
        <AppLogo styles={styles.AppLogo} />
        <Header />
      </div>
      <UserList users={users} onDelete={deleteUserFromList} />
      <RegistrationForm
        onNewUser={(newUser: User) => setUsers([...users, newUser])}
      />
      <button onClick={props.navToHome}>Home</button>
    </div>
  );
}
