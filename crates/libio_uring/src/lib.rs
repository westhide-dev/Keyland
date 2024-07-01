//! Linux [io_uring]
//!
//! # References
//!  - [Linux]
//!  - [io_uring header]
//!
//! [Linux]: https://man.archlinux.org/man/io_uring.7.en
//! [io_uring]: https://kernel.dk/io_uring.pdf
//! [io_uring header]: https://github.com/torvalds/linux/blob/master/include/uapi/linux/io_uring.h

pub mod flag;

use rustix::{
    fd::{AsFd, OwnedFd},
    io::Result,
    io_uring::{
        io_uring_cqe, io_uring_enter, io_uring_params, io_uring_register, io_uring_setup,
        io_uring_sqe, IoringEnterFlags, IoringRegisterOp,
    },
};

pub type CVoid = core::ffi::c_void;

pub type IoUringSqe = io_uring_sqe;
pub type IoUringCqe = io_uring_cqe;

pub type IoUringParams = io_uring_params;
pub type IoUringRegisterOp = IoringRegisterOp;

pub struct IoUring {
    pub sqe: IoUringSqe,
    pub cqe: IoUringCqe,
    pub params: IoUringParams,
    pub ring_fd: OwnedFd,
}

// System call
impl IoUring {
    pub fn new(entries: u32, mut params: IoUringParams) -> Result<Self> {
        let ring_fd = io_uring_setup(entries, &mut params)?;
        // TODO
        Ok(Self { params, ring_fd, sqe: Default::default(), cqe: Default::default() })
    }

    pub unsafe fn register(
        &self,
        opcode: IoUringRegisterOp,
        ptr: *const CVoid,
        size: u32,
    ) -> Result<u32> {
        io_uring_register(self.ring_fd.as_fd(), opcode, ptr, size)
    }

    pub unsafe fn enter(
        &self,
        to_submit: u32,
        min_complete: u32,
        flags: IoringEnterFlags,
        ptr: *const CVoid,
        size: usize,
    ) -> Result<u32> {
        io_uring_enter(self.ring_fd.as_fd(), to_submit, min_complete, flags, ptr, size)
    }
}

impl IoUring {}
