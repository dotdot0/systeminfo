# systeminfo 
### A npm module used to get system's information
### Based on sysinfo crate

> **_NOTE:_**  Please Install Rust on your system before installing the module(https://www.rust-lang.org/tools/install)

### Functions Available: 
> **_NOTE:_** ECMAScript modules not supported

```js
const sys = require('systeminfo');

console.log(sys.uptime());//Returns the uptime of the system
//Example Output: 11h48m33s

console.log(sys.bootTime());//Returns the time since the first boot
//Example Output: 19329d3h52m36s

console.log(sys.osVersion());//Returns the operating system version
//Example Output: MacOS 11.2 BigSur

console.log(sys.kernelVersion());//Returns the kernel version
//Example Output: 6.0.0-kali3-amd64

console.log(sys.disks());//Returns the disks list

console.log(sys.memory());//Returns the memory info
/* Example Output: {
  totalMemory: '15.5 GiB',//Total Memory installed on your system
  freeMemory: '2.7 GiB',//free memory refers to the unallocated memory
  availableMemory: '8.1 GiB',//Total memory that is available for (re)use
  usedMemory: '7.3 GiB'//Amount of memory used
} */

console.log(sys.systemName());//Name of the system or host name

console.log(sys.physicalCoreCount());//Physical core count of the CPU
```