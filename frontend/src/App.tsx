import React, { useState } from 'react';
import { Login } from './components/login/Login';
import { Main } from './components/main/Main';
import './global.css';
import { Modal } from './components/modal/Modal';
import { LoginState } from './Constants';

interface AdminContextI {
  isAdminLoginOpen: boolean;
  setAdminLoginOpen: (flag: boolean) => void;
  isAdminViewOpen: boolean;
  setAdminViewOpen: (flag: boolean) => void;
}

export const AdminContext = React.createContext({
  isAdminLoginOpen: false,
  setAdminLoginOpen: (_flag: boolean) => {},
  isAdminViewOpen: false,
  setAdminViewOpen: (_flag: boolean) => {},
});

function App() {
  const [isAdminLoginOpen, setAdminLoginOpen] = useState(false);
  const [isAdminViewOpen, setAdminViewOpen] = useState(false);

  const handleAdminLogin = (loginState: LoginState) => {
    if (loginState !== LoginState.LoggedInAdmin)
      throw 'Only Admin should be able to log in as admin';
    setAdminLoginOpen(false);
    setAdminViewOpen(true);
  };

  return (
    <AdminContext.Provider
      value={{
        isAdminLoginOpen,
        setAdminLoginOpen,
        isAdminViewOpen,
        setAdminViewOpen,
      }}
    >
      <Main />
      {isAdminLoginOpen && (
        <Modal children={<Login onLogin={handleAdminLogin} />} />
      )}
    </AdminContext.Provider>
  );
}

export default App;
