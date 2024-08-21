import React, { useContext, useState } from 'react';
import { Header } from '../components/header/Header';
import { DrinkHistory } from '../model/DrinkHistory';
import { Button } from '../components/button/Button';
import { View } from './View';
import { HistoryContext, ViewContext } from '../context/Contexts';
import styles from './HistoryView.module.css';

interface HistoryViewProps {
  historyDays: Map<string, DrinkHistory[]>;
}

export interface DateAndTime {
  date: string;
  time: string;
}

export function HistoryView(props: HistoryViewProps) {
  const { setActiveView } = useContext(ViewContext);
  const { setSelectedHistory } = useContext(HistoryContext);

  const handleChooseHistory = (dateAndTime: string) => {
    setSelectedHistory(props.historyDays.get(dateAndTime) || []);
    setActiveView(View.OneHistory);
  };

  return (
    <div className={styles.HistoryView}>
      <Header showLogo={true} />
      <ol className={styles.HistoryList}>
        {Array.from(props.historyDays.keys()).map((dateAndTime, idx) => {
          const { date, time } = JSON.parse(dateAndTime);
          return (
            <li key={`history-${idx}`} id={`history-${idx}`}>
              <Button
                styles=""
                text={`Beendet am ${date} um ${time}`}
                callback={() => handleChooseHistory(dateAndTime)}
              />
            </li>
          );
        })}
      </ol>
      <Button
        styles=""
        text="home"
        callback={() => setActiveView(View.Playground)}
      />
    </div>
  );
}
