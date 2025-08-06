import { useContext } from 'react';
import { ErrorContext } from '../../context/Contexts';
import styles from './GlobalError.module.css';

export function GlobalError() {
  const { currentError } = useContext(ErrorContext);
  return (
    <div className={styles.error}>
      <p>{currentError}</p>
    </div>
  );
}
