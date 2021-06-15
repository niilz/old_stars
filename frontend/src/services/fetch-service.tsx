import { API_URL, METHOD } from '../Constants';

const baseHeaders = new Headers();
baseHeaders.set('Accept', 'application/json');
baseHeaders.set('Content-Type', 'application/json');

interface OkRes {
  Ok: Object;
  Err?: never;
}
interface ErrRes {
  Ok?: never;
  Err: Object;
}
type ApiResponse = OkRes | ErrRes;

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

export function handleResponse(res: ApiResponse) {
  const { Ok } = res;
  const { Err } = res;
  if (Ok) {
    const user = Ok;
    return user;
  }
  const errMessage = Err;
  throw `Ooops... ${errMessage}`;
}
