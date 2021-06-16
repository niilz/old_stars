import { useState } from 'react';
import { User } from '../../model/User';
import { AdminConsole } from '../admin/AdminConsole';
import { Button } from '../button/Button';
import { Header } from '../header/Header';
import { Login, LoginState, LoginType } from '../login/Login';
import { AppLogo } from '../logo/Logo';
import styles from './Main.module.css';

interface MainProps {
  onLogin: (state: LoginState) => void;
  loginType: LoginType;
  setSessionUser: (user: User) => void;
  onRegister: (user: User) => void;
  users: User[];
  deleteUser: (id: Number) => void;
  openAdminLogin: (flag: boolean) => void;
  isAdminView: boolean;
  setAdminView: (flag: boolean) => void;
}

export function Main(props: MainProps) {
  return (
    <div className={styles.Main}>
      {!props.isAdminView ? (
        <>
          <Header
            showLogo={false}
            styles={{
              headerStripes: styles.headerStripes,
              title: styles.title,
            }}
          />
          <AppLogo styles={styles.logo} />
          <Login {...props} />
          {!props.isAdminView && (
            <Button
              text="admin"
              styles={styles.Btn}
              callback={() => props.openAdminLogin(true)}
            />
          )}
        </>
      ) : (
        <AdminConsole
          navToHome={() => props.setAdminView(false)}
          users={props.users}
          onDelete={props.deleteUser}
        />
      )}
    </div>
  );
}
