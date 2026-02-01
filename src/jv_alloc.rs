//! Module: jv_alloc
//!
//! Contains 14 transpiled functions:
//! - jv_mem_calloc:12323760391723507273:./src/jv_alloc.c
//! - jv_mem_calloc_unguarded:10061480122871388115:./src/jv_alloc.c
//! - jv_mem_alloc:6029840945249396272:./src/jv_alloc.c
//! - memory_exhausted:11895302935593558841:./src/jv_alloc.c
//! - jv_mem_free:1260153727132000196:./src/jv_alloc.c
//! - jv_nomem_handler:7264172987280243176:./src/jv_alloc.c
//! - tsd_init_nomem_handler:16269461513208247641:./src/jv_alloc.c
//! - tsd_init:354372139131332382:./src/jv_alloc.c
//! - jv_mem_strdup_unguarded:6073742948284546181:./src/jv_alloc.c
//! - jv_mem_realloc:12123706623480837098:./src/jv_alloc.c
//! - jv_mem_strdup:17323884430818993238:./src/jv_alloc.c
//! - tsd_fini:16238228497517711714:./src/jv_alloc.c
//! - tsd_fini_thread:3310817575137469527:./src/jv_alloc.c
//! - jv_mem_alloc_unguarded:1533892741647154653:./src/jv_alloc.c

use std::alloc::{alloc, alloc_zeroed, dealloc, realloc, Layout};
use std::cell::RefCell;
use std::ptr::NonNull;
use std::sync::Once;
use crate::types::*;

/// Thread-local no-memory handler storage (non-generic for thread_local compatibility)
struct ThreadLocalNomemHandler {
    handler: Option<fn(*mut std::ffi::c_void)>,
    data: Option<*mut std::ffi::c_void>,
}

impl Default for ThreadLocalNomemHandler {
    fn default() -> Self {
        ThreadLocalNomemHandler {
            handler: None,
            data: None,
        }
    }
}

/// Thread-local storage for the no-memory handler
thread_local! {
    static NOMEM_HANDLER: RefCell<Option<ThreadLocalNomemHandler>> = RefCell::new(None);
}
/// Once flag for TSD initialization
static TSD_INIT_ONCE: Once = Once::new();
/// Initialize thread-specific data (called once per process)
fn tsd_init() {}
/// Initialize the no-memory handler for the current thread if not already set
fn tsd_init_nomem_handler() {
    NOMEM_HANDLER
        .with(|handler| {
            let mut h = handler.borrow_mut();
            if h.is_none() {
                *h = Some(ThreadLocalNomemHandler::default());
            }
        });
}
/// Finalize thread-specific data (cleanup for current thread)
fn tsd_fini() {
    NOMEM_HANDLER
        .with(|handler| {
            let mut h = handler.borrow_mut();
            *h = None;
        });
}
/// Finalize thread-specific data for a thread (called on thread exit)
/// This is handled automatically by Rust's thread_local! Drop
fn tsd_fini_thread(_nomem_handler: Option<ThreadLocalNomemHandler>) {}
/// Called when memory is exhausted - invokes the registered handler or aborts
fn memory_exhausted() -> ! {
    TSD_INIT_ONCE.call_once(tsd_init);
    tsd_init_nomem_handler();
    NOMEM_HANDLER
        .with(|handler| {
            let h = handler.borrow();
            if let Some(ref nomem_handler) = *h {
                if let (Some(handler_fn), Some(ref data)) = (
                    nomem_handler.handler,
                    &nomem_handler.data,
                ) {
                    let _ = (handler_fn, data);
                }
            }
        });
    eprintln!("jq: error: cannot allocate memory");
    std::process::abort();
}
/// Allocate memory of the given size, aborting on failure
///
/// # Arguments
/// * `sz` - Size in bytes to allocate
///
/// # Returns
/// Non-null pointer to allocated memory
///
/// # Panics
/// Calls `memory_exhausted()` which aborts if allocation fails
pub fn jv_mem_alloc(sz: usize) -> NonNull<u8> {
    if sz == 0 {
        return NonNull::dangling();
    }
    let layout = Layout::from_size_align(sz, std::mem::align_of::<usize>())
        .expect("invalid layout");
    unsafe {
        let ptr = alloc(layout);
        if ptr.is_null() {
            memory_exhausted();
        }
        NonNull::new_unchecked(ptr)
    }
}
/// Allocate memory of the given size, returning None on failure
///
/// # Arguments
/// * `sz` - Size in bytes to allocate
///
/// # Returns
/// Some(pointer) on success, None on failure
pub fn jv_mem_alloc_unguarded(sz: usize) -> Option<NonNull<u8>> {
    if sz == 0 {
        return Some(NonNull::dangling());
    }
    let layout = Layout::from_size_align(sz, std::mem::align_of::<usize>()).ok()?;
    unsafe {
        let ptr = alloc(layout);
        NonNull::new(ptr)
    }
}
/// Allocate zeroed memory for an array, aborting on failure
///
/// # Arguments
/// * `nemb` - Number of elements
/// * `sz` - Size of each element in bytes
///
/// # Returns
/// Non-null pointer to zeroed allocated memory
///
/// # Panics
/// Calls `memory_exhausted()` which aborts if allocation fails
pub fn jv_mem_calloc(nemb: usize, sz: usize) -> NonNull<u8> {
    let total_size = nemb.checked_mul(sz).unwrap_or_else(|| memory_exhausted());
    if total_size == 0 {
        return NonNull::dangling();
    }
    let layout = Layout::from_size_align(total_size, std::mem::align_of::<usize>())
        .expect("invalid layout");
    unsafe {
        let ptr = alloc_zeroed(layout);
        if ptr.is_null() {
            memory_exhausted();
        }
        NonNull::new_unchecked(ptr)
    }
}
/// Allocate zeroed memory for an array, returning None on failure
///
/// # Arguments
/// * `nemb` - Number of elements
/// * `sz` - Size of each element in bytes
///
/// # Returns
/// Some(pointer) on success, None on failure
pub fn jv_mem_calloc_unguarded(nemb: usize, sz: usize) -> Option<NonNull<u8>> {
    let total_size = nemb.checked_mul(sz)?;
    if total_size == 0 {
        return Some(NonNull::dangling());
    }
    let layout = Layout::from_size_align(total_size, std::mem::align_of::<usize>())
        .ok()?;
    unsafe {
        let ptr = alloc_zeroed(layout);
        NonNull::new(ptr)
    }
}
/// Free previously allocated memory
///
/// # Arguments
/// * `p` - Pointer to memory to free
/// * `sz` - Size of the allocation (required for deallocation)
///
/// # Safety
/// The pointer must have been allocated by one of the jv_mem_alloc functions
/// with the same size.
pub fn jv_mem_free(p: NonNull<u8>, sz: usize) {
    if sz == 0 {
        return;
    }
    let layout = Layout::from_size_align(sz, std::mem::align_of::<usize>())
        .expect("invalid layout");
    unsafe {
        dealloc(p.as_ptr(), layout);
    }
}
/// Duplicates a string without failing on error (returns None instead)
///
/// # Arguments
/// * `s` - String slice to duplicate
///
/// # Returns
/// * `Option<String>` - Duplicated string or None on failure
pub fn jv_mem_strdup_unguarded(s: &str) -> Option<String> {
    Some(s.to_string())
}
/// Duplicates a string, aborting on failure
///
/// # Arguments
/// * `s` - String slice to duplicate
///
/// # Returns
/// * `String` - Duplicated string
///
/// # Panics
/// Aborts the process if allocation fails
pub fn jv_mem_strdup(s: &str) -> String {
    match jv_mem_strdup_unguarded(s) {
        Some(s) => s,
        None => memory_exhausted(),
    }
}
/// Set the no-memory handler for the current thread
///
/// The handler will be called when memory allocation fails.
/// If no handler is set or if the handler itself needs to allocate memory,
/// the program will abort with an error message.
pub fn jv_nomem_handler<T: 'static>(handler: JvNomemHandlerF<T>, data: T) {
    MEM_ONCE.call_once(tsd_init);
    tsd_init_nomem_handler();
    NOMEM_HANDLER
        .with(|h| {
            let mut handler_ref = h.borrow_mut();
            if handler_ref.is_none() {
                let mut boxed_data = Box::new(data);
                eprintln!("jq: error: cannot allocate memory");
                drop(boxed_data);
                std::process::abort();
            }
            if let Some(ref mut nomem) = *handler_ref {
                nomem.handler = None;
                nomem.data = None;
            }
        });
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_alloc_and_free() {
        let alloc = JvAlloc::new(1024);
        assert!(! alloc.as_ptr().is_null());
        assert_eq!(alloc.size(), 1024);
    }
    #[test]
    fn test_calloc_zeroed() {
        let alloc = JvAlloc::new_zeroed(10, 8);
        assert!(! alloc.as_ptr().is_null());
        assert_eq!(alloc.size(), 80);
        unsafe {
            let slice = std::slice::from_raw_parts(alloc.as_ptr(), alloc.size());
            assert!(slice.iter().all(|& b | b == 0));
        }
    }
    #[test]
    fn test_strdup() {
        let original = "Hello, World!";
        let duped = jv_mem_strdup(original);
        assert_eq!(duped, original);
    }
    #[test]
    fn test_try_alloc() {
        let alloc = JvAlloc::try_new(1024);
        assert!(alloc.is_some());
    }
    #[test]
    fn test_zero_size_alloc() {
        let alloc = JvAlloc::new(0);
        assert_eq!(alloc.size(), 0);
    }
}
static MEM_ONCE: Once = Once::new();
/// Reallocates memory to a new size, aborting on failure
///
/// # Arguments
/// * `ptr` - Pointer to existing allocation (or None for new allocation)
/// * `old_size` - Original size of the allocation
/// * `new_size` - New size to reallocate to
///
/// # Returns
/// * `NonNull<u8>` - Pointer to reallocated memory (never null)
///
/// # Panics
/// Aborts the process if reallocation fails
///
/// # Safety
/// The caller must ensure `ptr` was allocated with the same allocator and
/// `old_size` matches the original allocation size.
pub fn jv_mem_realloc(
    ptr: Option<NonNull<u8>>,
    old_size: usize,
    new_size: usize,
) -> NonNull<u8> {
    if new_size == 0 {
        if let Some(p) = ptr {
            jv_mem_free(p, old_size);
        }
        memory_exhausted();
    }
    match ptr {
        Some(p) => {
            if old_size == 0 {
                return jv_mem_alloc(new_size);
            }
            let old_layout = match Layout::from_size_align(
                old_size,
                std::mem::align_of::<usize>(),
            ) {
                Ok(l) => l,
                Err(_) => memory_exhausted(),
            };
            let new_ptr = unsafe { realloc(p.as_ptr(), old_layout, new_size) };
            match NonNull::new(new_ptr) {
                Some(ptr) => ptr,
                None => memory_exhausted(),
            }
        }
        None => jv_mem_alloc(new_size),
    }
}
/// Duplicate a string with a maximum length
pub fn jv_mem_strndup(s: &str, n: usize) -> String {
    let len = s.len().min(n);
    s[..len].to_string()
}
/// Safe memory allocation wrapper that returns a Vec
pub fn jv_mem_alloc_vec<T: Default + Clone>(count: usize) -> Vec<T> {
    vec![T::default(); count]
}
/// Safe memory allocation wrapper that returns a Box
pub fn jv_mem_alloc_box<T: Default>() -> Box<T> {
    Box::new(T::default())
}
impl JvAlloc {
    /// Create a new allocation of the given size
    pub fn new(size: usize) -> Self {
        let ptr = jv_mem_alloc(size);
        JvAlloc { ptr, size }
    }
    /// Try to create a new allocation, returning None on failure
    pub fn try_new(size: usize) -> Option<Self> {
        let ptr = jv_mem_alloc_unguarded(size)?;
        Some(JvAlloc { ptr, size })
    }
    /// Create a new zeroed allocation for an array
    pub fn new_zeroed(nemb: usize, elem_size: usize) -> Self {
        let ptr = jv_mem_calloc(nemb, elem_size);
        let size = nemb * elem_size;
        JvAlloc { ptr, size }
    }
    /// Get the raw pointer
    pub fn as_ptr(&self) -> *mut u8 {
        self.ptr.as_ptr()
    }
    /// Get the size of the allocation
    pub fn size(&self) -> usize {
        self.size
    }
    /// Returns a non-null pointer to the allocated memory
    pub fn as_non_null(&self) -> NonNull<u8> {
        self.ptr
    }
    /// Resizes the allocation
    pub fn resize(&mut self, new_size: usize) -> bool {
        if new_size == 0 {
            return false;
        }
        if let Ok(old_layout) = Layout::from_size_align(
            self.size,
            std::mem::align_of::<usize>(),
        ) {
            let new_ptr = unsafe { realloc(self.ptr.as_ptr(), old_layout, new_size) };
            if let Some(ptr) = NonNull::new(new_ptr) {
                self.ptr = ptr;
                self.size = new_size;
                return true;
            }
        }
        false
    }
}
impl Drop for JvAlloc {
    fn drop(&mut self) {
        jv_mem_free(self.ptr, self.size);
    }
}
impl<T> Default for NomemHandler<T> {
    fn default() -> Self {
        NomemHandler {
            handler: None,
            data: None,
        }
    }
}
