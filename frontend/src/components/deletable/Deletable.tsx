type DeletableProps = {
  id: number;
  text: string;
  deleteGotClicked: () => void;
};

export function DeletableListItem(props: DeletableProps) {
  return (
    <li>
      <div className="deletable-wrapper">
        <div>{props.text}</div>
        <button onClick={props.deleteGotClicked}>❌</button>
      </div>
    </li>
  );
}
