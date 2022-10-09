export const API_URL_DEV = 'http://localhost:8000';
export const API_URL = 'https://niilz.de';
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

export enum LoginType {
  Club,
  User,
  Admin,
  None,
}
