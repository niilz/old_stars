export interface DrinkHistory {
  id: number;
  timestamp: TimeStamp;
  name: string;
  beerCount: number;
  shotCount: number;
  otherCount: number;
  waterCount: number;
}

interface TimeStamp {
  nanos_since_epoch: number;
  secs_since_epoch: number;
}
