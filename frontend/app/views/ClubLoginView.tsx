import { useContext, useState } from 'react'
import { Header } from '../components/header/Header'
import { AppLogo } from '../components/logo/Logo'
import { ViewContext } from '../context/Contexts'
import styles from './ClubLoginView.module.css'
import { View } from './View'
import { Message, MsgType } from '../components/message/Message'
import { ClubLoginForm } from '../components/club-login/ClubLoginForm'

export function ClubLoginView() {
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
