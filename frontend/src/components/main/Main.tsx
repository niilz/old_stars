import React from 'react';
import { useContext, useEffect, useState } from 'react';
import { AppCtx } from '../../App';
import { User } from '../../model/User';
import { ApiResponse, handleResponse } from '../../services/fetch-service';
import {
  attachSession,
  getAllUsers,
  removeSession,
} from '../../services/user-service';
import styles from './Main.module.css';
import { LoginType, SESSION_TOKEN_HEADER_NAME } from '../../Constants';
import { ErrorContext, UserContext, ViewContext } from '../../context/Contexts';
import { View } from '../../views/View';
import { ClubLoginView } from '../../views/ClubLoginView';
import { UserLoginView } from '../../views/UserLoginView';
import { Playground } from '../../views/Playground';
import { AdminConsole } from '../../views/AdminConsole';
import { HistoryView } from '../../views/HistoryView';
import { fetchHistories } from '../../services/history-service';
import { DrinkHistory } from '../../model/DrinkHistory';

export function Main() {
  const [users, setUsers] = useState(new Array<User>());
  const [sessionUser, setSessionUser] = useState<User | null>(null);
  const [histories, setHistories] = useState(new Array<DrinkHistory>());

  const { setLoginType, setAdminLoginOpen } = useContext(AppCtx);
  const { activeView, setActiveView } = useContext(ViewContext);
  const { setCurrentError } = useContext(ErrorContext);

  const fetchUsers = async () => {
    try {
      const userResponse = await getAllUsers();
      const users = handleResponse(userResponse);
      setUsers(users as User[]);
    } catch (e) {
      setActiveView(View.ClubLogin);
      console.error(`Loading users failed: ${e}`);
      setCurrentError(`loading users failed`);
    }
  };

  useEffect(() => {
    fetchUsers();
  }, []);

  useEffect(() => {
    const tryAttachSession = async (sessionId: string) => {
      const attachResponse = await attachSession(sessionId);
      if (attachResponse.Err) {
        console.log(
          `Session login did not work. Err: ${attachResponse.Err}. Clearing token`
        );
        window.localStorage.removeItem(SESSION_TOKEN_HEADER_NAME);
      } else {
        console.log(`got attachResponse: ${attachResponse}`);
        const user = handleResponse(attachResponse);
        if (user) {
          setSessionUser(user as User);
          setActiveView(View.Playground);
        }
      }
    };
    const sessionId = window.localStorage.getItem(SESSION_TOKEN_HEADER_NAME);
    if (sessionId) {
      tryAttachSession(sessionId).catch((e) => {
        console.error(`Could not attach session: ${e}`);
        setCurrentError(`Attaching session failed`);
      });
    }
  }, []);

  const addUser = (user: User) => {
    const updatedUsers = [...users, user];
    setUsers(updatedUsers);
  };

  const deleteUser = async (res: Promise<ApiResponse>) => {
    const result = await res;
    handleResponse(result);
    fetchUsers();
  };

  const handleLogout = async () => {
    const removeSessionRes = await removeSession();
    if (removeSessionRes) {
      setActiveView(View.UserLogin);
      setLoginType(LoginType.User);
      setSessionUser(null);
      window.localStorage.removeItem(SESSION_TOKEN_HEADER_NAME);
    }
  };

  const handleAdminLogin = () => {
    setLoginType(LoginType.Admin);
    setAdminLoginOpen(true);
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

  const handleHistorize = async (historyResult: Promise<ApiResponse>) => {
    const result = await historyResult;
    console.log(`Histories: ${result}`);
  };

  const handleFetchHistories = async () => {
    const historyRes = await fetchHistories();
    const histories = handleResponse(historyRes);
    setHistories(histories as DrinkHistory[]);
    setActiveView(View.Histories);
  };

  return (
    <div className={styles.Main}>
      <UserContext.Provider value={{ addUser, setSessionUser }}>
        {activeView === View.ClubLogin && <ClubLoginView />}
        {activeView === View.UserLogin && <UserLoginView />}
        {activeView === View.Playground && sessionUser && (
          <Playground
            user={sessionUser}
            users={users}
            logout={handleLogout}
            openAdminLogin={handleAdminLogin}
            onUserUpdate={handleUpdateUserList}
            onHistories={handleFetchHistories}
            onRefresh={handleRefresh}
          />
        )}
        {activeView === View.AdminConsole && (
          <AdminConsole
            users={users}
            onDelete={deleteUser}
            onHistorize={handleHistorize}
          />
        )}
        {activeView === View.Histories && (
          <HistoryView historyDays={groupByDates(histories)} />
        )}
        {activeView === View.OneHistory && <OneHistoryView users={/*TODO  create OneHistoryView and mapper from selectedHistory-list to users*/}}
      </UserContext.Provider>
    </div>
  );
}

// TODO: First only fetch the days from backend
//   then (lazy) load the single hisories (user states)
//   for that date
function groupByDates(histories: DrinkHistory[]) {
  return histories.reduce((dates, history) => {
    const timeStampAsMillis = history.timestamp.secs_since_epoch * 1000;
    const date = new Date(timeStampAsMillis);
    const maybeHistories = dates.get(date);
    if (maybeHistories) {
      maybeHistories.push(history);
    } else {
      dates.set(date, [history]);
    }
    return dates;
  }, new Map<Date, DrinkHistory[]>());
}
