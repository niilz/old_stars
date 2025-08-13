import { useContext, useState } from 'react'
import styles from './AdminConsole.module.css'
import { User } from '../model/User'
import { ApiResponse, handleResponse } from '../services/fetch-service'
import { GlobalKeyValueStoreContext, ViewContext } from '../context/Contexts'
import { deleteUser } from '../services/user-service'
import { historizeDrinks, storeHistory } from '../services/history-service'
import { Header } from '../components/header/Header'
import { UserList } from '../components/user-list/UserList'
import { Button } from '../components/button/Button'
import { View } from './View'
import { SESSION_TOKEN_HEADER_NAME } from '../Constants'
import { Message, MsgType } from '../components/message/Message'
import { readErrorMessage } from '../model/Error'

interface AdminConsoleProps {
  users: User[]
  onDelete: (voidResult: Promise<ApiResponse>) => void
  updateUsersOnHistorize: () => void
}

export function AdminConsole(props: AdminConsoleProps) {
  const { setActiveView } = useContext(ViewContext)
  const { keyValueStore } = useContext(GlobalKeyValueStoreContext)
  const [historyData, setHistoryData] = useState('')
  const [currentMessage, setCurrentMessage] = useState({
    msgType: MsgType.NONE,
    msg: '',
  })

  const deleteUserFromList = (userId: number) => {
    const sessionId = keyValueStore.tryReadFromStorage(
      SESSION_TOKEN_HEADER_NAME
    )
    const voidResult = deleteUser(userId, sessionId)
    props.onDelete(voidResult)
  }

  const handleHistorizeDrinks = async () => {
    const sessionId = keyValueStore.tryReadFromStorage(
      SESSION_TOKEN_HEADER_NAME
    )
    const historiesResult = await historizeDrinks(sessionId)
    const savedHistoryEntries = handleResponse(historiesResult) as History[]
    setCurrentMessage({
      msgType: MsgType.INFO,
      msg: `saved ${savedHistoryEntries.length} history entries`,
    })
    props.updateUsersOnHistorize()
  }

  const handleSaveHistory = async () => {
    const sessionId = keyValueStore.tryReadFromStorage(
      SESSION_TOKEN_HEADER_NAME
    )
    const storeHistoryResult = await storeHistory(sessionId, historyData)
    try {
      const history = handleResponse(storeHistoryResult) as History[]
      setCurrentMessage({
        msgType: MsgType.INFO,
        msg: `stored history with ${history.length} items`,
      })
      setHistoryData('')
    } catch (err) {
      setCurrentMessage({
        msgType: MsgType.ERR,
        msg: readErrorMessage(err).msg,
      })
    }
  }

  return (
    <div className={styles.AdminConsole}>
      <Header showLogo={true} />
      <div className={styles.DataSection}>
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
        <Message msg={currentMessage.msg} type={currentMessage.msgType} />
        <textarea
          className={styles.textarea}
          placeholder="paste history as CSV here"
          onChange={(e) => setHistoryData(e.target.value)}
        />
        <Button
          text="save history"
          styles={styles.Btn}
          callback={handleSaveHistory}
        />
      </div>
    </div>
  )
}
