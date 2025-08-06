import { useContext } from 'react';
import { AppCtx } from '../../App';
import { AppLogo } from '../logo/Logo';
import styles from './Header.module.css';

interface HeaderProps {
  showLogo: boolean;
  styles?: { headerStripes: string; title: string };
}

export function Header(props: HeaderProps) {
  let { appHeight } = useContext(AppCtx);
  return (
    <header className={styles.Header}>
      {props.showLogo ? <AppLogo styles={styles.logo} /> : null}
      <div
        className={`${styles.headerStripes} ${
          props.styles ? props.styles.headerStripes : ''
        }`}
      >
        <h1
          className={`${styles.title} ${
            props.styles ? props.styles.title : ''
          }`}
        >
          Old-Stars App
        </h1>
      </div>
    </header>
  );
}
