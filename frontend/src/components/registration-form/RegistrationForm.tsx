import React, { useState } from 'react';
import { insertUser } from '../../services/user-service';

type RegistrationFormProps = {
  navToHome: () => void;
};

export function RegistrationForm(props: RegistrationFormProps) {
  const [userName, setUserName] = useState('');
  const [pwd, setPwd] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    console.log('Admin submission happenend');
    insertUser({ user_name: userName, pwd });
    props.navToHome();
  };
  return (
    <>
      <form onSubmit={handleSubmit} className="RegistrationForm">
        <input
          type="text"
          placeholder="user-name"
          onChange={(e) => setUserName(e.target.value)}
        />
        <input
          //type="password"
          type="text"
          placeholder="password"
          onChange={(e) => setPwd(e.target.value)}
        />
        <button>register</button>
      </form>
      <button onClick={props.navToHome}>back</button>
    </>
  );
}
