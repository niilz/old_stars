import { useContext } from 'react';
import styles from './AdminConsole.module.css';
import { User } from '../model/User';
import { ApiResponse } from '../services/fetch-service';
import { ViewContext } from '../context/Contexts';
import { deleteUser } from '../services/user-service';
import { historizeDrinks } from '../services/history-service';
import { Header } from '../components/header/Header';
import { UserList } from '../components/user-list/UserList';
import { Button } from '../components/button/Button';
import { View } from './View';

interface AdminConsoleProps {
  users: User[];
  onDelete: (voidResult: Promise<ApiResponse>) => void;
  onHistorize: (histories: Promise<ApiResponse>) => void;
}

export function AdminConsole(props: AdminConsoleProps) {
  const { setActiveView } = useContext(ViewContext);

  const deleteUserFromList = (id: number) => {
    const voidResult = deleteUser(id);
    props.onDelete(voidResult);
  };

  const handleHistorizeDrinks = () => {
    const historiesResult = historizeDrinks();
    props.onHistorize(historiesResult);
  };

  return (
    <div className={styles.AdminConsole}>
      <Header showLogo={true} />
      <UserList
        users={props.users}
        isEditable={true}
        onDelete={deleteUserFromList}
      />
      <Button
        text="historize"
        styles={styles.Btn}
        callback={handleHistorizeDrinks}
      />
      <Button
        text="Home"
        styles={styles.Btn}
        callback={() => setActiveView(View.Playground)}
      />
    </div>
  );
}
