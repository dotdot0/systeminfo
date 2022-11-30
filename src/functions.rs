use neon::prelude::*;
use sysinfo::{System ,SystemExt};

pub fn get_cpu_physical_core_count(mut cx: FunctionContext) -> JsResult<JsNumber>{
  let system = System::new_all();
  let count = cx.number(system.physical_core_count().unwrap() as f64);
  Ok(count)
} 