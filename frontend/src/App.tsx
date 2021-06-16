import { useEffect, useState } from 'react';
import { Login, LoginState, LoginType } from './components/login/Login';
import { Playground } from './components/playground/Playground';
import { AdminConsole } from './components/admin/AdminConsole';
import { Main } from './components/main/Main';
import styles from './App.module.css';
import './global.css';
import { User } from './model/User';
import { deleteUser, getAllUsers } from './services/user-service';
import { handleResponse } from './services/fetch-service';
import { Modal } from './components/modal/Modal';

function App() {
  const [loginState, setLoginState] = useState(LoginState.LoggedOut);
  const [users, setUsers] = useState(new Array<User>());
  const [loggedInUser, setLoggedInUser] = useState<User | undefined>();
  const [openAdminLogin, setOpenAdminLogin] = useState(false);
  const [isAdminView, setIsAdminView] = useState(false);

  useEffect(() => {
    const fetchUsers = async () => {
      const userResponse = await getAllUsers();
      const users = handleResponse(userResponse);
      setUsers(users as User[]);
    };
    fetchUsers();
  }, []);

  const addUser = (user: User) => {
    const updatedUsers = [...users, user];
    setUsers(updatedUsers);
  };
  const deleteUser = (id: Number) => {
    const updatedUsers = users.filter((user) => user['id'] !== id);
    setUsers(updatedUsers);
  };

  return (
    <>
      {loginState === LoginState.LoggedInUser && loggedInUser ? (
        <Playground
          user={loggedInUser}
          users={users}
          logout={() => setLoginState(LoginState.LoggedOut)}
        />
      ) : (
        <Main
          isAdminView={isAdminView}
          users={users}
          loginType={LoginType.Master}
          onRegister={addUser}
          onLogin={setLoginState}
          setSessionUser={setLoggedInUser}
          deleteUser={deleteUser}
          openAdminLogin={setOpenAdminLogin}
        />
      )}
      {openAdminLogin && (
        <Modal
          children={
            <Login
              loginType={LoginType.Admin}
              onLogin={() => console.log('called onLogin')}
              onRegister={() => console.log('onRegister has been called')}
              setSessionUser={() => console.log('setSessionUser')}
            />
          }
        />
      )}
    </>
  );
}

export default App;
