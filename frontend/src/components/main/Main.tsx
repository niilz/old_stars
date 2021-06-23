import React from 'react';
import { Login } from '../login/Login';
import { useContext, useEffect, useState } from 'react';
import { AdminContext } from '../../App';
import { User } from '../../model/User';
import { handleResponse } from '../../services/fetch-service';
import { getAllUsers } from '../../services/user-service';
import { AdminConsole } from '../admin/AdminConsole';
import { Button } from '../button/Button';
import { Header } from '../header/Header';
import { AppLogo } from '../logo/Logo';
import { Playground } from '../playground/Playground';
import styles from './Main.module.css';
import { LoginState, LoginType } from '../../Constants';

export interface LoginContextI {
  loginState: LoginState;
  setLoginState: Function;
  loginType: LoginType;
  setLoginType: Function;
}

interface LeftOverUserI {
  sessionUser: User | undefined;
  onRegister: (user: User) => void;
  users: User[];
  deleteUser: (id: Number) => void;
  onLogout: () => void;
  onUserUpdate: (user: User) => void;
}

interface UserContextI {
  addUser: (user: User) => void;
  setSessionUser: (user: User) => void;
}

export const UserContext = React.createContext({
  addUser: (_user: User) => {},
  setSessionUser: (_user: User) => {},
});

export const LoginContext = React.createContext({
  loginState: LoginState.LoggedOut,
  setLoginState: (_: LoginState) => {},
  loginType: LoginType.Club,
  setLoginType: (_: LoginType) => {},
});

export function Main() {
  const [users, setUsers] = useState(new Array<User>());
  const [sessionUser, setSessionUser] = useState<User | null>(null);
  const [loginState, setLoginState] = useState(LoginState.LoggedOut);
  const [loginType, setLoginType] = useState(LoginType.Club);

  const { isAdminViewOpen, setAdminViewOpen, setAdminLoginOpen } = useContext(
    AdminContext
  );

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

  const handleLogout = () => {
    setLoginState(LoginState.LoggedInClub);
    setLoginType(LoginType.User);
    setSessionUser(null);
    setAdminViewOpen(false);
  };

  const handleUpdateUserList = (updatedUser: User) => {
    const updatedUserList = users.map((user) =>
      user.id === updatedUser.id ? updatedUser : user
    );
    setUsers(updatedUserList);
    setSessionUser(updatedUser);
  };
  const handleAdminHomeClick = () => {
    setAdminViewOpen(false);
    setLoginType(sessionUser ? LoginType.None : LoginType.User);
    setLoginState(
      sessionUser ? LoginState.LoggedInUser : LoginState.LoggedInClub
    );
  };
  return (
    <LoginContext.Provider
      value={{ loginState, setLoginState, loginType, setLoginType }}
    >
      <div className={styles.Main}>
        {!isAdminViewOpen ? (
          <>
            {showBigHeaderAndStar(isAdminViewOpen, loginState) && (
              <>
                <Header
                  showLogo={false}
                  styles={{
                    headerStripes: styles.headerStripes,
                    title: styles.title,
                  }}
                />
                <AppLogo styles={styles.logo} />
              </>
            )}
            <UserContext.Provider value={{ addUser, setSessionUser }}>
              {sessionUser && showPlayground(loginState, sessionUser) ? (
                <Playground
                  user={sessionUser}
                  users={users}
                  logout={handleLogout}
                  onUserUpdate={handleUpdateUserList}
                />
              ) : (
                <Login onLogin={setLoginState} />
              )}
            </UserContext.Provider>
            {!isAdminViewOpen && (
              <Button
                text="admin"
                styles={styles.Btn}
                callback={() => setAdminLoginOpen(true)}
              />
            )}
          </>
        ) : (
          <AdminConsole
            navToHome={handleAdminHomeClick}
            users={users}
            onDelete={deleteUser}
          />
        )}
      </div>
    </LoginContext.Provider>
  );
}

function showBigHeaderAndStar(isAdminView: boolean, ls: LoginState) {
  return !isAdminView && ls !== LoginState.LoggedInUser;
}

function showPlayground(ls: LoginState, sessionUser: User) {
  return ls === LoginState.LoggedInUser && sessionUser;
}
