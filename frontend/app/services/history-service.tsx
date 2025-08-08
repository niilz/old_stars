import { METHOD } from '../Constants'
import { fetchWrapperUserSession } from './fetch-service'

export function historizeDrinks(sessionToken: string) {
  return fetchWrapperUserSession(METHOD.GET, `historize`, '', sessionToken)
}

export function fetchHistories(sessionToken: string) {
  return fetchWrapperUserSession(METHOD.GET, `histories`, '', sessionToken)
}
