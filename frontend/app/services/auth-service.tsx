import { CLUB_TOKEN_HEADER_NAME, METHOD } from '../Constants'
import { fetchWrapper, handleResponse } from './fetch-service'
import { UserCredentials } from '../model/User'

class AuthService {
  static async loginUser(user: UserCredentials, clubToken: string) {
    const loginResponse = await fetchWrapper(
      METHOD.POST,
      'login',
      JSON.stringify(user),
      CLUB_TOKEN_HEADER_NAME,
      clubToken
    )

    const sessionData = handleResponse(loginResponse)
    return sessionData
  }

  static async clubLogin(pwd: string) {
    const clubLogin = { name: 'club', pwd }
    const loginResponse = await fetchWrapper(
      METHOD.POST,
      'club/login',
      JSON.stringify(clubLogin)
    )

    const sessionData = handleResponse(loginResponse)
    return sessionData
  }
}

export default AuthService
