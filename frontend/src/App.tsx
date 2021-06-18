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
  const [loginType, setLoginType] = useState(LoginType.Master);
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

  const handleAdminLogin = (loginState: LoginState) => {
    if (loginState !== LoginState.LoggedInAdmin)
      throw 'Only Admin should be able to log in as admin';
    setLoginState(loginState);
    setOpenAdminLogin(false);
    setIsAdminView(true);
    setLoginType(loginType);
  };

  const handleLogout = () => {
    setLoginState(LoginState.LoggedInMaster);
    setLoginType(LoginType.User);
  };

  return (
    <>
      <Main
        isAdminView={isAdminView}
        users={users}
        sessionUser={loggedInUser}
        onRegister={addUser}
        onLogin={setLoginState}
        loginState={loginState}
        setLoginState={setLoginState}
        loginType={loginType}
        setLoginType={setLoginType}
        setSessionUser={setLoggedInUser}
        deleteUser={deleteUser}
        openAdminLogin={setOpenAdminLogin}
        setAdminView={setIsAdminView}
        onLogout={handleLogout}
      />
      {openAdminLogin && (
        <Modal
          children={
            <Login
              loginType={LoginType.Admin}
              setLoginType={setLoginType}
              onLogin={handleAdminLogin}
            />
          }
        />
      )}
    </>
  );
}

export default App;
