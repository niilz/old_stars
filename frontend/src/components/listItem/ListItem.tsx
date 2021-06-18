import { User } from '../../model/User';
import styles from './ListItem.module.css';

type ListItemProps = {
  id: number;
  user: User;
  isEditable: boolean;
  deleteGotClicked?: () => void;
};

export function ListItem(props: ListItemProps) {
  return (
    <tr className={styles.itemContent}>
      <td>
        <div className={styles.userData}>{props.user.name}</div>
      </td>
      <td className={styles.count}>{props.user.beerCount}</td>
      <td className={styles.count}>{props.user.shotCount}</td>
      <td className={styles.count}>{props.user.waterCount}</td>
      {props.isEditable ? (
        <td>
          <button className={styles.delBtn} onClick={props.deleteGotClicked}>
            ❌
          </button>
        </td>
      ) : null}
    </tr>
  );
}
