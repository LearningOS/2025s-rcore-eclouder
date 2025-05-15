//! Process management syscalls
use crate::task::{change_program_brk, exit_current_and_run_next, suspend_current_and_run_next,syscall_mmap,syscall_unmap,current_user_token};
use crate::config::PAGE_SIZE;
use crate::mm::page_table::v_addr_ptr2ppn;
use crate::mm::MapPermission;
use crate::task::is_mapped;
use crate::task::TASK_MANAGER;
use crate::task::get_v_addr_perm;
use crate::timer::get_time_us;
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
    let phy_addr =v_addr_ptr2ppn(token,_ts as usize);
    let us = get_time_us();
    unsafe {
        *(phy_addr as *mut TimeVal) = TimeVal {
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
    if (_trace_request == 1 || _trace_request == 0) && is_mapped(_id,_id) == false {
        return -1;
    }
    if _trace_request == 1 {
        let perm = get_v_addr_perm(_id);
        if perm.map_or(true, |p| !p.contains(MapPermission::W)) {
            return -1;
        }
    }
    let phy_ptr = phy_addr as *mut u8;
    match _trace_request {
        0 => {
            let value = unsafe {
                *phy_ptr as isize
            };
            return value;
        },
        1 => {
            unsafe {
                *phy_ptr = _data as u8;
            };
            return 0;
        },
        2 => {
            return TASK_MANAGER.get_syscall_count(_id) as isize;
        },
        _ => {
            return -1;
        },
    }

}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");

    if (_start % PAGE_SIZE != 0) &(_port & !0x7 != 0) &(_port & 0x7 == 0 ) {
        return -1;
    }
    syscall_mmap(_start,_len,_port)

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
