import { useEffect, useState } from 'react';
import { Login, LoginState } from './components/login/Login';
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
  const [isAdminView, setAdminView] = useState(false);
  const [users, setUsers] = useState(new Array<User>());
  const [loggedInUser, setLoggedInUser] = useState<User | undefined>();
  const [openAdminLogin, setOpenAdminLogin] = useState(false);

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
      {getMain(
        loginState,
        setLoginState,
        isAdminView,
        setAdminView,
        users,
        addUser,
        deleteUser,
        loggedInUser,
        setLoggedInUser
      )}
      {openAdminLogin && (
        <Modal
          children={
            <Login
              isUserLogin={false}
              onLogin={() => console.log('called onLogin')}
              isAdminView={true}
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

function getMain(
  loginState: LoginState,
  setLoginState: (lg: LoginState) => void,
  isAdminView: boolean,
  setAdminView: (ssa: boolean) => void,
  users: User[],
  addUser: (u: User) => void,
  deleteUser: (id: Number) => void,
  loggedInUser: User | undefined,
  setLoggedInUser: (u: User) => void
): JSX.Element {
  if (isAdminView) {
    return (
      <AdminConsole
        navToHome={() => setAdminView(false)}
        users={users}
        onRegister={addUser}
        onDelete={deleteUser}
        isAdminView={isAdminView}
      />
    );
  }

  switch (loginState) {
    case LoginState.LoggedOut:
      return (
        <Main
          isAdminView={isAdminView}
          setAdminView={setAdminView}
          isUserLogin={false}
          onRegister={addUser}
          onLogin={setLoginState}
          setSessionUser={setLoggedInUser}
        />
      );
    case LoginState.LoggedInMaster:
      return (
        <Main
          isAdminView={isAdminView}
          setAdminView={setAdminView}
          isUserLogin={true}
          onRegister={addUser}
          onLogin={setLoginState}
          setSessionUser={setLoggedInUser}
        />
      );
    case LoginState.LoggedInUser:
      if (!loggedInUser) {
        throw 'Tried to open Playground without a loggedInUser';
      }
      return (
        <Playground
          user={loggedInUser}
          users={users}
          logout={() => setLoginState(LoginState.LoggedOut)}
        />
      );
    case LoginState.LoginError:
      return <div>Das war total falsch!</div>;
    default:
      throw `Unreachabale state: ${loginState}`;
  }
}
