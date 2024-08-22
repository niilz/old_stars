import { METHOD } from '../Constants';
import { fetchWrapper } from './fetch-service';

export function historizeDrinks() {
  return fetchWrapper(METHOD.GET, `historize`, '');
}

export function fetchHistories() {
  return fetchWrapper(METHOD.GET, `histories`, '');
}
