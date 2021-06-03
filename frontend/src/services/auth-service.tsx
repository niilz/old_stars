const authApi = 'http://localhost:8000/';

const baseHeaders = new Headers();
baseHeaders.set('Accept', 'application/json');
baseHeaders.set('Content-Type', 'application/json');

const user = {
  user_name: 'test user',
  pwd: 'mega-secret',
};

const baseOptions: RequestInit = {
  method: 'POST',
  headers: baseHeaders,
  mode: 'cors',
  cache: 'default',
  body: JSON.stringify(user),
};

class AuthService {
  static checkPassword(pwd: String) {
    return fetch(`${authApi}login`, baseOptions)
      .then((body) => body.text())
      .then((json) => console.log(json))
      .then(() => true);
  }
}

export default AuthService;
