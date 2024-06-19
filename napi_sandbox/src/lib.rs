mod threadsafe_function;
use std::sync::mpsc::channel;
use std::thread;

use napi::threadsafe_function::ErrorStrategy;
use napi::threadsafe_function::ThreadsafeFunction;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;
use napi::Env;
use napi::JsFunction;
use napi::JsUndefined;
use napi::JsUnknown;
use napi_derive::napi;

use threadsafe_function::call_with_return_value_and_env;

#[napi]
pub fn foo(
  env: Env,
  callback: JsFunction,
) -> napi::Result<JsUndefined> {
  let tsfn: ThreadsafeFunction<usize, ErrorStrategy::Fatal> =
    callback.create_threadsafe_function(0, |_ctx| Ok(Vec::<JsUndefined>::new()))?;

  thread::spawn(move || {
    let (tx, rx) = channel();

    call_with_return_value_and_env(
      tsfn,
      42,
      ThreadsafeFunctionCallMode::Blocking,
      move |v: JsUnknown, env: Env| {
        let result = env.from_js_value::<bool, JsUnknown>(v).unwrap();
        tx.send(result).unwrap();
        Ok(())
      },
    );

    println!("{}", rx.recv().unwrap());
  });

  env.get_undefined()
}
