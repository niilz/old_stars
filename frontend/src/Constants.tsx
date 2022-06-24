export const API_URL_DEV = 'http://localhost:8000';
export const API_URL =
  'http://ec2-3-73-55-110.eu-central-1.compute.amazonaws.com:8000';
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
