export abstract class GlobalKeyValueStore {
  abstract storeItem(key: string, value: string): void
  abstract removeItem(key: string): void

  abstract readFromStorage(key: string): string | null

  tryReadFromStorage(key: string): string {
    const value = this.readFromStorage(key)
    if (!value) {
      throw `No value for key ${key} present`
    }
    return value
  }
}

export class LocalStorage extends GlobalKeyValueStore {
  readFromStorage(key: string): string | null {
    return window.localStorage.getItem(key)
  }
  removeItem(key: string) {
    window.localStorage.removeItem(key)
  }
  storeItem(key: string, value: string) {
    window.localStorage.setItem(key, value)
  }
}
