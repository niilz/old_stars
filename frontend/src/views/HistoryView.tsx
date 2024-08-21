import React, { useContext } from 'react';
import { Header } from '../components/header/Header';
import { DrinkHistory } from '../model/DrinkHistory';
import { Button } from '../components/button/Button';
import { View } from './View';
import { ViewContext } from '../context/Contexts';

interface HistoryViewProps {
  historyDays: Map<Date, DrinkHistory[]>;
}

export function HistoryView(props: HistoryViewProps) {
  const { setActiveView } = useContext(ViewContext);

  return (
    <div>
      <Header showLogo={true} />
      <ol>
        {new Array(props.historyDays.keys()).map((date, idx) => (
          <li key={`history-${idx}`} id={`history-${idx}`}>
            {date.toString()}
          </li>
        ))}
      </ol>
      <Button
        styles=""
        text="home"
        callback={() => setActiveView(View.Playground)}
      />
    </div>
  );
}
