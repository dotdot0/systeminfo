use neon::prelude::*;
use sysinfo::{Disk, DiskType};

pub struct DiskSys {
    disk_type: DiskType,
    name: String,
    file_system: String,
    mount_point: String,
    total_space: String,
    available_space: String,
    is_removable: bool,
}

impl DiskSys {
    pub fn new(
        disk_type: DiskType,
        name: String,
        file_system: String,
        mount_point: String,
        total_space: String,
        available_space: String,
        is_removable: bool,
    ) -> Self {
        Self {
            disk_type,
            name,
            file_system,
            mount_point,
            total_space,
            available_space,
            is_removable,
        }
    }

    pub fn to_object<'a>(&self, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();

        let disk_type = cx.string(disktype(self.disk_type));
        obj.set(cx, "diskType", disk_type)?;

        let name = cx.string(&self.name);
        obj.set(cx, "name", name)?;

        let file_system = cx.string(&self.file_system);
        obj.set(cx, "fileSystem", file_system)?;

        let mount_point = cx.string(&self.mount_point);
        obj.set(cx, "mountPoint", mount_point);

        let total_space = cx.string(&self.total_space);
        obj.set(cx, "totalSpace", total_space)?;

        let available_space = cx.string(&self.available_space);
        obj.set(cx, "availableSpace", available_space)?;

        let is_removable = cx.boolean(self.is_removable);
        obj.set(cx, "isRemovable", is_removable)?;

        Ok(obj)
    }
}

pub fn vec_to_array_disk<'a>(
    cx: &mut impl Context<'a>,
    disks: Vec<DiskSys>,
) -> JsResult<'a, JsArray> {
    let disk_array = cx.empty_array();

    for (i, disk) in disks.iter().enumerate() {
        let disk_item = disk.to_object(cx)?;
        disk_array.set(cx, i as u32, disk_item)?;
    }

    Ok(disk_array)
}

pub fn disktype(disktype: DiskType) -> String {
    match disktype {
        DiskType::HDD => "HDD".to_string(),
        DiskType::SSD => "SSD".to_string(),
        _ => "Other".to_string(),
    }
}
