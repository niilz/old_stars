import { useEffect, useState } from 'react';
import { LoginState } from './components/login/Login';
import Playground from './components/playground/Playground';
import { AdminConsole } from './components/admin/AdminConsole';
import { Main } from './components/main/Main';
import styles from './App.module.css';
import './global.css';
import { User } from './model/User';
import { deleteUser, getAllUsers } from './services/user-service';

function App() {
  const [loginState, setLoginState] = useState(LoginState.LoggedOut);
  const [showAdmin, setShowAdmin] = useState(false);
  const [users, setUsers] = useState(new Array<User>());
  useEffect(() => {
    const fetchUsers = async () => {
      const fetchedUsers = await getAllUsers();
      setUsers(fetchedUsers);
    };
    fetchUsers();
  }, []);

  return (
    <>
      {getMain(
        loginState,
        setLoginState,
        showAdmin,
        setShowAdmin,
        users,
        setUsers
      )}
    </>
  );
}

export default App;

function getMain(
  loginState: LoginState,
  setLoginState: (lg: LoginState) => void,
  showAdmin: boolean,
  setShowAdmin: (ssa: boolean) => void,
  users: User[],
  setUsers: (u: User[]) => void
): JSX.Element {
  if (showAdmin) {
    return (
      <AdminConsole
        navToHome={() => setShowAdmin(false)}
        users={users}
        onUsers={(users) => setUsers(users)}
      />
    );
  }

  switch (loginState) {
    case LoginState.LoggedOut:
      return (
        <Main
          setLoginState={setLoginState}
          showAdmin={showAdmin}
          setShowAdmin={setShowAdmin}
        />
      );
    case LoginState.LoggedIn:
      return <Playground users={users} />;
    case LoginState.LoginError:
      return <div>Das war total falsch!</div>;
    default:
      throw `Unreachabale state: ${loginState}`;
  }
}
