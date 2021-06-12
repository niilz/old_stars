import React, { useState } from 'react';
import { insertUser } from '../../services/user-service';

export function RegistrationForm() {
  const [userName, setUserName] = useState('');
  const [pwd, setPwd] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    insertUser({ user_name: userName, pwd });
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
