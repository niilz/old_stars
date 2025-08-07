import { MAX_ALC_TO_WATER_RATIO } from '../Constants'
import { User } from '../model/User'

export function needsWaterRound(user: User): boolean {
  const consumedAlcohols = user.beerCount + user.shotCount
  if (consumedAlcohols === 0) {
    return false
  }
  if (user.waterCount === 0) {
    return consumedAlcohols > MAX_ALC_TO_WATER_RATIO
  }
  if (user.waterCount === 1) {
    return consumedAlcohols - MAX_ALC_TO_WATER_RATIO > MAX_ALC_TO_WATER_RATIO
  }
  const waterAlcoholRatio = consumedAlcohols / user.waterCount
  return waterAlcoholRatio > MAX_ALC_TO_WATER_RATIO
}
