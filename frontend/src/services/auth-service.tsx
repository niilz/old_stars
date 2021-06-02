const authApi = 'http://localhost:8000/';

const baseHeaders = new Headers();
baseHeaders.set('Accept', 'text/plain');

const baseOptions: RequestInit = {
  method: 'GET',
  headers: baseHeaders,
  mode: 'cors',
  cache: 'default',
};

class AuthService {
  static checkPassword(pwd: String) {
    return fetch(authApi, baseOptions)
      .then((xx) => console.log(xx))
      .then(() => true);
  }
}

export default AuthService;
