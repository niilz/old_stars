import { useContext, useState } from 'react'
import { LoginState } from '../Constants'
import { Header } from '../components/header/Header'
import { Login } from '../components/login/Login'
import { AppLogo } from '../components/logo/Logo'
import { ViewContext } from '../context/Contexts'
import styles from './ClubLoginView.module.css'
import { View } from './View'
import { ClubLoginForm } from '../components/club-form/ClubLoginForm'
import { Message, MsgType } from '../components/message/Message'

interface ClubLoginViewProps {}

export function ClubLoginView(props: ClubLoginViewProps) {
  //return {!props.isAdminViewOpen ? (
  //{showBigHeaderAndStar(props.isAdminViewOpen, props.loginState) && (
  const { setActiveView } = useContext(ViewContext)
  const [message, setMessage] = useState('')
  const [msgType, setMsgType] = useState(MsgType.NONE)

  const handleError = (msgType: MsgType, msg: string) => {
    setMessage(msg)
    setMsgType(msgType)
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
      <Message msg={message} type={msgType} />
      <ClubLoginForm
        onLogin={() => setActiveView(View.UserLogin)}
        onError={handleError}
      />
    </>
  )
}

function showBigHeaderAndStar(isAdminView: boolean, ls: LoginState) {
  return !isAdminView && ls !== LoginState.LoggedInUser
}
