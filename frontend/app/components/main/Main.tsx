import React from 'react'
import { useContext, useEffect, useState } from 'react'
import { AppCtx } from '../../App'
import { User } from '../../model/User'
import { ApiResponse, handleResponse } from '../../services/fetch-service'
import {
  attachSession,
  getAllUsers,
  removeSession,
} from '../../services/user-service'
import styles from './Main.module.css'
import {
  CLUB_TOKEN_HEADER_NAME,
  SESSION_TOKEN_HEADER_NAME,
} from '../../Constants'
import {
  ErrorContext,
  GlobalKeyValueStoreContext,
  HistoryContext,
  UserContext,
  ViewContext,
} from '../../context/Contexts'
import { View } from '../../views/View'
import { ClubLoginView } from '../../views/ClubLoginView'
import { UserLoginView } from '../../views/UserLoginView'
import { Playground } from '../../views/Playground'
import { AdminConsole } from '../../views/AdminConsole'
import { fetchHistories } from '../../services/history-service'
import {
  DrinkHistory,
  mapToDateAndTime,
  mapToUser,
} from '../../model/DrinkHistory'
import { OneHistoryView } from '../../views/OneHistoryView'
import { ArchiveView } from '../../views/ArchiveView'
import AuthService from '../../services/auth-service'
import { Modal } from '../modal/Modal'
import { AdminLoginForm } from '../admin-login/AdminLoginForm'
import { Button } from '../button/Button'

export function Main() {
  const [users, setUsers] = useState(new Array<User>())
  const [sessionUser, setSessionUser] = useState<User | null>(null)
  const [allHistories, setAllHistories] = useState(new Array<DrinkHistory>())
  const [selectedHistory, setSelectedHistory] = useState<DrinkHistory[]>([])

  const { isAdminLoginOpen, setAdminLoginOpen } = useContext(AppCtx)
  const { activeView, setActiveView } = useContext(ViewContext)
  const { setCurrentError } = useContext(ErrorContext)
  const { keyValueStore } = useContext(GlobalKeyValueStoreContext)

  const fetchUsers = async () => {
    try {
      console.log('trying to fetch all users')
      const sessionId = keyValueStore.readFromStorage(SESSION_TOKEN_HEADER_NAME)
      if (sessionId) {
        const userResponse = await getAllUsers(sessionId)
        const users = handleResponse(userResponse)
        setUsers(users as User[])
      }
    } catch (e) {
      setActiveView(View.ClubLogin)
      console.error(`Loading users failed: ${e}`)
      setCurrentError(`loading users failed ${e}`)
    }
  }

  useEffect(() => {
    if (activeView === View.Playground) {
      fetchUsers()
    }
  }, [activeView])

  const tryAttachClubToken = async (clubToken: string) => {
    if (clubToken) {
      console.log('Checking if club-token is still valid')
      const hasClubAccess = await AuthService.hasClubAccess(clubToken)
      if (hasClubAccess === true) {
        console.log('Has club access')
        setActiveView(View.UserLogin)
      } else {
        console.log('Club token is not valid, removing club-token')
        keyValueStore.removeItem(CLUB_TOKEN_HEADER_NAME)
        setActiveView(View.ClubLogin)
      }
    }
  }

  useEffect(() => {
    const tryAttachSession = async (sessionId: string) => {
      console.log('trying to attach session')
      const attachResponse = await attachSession(sessionId)
      if (attachResponse.Err) {
        console.log(
          `Session login did not work. Err: ${attachResponse.Err}. Clearing token`
        )
        keyValueStore.removeItem(SESSION_TOKEN_HEADER_NAME)
        // If user-session did not work: try club-session
        const clubToken = keyValueStore.readFromStorage(CLUB_TOKEN_HEADER_NAME)
        if (clubToken) {
          tryAttachClubToken(clubToken)
        }
      } else {
        console.log(`got attachResponse: ${attachResponse}`)
        const user = handleResponse(attachResponse)
        setSessionUser(user as User)
        setActiveView(View.Playground)
        fetchUsers()
      }
    }

    const sessionId = keyValueStore.readFromStorage(SESSION_TOKEN_HEADER_NAME)
    if (sessionId) {
      tryAttachSession(sessionId).catch((e) => {
        console.error(`Could not attach session: ${e}`)
        setCurrentError(`Attaching session failed`)
      })
    }
  }, [])

  const addUser = (user: User) => {
    const updatedUsers = [...users, user]
    setUsers(updatedUsers)
  }

  const deleteUser = async (res: Promise<ApiResponse>) => {
    const result = await res
    handleResponse(result)
    fetchUsers()
  }

  const handleLogout = async () => {
    const sessionId = keyValueStore.tryReadFromStorage(
      SESSION_TOKEN_HEADER_NAME
    )
    const removeSessionRes = await removeSession(sessionId)
    if (removeSessionRes) {
      setActiveView(View.UserLogin)
      setSessionUser(null)
      keyValueStore.removeItem(SESSION_TOKEN_HEADER_NAME)
    }
  }

  const handleAdminLogin = () => {
    setAdminLoginOpen(true)
  }

  const handleUpdateUserList = (updatedUser: User) => {
    const updatedUserList = users.map((user) =>
      user.id === updatedUser.id ? updatedUser : user
    )
    setUsers(updatedUserList)
    setSessionUser(updatedUser)
  }

  const handleRefresh = async () => {
    const sessionId = keyValueStore.tryReadFromStorage(
      SESSION_TOKEN_HEADER_NAME
    )
    const allUsersResponse = await getAllUsers(sessionId)
    const allUsers = handleResponse(allUsersResponse)
    setUsers(allUsers as User[])
    if (sessionUser) {
      const currentUser = (allUsers as User[]).filter(
        (user) => user.id === sessionUser.id
      )[0]
      setSessionUser(currentUser)
    }
  }

  const handleHistorize = async (historyResult: Promise<ApiResponse>) => {
    const result = await historyResult
    console.log(`Histories: ${result}`)
  }

  const handleFetchHistories = async () => {
    const sessionId = keyValueStore.tryReadFromStorage(
      SESSION_TOKEN_HEADER_NAME
    )
    const historyRes = await fetchHistories(sessionId)
    const histories = handleResponse(historyRes)
    setAllHistories(histories as DrinkHistory[])
    setActiveView(View.Histories)
  }

  const handleOnAdminLogin = (view: View) => {
    setActiveView(view)
    setAdminLoginOpen(false)
  }

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
        <HistoryContext.Provider
          value={{ selectedHistory, setSelectedHistory }}
        >
          {activeView === View.Histories && (
            <ArchiveView historyDays={groupByDates(allHistories)} />
          )}
          {activeView === View.OneHistory && (
            <OneHistoryView
              dateAndTime={mapToDateAndTime(selectedHistory[0].timestamp)}
              users={selectedHistory.map((hist) => mapToUser(hist))}
            />
          )}
          {isAdminLoginOpen && (
            <Modal
              children={
                <>
                  <AdminLoginForm
                    onLogin={handleOnAdminLogin}
                    onError={(err) =>
                      setCurrentError(`Admin Login failed: ${err}`)
                    }
                    // TODO: Fix this non-null-check}
                    userName={sessionUser!!.name}
                  />
                  <Button
                    text={'cancel'}
                    callback={() => {
                      setAdminLoginOpen(false)
                      setCurrentError('')
                    }}
                    styles={''}
                  />
                </>
              }
            />
          )}
        </HistoryContext.Provider>
      </UserContext.Provider>
    </div>
  )
}

// TODO: First only fetch the days from backend
//   then (lazy) load the single hisories (user states)
//   for that date
function groupByDates(histories: DrinkHistory[]) {
  return histories.reduce((dates, history) => {
    const dateAndTime = mapToDateAndTime(history.timestamp)
    const dateTimeString = JSON.stringify(dateAndTime)
    const maybeHistories = dates.get(dateTimeString)
    if (maybeHistories) {
      maybeHistories.push(history)
    } else {
      dates.set(dateTimeString, [history])
    }
    return dates
  }, new Map<string, DrinkHistory[]>())
}
