import React from 'react'
import { User } from '../model/User'
import { LoginState } from '../Constants'
import { View } from '../views/View'
import { DrinkHistory } from '../model/DrinkHistory'
import { GlobalKeyValueStore, LocalStorage } from '../util/storage-util'

export const GlobalKeyValueStoreContext = React.createContext({
  keyValueStore: new LocalStorage(),
  setKeyValueStore: (_store: GlobalKeyValueStore) => {},
})

export const UserContext = React.createContext({
  addUser: (_user: User) => {},
  setSessionUser: (_user: User) => {},
})

export const LoginContext = React.createContext({
  loginState: LoginState.LoggedOut,
  setLoginState: (_: LoginState) => {},
})

export const ViewContext = React.createContext({
  activeView: View.ClubLogin,
  setActiveView: (_view: View) => {},
})

export const ErrorContext = React.createContext({
  currentError: '',
  setCurrentError: (_error: string) => {},
})

export const HistoryContext = React.createContext({
  selectedHistory: new Array(),
  setSelectedHistory: (_selectedHistory: DrinkHistory[]) => {},
})
