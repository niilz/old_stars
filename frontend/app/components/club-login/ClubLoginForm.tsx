import React, { useContext, useState } from 'react'
import AuthService from '../../services/auth-service'
import { Button } from '../button/Button'
import { MsgType } from '../message/Message'
import styles from './ClubLoginForm.module.css'
import { GlobalKeyValueStoreContext } from '../../context/Contexts'
import { CLUB_TOKEN_HEADER_NAME, LoginState } from '../../Constants'

interface ClubLoginFormProps {
  onLogin: () => void
  styles?: string
  onError: (type: MsgType, msg: string) => void
}

export function ClubLoginForm(props: ClubLoginFormProps) {
  const [pwd, setPwd] = useState('')
  const { keyValueStore } = useContext(GlobalKeyValueStoreContext)

  const login = () => {
    AuthService.clubLogin(pwd)
      .then((clubToken) => {
        setPwd('')
        if (typeof clubToken === 'string') {
          keyValueStore.storeItem(CLUB_TOKEN_HEADER_NAME, clubToken)
          props.onLogin()
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
