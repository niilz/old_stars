import { METHOD } from '../Constants';
import { UserCredentials } from '../model/User';
import { fetchWrapper, handleResponse } from './fetch-service';

export async function insertUser(user: UserCredentials) {
  const insertResponse = await fetchWrapper(
    METHOD.POST,
    'register',
    JSON.stringify(user)
  );
  const insertedUser = handleResponse(insertResponse);
  return insertedUser;
}

export function deleteUser(id: Number) {
  return fetchWrapper(METHOD.DELETE, `delete/${id}`, '');
}

export async function getAllUsers() {
  return fetchWrapper(METHOD.GET, 'all', '');
}

export async function attachSession(sessionId: string) {
  return fetchWrapper(METHOD.GET, 'start', '', sessionId);
}

export async function removeSession() {
  return fetchWrapper(METHOD.GET, 'logout', '');
}
