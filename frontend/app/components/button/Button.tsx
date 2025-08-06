import styles from './Button.module.css';

interface ButtonProps {
  callback?: () => void;
  text: string;
  styles: string;
}

export function Button(props: ButtonProps) {
  return (
    <button
      className={`${styles.Btn} ${props.styles}`}
      onClick={props.callback}
    >
      {props.text}
    </button>
  );
}
