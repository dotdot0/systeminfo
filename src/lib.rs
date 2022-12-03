#![allow(unused)]

mod utils;

use human_bytes::human_bytes;
use neon::prelude::*;
use sysinfo::{DiskExt, System, SystemExt};
use utils::disks::{disktype, vec_to_array_disk, DiskSys};
use utils::functions::{disks, get_cpu_physical_core_count, host_name, kernel_v, long_os_v, uptime, memory, boot_time};
use utils::memory::Memory;

fn vec_to_array<'a>(vec: &Vec<String>, cx: &mut impl Context<'a>) -> JsResult<'a, JsArray> {
    let a = JsArray::new(cx, vec.len() as u32);

    for (i, s) in vec.iter().enumerate() {
        let v = cx.string(s);
        a.set(cx, i as u32, v);
    }

    Ok(a)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    let system = System::new_all();

    // let disks: Vec<DiskSys> = system.disks().iter().map(|disk|{
    //   DiskSys::new(disk.type_(),
    //   disk.name().to_str().unwrap().to_string(),
    //   String::from_utf8_lossy(disk.file_system()).to_string(),
    //   disk.mount_point().to_str().unwrap().to_string(),
    //   human_bytes(disk.total_space() as f64),
    //   human_bytes(disk.available_space() as f64),
    //   disk.is_removable())
    // }).collect();

    // let disks_info = vec_to_array_disk(&mut cx, disks)?;

    // let memory_info = Memory {
    //     total_memory: human_bytes(system.total_memory() as f64),
    //     free_memory: human_bytes(system.free_memory() as f64),
    //     available_memory: human_bytes(system.available_memory() as f64),
    //     used_memory: human_bytes(system.used_memory() as f64),
    // };

    //Uptime of the system
    // let uptime = cx.string(compound_duration::format_dhms(system.uptime()));
    //Time when the the system first booted
    // let boot_time = cx.string(compound_duration::format_dhms(system.boot_time()));
// 
    // let memory_obj = memory_info.to_object(&mut cx)?;

    // //Import to javascript
    // cx.export_value("memoryInfo", memory_obj);
    // // cx.export_value("uptime", uptime);
    // //  cx.export_value("disks", disks_info);
    // cx.export_value("bootTime", boot_time);

    //Import as javascript version
    cx.export_function("physicalCoreCount", get_cpu_physical_core_count)?;
    cx.export_function("systemName", host_name)?;
    cx.export_function("kernelVersion", kernel_v)?;
    cx.export_function("osVersion", long_os_v)?;
    cx.export_function("disks", disks)?;
    cx.export_function("uptime", uptime)?;
    cx.export_function("memory", memory)?;
    cx.export_function("bootTime", boot_time);
    Ok(())
}
