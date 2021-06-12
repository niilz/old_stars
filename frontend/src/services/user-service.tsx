import { METHOD } from '../Constants';
import { UserCredentials } from '../model/User';
import { fetchWrapper } from './fetch-service';

export function insertUser(user: UserCredentials) {
  return fetchWrapper(METHOD.POST, 'register', JSON.stringify(user));
}

export function deleteUser(id: Number) {
  return fetchWrapper(METHOD.POST, `delete${id}`, '');
}

export function getAllUsers() {
  return fetchWrapper(METHOD.GET, 'all', '');
}
