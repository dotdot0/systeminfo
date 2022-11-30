#![allow(unused)]

mod disks;
mod memory;
mod functions;

use human_bytes::human_bytes;
use neon::prelude::*;
use sysinfo::{System, SystemExt, DiskExt};
use disks::{DiskSys, disktype, vec_to_array_disk};
use memory::Memory;
use functions::{get_cpu_physical_core_count, system_name, kernel_v};


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

  //Import to javascript
  cx.export_value("memoryInfo", memory_obj);
  cx.export_value("uptime", uptime);
  cx.export_value("disks", disks_info);

  //Import as javascript version
  cx.export_function("physicalCoreCount", get_cpu_physical_core_count)?;
  cx.export_function("systemName", system_name)?;
  cx.export_function("kernelVersion", kernel_v)?;
  Ok(())
}