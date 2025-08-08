import { useContext } from 'react'
import { Button } from '../components/button/Button'
import { Header } from '../components/header/Header'
import { Modal } from '../components/modal/Modal'
import { UserList } from '../components/user-list/UserList'
import { UserView } from '../components/user/UserView'
import { WaterRoundWarning } from '../components/waterround-warning/WaterRoundWarning'
import { GlobalKeyValueStoreContext } from '../context/Contexts'
import { OldStar, User } from '../model/User'
import { needsWaterRound } from '../util/DrinkUtil'
import styles from './Playground.module.css'
import { SESSION_TOKEN_HEADER_NAME } from '../Constants'

interface PlaygroundProps {
  logout: () => void
  openAdminLogin: () => void
  user: User
  users: User[]
  onUserUpdate: (user: User) => void
  onHistories: () => void
  onRefresh: () => void
}

export function Playground(props: PlaygroundProps) {
  const { keyValueStore } = useContext(GlobalKeyValueStoreContext)
  const userSession = keyValueStore.tryReadFromStorage(
    SESSION_TOKEN_HEADER_NAME
  )
  const oldstar = new OldStar(props.user)
  return (
    <>
      {needsWaterRound(oldstar) && (
        <Modal
          children={
            <WaterRoundWarning
              userId={props.user.id}
              onWaterConsumed={props.onUserUpdate}
              userSession={userSession}
            />
          }
        />
      )}
      <div className={styles.Playground}>
        <Header showLogo={true} />
        <UserView
          user={props.user}
          onUserUpdate={props.onUserUpdate}
          userSession={userSession}
        />
        <UserList users={props.users} isEditable={false} />
        <div className={styles.buttons}>
          <Button text="logout" styles={styles.Btn} callback={props.logout} />
          <Button
            text="🔄"
            styles={styles.refresh}
            callback={props.onRefresh}
          />
        </div>

        <Button
          text="archive"
          styles={styles.Btn}
          callback={props.onHistories}
        />

        <Button
          text="admin"
          styles={styles.Btn}
          callback={props.openAdminLogin}
        />
      </div>
    </>
  )
}
