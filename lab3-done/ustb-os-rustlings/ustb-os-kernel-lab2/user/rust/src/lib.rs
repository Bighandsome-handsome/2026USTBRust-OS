#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#[macro_use]
pub mod console;
mod lang_items;
mod syscall;
extern crate alloc;
extern crate core;
#[macro_use]
extern crate bitflags;
use buddy_system_allocator::LockedHeap;
pub use syscall::*;

const USER_HEAP_SIZE: usize = 16384;
static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];
#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    unsafe {
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
    exit(main());
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

bitflags! {
    pub struct SignalFlags: i32 {
        const SIGDEF = 1;
        const SIGHUP  = 1 << 1;
        const SIGINT  = 1 << 2;
        const SIGQUIT = 1 << 3;
        const SIGABRT = 1 << 6;
        const SIGFPE  = 1 << 8;
        const SIGKILL = 1 << 9;
        const SIGSEGV = 1 << 11;
        const SIGPIPE = 1 << 13;
        const SIGALRM = 1 << 14;
        const SIGTERM = 1 << 15;
        const SIGCHLD = 1 << 17;
        const SIGCONT = 1 << 18;
        const SIGSTOP = 1 << 19;
    }
}

#[repr(C)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[repr(C)]
pub struct Stat {
    pub dev: u64,
    pub ino: u64,
    pub mode: StatMode,
    pub nlink: u32,
    pad: [u64; 7],
}

bitflags! {
    pub struct StatMode: u32 {
        const NULL  = 0;
        const DIR   = 0o040000;
        const FILE  = 0o100000;
    }
}

pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    sys_read(fd, buf)
}
pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
pub fn exit(exit_code: i32) -> ! {
    sys_exit(exit_code)
}
pub fn yield_() -> isize {
    sys_yield()
}
pub fn get_time() -> isize {
    let mut time = TimeVal { sec: 0, usec: 0 };
    match sys_get_time(&mut time, 0) {
        0 => ((time.sec & 0xffff) * 1000 + time.usec / 1000) as isize,
        _ => -1,
    }
}
pub fn getpid() -> isize {
    sys_getpid()
}
pub fn fork() -> isize {
    sys_fork()
}
pub fn exec(path: &str, args: &[*const u8]) -> isize {
    sys_exec(path, args)
}
pub fn wait(exit_code: &mut i32) -> isize {
    loop {
        match sys_waitpid(-1, exit_code as *mut _) {
            -2 => { yield_(); }
            n => return n,
        }
    }
}
pub fn waitpid(pid: usize, exit_code: &mut i32) -> isize {
    loop {
        match sys_waitpid(pid as isize, exit_code as *mut _) {
            -2 => { yield_(); }
            n => return n,
        }
    }
}
pub fn kill(pid: usize, signal: i32) -> isize {
    sys_kill(pid, signal)
}
pub fn sleep(period_ms: usize) {
    let start = get_time();
    while get_time() < start + period_ms as isize {
        sys_yield();
    }
}
