import React, { useEffect, useState } from 'react';
import { Login, LoginState, LoginType } from './components/login/Login';
import { Main } from './components/main/Main';
import './global.css';
import { User } from './model/User';
import { getAllUsers } from './services/user-service';
import { handleResponse } from './services/fetch-service';
import { Modal } from './components/modal/Modal';

export interface ContextType {
  loginState: LoginState;
  setLoginState: Function;
  loginType: LoginType;
  setLoginType: Function;
}

export const LoginContext = React.createContext({
  loginState: LoginState.LoggedOut,
  setLoginState: (_: LoginState) => {},
  loginType: LoginType.Club,
  setLoginType: (_: LoginType) => {},
});

function App() {
  const [loginState, setLoginState] = useState(LoginState.LoggedOut);
  const [loginType, setLoginType] = useState(LoginType.Club);
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
    setLoginState(LoginState.LoggedInClub);
    setLoginType(LoginType.User);
    setLoggedInUser(undefined);
    setIsAdminView(false);
  };

  const handleUpdateUserList = (updatedUser: User) => {
    const updatedUserList = users.map((user) =>
      user.id === updatedUser.id ? updatedUser : user
    );
    setUsers(updatedUserList);
    setLoggedInUser(updatedUser);
  };

  return (
    <LoginContext.Provider
      value={{ loginState, setLoginState, loginType, setLoginType }}
    >
      <Main
        isAdminView={isAdminView}
        users={users}
        sessionUser={loggedInUser}
        onRegister={addUser}
        onLogin={setLoginState}
        setSessionUser={setLoggedInUser}
        deleteUser={deleteUser}
        openAdminLogin={setOpenAdminLogin}
        setAdminView={setIsAdminView}
        onLogout={handleLogout}
        onUserUpdate={handleUpdateUserList}
      />
      {openAdminLogin && (
        <Modal children={<Login onLogin={handleAdminLogin} />} />
      )}
    </LoginContext.Provider>
  );
}

export default App;
