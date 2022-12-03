const {osVersion, disks, memory, uptime, bootTime, kernelVersion} = require("./lib/index")


console.log(osVersion());
console.log(kernelVersion());
console.log(disks());
console.log(memory());
console.log(uptime());
console.log(bootTime());