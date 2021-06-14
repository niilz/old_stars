import { METHOD } from '../Constants';
import { fetchWrapper } from './fetch-service';
import { UserCredentials } from '../model/User';

class AuthService {
  static async loginUser(user: UserCredentials) {
    const loginResponse = await fetchWrapper(
      METHOD.POST,
      'login',
      JSON.stringify(user)
    );

    const { Ok } = loginResponse;
    const { Err } = loginResponse;
    if (Ok) {
      const user = Ok;
      return user;
    }
    const errMessage = Err;
    throw `Ooops... ${errMessage}`;
  }
}

export default AuthService;
