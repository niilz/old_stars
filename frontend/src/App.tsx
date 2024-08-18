import React, { useState, useEffect } from 'react';
import { Login } from './components/login/Login';
import { Main } from './components/main/Main';
import './global.css';
import { Modal } from './components/modal/Modal';
import { LoginState, LoginType } from './Constants';

export const AppCtx = React.createContext({
  isAdminLoginOpen: false,
  setAdminLoginOpen: (_flag: boolean) => {},
  isAdminViewOpen: false,
  setAdminViewOpen: (_flag: boolean) => {},
  loginType: LoginType.Club,
  setLoginType: (_lg: LoginType) => {},
  appHeight: 0,
});

function App() {
  const [isAdminLoginOpen, setAdminLoginOpen] = useState(false);
  const [isAdminViewOpen, setAdminViewOpen] = useState(false);
  const [loginType, setLoginType] = useState(LoginType.Club);
  const [appHeight, _setAppHeight] = useState(window.innerHeight);

  const handleAdminLogin = (loginState: LoginState) => {
    if (loginState !== LoginState.LoggedInAdmin) {
      throw 'Only Admin should be able to log in as admin';
    }
    setAdminLoginOpen(false);
    setAdminViewOpen(true);
  };

  return (
    <AppCtx.Provider
      value={{
        isAdminLoginOpen,
        setAdminLoginOpen,
        isAdminViewOpen,
        setAdminViewOpen,
        loginType,
        setLoginType,
        appHeight,
      }}
    >
      <div className="App" style={{ height: appHeight }}>
        <Main />
        {isAdminLoginOpen && (
          <Modal children={<Login onLogin={handleAdminLogin} />} />
        )}
      </div>
    </AppCtx.Provider>
  );
}

export default App;
