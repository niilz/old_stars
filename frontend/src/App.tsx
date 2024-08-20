import React, { useState, useEffect } from 'react';
import { Login } from './components/login/Login';
import { Main } from './components/main/Main';
import './global.css';
import { Modal } from './components/modal/Modal';
import { LoginState, LoginType } from './Constants';
import { ViewContext } from './context/Contexts';
import { View } from './views/View';

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

  const handleAdminLogin = (loginState: LoginState) => {
    if (loginState !== LoginState.LoggedInAdmin) {
      throw 'Only Admin should be able to log in as admin';
    }
    setActiveView(View.Playground);
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
      <ViewContext.Provider value={{ activeView, setActiveView }}>
        <div className="App" style={{ height: appHeight }}>
          <Main />
          {activeView === View.AdminLogin && (
            <Modal children={<Login onLogin={handleAdminLogin} />} />
          )}
        </div>
      </ViewContext.Provider>
    </AppCtx.Provider>
  );
}

export default App;
