import styles from './Deletable.module.css';

type DeletableProps = {
  id: number;
  text: string;
  deleteGotClicked: () => void;
};

export function DeletableListItem(props: DeletableProps) {
  return (
    <li>
      <div className={styles.itemContent}>
        <div className={styles.userData}>{props.text}</div>
        <button className={styles.delBtn} onClick={props.deleteGotClicked}>
          ❌
        </button>
      </div>
    </li>
  );
}
