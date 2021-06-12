import { METHOD } from '../Constants';
import { fetchWrapper } from './fetch-service';
import { UserCredentials } from '../model/User';

class AuthService {
  static checkPassword(user: UserCredentials) {
    return fetchWrapper(METHOD.POST, 'login', JSON.stringify(user));
  }
}

export default AuthService;
