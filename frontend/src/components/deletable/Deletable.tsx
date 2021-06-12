import React, { ReactChild } from 'react';
import { deleteUser } from '../../services/user-service';

type DeletableProps = {
  id: Number;
  text: String;
};

export function DeletableListItem(props: DeletableProps) {
  return (
    <li>
      <div className="deletable-wrapper">
        <div>props.text</div>
        <button onClick={() => deleteUser(props.id)}>❌</button>
      </div>
    </li>
  );
}
