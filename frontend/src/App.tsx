import React, { useState } from 'react';
import { Login } from './components/login/Login';
import { Main } from './components/main/Main';
import './global.css';
import { Modal } from './components/modal/Modal';
import { LoginState, LoginType } from './Constants';
import { View } from './views/View';
import { ErrorContext, ViewContext } from './context/Contexts';
import { GlobalError } from './components/error/GlobalError';

export const AppCtx = React.createContext({
  isAdminLoginOpen: false,
  setAdminLoginOpen: (_flag: boolean) => {},
  loginType: LoginType.Club,
  setLoginType: (_lg: LoginType) => {},
  appHeight: 0,
});

function App() {
  const [isAdminLoginOpen, setAdminLoginOpen] = useState(false);
  const [loginType, setLoginType] = useState(LoginType.Club);
  const [appHeight, _setAppHeight] = useState(window.innerHeight);
  const [activeView, setActiveView] = useState(View.ClubLogin);
  const [currentError, setCurrentError] = useState('');

  const handleAdminLogin = (loginState: LoginState) => {
    if (loginState !== LoginState.LoggedInAdmin) {
      throw 'Only Admin should be able to log in as admin';
    }
    setActiveView(View.AdminConsole);
    setAdminLoginOpen(false);
  };

  return (
    <AppCtx.Provider
      value={{
        isAdminLoginOpen,
        setAdminLoginOpen,
        loginType,
        setLoginType,
        appHeight,
      }}
    >
      <ErrorContext.Provider value={{ currentError, setCurrentError }}>
        <ViewContext.Provider value={{ activeView, setActiveView }}>
          <div className="App" style={{ height: appHeight }}>
            <Main />
            <GlobalError />
            {isAdminLoginOpen && (
              <Modal children={<Login onLogin={handleAdminLogin} />} />
            )}
          </div>
        </ViewContext.Provider>
      </ErrorContext.Provider>
    </AppCtx.Provider>
  );
}

export default App;
