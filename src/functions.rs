use neon::prelude::*;
use sysinfo::{System ,SystemExt};

//Physical CPU Core count
pub fn get_cpu_physical_core_count(mut cx: FunctionContext) -> JsResult<JsNumber>{
  let system = System::new_all();
  let count = cx.number(system.physical_core_count().unwrap() as f64);
  Ok(count)
}

//Returns the system name
pub fn host_name(mut cx: FunctionContext) -> JsResult<JsString>{
  let system = System::new_all();
  let name = cx.string(system.name().unwrap());
  Ok(name)
}

pub fn kernel_v(mut cx: FunctionContext) -> JsResult<JsString>{
  let system = System::new_all();
  let version = cx.string(system.kernel_version().unwrap());
  Ok(version)
}

pub fn long_os_v(mut cx: FunctionContext) -> JsResult<JsString>{
  let system = System::new_all();
  let version = cx.string(system.long_os_version().unwrap());
  Ok(version)
}