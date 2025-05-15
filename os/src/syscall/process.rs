//! Process management syscalls
use crate::task::{change_program_brk, exit_current_and_run_next, suspend_current_and_run_next,syscall_mmap,syscall_unmap,current_user_token};
use crate::config::PAGE_SIZE;
use crate::mm::page_table::v_addr_ptr2ppn;
use crate::task::is_mmaped;
use crate::task::TASK_MANAGER;
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let token = current_user_token();
    let phy_addr =v_addr_ptr2ppn(token,_id);
    let us = get_time_us();
    unsafe {
        *(phys_addr as *mut TimeVal) = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0

}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    let token = current_user_token();
    let phy_addr =v_addr_ptr2ppn(token,_id);
    if (trace_request == 1 || trace_request == 0) && is_mmaped(_id,_id) == false {
        return -1;
    }
    if trace_request == 1 {
        let perm = get_v_addr_perm(id);
        if perm.map_or(0, |p| !p.contains(MapPermission::W)) {
            return -1;
        }
    }
    let phys_ptr = phys_addr as *mut u8;
    match trace_request {
        0 => {
            let value = unsafe {
                *phys_ptr as isize
            };
            value
        },
        1 => {

            unsafe {
                *phys_ptr = data as u8;
            };
            0
        },
        2 => {
            TASK_MANAGER.get_syscall_count(_id) as isize
        },
        _ => {
            -1
        },
    }
    -1
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");

    if (_start % PAGE_SIZE != 0) &(prot & !0x7 != 0) &(prot & 0x7 = 0 ) {
        return -1;
    }
    syscall_mmap(_start,_len,_port);

}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    syscall_unmap(_start,_len)

}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
