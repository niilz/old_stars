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

export class OldStar implements User {
  id: number;
  name: string;
  pwd: string;
  beerCount: number;
  shotCount: number;
  waterCount: number;

  constructor(user: User) {
    this.id = user.id;
    this.name = user.name;
    this.pwd = user.pwd;
    this.beerCount = user.beerCount;
    this.shotCount = user.shotCount;
    this.waterCount = user.waterCount;
  }

  needsWaterRound(): boolean {
    return true;
  }
}
