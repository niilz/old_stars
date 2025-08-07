import { Button } from '../components/button/Button'
import { Header } from '../components/header/Header'
import { Modal } from '../components/modal/Modal'
import { UserList } from '../components/user-list/UserList'
import { UserView } from '../components/user/UserView'
import { WaterRoundWarning } from '../components/waterround-warning/WaterRoundWarning'
import { OldStar, User } from '../model/User'
import { needsWaterRound } from '../util/DrinkUtil'
import styles from './Playground.module.css'

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
  const oldstar = new OldStar(props.user)
  return (
    <>
      {needsWaterRound(oldstar) && (
        <Modal
          children={
            <WaterRoundWarning
              userId={props.user.id}
              onWaterConsumed={props.onUserUpdate}
            />
          }
        />
      )}
      <div className={styles.Playground}>
        <Header showLogo={true} />
        <UserView user={props.user} onUserUpdate={props.onUserUpdate} />
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
