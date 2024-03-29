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

export interface SessionData {
  user: User;
  sessionId: string;
}

export class OldStar implements User {
  id: number;
  name: string;
  pwd: string;
  beerCount: number;
  shotCount: number;
  waterCount: number;

  static MAX_ALC_TO_WATER_RATIO = 3;

  constructor(user: User) {
    this.id = user.id;
    this.name = user.name;
    this.pwd = user.pwd;
    this.beerCount = user.beerCount;
    this.shotCount = user.shotCount;
    this.waterCount = user.waterCount;
  }

  needsWaterRound(): boolean {
    const consumedAlcohols = this.beerCount + this.shotCount;
    const waterAlcoholRatio = consumedAlcohols / this.waterCount;
    return waterAlcoholRatio > OldStar.MAX_ALC_TO_WATER_RATIO;
  }
}
