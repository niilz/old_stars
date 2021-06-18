import { METHOD } from '../../Constants';
import { User } from '../../model/User';
import { fetchWrapper, handleResponse } from '../../services/fetch-service';
import styles from './UserView.module.css';

interface UserProps {
  user: User;
  onUserUpdate: (user: User) => void;
}
export function UserView(props: UserProps) {
  const handleUpdate = async (drink: string) => {
    const res = await fetchWrapper(METHOD.GET, `${drink}/${props.user.id}`, '');
    const updatedUser = handleResponse(res);
    props.onUserUpdate(updatedUser as User);
  };
  return (
    <div className={styles.User}>
      <p className={styles.name}>{props.user.name}</p>
      <div className={styles.statsPanel}>
        <div className={styles.icons}>
          <button onClick={() => handleUpdate('beer')} className={styles.beer}>
            🍺
          </button>
          <button onClick={() => handleUpdate('shot')} className={styles.shots}>
            🥃
          </button>
          <button
            onClick={() => handleUpdate('water')}
            className={styles.water}
          >
            🚰
          </button>
        </div>
        <div className={styles.stats}>
          <p className={styles.count}>{props.user.beerCount}</p>
          <p className={styles.count}>{props.user.shotCount}</p>
          <p className={styles.count}>{props.user.waterCount}</p>
        </div>
      </div>
    </div>
  );
}
