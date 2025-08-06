import styles from './Message.module.css';

export enum MsgType {
  ERR = 'error',
  INFO = 'info',
  NONE = 'none',
}

interface MessageProps {
  msg: string;
  type: MsgType;
}

export function Message(props: MessageProps) {
  return (
    <p className={`${styles.Message} ${styles[props.type]}`}>{props.msg}</p>
  );
}
