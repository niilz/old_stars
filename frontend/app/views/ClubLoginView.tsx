import { useContext } from 'react';
import { LoginState } from '../Constants';
import { Header } from '../components/header/Header';
import { Login } from '../components/login/Login';
import { AppLogo } from '../components/logo/Logo';
import { ViewContext } from '../context/Contexts';
import styles from './ClubLoginView.module.css';
import { View } from './View';

interface ClubLoginViewProps {}

export function ClubLoginView(props: ClubLoginViewProps) {
  //return {!props.isAdminViewOpen ? (
  //{showBigHeaderAndStar(props.isAdminViewOpen, props.loginState) && (
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
      <Login onLogin={() => setActiveView(View.UserLogin)} />
    </>
  );
}

function showBigHeaderAndStar(isAdminView: boolean, ls: LoginState) {
  return !isAdminView && ls !== LoginState.LoggedInUser;
}
