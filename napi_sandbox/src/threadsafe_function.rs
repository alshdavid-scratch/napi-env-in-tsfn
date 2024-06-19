use napi::bindgen_prelude::FromNapiValue;
use napi::threadsafe_function::ErrorStrategy;
use napi::threadsafe_function::ThreadsafeFunction;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;
use napi::Env;
use napi::JsUnknown;
use napi::Status;
use napi::ValueType;

#[allow(dead_code)]
pub fn call_with_return_value_and_env<D, F, T, ES>(
  tsfn: ThreadsafeFunction<T, ES>,
  value: T,
  mode: ThreadsafeFunctionCallMode,
  cb: F,
) -> Status
where
  D: FromNapiValue,
  F: 'static + FnOnce(D, Env) -> napi::Result<()>,
  T: 'static,
  ES: ErrorStrategy::T,
{
  let callback = move |incoming: napi::Result<JsUnknown>| {
    match incoming {
      Ok(incoming) => {
        // Private structs are copied into this module and their values transmuted into
        // their equivalents. This is done to gain access to the private properties
        let incoming_ptr = &incoming as *const JsUnknown;
        let incoming_t =
          unsafe { std::mem::transmute::<*const JsUnknown, &JsUnknownCustom>(incoming_ptr) };

        // The rest is all taken from napi-rs
        let env_ptr = incoming_t.0.env;
        let value_ptr = incoming_t.0.value;

        let value = unsafe { D::from_napi_value(env_ptr, value_ptr)? };
        let env = unsafe { Env::from_raw(env_ptr) };

        cb(value, env)
      }
      Err(error) => Err(error),
    }
  };

  let data = ThreadsafeFunctionCallJsBackData {
    data: value,
    call_variant: 1,
    callback: Box::new(callback),
  };

  unsafe {
    // This is the underlying C++ napi call
    napi::sys::napi_call_threadsafe_function(
      tsfn.raw(),
      Box::into_raw(Box::new(data)).cast(),
      mode.into(),
    )
    .into()
  }
}

// These are from napi-rs and are private
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct ValueCustom {
  pub env: napi::sys::napi_env,
  pub value: napi::sys::napi_value,
  pub value_type: ValueType,
}

#[allow(dead_code)]
pub struct JsUnknownCustom(pub ValueCustom);

#[allow(dead_code)]
struct ThreadsafeFunctionCallJsBackData<T> {
  data: T,
  call_variant: usize,
  callback: Box<dyn FnOnce(napi::Result<JsUnknown>) -> napi::Result<()>>,
}
