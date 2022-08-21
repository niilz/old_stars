import { METHOD } from '../../Constants';
import { User } from '../../model/User';
import { fetchWrapper, handleResponse } from '../../services/fetch-service';
import { Button } from '../button/Button';
import styles from './WaterRoundWarning.module.css';
type WaterRoundWarningProps = {
  userId: number;
  onWaterConsumed: (user: User) => void;
};

export function WaterRoundWarning(props: WaterRoundWarningProps) {
  const handleDrinkWater = async () => {
    const addWaterResponse = await fetchWrapper(
      METHOD.GET,
      `water/${props.userId}`,
      ''
    );
    const updatedUser = handleResponse(addWaterResponse);
    props.onWaterConsumed(updatedUser as User);
  };
  return (
    <div>
      <p className={styles.warning}>Du musst eine Wasserrunde trinken</p>
      <Button text="Wasser getrunken" styles={''} callback={handleDrinkWater} />
    </div>
  );
}
