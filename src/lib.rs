#![allow(unused)]
use human_bytes::human_bytes;
use neon::prelude::*;
use sysinfo::{System, SystemExt};

struct Memory{
  total_memory: String,
  free_memory: String,
  available_memory: String,
  used_memory: String
}

impl Memory{
  fn to_object<'a>(&self, cx: &mut impl Context<'a>) -> JsResult<'a ,JsObject>{
    let obj = cx.empty_object();

    let total_memory = cx.string(&self.total_memory);
    obj.set(cx, "totalMemory", total_memory);

    let free_memory = cx.string(&self.free_memory);
    obj.set(cx, "freeMemory", free_memory);

    let available_memory = cx.string(&self.available_memory);
    obj.set(cx, "availableMemory", available_memory);

    let used_memory = cx.string(&self.used_memory);
    obj.set(cx, "usedMemory", used_memory);

    Ok(obj)
  }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()>{
  let system = System::new_all();

  let memory_info = Memory{
    total_memory: human_bytes(system.total_memory() as f64),
    free_memory: human_bytes(system.free_memory() as f64),
    available_memory: human_bytes(system.available_memory() as f64),
    used_memory: human_bytes(system.used_memory() as f64)
  };

  let uptime = cx.string(compound_duration::format_dhms(system.uptime()));

  let memory_obj = memory_info.to_object(&mut cx)?;

  cx.export_value("memoryInfo", memory_obj);
  cx.export_value("uptime", uptime);
  Ok(())
}