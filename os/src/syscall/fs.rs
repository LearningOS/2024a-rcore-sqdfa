//! File and filesystem-related syscalls

use crate::mm::translated_byte_buffer;
use crate::task::current_user_token;

const FD_STDOUT: usize = 1;

/// write buf of length `len`  to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    trace!("kernel: sys_write");
    match fd {
        FD_STDOUT => {
            let buffers = translated_byte_buffer(current_user_token(), buf, len);
            for buffer in buffers {
                print!("{}", core::str::from_utf8(buffer).unwrap());
            }
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}

/// copy 
pub fn copy_to_virt<T>(src: &T,dst:*mut T){
    let src_buf_ptr: *const u8 = unsafe { core::mem::transmute(src)};
    let dst_buf_ptr: *const u8 = unsafe { core::mem::transmute(dst)};
    let len = core::mem::size_of::<T>();
    let dst_buf = translated_byte_buffer(current_user_token(),dst_buf_ptr,len);
    let mut offset =0;
    for dst_one in dst_buf {
        dst_one.copy_from_slice(unsafe {core::slice::from_raw_parts(src_buf_ptr.add(offset), dst_one.len())});
        offset += dst_one.len();
    }
}