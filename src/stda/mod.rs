pub mod FS;
/**
std库和core库一个不同的点在于:std库的存在依赖操作系统的具体实现,而core库及alloc库则不依赖于操作系统的 实现

操作系统一般提供给应用的功能一般是:

内存管理，文件描述符，进程/线程管理，进程/线程间通信，IO, 网络，文件, 时间，异步编程，杂项
 */
pub mod IO;
pub mod date;
