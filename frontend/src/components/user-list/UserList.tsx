import { User } from '../../model/User';
import { ListItem } from '../listItem/ListItem';
import styles from './UserList.module.css';

type UserListProps = {
  users: User[];
  isEditable: boolean;
  onDelete?: (id: number) => void;
};

export function UserList(props: UserListProps) {
  return (
    <table className={styles.UserList}>
      <thead>
        <tr>
          <th></th>
          <th>🍺</th>
          <th>🥃</th>
          <th>🚰</th>
        </tr>
      </thead>
      <tbody>
        {props.users.map((user) => (
          <ListItem
            key={user['id']}
            id={user['id']}
            user={user}
            isEditable={props.isEditable}
            deleteGotClicked={() =>
              props.onDelete ? props.onDelete(user['id']) : null
            }
          />
        ))}
      </tbody>
    </table>
  );
}
