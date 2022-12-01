const sys = require("./index");

console.log(sys.memoryInfo);

console.log(sys.uptime);

console.log(sys.disks);

console.log(sys.physicalCoreCount());

console.log(sys.systemName());

console.log(sys.kernelVersion());

console.log(sys.osVersion());