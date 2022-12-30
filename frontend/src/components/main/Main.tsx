import React from 'react';
import { Login } from '../login/Login';
import { useContext, useEffect, useState } from 'react';
import { AppCtx } from '../../App';
import { User } from '../../model/User';
import { handleResponse } from '../../services/fetch-service';
import { attachSession, getAllUsers } from '../../services/user-service';
import { AdminConsole } from '../admin/AdminConsole';
import { Button } from '../button/Button';
import { Header } from '../header/Header';
import { AppLogo } from '../logo/Logo';
import { Playground } from '../playground/Playground';
import styles from './Main.module.css';
import { LoginState, LoginType } from '../../Constants';

export const UserContext = React.createContext({
  addUser: (_user: User) => {},
  setSessionUser: (_user: User) => {},
});

export const LoginContext = React.createContext({
  loginState: LoginState.LoggedOut,
  setLoginState: (_: LoginState) => {},
});

export function Main() {
  const [users, setUsers] = useState(new Array<User>());
  const [sessionUser, setSessionUser] = useState<User | null>(null);
  const [loginState, setLoginState] = useState(LoginState.LoggedOut);

  const { setLoginType, isAdminViewOpen, setAdminViewOpen, setAdminLoginOpen } =
    useContext(AppCtx);

  useEffect(() => {
    const fetchUsers = async () => {
      const userResponse = await getAllUsers();
      const users = handleResponse(userResponse);
      setUsers(users as User[]);
    };
    fetchUsers();
  }, []);

  useEffect(() => {
    const tryAttachSession = async () => {
      const attachResponse = await attachSession();
      const user = handleResponse(attachResponse);
      if (user) {
        setSessionUser(user as User);
        setLoginState(LoginState.LoggedInUser);
      }
    };
    tryAttachSession();
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

  const handleRefresh = async () => {
    const allUsersResponse = await getAllUsers();
    const allUsers = handleResponse(allUsersResponse);
    setUsers(allUsers as User[]);
  };

  const handleOpenAdminLogin = () => {
    setAdminLoginOpen(true);
    setLoginType(LoginType.Admin);
  };

  const handleAdminHomeClick = () => {
    setAdminViewOpen(false);
    setLoginType(sessionUser ? LoginType.None : LoginType.User);
    setLoginState(
      sessionUser ? LoginState.LoggedInUser : LoginState.LoggedInClub
    );
  };
  return (
    <LoginContext.Provider value={{ loginState, setLoginState }}>
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
                  onRefresh={handleRefresh}
                />
              ) : (
                <Login onLogin={setLoginState} />
              )}
            </UserContext.Provider>
            {!isAdminViewOpen && (
              <Button
                text="admin"
                styles={styles.Btn}
                callback={handleOpenAdminLogin}
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
