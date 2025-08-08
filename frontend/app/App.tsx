import React, { useState } from 'react'
import { Login } from './components/login/Login'
import { Main } from './components/main/Main'
import './global.css'
import { Modal } from './components/modal/Modal'
import { LoginState } from './Constants'
import { View } from './views/View'
import { ErrorContext, ViewContext } from './context/Contexts'
import { GlobalError } from './components/error/GlobalError'
import { Button } from './components/button/Button'
import { AdminLoginForm } from './components/admin-login/AdminLoginForm'

export const AppCtx = React.createContext({
  isAdminLoginOpen: false,
  setAdminLoginOpen: (_flag: boolean) => {},
  appHeight: 0,
})

function App() {
  const [appHeight, _setAppHeight] = useState(window.innerHeight)
  const [activeView, setActiveView] = useState(View.ClubLogin)
  const [currentError, setCurrentError] = useState('')

  const [isAdminLoginOpen, setAdminLoginOpen] = useState(false)
  const handleAdminLogin = (loginState: LoginState) => {
    if (loginState !== LoginState.LoggedInAdmin) {
      throw 'Only Admin should be able to log in as admin'
    }
    setActiveView(View.AdminConsole)
    setAdminLoginOpen(false)
  }

  return (
    <AppCtx.Provider
      value={{
        isAdminLoginOpen,
        setAdminLoginOpen,
        appHeight,
      }}
    >
      <ErrorContext.Provider value={{ currentError, setCurrentError }}>
        <ViewContext.Provider value={{ activeView, setActiveView }}>
          <div className="App" style={{ height: appHeight }}>
            <Main />
            <GlobalError />
          </div>
        </ViewContext.Provider>
      </ErrorContext.Provider>
    </AppCtx.Provider>
  )
}

export default App
