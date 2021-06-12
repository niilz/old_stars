import { User } from '../../model/User';
import { DeletableListItem } from '../deletable/Deletable';

type UserListProps = {
  users: User[];
  onDelete: (id: number) => void;
};

export function UserList(props: UserListProps) {
  return (
    <ul>
      {props.users.map((user) => (
        <DeletableListItem
          key={user['id']}
          id={user['id']}
          text={user['name']}
          deleteGotClicked={() => props.onDelete(user['id'])}
        />
      ))}
    </ul>
  );
}
