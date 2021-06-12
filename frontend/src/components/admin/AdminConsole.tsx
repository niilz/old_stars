import { useEffect, useState } from 'react';
import { User } from '../../model/User';
import { deleteUser, getAllUsers } from '../../services/user-service';
import { Header } from '../header/Header';
import { AppLogo } from '../logo/Logo';
import { RegistrationForm } from '../registration-form/RegistrationForm';
import { UserList } from '../user-list/UserList';
import './AdminConsole.css';

type AdminConsoleProps = {
  navToHome: () => void;
};

export function AdminConsole(props: AdminConsoleProps) {
  const [users, setUsers] = useState(new Array<User>());
  useEffect(() => {
    const fetchUsers = async () => {
      const fetchedUsers = await getAllUsers();
      console.log('fetchedUsers:', fetchedUsers);
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
    <>
      <div className="admin-header">
        <AppLogo addClass="small-logo" />
        <Header />
      </div>
      <UserList users={users} onDelete={deleteUserFromList} />
      <RegistrationForm
        onNewUser={(newUser: User) => setUsers([...users, newUser])}
      />
      <button onClick={props.navToHome}>Home</button>
    </>
  );
}
