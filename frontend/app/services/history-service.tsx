import { METHOD } from '../Constants'
import { fetchWrapperUserSession } from './fetch-service'

export async function historizeDrinks(adminSessionToken: string) {
  return fetchWrapperUserSession(METHOD.GET, `historize`, '', adminSessionToken)
}

export async function fetchHistories(sessionToken: string) {
  return fetchWrapperUserSession(METHOD.GET, `histories`, '', sessionToken)
}

export async function storeHistory(
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
