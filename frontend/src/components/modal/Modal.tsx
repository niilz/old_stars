import { ReactChild, ReactChildren } from 'react';
import styles from './Modal.module.css';

interface ModalProps {
  children: ReactChild;
}
export function Modal(props: ModalProps) {
  return <div className={styles.Modal}>{props.children}</div>;
}
