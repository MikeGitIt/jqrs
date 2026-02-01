//! Module: locfile
//!
//! Contains 6 transpiled functions:
//! - locfile_locate:12870312244354819823:./src/locfile.c
//! - locfile_free:617776991218081734:./src/locfile.c
//! - locfile_retain:8061696545339553051:./src/locfile.c
//! - locfile_init:14066084285262327997:./src/locfile.c
//! - locfile_get_line:7512684424555435328:./src/locfile.c
//! - locfile_line_length:8737846313694366363:./src/locfile.c
use std::ptr::NonNull;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use crate::execute::jq_report_error;
use crate::types::*;
/// Standalone function wrapper for locfile_init
pub fn locfile_init<T>(
    jq: Option<Rc<RefCell<JqState<T>>>>,
    fname: &str,
    data: &str,
    length: i32,
) -> Rc<RefCell<Locfile<T>>> {
    Locfile::locfile_init(jq, fname, data, length)
}
/// Standalone function wrapper for locfile_retain
pub fn locfile_retain<T>(l: &Rc<RefCell<Locfile<T>>>) -> Rc<RefCell<Locfile<T>>> {
    Locfile::locfile_retain(l)
}
/// Standalone function wrapper for locfile_free
pub fn locfile_free<T>(l: &Rc<RefCell<Locfile<T>>>) {
    Locfile::locfile_free(l)
}
/// Standalone function wrapper for locfile_get_line
pub fn locfile_get_line<T>(l: &Locfile<T>, pos: i32) -> i32 {
    l.locfile_get_line(pos)
}
/// Standalone function wrapper for locfile_line_length
pub fn locfile_line_length<T>(l: &Locfile<T>, line: i32) -> i32 {
    l.locfile_line_length(line)
}
/// Standalone function wrapper for locfile_locate
pub fn locfile_locate<T>(
    l: &Locfile<T>,
    loc: Location,
    fmt: &str,
    args: &[&dyn fmt::Display],
) {
    l.locfile_locate(loc, fmt, args)
}
/// Check if a jv is valid (standalone function matching C API)
pub fn jv_is_valid(x: &Jv) -> bool {
    x.is_valid()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_locfile_init() {
        let data = "line1\nline2\nline3";
        let locfile = locfile_init::<()>(None, "test.jq", data, data.len() as i32);
        let borrowed = locfile.borrow();
        assert_eq!(borrowed.nlines, 3);
        assert_eq!(borrowed.fname, "test.jq");
        assert_eq!(borrowed.refct, 1);
    }
    #[test]
    fn test_locfile_get_line() {
        let data = "line1\nline2\nline3";
        let locfile = locfile_init::<()>(None, "test.jq", data, data.len() as i32);
        let borrowed = locfile.borrow();
        assert_eq!(borrowed.locfile_get_line(0), 0);
        assert_eq!(borrowed.locfile_get_line(6), 1);
        assert_eq!(borrowed.locfile_get_line(12), 2);
    }
    #[test]
    fn test_locfile_retain_free() {
        let data = "test";
        let locfile = locfile_init::<()>(None, "test.jq", data, data.len() as i32);
        assert_eq!(locfile.borrow().refct, 1);
        let _retained = locfile_retain(&locfile);
        assert_eq!(locfile.borrow().refct, 2);
        locfile_free(&locfile);
        assert_eq!(locfile.borrow().refct, 1);
    }
    #[test]
    fn test_location_default() {
        let loc = Location::default();
        assert_eq!(loc.start, 0);
        assert_eq!(loc.end, 0);
    }
}
impl<T> Locfile<T> {
    /// Initialize a new locfile
    pub fn locfile_init(
        jq: Option<Rc<RefCell<JqState<T>>>>,
        fname: &str,
        data: &str,
        length: i32,
    ) -> Rc<RefCell<Self>> {
        let length = length as usize;
        let data_str = if data.len() >= length { &data[..length] } else { data };
        let mut nlines = 1i32;
        for ch in data_str.chars() {
            if ch == '\n' {
                nlines += 1;
            }
        }
        let mut linemap = vec![0i32; (nlines + 1) as usize];
        linemap[0] = 0;
        let mut line = 1usize;
        for (i, ch) in data_str.char_indices() {
            if ch == '\n' {
                linemap[line] = (i + 1) as i32;
                line += 1;
            }
        }
        linemap[nlines as usize] = (length + 1) as i32;
        Rc::new(
            RefCell::new(Locfile {
                fname: fname.to_string(),
                data: data_str.to_string(),
                length: length as i32,
                linemap,
                nlines,
                error: None,
                jq,
                refct: 1,
            }),
        )
    }
    /// Retain (increment reference count)
    pub fn locfile_retain(l: &Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        l.borrow_mut().refct += 1;
        Rc::clone(l)
    }
    /// Free (decrement reference count)
    pub fn locfile_free(l: &Rc<RefCell<Self>>) {
        let mut borrowed = l.borrow_mut();
        borrowed.refct -= 1;
    }
    /// Get line number for a position
    pub fn locfile_get_line(&self, pos: i32) -> i32 {
        assert!(pos < self.length, "pos < l->length");
        let mut line = 1i32;
        while line < self.linemap.len() as i32 && self.linemap[line as usize] <= pos {
            line += 1;
        }
        assert!(line - 1 < self.nlines, "line-1 < l->nlines");
        line - 1
    }
    /// Get line length for a given line number
    pub fn locfile_line_length(&self, line: i32) -> i32 {
        assert!(line < self.nlines, "line < l->nlines");
        self.linemap[(line + 1) as usize] - self.linemap[line as usize] - 1
    }
    /// Locate and report error at a location
    pub fn locfile_locate(&self, loc: Location, fmt: &str, args: &[&dyn fmt::Display]) {
        let m1 = if args.is_empty() {
            fmt.to_string()
        } else {
            let mut result = fmt.to_string();
            for arg in args {
                if let Some(pos) = result.find("{}") {
                    result = format!(
                        "{}{}{}", & result[..pos], arg, & result[pos + 2..]
                    );
                }
            }
            result
        };
        if loc.start == -1 {
            let error_msg = format!("jq: error: {}\n<unknown location>", m1);
            if let Some(ref jq) = self.jq {
                let mut jq_borrowed = jq.borrow_mut();
                jq_borrowed.error_message = Jv::string(&error_msg);
            }
            return;
        }
        let startline = self.locfile_get_line(loc.start);
        let offset = self.linemap[startline as usize];
        let line_length = self.locfile_line_length(startline);
        let line_start = offset as usize;
        let line_end = (offset + line_length) as usize;
        let line_content = if line_end <= self.data.len() {
            &self.data[line_start..line_end]
        } else if line_start < self.data.len() {
            &self.data[line_start..]
        } else {
            ""
        };
        let caret_pos = (loc.start - offset) as usize;
        let padding = " ".repeat(caret_pos);
        let m2 = format!(
            "{} at {}, line {}:\n{}{}", m1, self.fname, startline + 1, line_content,
            padding
        );
        if let Some(ref jq) = self.jq {
            let mut jq_borrowed = jq.borrow_mut();
            jq_borrowed.error_message = Jv::string(&m2);
        }
    }
}
