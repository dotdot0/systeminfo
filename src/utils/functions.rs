use crate::{
    utils::disks::{vec_to_array_disk, DiskSys},
    utils::memory::Memory
};
use neon::prelude::*;
use sysinfo::{System, SystemExt, DiskExt};
use human_bytes::human_bytes;

//Physical CPU Core count
pub fn get_cpu_physical_core_count(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let system = System::new_all();
    let count = cx.number(system.physical_core_count().unwrap() as f64);
    Ok(count)
}

//Returns the system name
pub fn host_name(mut cx: FunctionContext) -> JsResult<JsString> {
    let system = System::new_all();
    let name = cx.string(system.name().unwrap());
    Ok(name)
}

//Returns the kernel version
pub fn kernel_v(mut cx: FunctionContext) -> JsResult<JsString> {
    let system = System::new_all();
    let version = cx.string(system.kernel_version().unwrap());
    Ok(version)
}

//Returns the os version
pub fn long_os_v(mut cx: FunctionContext) -> JsResult<JsString> {
    let system = System::new_all();
    let version = cx.string(system.long_os_version().unwrap());
    Ok(version)
}

pub fn disks(mut cx: FunctionContext) -> JsResult<JsArray> {
    let system = System::new_all();
    let disks: Vec<DiskSys> = system
        .disks()
        .iter()
        .map(|disk| {
            DiskSys::new(
                disk.type_(),
                disk.name().to_str().unwrap().to_string(),
                String::from_utf8_lossy(disk.file_system()).to_string(),
                disk.mount_point().to_str().unwrap().to_string(),
                human_bytes(disk.total_space() as f64),
                human_bytes(disk.available_space() as f64),
                disk.is_removable(),
            )
        })
        .collect();
    let disks_array = vec_to_array_disk(&mut cx, disks)?;
    Ok(disks_array)
}

pub fn uptime(mut cx: FunctionContext) -> JsResult<JsString>{
    let system = System::new_all();
    let uptime = cx.string(compound_duration::format_dhms(system.uptime()));
    Ok(uptime)
}

pub fn memory(mut cx: FunctionContext) -> JsResult<JsObject>{
    let system = System::new_all();
    let memory_info = Memory {
        total_memory: human_bytes(system.total_memory() as f64),
        free_memory: human_bytes(system.free_memory() as f64),
        available_memory: human_bytes(system.available_memory() as f64),
        used_memory: human_bytes(system.used_memory() as f64),
    };
    let memory_obj = memory_info.to_object(&mut cx)?;
    Ok(memory_obj)
}

pub fn boot_time(mut cx: FunctionContext) -> JsResult<JsString>{
    let system = System::new_all();
    let boot_time = cx.string(compound_duration::format_dhms(system.boot_time()));
    Ok(boot_time)
}