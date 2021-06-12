import React from 'react';
import { User } from '../../model/User';
import {DeletableListItem} from '../deletable/Deletable';

export function UserList(list: [User]) {
	return <div>{list.map((user) => <DeletableListItem id={user["id"]} text={user["user_name"]})}</div>;
}
