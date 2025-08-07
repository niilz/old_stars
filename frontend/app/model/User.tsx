export interface User {
  id: number
  name: string
  role: string | null
  pwd: string
  beerCount: number
  shotCount: number
  otherCount: number
  waterCount: number
}

export interface UserCredentials {
  name: string
  pwd: string
}

export interface SessionData {
  user: User
  sessionId: string
}

export class OldStar implements User {
  id: number
  name: string
  role: string | null
  pwd: string
  beerCount: number
  shotCount: number
  otherCount: number
  waterCount: number

  constructor(user: User) {
    this.id = user.id
    this.name = user.name
    this.role = user.role
    this.pwd = user.pwd
    this.beerCount = user.beerCount
    this.shotCount = user.shotCount
    this.otherCount = user.otherCount
    this.waterCount = user.waterCount
  }
}
