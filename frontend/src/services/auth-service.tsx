import { METHOD } from '../Constants';
import { fetchWrapper } from './fetch-service';
import { UserCredentials } from '../model/User';

class AuthService {
  static async checkPassword(user: UserCredentials) {
    const isLoginSuccess = await fetchWrapper(
      METHOD.POST,
      'login',
      JSON.stringify(user)
    );
    return isLoginSuccess;
  }
}

export default AuthService;
