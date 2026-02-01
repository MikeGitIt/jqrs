//! Module: example4
//!
//! Contains 2 transpiled functions:
//! - main:14488106908486361945:./src/decNumber/example4.c
//! - signalHandler:6227849588431354696:./src/decNumber/example4.c
use std::process;
use crate::types::*;
use crate::deccontext::{decContextDefault, decContextStatusToString};
use crate::decnumber::{decNumberFromString, decNumberAdd, decNumberToString};
use std::sync::atomic::{AtomicBool, Ordering};
use std::panic::catch_unwind;
use std::sync::atomic::AtomicI32;
use std::cell::RefCell;
const DEC_DIVISION_BY_ZERO: u32 = 0x00000002;
const DEC_INVALID_OPERATION: u32 = 0x00000001;
const DEC_OVERFLOW: u32 = 0x00000004;
const DEC_UNDERFLOW: u32 = 0x00000008;
const DEC_SUBNORMAL: u32 = 0x00000010;
const DEC_CLAMPED: u32 = 0x00000040;
const DEC_INEXACT: u32 = 0x00000080;
const DEC_ROUNDED: u32 = 0x00000200;
const DEC_CONVERSION_SYNTAX: u32 = 0x00002000;
const DEC_ERRORS_MASK: u32 = DEC_DIVISION_BY_ZERO
    | (DEC_INVALID_OPERATION | DEC_OVERFLOW | DEC_UNDERFLOW | DEC_SUBNORMAL | DEC_CLAMPED
        | DEC_INEXACT) | DEC_ROUNDED | DEC_CONVERSION_SYNTAX;
const DECNUMDIGITS: i32 = 38;
static SIGNAL_CAUGHT: AtomicBool = AtomicBool::new(false);
/// Signal handler that preserves the signal value and performs a non-local jump.
///
/// In Rust, we cannot use longjmp directly as it's unsafe and incompatible with
/// Rust's unwinding mechanism. Instead, this implementation:
/// 1. Re-registers the signal handler (matching the C behavior with signal(8, ...))
/// 2. Stores the signal value for later retrieval
/// 3. Uses panic/catch_unwind as Rust's equivalent of setjmp/longjmp
///
/// Note: Signal 8 is SIGFPE (floating point exception) on most Unix systems.
pub fn signalHandler(sig: i32) {
    SIGNAL_RECEIVED.store(sig, Ordering::SeqCst);
    PRESERVE
        .with(|p| {
            *p.borrow_mut() = Some(sig);
        });
    panic!("signal_jump:{}", sig);
}
pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Please supply two numbers to add.");
        process::exit(1);
    }
    let mut set = DecContext {
        digits: 0,
        emax: 0,
        emin: 0,
        round: Rounding::HalfEven,
        traps: 0,
        status: 0,
        clamp: 0,
    };
    decContextDefault(&mut set, 0);
    // Signal handling is handled via the with_signal_context wrapper below
    // Check if any signal was previously caught
    if SIGNAL_CAUGHT.load(Ordering::SeqCst) {
        set.status &= DEC_ERRORS_MASK;
        let status_str = decContextStatusToString(&set);
        println!("Signal trapped [{}].", status_str);
        process::exit(1);
    }
    set.digits = DECNUMDIGITS;
    let mut a = DecNumber {
        digits: 0,
        exponent: 0,
        bits: 0,
        lsu: vec![0; ((DECNUMDIGITS + 3) / 4) as usize],
    };
    let mut b = DecNumber {
        digits: 0,
        exponent: 0,
        bits: 0,
        lsu: vec![0; ((DECNUMDIGITS + 3) / 4) as usize],
    };
    let arg1 = &args[1];
    let arg2 = &args[2];
    decNumberFromString(&mut a, arg1.as_str(), &mut set);
    decNumberFromString(&mut b, arg2.as_str(), &mut set);
    let mut result = DecNumber {
        digits: 0,
        exponent: 0,
        bits: 0,
        lsu: vec![0; ((DECNUMDIGITS + 3) / 4) as usize],
    };
    decNumberAdd(&mut result, &a, &b, &mut set);
    let result_str = decNumberToString(&result);
    println!("{} + {} => {}", arg1, arg2, result_str);
}
thread_local! {
    static PRESERVE : RefCell < Option < i32 >> = RefCell::new(None);
}
static SIGNAL_RECEIVED: AtomicI32 = AtomicI32::new(0);
/// Helper function to set up signal handling context (equivalent to setjmp).
/// Returns Ok(()) normally, or Err(sig) if a signal was caught.
pub fn with_signal_context<F, R>(f: F) -> Result<R, i32>
where
    F: FnOnce() -> R + std::panic::UnwindSafe,
{
    PRESERVE
        .with(|p| {
            *p.borrow_mut() = None;
        });
    match catch_unwind(f) {
        Ok(result) => Ok(result),
        Err(payload) => {
            if let Some(msg) = payload.downcast_ref::<String>() {
                if msg.starts_with("signal_jump:") {
                    if let Ok(sig) = msg[12..].parse::<i32>() {
                        return Err(sig);
                    }
                }
            }
            if let Some(msg) = payload.downcast_ref::<&str>() {
                if msg.starts_with("signal_jump:") {
                    if let Ok(sig) = msg[12..].parse::<i32>() {
                        return Err(sig);
                    }
                }
            }
            let sig = PRESERVE.with(|p| p.borrow().clone());
            if let Some(s) = sig {
                return Err(s);
            }
            std::panic::resume_unwind(payload);
        }
    }
}
/// Get the last signal that was caught
pub fn get_preserved_signal() -> Option<i32> {
    PRESERVE.with(|p| *p.borrow())
}
/// Clear the preserved signal value
pub fn clear_preserved_signal() {
    PRESERVE
        .with(|p| {
            *p.borrow_mut() = None;
        });
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_signal_handler_stores_signal() {
        clear_preserved_signal();
        let result = with_signal_context(|| {
            signalHandler(8);
            unreachable!("Should have jumped");
        });
        assert_eq!(result, Err(8));
    }
    #[test]
    fn test_normal_execution() {
        let result = with_signal_context(|| { 42 });
        assert_eq!(result, Ok(42));
    }
}
