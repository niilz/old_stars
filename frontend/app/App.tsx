import React, { useState } from 'react'
import { Main } from './components/main/Main'
import './global.css'
import { Modal } from './components/modal/Modal'
import { View } from './views/View'
import { ErrorContext, ViewContext } from './context/Contexts'
import { GlobalError } from './components/error/GlobalError'
import { Button } from './components/button/Button'
import { AdminLoginForm } from './components/admin-login/AdminLoginForm'
import { readErrorMessage } from './model/Error'

export const AppCtx = React.createContext({
  isAdminLoginOpen: false,
  setAdminLoginOpen: (_flag: boolean) => {},
  appHeight: 0,
})

function App() {
  const [appHeight, _setAppHeight] = useState(window.innerHeight)
  const [activeView, setActiveView] = useState(View.ClubLogin)
  const [currentError, setCurrentError] = useState('')
  const [userName, setUserName] = useState('')

  const [isAdminLoginOpen, setAdminLoginOpen] = useState(false)
  const handleOnAdminLogin = (view: View) => {
    setActiveView(view)
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
            <Main onUserLogin={setUserName} />
            {isAdminLoginOpen && (
              <Modal
                children={
                  <>
                    <AdminLoginForm
                      onLogin={handleOnAdminLogin}
                      onError={(err) =>
                        setCurrentError(
                          `Admin Login failed: ${readErrorMessage(err).msg}`
                        )
                      }
                      userName={userName}
                    />
                    <Button
                      text={'cancel'}
                      callback={() => {
                        setAdminLoginOpen(false)
                        setCurrentError('')
                      }}
                      styles={''}
                    />
                  </>
                }
              />
            )}
            <GlobalError />
          </div>
        </ViewContext.Provider>
      </ErrorContext.Provider>
    </AppCtx.Provider>
  )
}

export default App
