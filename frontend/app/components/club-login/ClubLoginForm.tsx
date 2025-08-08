import React, { useContext, useState } from 'react'
import {
  CLUB_TOKEN_HEADER_NAME,
  SESSION_TOKEN_HEADER_NAME,
} from '../../Constants'
import AuthService from '../../services/auth-service'
import { Button } from '../button/Button'
import { MsgType } from '../message/Message'
import styles from './ClubLoginForm.module.css'
import { GlobalKeyValueStoreContext } from '../../context/Contexts'

interface AdminLoginFormProps {
  onLogin: () => void
  styles?: string
  onError: (type: MsgType, msg: string) => void
}

export function ClubLoginForm(props: AdminLoginFormProps) {
  const [pwd, setPwd] = useState('')
  const { keyValueStore } = useContext(GlobalKeyValueStoreContext)

  const login = () => {
    const sessionId = keyValueStore.tryReadFromStorage(
      SESSION_TOKEN_HEADER_NAME
    )
    AuthService.isAdmin(pwd, sessionId)
      .then((isAdmin) => {
        setPwd('')
        if (isAdmin === true) {
          console.log('user is admin')
          return true
        } else {
          console.log('user is no admin')
          return false
        }
      })
      .catch((e) => props.onError(MsgType.ERR, e))
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
        <Button text="Login" styles={styles.registerBtn} callback={login} />
      </form>
    </>
  )
}

function preventFormSubmission(e: React.FormEvent) {
  e.preventDefault()
}
