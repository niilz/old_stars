import React, { useContext, useState } from 'react';
import { Header } from '../components/header/Header';
import { DrinkHistory } from '../model/DrinkHistory';
import { Button } from '../components/button/Button';
import { View } from './View';
import { HistoryContext, ViewContext } from '../context/Contexts';
import styles from './HistoryView.module.css';

interface HistoryViewProps {
  historyDays: Map<Date, DrinkHistory[]>;
}

export function HistoryView(props: HistoryViewProps) {
  const { setActiveView } = useContext(ViewContext);
  const { setSelectedHistory } = useContext(HistoryContext);

  const handleChooseHistory = (date: Date) => {
    setSelectedHistory(props.historyDays.get(date) || []);
    setActiveView(View.OneHistory);
  };

  return (
    <div className={styles.HistoryView}>
      <Header showLogo={true} />
      <ol className={styles.HistoryList}>
        {Array.from(props.historyDays.keys()).map((date, idx) => (
          <li key={`history-${idx}`} id={`history-${idx}`}>
            <Button
              styles=""
              text={`Beendet am ${date.toLocaleDateString()} um ${date.toLocaleTimeString()}`}
              callback={() => handleChooseHistory(date)}
            />
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
