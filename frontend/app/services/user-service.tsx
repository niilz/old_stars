import { CLUB_TOKEN_HEADER_NAME, METHOD } from '../Constants'
import { UserCredentials } from '../model/User'
import {
  fetchWrapper,
  fetchWrapperUserSession,
  handleResponse,
} from './fetch-service'

export async function insertUser(user: UserCredentials, clubToken: string) {
  const insertResponse = await fetchWrapper(
    METHOD.POST,
    'register',
    JSON.stringify(user),
    CLUB_TOKEN_HEADER_NAME,
    clubToken
  )
  const insertedUser = handleResponse(insertResponse)
  return insertedUser
}

export function deleteUser(id: Number, sessionId: string) {
  return fetchWrapperUserSession(METHOD.DELETE, `delete/${id}`, '', sessionId)
}

export async function getUser(sessionId: string, name: string) {
  return fetchWrapperUserSession(METHOD.GET, `user/${name}`, '', sessionId)
}

export async function getAllUsers(sessionId: string) {
  return fetchWrapperUserSession(METHOD.GET, 'all', '', sessionId)
}

export async function attachSession(sessionId: string) {
  return fetchWrapperUserSession(METHOD.GET, 'start', '', sessionId)
}

export async function removeSession(sessionId: string) {
  return fetchWrapperUserSession(METHOD.GET, 'logout', '', sessionId)
}
