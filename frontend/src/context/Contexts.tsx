import React from 'react';
import { User } from '../model/User';
import { LoginState } from '../Constants';
import { View } from '../views/View';

export const UserContext = React.createContext({
  addUser: (_user: User) => {},
  setSessionUser: (_user: User) => {},
});

export const LoginContext = React.createContext({
  loginState: LoginState.LoggedOut,
  setLoginState: (_: LoginState) => {},
});

export const ViewContext = React.createContext({
  activeView: View.ClubLogin,
  setActiveView: (view: View) => {},
});
