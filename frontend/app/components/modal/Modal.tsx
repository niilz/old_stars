import { ReactNode, useContext } from 'react';
import { AppCtx } from '../../App';
import styles from './Modal.module.css';

interface ModalProps {
  children: ReactNode;
}
export function Modal(props: ModalProps) {
  let { appHeight } = useContext(AppCtx);

  return (
    <div className={styles.Modal} style={{ height: appHeight }}>
      {props.children}
    </div>
  );
}
