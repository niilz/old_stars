import { User } from '../../model/User';
import { deleteUser } from '../../services/user-service';
import { Button } from '../button/Button';
import { Header } from '../header/Header';
import { RegisterLoginForm } from '../register-login-form/RegisterLoginForm';
import { UserList } from '../user-list/UserList';
import styles from './AdminConsole.module.css';

interface AdminConsoleProps {
  navToHome: () => void;
  users: User[];
  onRegister: (users: User) => void;
  onDelete: (id: Number) => void;
  isAdminView: boolean;
}

export function AdminConsole(props: AdminConsoleProps) {
  const deleteUserFromList = (id: number) => {
    deleteUser(id);
    props.onDelete(id);
  };
  return (
    <div className={styles.AdminConsole}>
      <Header showLogo={true} />
      <UserList
        users={props.users}
        isEditable={true}
        onDelete={deleteUserFromList}
      />
      <RegisterLoginForm
        isMasterLogin={false}
        onRegister={props.onRegister}
        isAdminView={props.isAdminView}
      />
      <Button text="Home" styles={styles.Btn} callback={props.navToHome} />
    </div>
  );
}
