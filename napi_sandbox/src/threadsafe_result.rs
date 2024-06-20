use napi::bindgen_prelude::FromNapiValue;
use napi::sys::napi_env;
use napi::sys::napi_value;
use napi::Env;
use napi::JsUnknown;

pub struct ThreadsafeResult(pub JsUnknown, pub Env);

impl FromNapiValue for ThreadsafeResult {
  unsafe fn from_napi_value(
    env: napi_env,
    napi_val: napi_value,
  ) -> napi::Result<Self> {
    let value = JsUnknown::from_napi_value(env, napi_val)?;
    let env = Env::from_raw(env);
    Ok(Self(value, env))
  }
}
