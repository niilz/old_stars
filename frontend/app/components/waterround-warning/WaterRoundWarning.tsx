import { METHOD } from '../../Constants'
import { User } from '../../model/User'
import {
  fetchWrapperUserSession,
  handleResponse,
} from '../../services/fetch-service'
import { Button } from '../button/Button'
import styles from './WaterRoundWarning.module.css'
type WaterRoundWarningProps = {
  userId: number
  onWaterConsumed: (user: User) => void
  userSession: string
}

export function WaterRoundWarning(props: WaterRoundWarningProps) {
  const handleDrinkWater = async () => {
    const addWaterResponse = await fetchWrapperUserSession(
      METHOD.GET,
      `water/${props.userId}`,
      '',
      props.userSession
    )
    const updatedUser = handleResponse(addWaterResponse)
    props.onWaterConsumed(updatedUser as User)
  }
  return (
    <div className={styles.warningWrapper}>
      <p className={styles.warning}>Du musst eine Wasserrunde trinken</p>
      <Button text="Wasser getrunken" styles={''} callback={handleDrinkWater} />
    </div>
  )
}
