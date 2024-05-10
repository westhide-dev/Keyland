pub mod comment;
pub mod luts;
pub mod unit;

use std::marker::PhantomData;

use crate::scaner::unit::Unit;

/// High performance u8 slice scanner, Inspired by [slice::Iter]
#[derive(Debug)]
pub struct Scaner<'s> {
    ptr: *const u8,
    end: *const u8,

    lo: *const u8,
    hi: *const u8,

    _marker: PhantomData<&'s u8>,
}

impl<'s> Scaner<'s> {
    pub fn new(str: &'s str) -> Self {
        // SAFETY: &str guaranteed ptr and end is valid
        unsafe {
            let ptr = str.as_ptr();
            let end = ptr.add(str.len());

            Self { ptr, end, lo: ptr, hi: ptr, _marker: PhantomData }
        }
    }

    pub fn byte(&self) -> u8 {
        unsafe { *self.ptr }
    }

    pub fn peek(&self, count: isize) -> u8 {
        unsafe { *self.ptr.offset(count) }
    }

    pub fn skip(&mut self, count: usize) {
        unsafe { self.ptr = self.ptr.add(count) }
    }

    pub fn eat(&mut self, byte: u8) -> bool {
        if self.byte() == byte {
            self.skip(1);
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        unsafe { self.end.sub_ptr(self.ptr) }
    }

    pub fn is_empty(&self) -> bool {
        self.ptr == self.end
    }
}

impl<'s> Scaner<'s> {
    pub fn mark_lo(&mut self) {
        self.lo = self.ptr
    }

    pub fn mark_hi(&mut self) {
        self.hi = self.ptr
    }

    pub fn data(&self) -> &'s str {
        unsafe { std::str::from_utf8_unchecked(std::slice::from_ptr_range(self.lo..self.hi)) }
    }
}

impl<'s> Scaner<'s> {
    pub fn next_unit(&mut self) -> Option<Unit<'s>> {
        self.scan_space();

        self.lookup_entity()
    }
}
