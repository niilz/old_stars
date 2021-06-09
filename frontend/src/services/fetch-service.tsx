import { API_URL, METHOD } from '../Constants';

const baseHeaders = new Headers();
baseHeaders.set('Accept', 'application/json');
baseHeaders.set('Content-Type', 'application/json');

export function fetchWrapper(method: METHOD, endpoint: String, body: string) {
  const options: RequestInit = {
    method: method,
    headers: baseHeaders,
    mode: 'cors',
    cache: 'default',
    body: body,
  };
  return fetch(`${API_URL}/${endpoint}`, options)
    .then((body) => body.text())
    .then((json) => console.log(json))
    .then(() => true);
}
