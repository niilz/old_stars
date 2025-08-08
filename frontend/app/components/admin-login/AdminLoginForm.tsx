import React, { useContext, useState } from 'react'
import {
  CLUB_TOKEN_HEADER_NAME,
  SESSION_TOKEN_HEADER_NAME,
} from '../../Constants'
import AuthService from '../../services/auth-service'
import { Button } from '../button/Button'
import { MsgType } from '../message/Message'
import styles from './AdminLoginForm.module.css'
import { GlobalKeyValueStoreContext } from '../../context/Contexts'
import { View } from '../../views/View'
import { SessionData, UserCredentials } from '../../model/User'

interface AdminLoginFormProps {
  onLogin: (nextView: View) => void
  styles?: string
  onError: (type: MsgType, msg: string) => void
  userName: string
}

export function AdminLoginForm(props: AdminLoginFormProps) {
  const [pwd, setPwd] = useState('')
  const { keyValueStore } = useContext(GlobalKeyValueStoreContext)

  const login = async () => {
    console.log('calling login')
    const freshSessionToken = await AuthService.loginUser(
      { name: props.userName, pwd },
      keyValueStore.tryReadFromStorage(CLUB_TOKEN_HEADER_NAME)
    )
    console.log('got a fresh token')
    setPwd('')
    const token = freshSessionToken as SessionData
    keyValueStore.storeItem(SESSION_TOKEN_HEADER_NAME, token.sessionId)

    const isAdmin = await AuthService.isAdmin(token.sessionId)
    if (isAdmin === true) {
      console.log('Admin-Login succeeded')
      props.onLogin(View.AdminConsole)
    } else {
      console.log(`user ${props.userName} is no admin`)
    }
  }

  return (
    <>
      <form
        onSubmit={preventFormSubmission}
        className={`${styles.RegisterLoginForm}`}
      >
        <input
          type="password"
          value={pwd}
          placeholder={'password'}
          onChange={(e) => setPwd(e.target.value)}
        />
        <Button
          text="Login"
          styles={styles.registerBtn}
          callback={() => login().catch((e) => props.onError(MsgType.ERR, e))}
        />
      </form>
    </>
  )
}

function preventFormSubmission(e: React.FormEvent) {
  e.preventDefault()
}
