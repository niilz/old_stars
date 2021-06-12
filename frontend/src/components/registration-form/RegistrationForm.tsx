import React, { useState } from 'react';
import { User } from '../../model/User';
import { insertUser } from '../../services/user-service';

interface RegistrationFormProps {
  onNewUser: (user: User) => void;
}

export function RegistrationForm(props: RegistrationFormProps) {
  const [userName, setUserName] = useState('');
  const [pwd, setPwd] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const newUser = await insertUser({ name: userName, pwd });
    console.log('registerd The User:', newUser);
    props.onNewUser(newUser);
    setUserName('');
    setPwd('');
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
    </>
  );
}
