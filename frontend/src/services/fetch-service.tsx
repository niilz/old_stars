import { API_URL, METHOD } from '../Constants';

const baseHeaders = new Headers();
baseHeaders.set('Accept', 'application/json');
baseHeaders.set('Content-Type', 'application/json');

export async function fetchWrapper(
  method: METHOD,
  endpoint: String,
  body: string
) {
  const options: RequestInit = {
    method: method,
    headers: baseHeaders,
    mode: 'cors',
    cache: 'default',
  };
  if (method === METHOD.POST) {
    options.body = body;
  }

  const apiRes = await fetch(`${API_URL}/${endpoint}`, options);
  const resJson = await apiRes.json();
  return resJson;
}
