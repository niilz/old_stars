import '@testing-library/jest-dom'
import { User } from '../app/model/User'
import { needsWaterRound } from '../app/util/DrinkUtil'
import { MAX_ALC_TO_WATER_RATIO } from '../app/Constants'

describe('User', () => {
  it('User does not have to drink water if max-count is not reached', () => {
    const userDummy = createUserDummy()
    expect(needsWaterRound(userDummy)).toBe(false)
  })
})

describe('User', () => {
  it('User must drink water if max-count is reached', () => {
    const userDummy = createUserDummy()
    userDummy.beerCount = 3
    userDummy.shotCount = 1
    expect(needsWaterRound(userDummy)).toBe(true)
  })
})

describe('User', () => {
  it(`1 water allows for ${MAX_ALC_TO_WATER_RATIO} more drinks`, () => {
    const userDummy = createUserDummy()
    userDummy.beerCount = 4
    expect(needsWaterRound(userDummy)).toBe(true)
    userDummy.waterCount = 1
    expect(needsWaterRound(userDummy)).toBe(false)
  })
})

describe('User', () => {
  const threeTimesMax = MAX_ALC_TO_WATER_RATIO * 3
  it(`3 waters allow for ${threeTimesMax} drinks`, () => {
    const userDummy = createUserDummy()
    userDummy.beerCount = threeTimesMax
    expect(needsWaterRound(userDummy)).toBe(true)
    userDummy.waterCount = 1
    expect(needsWaterRound(userDummy)).toBe(true)
    userDummy.waterCount = 2
    expect(needsWaterRound(userDummy)).toBe(true)
    userDummy.waterCount = 3
    expect(needsWaterRound(userDummy)).toBe(false)
  })
})

function createUserDummy(): User {
  return {
    name: 'Test User',
    pwd: 'password',
    id: 1,
    role: null,
    beerCount: 3,
    shotCount: 0,
    otherCount: 0,
    waterCount: 0,
  }
}
