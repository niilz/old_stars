import { MAX_ALC_TO_WATER_RATIO } from '../Constants'
import { User } from '../model/User'

export function needsWaterRound(user: User): boolean {
  const consumedAlcohols = user.beerCount + user.shotCount
  if (consumedAlcohols < MAX_ALC_TO_WATER_RATIO) {
    return false
  }
  const maxAlcAllowed =
    user.waterCount * MAX_ALC_TO_WATER_RATIO + MAX_ALC_TO_WATER_RATIO
  console.log({ maxAlcAllowed })
  return consumedAlcohols > maxAlcAllowed
}
