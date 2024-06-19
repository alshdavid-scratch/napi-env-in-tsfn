import * as napi from '@workspace/napi_sandbox'

const callback = (...input) => {
  console.log(input)
  return true
}

napi.default.foo(callback)

// setTimeout(() => {}, 5000)