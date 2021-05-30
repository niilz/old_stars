import React, { useState } from 'react';

type LoginProps = {
  login: (loginGranted: boolean) => void;
};

function Login(props: LoginProps) {
  const [password, setPassword] = useState('');

  const handleLogin = (e: React.MouseEvent) => {
    e.preventDefault();
    const passwordOk = PasswordService.checkPassword(password);
    props.login(passwordOk);
  };

  return (
    <form>
      <input
        //type="password"
        type="text"
        placeholder="Master Passwort"
        onChange={(e) => setPassword(e.target.value)}
      />
      <button onClick={handleLogin}>Login</button>
    </form>
  );
}

export default Login;

class PasswordService {
  static checkPassword(pwd: String) {
    return pwd == 'geheim';
  }
}
