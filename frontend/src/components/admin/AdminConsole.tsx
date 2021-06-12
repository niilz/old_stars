import React, { useEffect, useState } from 'react';
import { getAllUsers } from '../../services/user-service';
import { RegistrationForm } from '../registration-form/RegistrationForm';
import { UserList } from '../user-list/UserList';

type AdminConsoleProps = {
  navToHome: () => void;
};

export function AdminConsole(props: AdminConsoleProps) {
  const [users, setUsers] = useState([]);

  useEffect(() => {
    getAllUsers().then((users) => setUsers(users));
  }, [users]);
  return (
    <>
      <UserList />
      <RegistrationForm />
      <button onClick={props.navToHome}>Home</button>
    </>
  );
}
