import { User } from '../../model/User';
import { ListItem } from '../deletable/Deletable';

type UserListProps = {
  users: User[];
  isEditable: boolean;
  onDelete?: (id: number) => void;
};

export function UserList(props: UserListProps) {
  return (
    <ul>
      {props.users.map((user) => (
        <ListItem
          key={user['id']}
          id={user['id']}
          text={user['name']}
          isEditable={props.isEditable}
          deleteGotClicked={() =>
            props.onDelete ? props.onDelete(user['id']) : null
          }
        />
      ))}
    </ul>
  );
}
