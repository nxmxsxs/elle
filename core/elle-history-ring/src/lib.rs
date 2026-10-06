use std::{
    cell::UnsafeCell,
    sync::{
        Arc,
        atomic::{AtomicU8, AtomicU64, Ordering},
    },
};

pub trait ReadBuf {
    fn append_payload(&mut self, payload: &[u8]);
    fn resize(&mut self, new_len: usize);
    fn clear(&mut self);
}

pub enum ReadResult {
    Read { len: u32 },
    End,
    Overrun,
    Empty,
}

pub trait Reader: Clone {
    fn read_next_into<B: ReadBuf>(&mut self, buf: &mut B) -> ReadResult;

    fn read_prev_into<B: ReadBuf>(&mut self, buf: &mut B) -> ReadResult;
}

pub trait Writer {}

pub trait Storage {
    type Reader: Reader;

    type Writer: Writer;
}
