// Thread-ID -- Get a unique thread ID
// Copyright 2016 Ruud van Asseldonk
//
// Licensed under either the Apache License, Version 2.0, or the MIT license, at
// your option. A copy of both licenses has been included in the root of the
// repository.

//! Thread-ID: get a unique ID for the current thread.
//!
//! For diagnostics and debugging it can often be useful to get an ID that is
//! different for every thread. This crate provides that functionality.
//!
//! # Example
//!
//! ```
//! use std::thread;
//! use thread_id;
//!
//! let handle = thread::spawn(move || {
//!     println!("spawned thread has id {}", thread_id::get());
//! });
//!
//! println!("main thread has id {}", thread_id::get());
//!
//! handle.join().unwrap();
//! ```

#![warn(missing_docs)]
#![feature(thread_id_value)]

#[cfg(unix)]
extern crate libc;

#[cfg(windows)]
extern crate winapi;

#[cfg(target_os = "redox")]
extern crate syscall;

/// Returns a number that is unique to the calling thread.
///
/// Calling this function twice from the same thread will return the same
/// number. Calling this function from a different thread will return a
/// different number.
#[inline]
pub fn get() -> usize {
    get_internal()
}

#[cfg(target_os = "switch")]
fn get_internal() -> usize {
    std::thread::current().id().as_u64().get() as usize
}

#[cfg(unix)]
#[inline]
fn get_internal() -> usize {
    unsafe { libc::pthread_self() as usize }
}

#[cfg(windows)]
#[inline]
fn get_internal() -> usize {
    unsafe { winapi::um::processthreadsapi::GetCurrentThreadId() as usize }
}

#[cfg(target_os = "redox")]
#[inline]
fn get_internal() -> usize {
    // Each thread has a separate pid on Redox.
    syscall::getpid().unwrap()
}

#[test]
fn distinct_threads_have_distinct_ids() {
    use std::sync::mpsc;
    use std::thread;

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || tx.send(::get()).unwrap()).join().unwrap();

    let main_tid = ::get();
    let other_tid = rx.recv().unwrap();
    assert!(main_tid != other_tid);
}
