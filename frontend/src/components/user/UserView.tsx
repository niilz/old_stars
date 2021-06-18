import { User } from '../../model/User';
import styles from './UserView.module.css';

interface UserProps {
  user: User;
}
export function UserView(props: UserProps) {
  return (
    <div className={styles.User}>
      <p className={styles.name}>{props.user.name}</p>
      <div className={styles.statsPanel}>
        <div className={styles.icons}>
          <p className={styles.beer}>🍺</p>
          <p className={styles.shots}>🥃</p>
          <p className={styles.water}>🚰</p>
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
