export type OldStarsError = {
  msg: string
}

export function readErrorMessage(err: unknown): OldStarsError {
  if (!err) {
    return { msg: 'error is null' }
  }
  if (typeof err !== 'object') {
    return { msg: `simple type error: ${err}` }
  }
  let msg
  if ('msg' in err) {
    msg = err.msg
  }
  if ('message' in err) {
    msg = err.message
  }
  if (!msg) {
    return { msg: `unknown message-prop: ${err}` }
  }
  if (typeof msg === 'string') {
    return { msg }
  } else {
    return { msg: `unknwon message-type: ${err}` }
  }
}
