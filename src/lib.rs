#![allow(unused)]

mod disks;


use human_bytes::human_bytes;
use neon::prelude::*;
use sysinfo::{System, SystemExt, DiskExt};
use disks::{DiskSys, disktype, vec_to_array_disk};

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

fn vec_to_array<'a>(vec: &Vec<String>, cx: &mut impl Context<'a>) -> JsResult<'a, JsArray>{
  let a = JsArray::new(cx, vec.len() as u32);

  for (i, s) in vec.iter().enumerate(){
    let v = cx.string(s);
    a.set(cx, i as u32, v);
  }

  Ok(a)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()>{
  let system = System::new_all();

  let disks: Vec<DiskSys> = system.disks().iter().map(|disk|{
    DiskSys::new(disk.type_(), 
    disk.name().to_str().unwrap().to_string(), 
    String::from_utf8_lossy(disk.file_system()).to_string(), 
    disk.mount_point().to_str().unwrap().to_string(),
    human_bytes(disk.total_space() as f64), 
    human_bytes(disk.available_space() as f64), 
    disk.is_removable())
  }).collect();

  let disks_info = vec_to_array_disk(&mut cx, disks)?;

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
  cx.export_value("disks", disks_info);
  Ok(())
}