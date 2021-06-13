import styles from './HomeButton.module.css';

interface HomeButtonProps {
  callback: () => void;
}

export function HomeButton(props: HomeButtonProps) {
  return (
    <button className={styles.homeBtn} onClick={props.callback}>
      Home
    </button>
  );
}
