import '@testing-library/jest-dom'
import { User } from '../app/model/User'
import { needsWaterRound } from '../app/util/DrinkUtil'

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
