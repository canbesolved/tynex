// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Two `wl_shm` buffers on one `memfd`, mapped once.

use std::ffi::c_void;
use std::os::fd::{AsFd, OwnedFd};
use std::ptr::{self, NonNull};

use rustix::fs::{MemfdFlags, ftruncate, memfd_create};
use rustix::mm::{MapFlags, ProtFlags, mmap, munmap};
use wayland_client::QueueHandle;
use wayland_client::protocol::wl_buffer::WlBuffer;
use wayland_client::protocol::wl_shm::{Format, WlShm};
use wayland_client::protocol::wl_shm_pool::WlShmPool;

use crate::platform::linux::wayland::state::Wl;

/// Double buffer of `width` x `height` pixels, 4 bytes each.
pub(super) struct Buffers {
    pub width: u32,
    pub height: u32,
    pub busy: [bool; 2], // Held by the compositor until released
    handles: [WlBuffer; 2],
    pool: WlShmPool,
    map: Mapping,
}

impl Buffers {
    /// Allocates two buffers of `width` x `height` in `format`.
    pub fn new(
        shm: &WlShm,
        qh: &QueueHandle<Wl>,
        (width, height): (u32, u32),
        format: Format,
    ) -> rustix::io::Result<Self> {
        let stride = width * 4;
        let size = (stride * height) as usize;
        let fd = memfd_create("tynex", MemfdFlags::CLOEXEC)?;

        ftruncate(&fd, 2 * size as u64)?;

        let map = Mapping::new(&fd, 2 * size)?;
        let pool = shm.create_pool(fd.as_fd(), (2 * size) as i32, qh, ());
        let buffer = |i: usize| {
            let (w, h, s) = (width as i32, height as i32, stride as i32);

            pool.create_buffer((i * size) as i32, w, h, s, format, qh, i)
        };
        let handles = [buffer(0), buffer(1)];

        Ok(Self {
            width,
            height,
            busy: [false; 2],
            handles,
            pool,
            map,
        })
    }

    /// Returns a free buffer index and its pixels, if any is free.
    pub fn free(&mut self) -> Option<(usize, &mut [u8])> {
        let i = self.busy.iter().position(|&busy| !busy)?;
        let size = (self.width * self.height * 4) as usize;

        Some((i, &mut self.map.bytes()[i * size..(i + 1) * size]))
    }

    pub fn buffer(&self, i: usize) -> &WlBuffer {
        &self.handles[i]
    }
}

impl Drop for Buffers {
    fn drop(&mut self) {
        for buffer in &self.handles {
            buffer.destroy();
        }
        self.pool.destroy();
    }
}

/// A shared, writable memory mapping, unmapped on drop.
struct Mapping {
    ptr: NonNull<c_void>,
    len: usize,
}

impl Mapping {
    fn new(fd: &OwnedFd, len: usize) -> rustix::io::Result<Self> {
        // SAFETY: a fresh shared mapping of our own memfd, `len` bytes long,
        // which the file was just truncated to.
        let ptr = unsafe {
            mmap(
                ptr::null_mut(),
                len,
                ProtFlags::READ | ProtFlags::WRITE,
                MapFlags::SHARED,
                fd,
                0,
            )?
        };

        NonNull::new(ptr)
            .map(|ptr| Self { ptr, len })
            .ok_or(rustix::io::Errno::NOMEM)
    }

    fn bytes(&mut self) -> &mut [u8] {
        // SAFETY: the mapping is valid for `len` bytes until drop, and the
        // compositor only reads buffers we have committed; `free` hands out
        // released buffers only.
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr.as_ptr().cast(), self.len)
        }
    }
}

impl Drop for Mapping {
    fn drop(&mut self) {
        // SAFETY: unmaps exactly the range mapped in `new`.
        let _ = unsafe { munmap(self.ptr.as_ptr(), self.len) };
    }
}
