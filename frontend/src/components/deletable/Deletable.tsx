import styles from './Deletable.module.css';

type DeletableProps = {
  id: number;
  text: string;
  isEditable: boolean;
  deleteGotClicked?: () => void;
};

export function ListItem(props: DeletableProps) {
  return (
    <li>
      <div className={styles.itemContent}>
        <div className={styles.userData}>{props.text}</div>
        {props.isEditable ? (
          <button className={styles.delBtn} onClick={props.deleteGotClicked}>
            ❌
          </button>
        ) : null}
      </div>
    </li>
  );
}
