import { METHOD } from '../Constants';
import { fetchWrapper, handleResponse } from './fetch-service';
import { UserCredentials } from '../model/User';

class AuthService {
  static async loginUser(user: UserCredentials) {
    const loginResponse = await fetchWrapper(
      METHOD.POST,
      'login',
      JSON.stringify(user)
    );

    const loggedInUser = handleResponse(loginResponse);
    return loggedInUser;
  }
}

export default AuthService;
