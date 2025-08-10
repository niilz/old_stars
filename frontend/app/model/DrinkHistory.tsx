import { User } from './User'

export interface DrinkHistory {
  id: number
  timestamp: TimeStamp
  userName: string
  beerCount: number
  shotCount: number
  otherCount: number
  waterCount: number
}

interface TimeStamp {
  nanos_since_epoch: number
  secs_since_epoch: number
}

export function csvToDrinkHistory(csvData: string) {
  return ''
}

export function mapToUser(history: DrinkHistory): User {
  // We use the history as the user-id so that the list has different Keys and IDs
  return {
    id: history.id,
    name: history.userName,
    role: null,
    pwd: '',
    beerCount: history.beerCount,
    shotCount: history.shotCount,
    otherCount: history.otherCount,
    waterCount: history.waterCount,
  }
}

export function mapToDateAndTime(timestamp: TimeStamp) {
  const timeStampAsMillis = timestamp.secs_since_epoch * 1000
  const date = new Date(timeStampAsMillis)
  return {
    date: date.toLocaleDateString(),
    time: date.toLocaleTimeString(),
  }
}
