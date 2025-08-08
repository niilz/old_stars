import { useContext } from 'react'
import styles from './AdminConsole.module.css'
import { User } from '../model/User'
import { ApiResponse } from '../services/fetch-service'
import { GlobalKeyValueStoreContext, ViewContext } from '../context/Contexts'
import { deleteUser } from '../services/user-service'
import { historizeDrinks } from '../services/history-service'
import { Header } from '../components/header/Header'
import { UserList } from '../components/user-list/UserList'
import { Button } from '../components/button/Button'
import { View } from './View'
import { SESSION_TOKEN_HEADER_NAME } from '../Constants'

interface AdminConsoleProps {
  users: User[]
  onDelete: (voidResult: Promise<ApiResponse>) => void
  onHistorize: (histories: Promise<ApiResponse>) => void
}

export function AdminConsole(props: AdminConsoleProps) {
  const { setActiveView } = useContext(ViewContext)
  const { keyValueStore } = useContext(GlobalKeyValueStoreContext)

  const deleteUserFromList = (userId: number) => {
    const sessionId = keyValueStore.tryReadFromStorage(
      SESSION_TOKEN_HEADER_NAME
    )
    const voidResult = deleteUser(userId, sessionId)
    props.onDelete(voidResult)
  }

  const handleHistorizeDrinks = () => {
    const sessionId = keyValueStore.tryReadFromStorage(
      SESSION_TOKEN_HEADER_NAME
    )
    const historiesResult = historizeDrinks(sessionId)
    props.onHistorize(historiesResult)
  }

  return (
    <div className={styles.AdminConsole}>
      <Header showLogo={true} />
      <UserList
        users={props.users}
        isEditable={true}
        onDelete={deleteUserFromList}
      />
      <Button
        text="historize"
        styles={styles.Btn}
        callback={handleHistorizeDrinks}
      />
      <Button
        text="Home"
        styles={styles.Btn}
        callback={() => setActiveView(View.Playground)}
      />
    </div>
  )
}
