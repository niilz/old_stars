import { METHOD } from '../Constants';
import { fetchWrapper } from './fetch-service';
import { User } from '../model/User';

class AuthService {
  static checkPassword(user: User) {
    return fetchWrapper(METHOD.POST, 'login', JSON.stringify(user));
  }
}

export default AuthService;
