import React from 'react';
import { User } from '../../model/User';
import { DeletableListItem } from '../deletable/Deletable';

type UserListProps = {
  users: User[];
};

export function UserList(props: UserListProps) {
  return (
    <div>
      {props.users.map((user) => (
        <DeletableListItem id={user['id']} text={user['user_name']} />
      ))}
    </div>
  );
}
