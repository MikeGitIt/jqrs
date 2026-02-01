//! Module: jv_dtoa_tsd
//!
//! Contains 4 transpiled functions:
//! - tsd_dtoa_context_get:11696085931643755727:./src/jv_dtoa_tsd.c
//! - jv_tsd_dtoa_ctx_init:5102126143990081200:./src/jv_dtoa_tsd.c
//! - tsd_dtoa_ctx_dtor:7389094474673291681:./src/jv_dtoa_tsd.c
//! - jv_tsd_dtoa_ctx_fini:3305793591711656057:./src/jv_dtoa_tsd.c

use std::cell::RefCell;
use std::sync::Once;
use crate::jv_dtoa::{DtoaContext, jvp_dtoa_context_init, jvp_dtoa_context_free};
use crate::types::*;
/// Thread-local storage for dtoa context
/// Each thread gets its own DtoaContext to avoid synchronization overhead
thread_local! {
    static DTOA_CTX : RefCell < Option < Box < DtoaContext >>> = RefCell::new(None);
}
/// Once guard for initialization
static INIT_ONCE: Once = Once::new();
/// Flag to track if the module has been initialized
static mut INITIALIZED: bool = false;
/// Initialize the thread-specific dtoa context system
///
/// This function sets up the thread-local storage infrastructure for dtoa contexts.
/// It is called once per process via Once::call_once.
///
/// # Panics
///
/// This function will panic if initialization fails (matching the C behavior of abort())
fn jv_tsd_dtoa_ctx_init() {
    unsafe {
        INITIALIZED = true;
    }
}
/// Destructor for thread-specific dtoa context
///
/// Called when a thread exits or when the context needs to be freed.
///
/// # Arguments
///
/// * `ctx` - Optional boxed DtoaContext to destroy
fn tsd_dtoa_ctx_dtor(ctx: Option<Box<DtoaContext>>) {
    if let Some(mut context) = ctx {
        jvp_dtoa_context_free(&mut context);
    }
}
/// Get the thread-specific dtoa context
///
/// Returns a reference to the current thread's DtoaContext, creating one if it doesn't exist.
/// This function ensures each thread has its own context for dtoa operations.
///
/// # Returns
///
/// A mutable reference to the thread's DtoaContext
///
/// # Panics
///
/// Panics if the context cannot be created or set
pub fn tsd_dtoa_context_get() -> &'static mut DtoaContext {
    INIT_ONCE
        .call_once(|| {
            jv_tsd_dtoa_ctx_init();
        });
    DTOA_CTX
        .with(|cell| {
            let mut borrowed = cell.borrow_mut();
            if borrowed.is_none() {
                let mut ctx = Box::new(DtoaContext {
                    freelist: Default::default(),
                    p5s: None,
                });
                jvp_dtoa_context_init(&mut ctx);
                *borrowed = Some(ctx);
            }
            unsafe {
                let ptr = borrowed.as_mut().unwrap().as_mut() as *mut DtoaContext;
                &mut *ptr
            }
        })
}
/// Finalize the thread-specific dtoa context
///
/// Called at process exit to clean up the current thread's context.
/// Other threads' contexts are cleaned up when those threads exit.
fn jv_tsd_dtoa_ctx_fini() {
    DTOA_CTX
        .with(|cell| {
            let ctx = cell.borrow_mut().take();
            tsd_dtoa_ctx_dtor(ctx);
        });
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_context_get() {
        let ctx1 = tsd_dtoa_context_get() as *const DtoaContext;
        let ctx2 = tsd_dtoa_context_get() as *const DtoaContext;
        assert_eq!(ctx1, ctx2);
    }
    #[test]
    fn test_context_initialization() {
        let ctx = tsd_dtoa_context_get();
        assert!(ctx.p5s.is_none());
    }
}
impl Drop for DtoaContextGuard {
    fn drop(&mut self) {
        jv_tsd_dtoa_ctx_fini();
    }
}
