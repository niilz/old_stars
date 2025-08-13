import { METHOD } from '../Constants'
import { fetchWrapperUserSession } from './fetch-service'

export function historizeDrinks(adminSessionToken: string) {
  return fetchWrapperUserSession(METHOD.GET, `historize`, '', adminSessionToken)
}

export function fetchHistories(sessionToken: string) {
  return fetchWrapperUserSession(METHOD.GET, `histories`, '', sessionToken)
}

export function storeHistory(
  adminSessionToken: string,
  archiveDataCsv: string
) {
  return fetchWrapperUserSession(
    METHOD.POST,
    `history`,
    archiveDataCsv,
    adminSessionToken
  )
}
