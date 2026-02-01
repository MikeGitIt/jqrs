//! Module: inject_errors
//!
//! Contains 7 transpiled functions:
//! - fread:2141841016214694301:./src/inject_errors.c
//! - fopen:16159857584795054044:./src/inject_errors.c
//! - ferror:17724295467079756131:./src/inject_errors.c
//! - fwrite:5994822233787403335:./src/inject_errors.c
//! - clearerr:12304907447177794142:./src/inject_errors.c
//! - fclose:354723904888062834:./src/inject_errors.c
//! - fgets:5430388227558383121:./src/inject_errors.c

use std::cell::RefCell;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::types::*;
thread_local! {
    static ERROR_STATE : RefCell < ErrorInjectionState > =
    RefCell::new(ErrorInjectionState { fail : None, fail_read : None, fail_write : None,
    fail_close : None, error : 5, });
}
/// Open a file with error injection support
///
/// Special path names trigger error injection:
/// - "fail_read" - subsequent reads will fail
/// - "fail_write" - subsequent writes will fail
/// - "fail_write_enospc" - writes fail with ENOSPC
/// - "fail_close" - close will fail
/// - "fail_close_enospc" - close fails with ENOSPC
pub fn fopen(path: &str, mode: &str) -> Option<Box<InjectedFile>> {
    ERROR_STATE
        .with(|state| {
            let mut state = state.borrow_mut();
            state.fail = None;
            state.fail_read = None;
            state.fail_write = None;
            state.fail_close = None;
            state.error = 5;
        });
    let file_result = if mode.contains('w') {
        if mode.contains('+') {
            File::options().read(true).write(true).create(true).truncate(true).open(path)
        } else {
            File::create(path)
        }
    } else if mode.contains('a') {
        File::options().append(true).create(true).open(path)
    } else {
        File::open(path)
    };
    let file = match file_result {
        Ok(f) => f,
        Err(_) => return None,
    };
    let mut injected = Box::new(InjectedFile::new(file));
    let file_id = injected.id;
    ERROR_STATE
        .with(|state| {
            let mut state = state.borrow_mut();
            if path == "fail_read" {
                state.fail = Some(file_id);
                state.fail_read = Some(file_id);
            } else if path.starts_with("fail_write") {
                state.fail = Some(file_id);
                state.fail_write = Some(file_id);
                if path == "fail_write_enospc" {
                    state.error = 28;
                }
            } else if path.starts_with("fail_close") {
                state.fail = Some(file_id);
                state.fail_close = Some(file_id);
                if path == "fail_close_enospc" {
                    state.error = 28;
                }
            }
        });
    Some(injected)
}
/// Read from a file with error injection support
///
/// Returns the number of complete elements read (not bytes)
pub fn fread(buf: &mut [u8], sz: usize, nemb: usize, f: &mut InjectedFile) -> u64 {
    let should_fail = ERROR_STATE
        .with(|state| {
            let state = state.borrow();
            state.fail_read == Some(f.id)
        });
    if should_fail {
        f.has_error = true;
        return 0;
    }
    let total_bytes = sz.saturating_mul(nemb);
    let read_len = total_bytes.min(buf.len());
    match f.inner.read(&mut buf[..read_len]) {
        Ok(bytes_read) => if sz == 0 { 0 } else { (bytes_read / sz) as u64 }
        Err(_) => {
            f.has_error = true;
            0
        }
    }
}
/// Read a line from a file with error injection support
///
/// Returns None on failure or EOF, Some with the line on success
pub fn fgets(buf: &mut [u8], len: i32, f: &mut InjectedFile) -> Option<String> {
    let should_fail = ERROR_STATE
        .with(|state| {
            let state = state.borrow();
            state.fail_read == Some(f.id)
        });
    if should_fail {
        f.has_error = true;
        return None;
    }
    let max_len = (len as usize).min(buf.len());
    if max_len == 0 {
        return None;
    }
    let mut bytes_read = 0;
    let mut one_byte = [0u8; 1];
    while bytes_read < max_len - 1 {
        match f.inner.read(&mut one_byte) {
            Ok(0) => {
                if bytes_read == 0 {
                    return None;
                }
                break;
            }
            Ok(1) => {
                buf[bytes_read] = one_byte[0];
                bytes_read += 1;
                if one_byte[0] == b'\n' {
                    break;
                }
            }
            Ok(_) => unreachable!(),
            Err(_) => {
                f.has_error = true;
                return None;
            }
        }
    }
    buf[bytes_read] = 0;
    String::from_utf8(buf[..bytes_read].to_vec()).ok()
}
/// Write to a file with error injection support
///
/// Returns the number of complete elements written (not bytes)
pub fn fwrite(buf: &[u8], sz: usize, nemb: usize, f: &mut InjectedFile) -> u64 {
    let should_fail = ERROR_STATE
        .with(|state| {
            let state = state.borrow();
            state.fail_write == Some(f.id)
        });
    if should_fail {
        f.has_error = true;
        return 0;
    }
    let total_bytes = sz.saturating_mul(nemb);
    let write_len = total_bytes.min(buf.len());
    match f.inner.write(&buf[..write_len]) {
        Ok(bytes_written) => if sz == 0 { 0 } else { (bytes_written / sz) as u64 }
        Err(_) => {
            f.has_error = true;
            0
        }
    }
}
/// Close a file, potentially simulating an error
///
/// Returns 0 on success, -1 (EOF) on failure (matches C API)
pub fn fclose(f: Box<InjectedFile>) -> i32 {
    let should_fail = ERROR_STATE
        .with(|state| {
            let s = state.borrow();
            s.fail_close == Some(f.id)
        });
    if should_fail {
        ERROR_STATE
            .with(|state| {
                let mut s = state.borrow_mut();
                s.fail = None;
                s.fail_read = None;
                s.fail_write = None;
                s.fail_close = None;
            });
        return -1; // EOF
    }
    match f.inner.sync_all() {
        Ok(()) => 0,
        Err(_) => -1,
    }
}
/// Check if an error has occurred on the file
///
/// Returns non-zero if an error has occurred
pub fn ferror(f: &InjectedFile) -> i32 {
    let has_injection_error = ERROR_STATE
        .with(|state| {
            let state = state.borrow();
            state.fail == Some(f.id)
        });
    if has_injection_error || f.has_error { 1 } else { 0 }
}
/// Clear the error indicator for a file
pub fn clearerr(f: &mut InjectedFile) {
    f.has_error = false;
    ERROR_STATE
        .with(|state| {
            let mut state = state.borrow_mut();
            if state.fail == Some(f.id) {
                state.fail = None;
            }
            if state.fail_read == Some(f.id) {
                state.fail_read = None;
            }
            if state.fail_write == Some(f.id) {
                state.fail_write = None;
            }
            if state.fail_close == Some(f.id) {
                state.fail_close = None;
            }
        });
}
/// Get the current error code for injected errors
pub fn get_injected_error() -> i32 {
    ERROR_STATE.with(|state| state.borrow().error)
}
/// Set the error code for future injected errors
pub fn set_injected_error(error: i32) {
    ERROR_STATE
        .with(|state| {
            state.borrow_mut().error = error;
        });
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    #[test]
    fn test_normal_file_operations() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "Hello, World!").unwrap();
        let path = temp.path().to_str().unwrap();
        let mut file = fopen(path, "r").unwrap();
        let mut buf = [0u8; 100];
        let read = fread(&mut buf, 1, 13, &mut file);
        assert!(read > 0);
        assert_eq!(ferror(& file), 0);
        assert_eq!(fclose(file), 0);
    }
    #[test]
    fn test_error_injection() {
        ERROR_STATE
            .with(|state| {
                let state = state.borrow();
                assert!(state.fail.is_none());
                assert!(state.fail_read.is_none());
            });
    }
}
/// Global counter for generating unique file IDs
static FILE_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);
/// Set up error injection for a specific file
pub fn inject_error(file_id: FileId, error_code: i32) {
    ERROR_STATE
        .with(|state| {
            let mut s = state.borrow_mut();
            s.fail = Some(file_id);
            s.fail_read = Some(file_id);
            s.fail_write = Some(file_id);
            s.fail_close = Some(file_id);
            s.error = error_code;
        });
}
/// Clear all error injection state
pub fn clear_errors() {
    ERROR_STATE
        .with(|state| {
            let mut s = state.borrow_mut();
            s.fail = None;
            s.fail_read = None;
            s.fail_write = None;
            s.fail_close = None;
            s.error = 0;
        });
}
impl InjectedFile {
    fn new(file: File) -> Self {
        let id = FILE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        InjectedFile {
            id,
            inner: file,
            has_error: false,
        }
    }
}
