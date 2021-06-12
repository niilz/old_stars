import { METHOD } from '../Constants';
import { User } from '../model/User';
import { fetchWrapper } from './fetch-service';

export function insertUser(user: User) {
  return fetchWrapper(METHOD.POST, 'register', JSON.stringify(user));
}

export function deleteUser(id: Number) {
  return fetchWrapper(METHOD.POST, `delete${id}`, '');
}

export function getAllUsers() {
  return fetchWrapper(METHOD.GET, 'all', '');
}
