import React, { useContext, useState } from 'react'
import {
  CLUB_TOKEN_HEADER_NAME,
  SESSION_TOKEN_HEADER_NAME,
} from '../../Constants'
import { SessionData, User } from '../../model/User'
import AuthService from '../../services/auth-service'
import { insertUser } from '../../services/user-service'
import { Button } from '../button/Button'
import { MsgType } from '../message/Message'
import styles from './RegisterLoginForm.module.css'
import { GlobalKeyValueStoreContext, UserContext } from '../../context/Contexts'
import { readErrorMessage } from '../../model/Error'

interface RegisterLoginFormProps {
  onRegister: (user: User) => void
  onLogin: (userName: string) => void
  styles?: string
  onError: (type: MsgType, msg: string) => void
}

export function RegisterLoginForm(props: RegisterLoginFormProps) {
  const { setSessionUser } = useContext(UserContext)
  const { keyValueStore } = useContext(GlobalKeyValueStoreContext)

  const [userName, setUserName] = useState('')
  const [pwd, setPwd] = useState('')

  const handleRegister = async () => {
    try {
      const clubToken = keyValueStore.tryReadFromStorage(CLUB_TOKEN_HEADER_NAME)
      const newUser = await insertUser({ name: userName, pwd }, clubToken)
      props.onRegister(newUser as User)
      setUserName('')
      setPwd('')
    } catch (err) {
      props.onError(MsgType.ERR, readErrorMessage(err).msg)
    }
  }

  const login = () => {
    const clubToken = keyValueStore.tryReadFromStorage(CLUB_TOKEN_HEADER_NAME)
    AuthService.loginUser(
      {
        name: userName,
        pwd: pwd,
      },
      clubToken
    )
      .then((sessionData) => {
        setUserName('')
        setPwd('')
        const sessionDataCasted = sessionData as SessionData
        setSessionUser(sessionDataCasted.user)
        keyValueStore.storeItem(
          SESSION_TOKEN_HEADER_NAME,
          sessionDataCasted.sessionId
        )
        props.onLogin(sessionDataCasted.user.name)
      })
      .catch((err) => props.onError(MsgType.ERR, readErrorMessage(err).msg))
  }

  return (
    <>
      <form
        onSubmit={preventFormSubmission}
        className={`${styles.RegisterLoginForm}`}
      >
        <input
          type="text"
          placeholder="user-name"
          value={userName}
          onChange={(e) => setUserName(e.target.value)}
        />
        <input
          type="password"
          value={pwd}
          placeholder={'password'}
          onChange={(e) => setPwd(e.target.value)}
        />
        <Button text="Login" styles={styles.registerBtn} callback={login} />
        <Button
          text="Register"
          styles={styles.registerBtn}
          callback={handleRegister}
        />
      </form>
    </>
  )
}

function preventFormSubmission(e: React.FormEvent) {
  e.preventDefault()
}
