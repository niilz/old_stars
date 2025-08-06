import { useContext } from 'react';
import { Header } from '../components/header/Header';
import { Login } from '../components/login/Login';
import { AppLogo } from '../components/logo/Logo';
import { ViewContext } from '../context/Contexts';
import styles from './UserLoginView.module.css';
import { View } from './View';

export function UserLoginView() {
  const { setActiveView } = useContext(ViewContext);

  return (
    <>
      <Header
        showLogo={false}
        styles={{
          headerStripes: styles.headerStripes,
          title: styles.title,
        }}
      />
      <AppLogo styles={styles.logo} />
      <Login onLogin={() => setActiveView(View.Playground)} />
    </>
  );
}
