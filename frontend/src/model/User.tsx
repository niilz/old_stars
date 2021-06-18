export interface User {
  id: number;
  name: string;
  pwd: string;
  beerCount: number;
  shotCount: number;
  waterCount: number;
}

export interface UserCredentials {
  name: string;
  pwd: string;
}
