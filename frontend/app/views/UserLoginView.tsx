import { useContext } from 'react'
import { Header } from '../components/header/Header'
import { Login } from '../components/login/Login'
import { AppLogo } from '../components/logo/Logo'
import { ViewContext } from '../context/Contexts'
import styles from './UserLoginView.module.css'
import { View } from './View'

interface UserLoginViewProps {
  onLogin: (userName: string) => void
}

export function UserLoginView(props: UserLoginViewProps) {
  const { setActiveView } = useContext(ViewContext)

  const handleLogin = (userName: string) => {
    setActiveView(View.Playground)

    props.onLogin(userName)
  }

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
      <Login onLogin={handleLogin} />
    </>
  )
}
