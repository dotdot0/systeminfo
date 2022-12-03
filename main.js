const {osVersion, disks, memory, uptime, bootTime} = require("./lib/index")


console.log(osVersion());
console.log(disks());
console.log(memory());
console.log(uptime());
console.log(bootTime());