export const API_URL_DEV = 'http://localhost:8000'
export const API_URL = 'https://niilz.de'
export const API_URL_LOCAL_NET = 'https://oldstars-backend.ngrok.io'
export const SESSION_TOKEN_HEADER_NAME = 'old-star-user-session'
export const MAX_ALC_TO_WATER_RATIO = 3

export enum METHOD {
  GET = 'GET',
  POST = 'POST',
  DELETE = 'DELETE',
  HEAD = 'HEAD',
  OPTIONS = 'OPTIONS',
}

export enum LoginState {
  LoggedInClub,
  LoggedInAdmin,
  LoggedInUser,
  LoggedOut,
  LoginError,
}
