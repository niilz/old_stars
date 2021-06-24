import React, { useState } from 'react';
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
});

function App() {
  const [isAdminLoginOpen, setAdminLoginOpen] = useState(false);
  const [isAdminViewOpen, setAdminViewOpen] = useState(false);
  const [loginType, setLoginType] = useState(LoginType.Club);

  const handleAdminLogin = (loginState: LoginState) => {
    if (loginState !== LoginState.LoggedInAdmin)
      throw 'Only Admin should be able to log in as admin';
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
      }}
    >
      <Main />
      {isAdminLoginOpen && (
        <Modal children={<Login onLogin={handleAdminLogin} />} />
      )}
    </AppCtx.Provider>
  );
}

export default App;
