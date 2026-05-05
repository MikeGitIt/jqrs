//! Module: lexer
//!
//! Contains 48 transpiled functions:
//! - enter:17835817121307025908:./src/lexer.c
//! - jq_yyset_debug:13900822124163998514:./src/lexer.c
//! - jq_yyset_lineno:5626326702541417601:./src/lexer.c
//! - jq_yyget_text:15803929213933663066:./src/lexer.c
//! - jq_yy_scan_string:4160379590522492099:./src/lexer.c
//! - jq_yyget_in:14037190789385789960:./src/lexer.c
//! - yy_fatal_error:4407648724049096877:./src/lexer.c
//! - yy_pop_state:15581872344081035501:./src/lexer.c
//! - jq_yyget_lloc:17119625335463418208:./src/lexer.c
//! - jq_yy_flush_buffer:14596305105062022191:./src/lexer.c
//! - jq_yy_scan_bytes:7520210792062396341:./src/lexer.c
//! - jq_yypush_buffer_state:3607477944224778981:./src/lexer.c
//! - yy_get_previous_state:10064195555265601727:./src/lexer.c
//! - yy_try_NUL_trans:1589344492118420816:./src/lexer.c
//! - jq_yyrestart:15388296296365138535:./src/lexer.c
//! - yy_push_state:6360973336899204563:./src/lexer.c
//! - jq_yy_init_buffer:18020687669655737512:./src/lexer.c
//! - jq_yyget_lineno:10636410357636958319:./src/lexer.c
//! - jq_yyensure_buffer_stack:5411548895527179867:./src/lexer.c
//! - jq_yy_switch_to_buffer:18398253593765327693:./src/lexer.c
//! - jq_yypop_buffer_state:17792631659659163983:./src/lexer.c
//! - jq_yylex_init:18030998157122503589:./src/lexer.c
//! - jq_yyget_column:15516954892489305980:./src/lexer.c
//! - jq_yyset_out:16297743307259145432:./src/lexer.c
//! - yy_get_next_buffer:17166952161134505346:./src/lexer.c
//! - jq_yyfree:15911256181705227944:./src/lexer.c
//! - jq_yyset_lloc:3025133105262238633:./src/lexer.c
//! - jq_yy_scan_buffer:16320576613716426105:./src/lexer.c
//! - jq_yyset_column:11229822649184555251:./src/lexer.c
//! - jq_yyget_leng:3844557654487927531:./src/lexer.c
//! - yy_init_globals:2667443728629815690:./src/lexer.c
//! - jq_yyget_debug:16723452304997161933:./src/lexer.c
//! - jq_yyget_out:10584120814840450002:./src/lexer.c
//! - yy_top_state:15961475938032811972:./src/lexer.c
//! - jq_yyset_lval:17719401958830857067:./src/lexer.c
//! - jq_yylex_destroy:8905234352414078438:./src/lexer.c
//! - jq_yy_delete_buffer:5474937009415972080:./src/lexer.c
//! - jq_yy_create_buffer:14698101113482342726:./src/lexer.c
//! - jq_yyrealloc:10236148446413819419:./src/lexer.c
//! - jq_yyset_in:6194112313169778632:./src/lexer.c
//! - jq_yylex_init_extra:12447244958279694559:./src/lexer.c
//! - try_exit:1212846620727669040:./src/lexer.c
//! - jq_yyget_lval:2077587114619037306:./src/lexer.c
//! - jq_yyalloc:12931701928175904340:./src/lexer.c
//! - jq_yy_load_buffer_state:14935170529001230761:./src/lexer.c
//! - jq_yyget_extra:4161028146736832885:./src/lexer.c
//! - jq_yyset_extra:5813580812217622942:./src/lexer.c
//! - jq_yylex:16998312211319048518:./src/lexer.c

use std::io::{self, Read, Write};
use std::ptr;
use std::process;
use crate::jv_alloc::{jv_mem_free, jv_mem_realloc, jv_mem_alloc};
use crate::inject_errors::{clearerr, fread, ferror};
use crate::types::*;
use crate::jv::{Jv, jv_string, jv_string_sized, JvKind};
use crate::jv_parse::jv_parse_sized;
/// Get the current column number
pub fn jq_yyget_column(yyscanner: &YyGutsT) -> i32 {
    if yyscanner.yy_buffer_stack.is_empty() {
        return 0;
    }
    match yyscanner.yy_buffer_stack.get(yyscanner.yy_buffer_stack_top) {
        Some(Some(buffer)) => buffer.yy_bs_column,
        _ => 0,
    }
}
/// Initialize a buffer with a file
pub fn jq_yy_init_buffer(
    b: &mut YyBufferState,
    file: Option<Box<dyn Read>>,
    yyscanner: &mut YyGutsT,
) {
    jq_yy_flush_buffer_internal(b);
    b.yy_input_file = file;
    b.yy_fill_buffer = true;
    let is_current = if !yyscanner.yy_buffer_stack.is_empty() {
        if let Some(Some(current)) = yyscanner.yy_buffer_stack.get(yyscanner.yy_buffer_stack_top) {
            std::ptr::eq(b as *const _, current.as_ref() as *const _)
        } else {
            false
        }
    } else {
        false
    };
    if !is_current {
        b.yy_bs_lineno = 1;
        b.yy_bs_column = 0;
    }
    b.yy_is_interactive = false;
}
/// Internal helper to flush a buffer
fn jq_yy_flush_buffer_internal(b: &mut YyBufferState) {
    b.yy_n_chars = 0;
    if b.yy_ch_buf.len() >= 2 {
        b.yy_ch_buf[0] = 0;
        b.yy_ch_buf[1] = 0;
    }
    b.yy_buf_pos = 0;
    b.yy_at_bol = 1;
    b.yy_buffer_status = 0;
}
/// Flush buffer
///
/// # Arguments
/// * `buffer` - Buffer to flush
/// * `yyscanner` - Scanner instance
pub fn jq_yy_flush_buffer(buffer: &mut YyBufferState, yyscanner: &mut YyGutsT) {
    buffer.yy_n_chars = 0;
    buffer.yy_buf_pos = 0;
    if buffer.yy_ch_buf.len() >= 2 {
        buffer.yy_ch_buf[0] = 0;
        buffer.yy_ch_buf[1] = 0;
    }
    buffer.yy_at_bol = 1;
    buffer.yy_buffer_status = 0;
    if yyscanner.yy_buffer_stack_top < yyscanner.yy_buffer_stack.len() {
        if let Some(Some(ref current)) = yyscanner
            .yy_buffer_stack
            .get(yyscanner.yy_buffer_stack_top)
        {
            if std::ptr::eq(current.as_ref(), buffer) {
                jq_yy_load_buffer_state(yyscanner);
            }
        }
    }
}
/// Get debug flag
///
/// # Arguments
/// * `yyscanner` - Scanner instance
///
/// # Returns
/// Debug flag value
pub fn jq_yyget_debug(yyscanner: &YyGutsT) -> i32 {
    yyscanner.yy_flex_debug_r
}
/// Get text length
///
/// # Arguments
/// * `yyscanner` - Scanner instance
///
/// # Returns
/// Length of current text
pub fn jq_yyget_leng(yyscanner: &YyGutsT) -> i32 {
    yyscanner.yyleng_r
}
/// Create a new buffer with given size
///
/// # Arguments
/// * `size` - Size of buffer to create
///
/// # Returns
/// New buffer state
pub fn jq_yy_create_buffer(size: i32) -> Box<YyBufferState> {
    let mut buffer = Box::new(YyBufferState::default());
    buffer.yy_buf_size = size;
    buffer.yy_ch_buf = vec![0u8; (size + 2) as usize];
    buffer.yy_is_our_buffer = true;
    buffer.yy_fill_buffer = true;
    buffer.yy_buffer_status = 0;
    buffer
}
/// Pop the state stack
pub fn yy_pop_state(yyscanner: &mut YyGutsT) {
    yyscanner.yy_start_stack_ptr -= 1;
    if yyscanner.yy_start_stack_ptr < 0 {
        yy_fatal_error("start-condition stack underflow", yyscanner);
    }
    let idx = yyscanner.yy_start_stack_ptr as usize;
    if idx < yyscanner.yy_start_stack.len() {
        yyscanner.yy_start = yyscanner.yy_start_stack[idx];
    }
}
/// Push state onto state stack
///
/// # Arguments
/// * `new_state` - State to push
/// * `yyscanner` - Scanner instance
pub fn yy_push_state(new_state: i32, yyscanner: &mut YyGutsT) {
    if yyscanner.yy_start_stack_ptr as usize >= yyscanner.yy_start_stack.len() {
        yyscanner.yy_start_stack.push(0);
        yyscanner.yy_start_stack_depth = yyscanner.yy_start_stack.len() as i32;
    }
    yyscanner.yy_start_stack[yyscanner.yy_start_stack_ptr as usize] = yyscanner.yy_start;
    yyscanner.yy_start_stack_ptr += 1;
    yyscanner.yy_start = new_state;
}
/// Set output stream
///
/// # Arguments
/// * `out_str` - Output stream
/// * `yyscanner` - Scanner instance
pub fn jq_yyset_out<W: Write + Send + 'static>(
    out_str: Option<W>,
    yyscanner: &mut YyGutsT,
) {
    yyscanner.yyout_r = out_str.map(|w| Box::new(w) as Box<dyn std::any::Any>);
}
/// Enter a lexer state based on character
///
/// # Arguments
/// * `c` - Character that triggered state change
/// * `currstate` - Current state
/// * `yyscanner` - Scanner instance
///
/// # Returns
/// New state
pub fn enter(c: i32, currstate: i32, yyscanner: &mut YyGutsT) -> i32 {
    currstate
}
/// Load the buffer state into the scanner
fn jq_yy_load_buffer_state_internal(yyg: &mut YyGutsT) {
    if yyg.yy_buffer_stack.is_empty() {
        return;
    }
    if let Some(Some(buffer)) = yyg.yy_buffer_stack.get(yyg.yy_buffer_stack_top) {
        yyg.yy_n_chars = buffer.yy_n_chars;
        yyg.yy_c_buf_p = buffer.yy_buf_pos;
        yyg.yytext_r = buffer.yy_ch_buf.clone();
        if buffer.yy_buf_pos < buffer.yy_ch_buf.len() {
            yyg.yy_hold_char = buffer.yy_ch_buf[buffer.yy_buf_pos];
        }
    }
}
/// Load buffer state into scanner
///
/// # Arguments
/// * `yyscanner` - Scanner instance
pub fn jq_yy_load_buffer_state(yyscanner: &mut YyGutsT) {
    if yyscanner.yy_buffer_stack_top < yyscanner.yy_buffer_stack.len() {
        if let Some(Some(ref buffer)) = yyscanner
            .yy_buffer_stack
            .get(yyscanner.yy_buffer_stack_top)
        {
            yyscanner.yy_n_chars = buffer.yy_n_chars;
            yyscanner.yy_c_buf_p = buffer.yy_buf_pos;
            yyscanner.yylineno_r = buffer.yy_bs_lineno;
            if buffer.yy_buf_pos < buffer.yy_ch_buf.len() {
                yyscanner.yy_hold_char = buffer.yy_ch_buf[buffer.yy_buf_pos];
            }
        }
    }
}
/// Fatal error handler - prints message and exits
///
/// # Arguments
/// * `msg` - Error message to display
/// * `_yyscanner` - Scanner instance (unused but kept for API compatibility)
pub fn yy_fatal_error(msg: &str, _yyscanner: &mut YyGutsT) -> ! {
    eprintln!("{}", msg);
    process::exit(2);
}
/// Try to exit a lexer state
///
/// # Arguments
/// * `c` - Character that might trigger exit
/// * `state` - Current state
/// * `yyscanner` - Scanner instance
///
/// # Returns
/// 1 if should exit, 0 otherwise
pub fn try_exit(c: i32, state: i32, yyscanner: &mut YyGutsT) -> i32 {
    0
}
/// Get top state from state stack
///
/// # Arguments
/// * `yyscanner` - Scanner instance
///
/// # Returns
/// The top state value, or 0 if stack is empty
pub fn yy_top_state(yyscanner: &YyGutsT) -> i32 {
    if yyscanner.yy_start_stack_ptr > 0 {
        yyscanner.yy_start_stack[(yyscanner.yy_start_stack_ptr - 1) as usize]
    } else {
        0
    }
}
/// Get extra data from scanner
///
/// # Arguments
/// * `yyscanner` - Scanner instance
///
/// # Returns
/// The extra data value stored in the scanner
pub fn jq_yyget_extra(yyscanner: &YyGutsT) -> i32 {
    yyscanner.yyextra_r
}
/// Set extra data in scanner
///
/// # Arguments
/// * `user_defined` - User-defined extra data
/// * `yyscanner` - Scanner instance
pub fn jq_yyset_extra(user_defined: i32, yyscanner: &mut YyGutsT) {
    yyscanner.yyextra_r = user_defined;
}
/// Get input file
pub fn jq_yyget_in(yyscanner: &YyscanT) -> Option<&Box<dyn std::any::Any>> {
    match yyscanner {
        Some(ref yyg) => yyg.yyin_r.as_ref(),
        None => None,
    }
}
/// Set input stream
///
/// # Arguments
/// * `in_str` - Input stream
/// * `yyscanner` - Scanner instance
pub fn jq_yyset_in<R: Read + Send + 'static>(
    in_str: Option<R>,
    yyscanner: &mut YyGutsT,
) {
    yyscanner.yyin_r = in_str.map(|r| Box::new(r) as Box<dyn std::any::Any>);
}
/// Get output stream
///
/// # Arguments
/// * `yyscanner` - Scanner instance
///
/// # Returns
/// Reference to output stream if set
pub fn jq_yyget_out(yyscanner: &YyGutsT) -> Option<&Box<dyn std::any::Any>> {
    yyscanner.yyout_r.as_ref()
}
/// Get text from scanner
///
/// # Arguments
/// * `yyscanner` - Scanner instance
///
/// # Returns
/// Reference to the text buffer
pub fn jq_yyget_text(yyscanner: &YyGutsT) -> &[u8] {
    &yyscanner.yytext_r
}
/// Get line number from scanner
///
/// # Arguments
/// * `yyscanner` - Scanner instance
///
/// # Returns
/// Current line number
pub fn jq_yyget_lineno(yyscanner: &YyGutsT) -> i32 {
    yyscanner.yylineno_r
}
/// Set line number for the scanner
pub fn jq_yyset_lineno(line_number: i32, yyscanner: &mut YyGutsT) {
    let current_buffer = if !yyscanner.yy_buffer_stack.is_empty() {
        yyscanner.yy_buffer_stack.get_mut(yyscanner.yy_buffer_stack_top)
    } else {
        None
    };
    match current_buffer {
        Some(Some(buffer)) => {
            buffer.yy_bs_lineno = line_number;
        }
        _ => {
            yy_fatal_error("yyset_lineno called with no buffer", yyscanner);
        }
    }
}
/// Set column number
pub fn jq_yyset_column(_column_no: i32, yyscanner: &mut YyGutsT) {
    if yyscanner.get_current_buffer().is_none() {
        yy_fatal_error("yyset_column called with no buffer", yyscanner);
    }
    if let Some(buf) = yyscanner.get_current_buffer_mut() {
        buf.yy_bs_column = _column_no;
    }
}
/// Set debug flag
///
/// # Arguments
/// * `_bdebug` - Debug flag value
/// * `yyscanner` - Scanner instance
pub fn jq_yyset_debug(_bdebug: i32, yyscanner: &mut YyGutsT) {
    yyscanner.yy_flex_debug_r = _bdebug;
}
/// Get lval (semantic value)
///
/// # Arguments
/// * `yyscanner` - Scanner instance
///
/// # Returns
/// Reference to lval if set
pub fn jq_yyget_lval(yyscanner: &YyGutsT) -> Option<&YySType> {
    yyscanner.yylval_r.as_ref().map(|b| b.as_ref())
}
/// Set lval (semantic value)
///
/// # Arguments
/// * `yylval_param` - Value to set
/// * `yyscanner` - Scanner instance
pub fn jq_yyset_lval(yylval_param: YySType, yyscanner: &mut YyGutsT) {
    yyscanner.yylval_r = Some(Box::new(yylval_param));
}
/// Get location from scanner
///
/// # Arguments
/// * `yyscanner` - Scanner instance
///
/// # Returns
/// Reference to the location, or None if not set
pub fn jq_yyget_lloc(yyscanner: &YyGutsT) -> Option<&Location> {
    yyscanner.yylloc_r.as_ref().map(|b| b.as_ref())
}
/// Set location
///
/// # Arguments
/// * `lloc_param` - Location to set
/// * `yyscanner` - Scanner instance
pub fn jq_yyset_lloc(lloc_param: Location, yyscanner: &mut YyGutsT) {
    yyscanner.yylloc_r = Some(Box::new(lloc_param));
}
/// Initialize the lexer
pub fn jq_yylex_init(yyscanner: &mut Option<Box<YyGutsT>>) -> i32 {
    let mut yyg = Box::new(YyGutsT::new());
    yy_init_globals(&mut yyg);
    *yyscanner = Some(yyg);
    0
}
/// Initialize scanner with extra data
///
/// # Arguments
/// * `yy_user_defined` - User-defined extra data
///
/// # Returns
/// Result containing the new scanner or an error code
pub fn jq_yylex_init_extra(yy_user_defined: i32) -> Result<Box<YyGutsT>, i32> {
    let mut scanner = Box::new(YyGutsT::default());
    scanner.yyextra_r = yy_user_defined;
    yy_init_globals(&mut scanner);
    Ok(scanner)
}
/// Initialize global state of scanner
///
/// # Arguments
/// * `yyscanner` - Scanner instance to initialize
fn yy_init_globals(yyscanner: &mut YyGutsT) {
    yyscanner.yy_buffer_stack = Vec::new();
    yyscanner.yy_buffer_stack_top = 0;
    yyscanner.yy_buffer_stack_max = 0;
    yyscanner.yy_c_buf_p = 0;
    yyscanner.yy_init = false;
    yyscanner.yy_start = 0;
    yyscanner.yy_start_stack_ptr = 0;
    yyscanner.yy_start_stack_depth = 0;
    yyscanner.yy_start_stack = Vec::new();
    yyscanner.yyin_r = None;
    yyscanner.yyout_r = None;
    yyscanner.yylineno_r = 1;
    yyscanner.yy_flex_debug_r = 0;
    yyscanner.yytext_r = Vec::new();
    yyscanner.yyleng_r = 0;
}
/// Destroy the lexer
pub fn jq_yylex_destroy(yyscanner: &mut YyGutsT) -> i32 {
    while yyscanner.current_buffer().is_some() {
        let top = yyscanner.yy_buffer_stack_top;
        if top < yyscanner.yy_buffer_stack.len() {
            yyscanner.yy_buffer_stack[top] = None;
        }
        jq_yypop_buffer_state_internal(yyscanner);
    }
    yyscanner.yy_buffer_stack.clear();
    yyscanner.yy_start_stack.clear();
    yy_init_globals(yyscanner);
    0
}
/// Ensure buffer stack has capacity
///
/// # Arguments
/// * `yyscanner` - Scanner instance
pub fn jq_yyensure_buffer_stack(yyscanner: &mut YyGutsT) {
    if yyscanner.yy_buffer_stack.is_empty() {
        yyscanner.yy_buffer_stack = Vec::with_capacity(1);
        yyscanner.yy_buffer_stack_max = 1;
    }
    if yyscanner.yy_buffer_stack_top >= yyscanner.yy_buffer_stack.len() {
        let needed = yyscanner.yy_buffer_stack_top + 1;
        yyscanner.yy_buffer_stack.resize_with(needed, || None);
        yyscanner.yy_buffer_stack_max = needed;
    }
}
/// Switch to a different buffer
pub fn jq_yy_switch_to_buffer(
    new_buffer: Option<Box<YyBufferState>>,
    yyscanner: &mut YyscanT,
) {
    let yyg = match yyscanner {
        Some(ref mut g) => g,
        None => return,
    };
    jq_yy_switch_to_buffer_internal(new_buffer, yyg);
}

/// Internal helper for switching buffer (takes YyGutsT directly)
fn jq_yy_switch_to_buffer_internal(
    new_buffer: Option<Box<YyBufferState>>,
    yyg: &mut YyGutsT,
) {
    jq_yyensure_buffer_stack_internal(yyg);
    let current_buffer_ptr = if !yyg.yy_buffer_stack.is_empty() {
        yyg.yy_buffer_stack
            .get(yyg.yy_buffer_stack_top)
            .and_then(|b| b.as_ref().map(|_| true))
    } else {
        None
    };
    if current_buffer_ptr.is_some() && new_buffer.is_some() {}
    if !yyg.yy_buffer_stack.is_empty() {
        if let Some(Some(ref mut buffer)) = yyg
            .yy_buffer_stack
            .get_mut(yyg.yy_buffer_stack_top)
        {
            if yyg.yy_c_buf_p < buffer.yy_ch_buf.len() {
                buffer.yy_ch_buf[yyg.yy_c_buf_p] = yyg.yy_hold_char;
            }
            buffer.yy_buf_pos = yyg.yy_c_buf_p;
            buffer.yy_n_chars = yyg.yy_n_chars;
        }
    }
    if yyg.yy_buffer_stack_top < yyg.yy_buffer_stack.len() {
        yyg.yy_buffer_stack[yyg.yy_buffer_stack_top] = new_buffer;
    } else if !yyg.yy_buffer_stack.is_empty() {
        yyg.yy_buffer_stack.push(new_buffer);
        yyg.yy_buffer_stack_top = yyg.yy_buffer_stack.len() - 1;
    }
    jq_yy_load_buffer_state_internal(yyg);
    yyg.yy_did_buffer_switch_on_eof = true;
}
/// Delete a buffer
pub fn jq_yy_delete_buffer(b: Option<Box<YyBufferState>>, yyscanner: &mut YyGutsT) {
    let b = match b {
        Some(buf) => buf,
        None => return,
    };
    let is_current = if let Some(current) = yyscanner.get_current_buffer() {
        std::ptr::eq(current, b.as_ref())
    } else {
        false
    };
    if is_current && !yyscanner.yy_buffer_stack.is_empty() {
        let top = yyscanner.yy_buffer_stack_top;
        if top < yyscanner.yy_buffer_stack.len() {
            yyscanner.yy_buffer_stack[top] = None;
        }
    }
}
/// Push a new buffer state onto the stack
///
/// # Arguments
/// * `new_buffer` - Buffer to push
/// * `yyscanner` - Scanner instance
pub fn jq_yypush_buffer_state(
    new_buffer: Option<Box<YyBufferState>>,
    yyscanner: &mut YyGutsT,
) {
    let new_buffer = match new_buffer {
        Some(b) => b,
        None => return,
    };
    jq_yyensure_buffer_stack(yyscanner);
    if yyscanner.yy_buffer_stack_top < yyscanner.yy_buffer_stack.len() {
        if let Some(Some(ref mut current)) = yyscanner
            .yy_buffer_stack
            .get_mut(yyscanner.yy_buffer_stack_top)
        {
            current.yy_buf_pos = yyscanner.yy_c_buf_p;
            current.yy_n_chars = yyscanner.yy_n_chars;
            if yyscanner.yy_c_buf_p < current.yy_ch_buf.len() {
                current.yy_ch_buf[yyscanner.yy_c_buf_p] = yyscanner.yy_hold_char;
            }
        }
    }
    let has_current = yyscanner.yy_buffer_stack_top < yyscanner.yy_buffer_stack.len()
        && yyscanner.yy_buffer_stack[yyscanner.yy_buffer_stack_top].is_some();
    if has_current {
        yyscanner.yy_buffer_stack_top += 1;
    }
    if yyscanner.yy_buffer_stack_top >= yyscanner.yy_buffer_stack.len() {
        yyscanner.yy_buffer_stack.push(Some(new_buffer));
    } else {
        yyscanner.yy_buffer_stack[yyscanner.yy_buffer_stack_top] = Some(new_buffer);
    }
    jq_yy_load_buffer_state(yyscanner);
    yyscanner.yy_did_buffer_switch_on_eof = true;
}
/// Pop a buffer state
pub fn jq_yypop_buffer_state(yyscanner: &mut YyGutsT) {
    jq_yypop_buffer_state_internal(yyscanner);
}

/// Internal helper for popping buffer state
fn jq_yypop_buffer_state_internal(yyg: &mut YyGutsT) {
    if yyg.yy_buffer_stack.is_empty() || yyg.yy_buffer_stack_top == 0 {
        return;
    }
    yyg.yy_buffer_stack[yyg.yy_buffer_stack_top] = None;
    yyg.yy_buffer_stack_top -= 1;
    if !yyg.yy_buffer_stack.is_empty()
        && yyg.yy_buffer_stack[yyg.yy_buffer_stack_top].is_some()
    {
        jq_yy_load_buffer_state_internal(yyg);
        yyg.yy_did_buffer_switch_on_eof = true;
    }
}
/// Scan a string
///
/// # Arguments
/// * `yystr` - String to scan
/// * `yyscanner` - Scanner instance
///
/// # Returns
/// New buffer state
pub fn jq_yy_scan_string(yystr: &str, yyscanner: &mut YyGutsT) -> Box<YyBufferState> {
    jq_yy_scan_bytes(yystr.as_bytes(), yyscanner)
}
/// Scan bytes
///
/// # Arguments
/// * `yybytes` - Bytes to scan
/// * `yyscanner` - Scanner instance
///
/// # Returns
/// New buffer state
pub fn jq_yy_scan_bytes(yybytes: &[u8], yyscanner: &mut YyGutsT) -> Box<YyBufferState> {
    let buf_size = yybytes.len() + 2;
    let mut ch_buf = vec![0u8; buf_size];
    ch_buf[..yybytes.len()].copy_from_slice(yybytes);
    ch_buf[yybytes.len()] = 0;
    ch_buf[yybytes.len() + 1] = 0;

    // Create the buffer to return
    let buffer = Box::new(YyBufferState {
        yy_input_file: None,
        yy_ch_buf: ch_buf.clone(),
        yy_buf_pos: 0,
        yy_buf_size: yybytes.len() as i32,
        yy_n_chars: yybytes.len() as i32,
        yy_is_our_buffer: true,
        yy_is_interactive: false,
        yy_at_bol: 1,
        yy_bs_lineno: 0,
        yy_bs_column: 0,
        yy_fill_buffer: false,
        yy_buffer_status: 0,
    });

    // Create a separate buffer for switch_to_buffer
    let buffer_for_switch = Box::new(YyBufferState {
        yy_input_file: None,
        yy_ch_buf: ch_buf,
        yy_buf_pos: 0,
        yy_buf_size: yybytes.len() as i32,
        yy_n_chars: yybytes.len() as i32,
        yy_is_our_buffer: true,
        yy_is_interactive: false,
        yy_at_bol: 1,
        yy_bs_lineno: 0,
        yy_bs_column: 0,
        yy_fill_buffer: false,
        yy_buffer_status: 0,
    });

    jq_yy_switch_to_buffer_internal(Some(buffer_for_switch), yyscanner);
    buffer
}
/// Scan a buffer directly
pub fn jq_yy_scan_buffer(
    base: Vec<u8>,
    size: YySizeT,
    yyscanner: &mut YyscanT,
) -> Option<Box<YyBufferState>> {
    if size < 2 {
        return None;
    }
    if base.len() < size || base[size - 2] != 0 || base[size - 1] != 0 {
        return None;
    }
    let mut buffer = Box::new(YyBufferState {
        yy_buf_size: (size - 2) as i32,
        yy_ch_buf: base,
        yy_buf_pos: 0,
        yy_is_our_buffer: false,
        yy_input_file: None,
        yy_n_chars: (size - 2) as i32,
        yy_is_interactive: false,
        yy_at_bol: 1,
        yy_bs_lineno: 0,
        yy_bs_column: 0,
        yy_fill_buffer: false,
        yy_buffer_status: YY_BUFFER_NEW,
    });
    let buffer_for_switch = Some(
        Box::new(YyBufferState {
            yy_buf_size: buffer.yy_buf_size,
            yy_ch_buf: buffer.yy_ch_buf.clone(),
            yy_buf_pos: buffer.yy_buf_pos,
            yy_is_our_buffer: buffer.yy_is_our_buffer,
            yy_input_file: None,
            yy_n_chars: buffer.yy_n_chars,
            yy_is_interactive: buffer.yy_is_interactive,
            yy_at_bol: buffer.yy_at_bol,
            yy_bs_lineno: buffer.yy_bs_lineno,
            yy_bs_column: buffer.yy_bs_column,
            yy_fill_buffer: buffer.yy_fill_buffer,
            yy_buffer_status: buffer.yy_buffer_status,
        }),
    );
    jq_yy_switch_to_buffer(buffer_for_switch, yyscanner);
    Some(buffer)
}
/// Restart lexer with new input
///
/// # Arguments
/// * `input_file` - New input source
/// * `yyscanner` - Scanner instance
pub fn jq_yyrestart<R: Read + Send + 'static>(
    input_file: Option<R>,
    yyscanner: &mut YyGutsT,
) {
    jq_yyensure_buffer_stack(yyscanner);
    let current_buffer = if yyscanner.yy_buffer_stack_top
        < yyscanner.yy_buffer_stack.len()
    {
        yyscanner
            .yy_buffer_stack
            .get(yyscanner.yy_buffer_stack_top)
            .and_then(|b| b.as_ref())
    } else {
        None
    };
    if current_buffer.is_none() {
        let new_buffer = jq_yy_create_buffer(YY_BUF_SIZE);
        if yyscanner.yy_buffer_stack_top >= yyscanner.yy_buffer_stack.len() {
            yyscanner.yy_buffer_stack.push(Some(new_buffer));
        } else {
            yyscanner.yy_buffer_stack[yyscanner.yy_buffer_stack_top] = Some(new_buffer);
        }
    }
    if let Some(buffer) = yyscanner
        .yy_buffer_stack
        .get_mut(yyscanner.yy_buffer_stack_top)
    {
        if let Some(ref mut buf) = buffer {
            jq_yy_init_buffer_internal(buf, input_file);
        }
    }
    jq_yy_load_buffer_state(yyscanner);
}
/// Allocate memory for lexer
///
/// # Arguments
/// * `sz` - Size to allocate
///
/// # Returns
/// Allocated memory as a vector
pub fn jq_yyalloc(sz: usize) -> Vec<u8> {
    vec![0u8; sz]
}
/// Reallocate memory for lexer
///
/// # Arguments
/// * `p` - Existing allocation
/// * `sz` - New size
///
/// # Returns
/// Reallocated memory
pub fn jq_yyrealloc(mut p: Vec<u8>, sz: usize) -> Vec<u8> {
    p.resize(sz, 0);
    p
}
/// Free memory allocated by lexer
///
/// # Arguments
/// * `p` - Pointer to memory (as raw pointer for compatibility)
/// * `_extra` - Extra data (unused)
///
/// Note: In idiomatic Rust, this is typically handled by Drop trait,
/// but we provide this for API compatibility
pub fn jq_yyfree<T>(_p: Option<Box<T>>, _extra: &mut ()) {}
/// Check if jv value is valid
///
/// # Arguments
/// * `x` - Value to check
///
/// # Returns
/// 1 if valid, 0 otherwise
pub fn jv_is_valid(x: &crate::jv::Jv) -> i32 {
    1
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_scanner_creation() {
        let scanner = YyGutsT::default();
        assert_eq!(jq_yyget_debug(&scanner), 0);
    }
    #[test]
    fn test_location_default() {
        let loc = Location::default();
        assert_eq!(loc.start, 0);
        assert_eq!(loc.end, 0);
    }
    #[test]
    fn test_push_pop_state() {
        let mut scanner = YyGutsT::default();
        yy_push_state(1, &mut scanner);
        assert!(scanner.yy_start_stack_ptr > 0);
        yy_pop_state(&mut scanner);
        assert_eq!(scanner.yy_start_stack_ptr, 0);
    }
    #[test]
    fn test_enter_exit() {
        let mut scanner = YyGutsT::default();
        let result = enter('(' as i32, 0, &mut scanner);
        // enter() currently just returns currstate
        assert_eq!(result, 0);
    }
    #[test]
    fn test_try_exit() {
        let mut scanner = YyGutsT::default();
        yy_push_state(1, &mut scanner);
        let result = try_exit(']' as i32, 1, &mut scanner);
        // try_exit() currently just returns 0
        assert_eq!(result, 0);
    }

    #[test]
    fn test_dot_token() {
        // Test that '.' produces the correct token (ASCII 46)
        let mut scanner = jq_yylex_init_extra(0).expect("scanner init");
        let _buf = jq_yy_scan_bytes(b".", &mut scanner);
        let mut yylval = YYSType::default();
        let mut yylloc = Location::default();
        let token = jq_yylex(&mut yylval, &mut yylloc, &mut scanner);
        // '.' should return ASCII 46
        assert_eq!(token, 46, "dot should return ASCII 46, got {}", token);
    }

    #[test]
    fn test_dot_via_pointer() {
        // Test lexing through a pointer (like jq_parse does)
        let mut lexer = jq_yylex_init_extra(0).expect("scanner init");
        let _buf = jq_yy_scan_bytes(b".", &mut lexer);

        // Get pointer to scanner like jq_parse does
        let scanner_ptr: *mut YyGutsT = &mut *lexer as *mut _;

        let mut yylval = YYSType::default();
        let mut yylloc = Location::default();

        // Call through pointer like yylex() in parser.rs does
        let token = unsafe {
            let scanner = &mut *scanner_ptr;
            jq_yylex(&mut yylval, &mut yylloc, scanner)
        };

        assert_eq!(token, 46, "dot via pointer should return 46, got {}", token);
    }
}
/// Buffer size constant
const YY_BUF_SIZE: i32 = 16384;
/// Error codes
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
/// Initialize a buffer with input
fn jq_yy_init_buffer_internal<R: Read + 'static>(
    buffer: &mut YyBufferState,
    input: Option<R>,
) {
    buffer.yy_input_file = input.map(|r| Box::new(r) as Box<dyn Read>);
    buffer.yy_buf_pos = 0;
    buffer.yy_n_chars = 0;
    buffer.yy_at_bol = 1;
    buffer.yy_bs_lineno = 1;
    buffer.yy_bs_column = 0;
    buffer.yy_fill_buffer = true;
    buffer.yy_buffer_status = 0;
    for byte in buffer.yy_ch_buf.iter_mut() {
        *byte = 0;
    }
}
/// Get the matched text as a string slice
fn get_yytext(yyg: &YyGutsT, yy_bp: usize, yy_cp: usize) -> String {
    let buf = yyg.get_current_buffer();
    if let Some(b) = buf {
        if yy_cp <= b.yy_ch_buf.len() && yy_bp < yy_cp {
            let bytes = &b.yy_ch_buf[yy_bp..yy_cp];
            return String::from_utf8_lossy(bytes).to_string();
        }
    }
    String::new()
}

/// Get matched text length
fn get_yyleng(yy_bp: usize, yy_cp: usize) -> i32 {
    (yy_cp - yy_bp) as i32
}

/// Main lexer function - performs DFA-based tokenization
pub fn jq_yylex(
    yylval_param: &mut YYSType,
    yylloc_param: &mut Location,
    yyscanner: &mut YyGutsT,
) -> i32 {
    // Initialize scanner state on first call
    if !yyscanner.yy_init {
        yyscanner.yy_init = true;
        if yyscanner.yy_start == 0 {
            yyscanner.yy_start = 1;  // INITIAL + 1
        }
        if yyscanner.get_current_buffer().is_none() {
            jq_yyensure_buffer_stack(yyscanner);
            let buf = jq_yy_create_buffer(YY_BUF_SIZE);
            let top = yyscanner.yy_buffer_stack_top;
            if top < yyscanner.yy_buffer_stack.len() {
                yyscanner.yy_buffer_stack[top] = Some(buf);
            }
        }
        jq_yy_load_buffer_state(yyscanner);
    }

    // Main tokenization loop
    loop {
        // Get current buffer
        let (ch_buf, buf_size, n_chars) = {
            let buf = match yyscanner.get_current_buffer() {
                Some(b) => b,
                None => return 0,  // EOF
            };
            (buf.yy_ch_buf.clone(), buf.yy_buf_size, buf.yy_n_chars)
        };

        // Start of current token
        let yy_bp = yyscanner.yy_c_buf_p;
        let mut yy_cp = yy_bp;

        // Check if we're at end of buffer BEFORE trying to match
        if yy_bp >= n_chars as usize {
            // At end of buffer - try to get more input
            let action = yy_get_next_buffer(yyscanner);
            match action {
                EOB_ACT_END_OF_FILE => {
                    return 0;  // EOF - no more input
                }
                EOB_ACT_CONTINUE_SCAN => {
                    continue;  // More input available, rescan
                }
                EOB_ACT_LAST_MATCH => {
                    continue;  // Process any remaining match
                }
                _ => return 0,
            }
        }

        // Restore hold char at current position
        if yy_cp < ch_buf.len() {
            // hold char would be restored here in C version
        }

        // Get start state
        let mut yy_current_state = yyscanner.yy_start;

        // DFA matching loop - find longest match
        loop {
            // Get current character
            let ch = if yy_cp < ch_buf.len() { ch_buf[yy_cp] } else { 0 };

            // Convert to equivalence class
            // NUL (0) is specially handled as equivalence class 1 (end-of-buffer marker)
            // This matches the C flex behavior: (*yy_cp ? yy_ec[*yy_cp] : 1)
            let yy_c = if ch != 0 { YY_EC[ch as usize] } else { 1 };

            // Check for accepting state
            if YY_ACCEPT[yy_current_state as usize] != 0 {
                yyscanner.yy_last_accepting_state = yy_current_state;
                yyscanner.yy_last_accepting_cpos = yy_cp;
            }

            // Find next state via DFA transitions
            let mut c = yy_c;
            while YY_CHK[(YY_BASE[yy_current_state as usize] + c as i16) as usize] != yy_current_state as i16 {
                yy_current_state = YY_DEF[yy_current_state as usize] as i32;
                if yy_current_state >= 168 {
                    c = YY_META[c as usize];
                }
            }
            yy_current_state = YY_NXT[(YY_BASE[yy_current_state as usize] + c as i16) as usize] as i32;
            yy_cp += 1;

            // Check if we've reached end of DFA (base == 332 indicates final state)
            if YY_BASE[yy_current_state as usize] == 332 {
                break;
            }
        }

        // Get the action for the accepting state
        let mut yy_act = YY_ACCEPT[yy_current_state as usize];

        // Handle backup if no accepting state found
        if yy_act == 0 {
            yy_cp = yyscanner.yy_last_accepting_cpos;
            yy_current_state = yyscanner.yy_last_accepting_state;
            yy_act = YY_ACCEPT[yy_current_state as usize];
        }

        // Get matched text
        let yytext = get_yytext(yyscanner, yy_bp, yy_cp);
        let yyleng = get_yyleng(yy_bp, yy_cp);

        // Update location
        yylloc_param.start = yyscanner.yyextra_r;
        yylloc_param.end = yylloc_param.start + yyleng;
        yyscanner.yyextra_r = yylloc_param.end;
        yyscanner.yyleng_r = yyleng;

        // Update buffer position for next token
        yyscanner.yy_c_buf_p = yy_cp;
        if yy_cp < ch_buf.len() {
            yyscanner.yy_hold_char = ch_buf[yy_cp];
        }

        // Execute token action based on DFA accepting state
        let debug_lexer = std::env::var("DEBUG_PARSER").is_ok();
        if debug_lexer {
            eprintln!("LEXER: yy_act={} yytext={:?} yy_bp={} yy_cp={}", yy_act, yytext, yy_bp, yy_cp);
        }
        match yy_act {
            // Case 0: must back up (handled above)
            0 => {
                continue;
            }

            // Case 1: "#" - start comment
            1 => {
                yy_push_state(1 + 2 * IN_COMMENT, yyscanner);
                continue;  // No token returned, continue scanning
            }

            // Case 2: comment content (any char except newline)
            2 => {
                continue;  // Consume and continue
            }

            // Case 3: newline in comment - end comment
            3 => {
                yy_pop_state(yyscanner);
                continue;
            }

            // Case 4: "!=" -> NEQ
            4 => {
                return NEQ;
            }

            // Case 5: "==" -> EQ
            5 => {
                return EQ;
            }

            // Case 6: "as" -> AS
            6 => {
                return AS;
            }

            // Case 7: "import" -> IMPORT
            7 => {
                return IMPORT;
            }

            // Case 8: "include" -> INCLUDE
            8 => {
                return INCLUDE;
            }

            // Case 9: "module" -> MODULE
            9 => {
                return MODULE;
            }

            // Case 10: "def" -> DEF
            10 => {
                return DEF;
            }

            // Case 11: "if" -> IF
            11 => {
                return IF;
            }

            // Case 12: "then" -> THEN
            12 => {
                return THEN;
            }

            // Case 13: "else" -> ELSE
            13 => {
                return ELSE;
            }

            // Case 14: "elif" -> ELSE_IF
            14 => {
                return ELSE_IF;
            }

            // Case 15: "and" -> AND
            15 => {
                return AND;
            }

            // Case 16: "or" -> OR
            16 => {
                return OR;
            }

            // Case 17: "end" -> END
            17 => {
                return END;
            }

            // Case 18: "reduce" -> REDUCE
            18 => {
                return REDUCE;
            }

            // Case 19: "foreach" -> FOREACH
            19 => {
                return FOREACH;
            }

            // Case 20: "//" -> DEFINEDOR
            20 => {
                return DEFINEDOR;
            }

            // Case 21: "try" -> TRY
            21 => {
                return TRY;
            }

            // Case 22: "catch" -> CATCH
            22 => {
                return CATCH;
            }

            // Case 23: "label" -> LABEL
            23 => {
                return LABEL;
            }

            // Case 24: "break" -> BREAK
            24 => {
                return BREAK;
            }

            // Case 25: "$__loc__" -> LOC
            25 => {
                return LOC;
            }

            // Case 26: "|=" -> SETPIPE
            26 => {
                return SETPIPE;
            }

            // Case 27: "+=" -> SETPLUS
            27 => {
                return SETPLUS;
            }

            // Case 28: "-=" -> SETMINUS
            28 => {
                return SETMINUS;
            }

            // Case 29: "*=" -> SETMULT
            29 => {
                return SETMULT;
            }

            // Case 30: "/=" -> SETDIV
            30 => {
                return SETDIV;
            }

            // Case 31: "%=" -> SETMOD
            31 => {
                return SETMOD;
            }

            // Case 32: "//=" -> SETDEFINEDOR
            32 => {
                return SETDEFINEDOR;
            }

            // Case 33: "<=" -> LESSEQ
            33 => {
                return LESSEQ;
            }

            // Case 34: ">=" -> GREATEREQ
            34 => {
                return GREATEREQ;
            }

            // Case 35: ".." -> REC
            35 => {
                return REC;
            }

            // Case 36: "?//" -> ALTERNATION
            36 => {
                return ALTERNATION;
            }

            // Case 37: single char operators (. | @ : ; , ? $ = < > ! + - * / % ^)
            37 => {
                if !yytext.is_empty() {
                    return yytext.chars().next().unwrap() as i32;
                }
                return INVALID_CHARACTER;
            }

            // Case 38: '(' '[' '{' - opening brackets
            38 => {
                if !yytext.is_empty() {
                    let c = yytext.chars().next().unwrap();
                    return enter_bracket(c as i32, yyscanner.yy_start, yyscanner);
                }
                return INVALID_CHARACTER;
            }

            // Case 39: ')' ']' '}' - closing brackets
            39 => {
                if !yytext.is_empty() {
                    let c = yytext.chars().next().unwrap();
                    return try_exit_bracket(c as i32, yyscanner.yy_start, yyscanner);
                }
                return INVALID_CHARACTER;
            }

            // Case 40: @format - format string
            40 => {
                if yytext.len() > 1 {
                    let format_str = &yytext[1..];  // Skip the '@'
                    *yylval_param = YYSType::Literal(jv_string(format_str));
                    return FORMAT;
                }
                return INVALID_CHARACTER;
            }

            // Case 41: number literal
            41 => {
                let parsed = jv_parse_sized(&yytext, yyleng);
                *yylval_param = YYSType::Literal(parsed);
                return LITERAL;
            }

            // Case 42: '"' - start quoted string
            42 => {
                yy_push_state(1 + 2 * IN_QQSTRING, yyscanner);
                return QQSTRING_START;
            }

            // Case 43: "\\(" in string - start interpolation
            43 => {
                let token = enter_bracket(QQSTRING_INTERP_START, yyscanner.yy_start, yyscanner);
                return token;
            }

            // Case 44: '"' in string - end quoted string
            44 => {
                yy_pop_state(yyscanner);
                return QQSTRING_END;
            }

            // Case 45: escape sequences in string (\\., \\n, etc)
            45 => {
                // Parse escape sequence through JSON parser
                let escaped = format!("\"{}\"", yytext);
                let parsed = jv_parse_sized(&escaped, escaped.len() as i32);
                *yylval_param = YYSType::Literal(parsed);
                return QQSTRING_TEXT;
            }

            // Case 46: regular text in string (non-special chars)
            46 => {
                *yylval_param = YYSType::Literal(jv_string_sized(&yytext, yytext.len()));
                return QQSTRING_TEXT;
            }

            // Case 47: invalid char in string
            47 => {
                return INVALID_CHARACTER;
            }

            // Case 48: identifier
            48 => {
                *yylval_param = YYSType::Literal(jv_string(&yytext));
                return IDENT;
            }

            // Case 49: .field (field access)
            49 => {
                if yytext.len() > 1 {
                    let field_name = &yytext[1..];  // Skip the '.'
                    *yylval_param = YYSType::Literal(jv_string(field_name));
                    return FIELD;
                }
                return INVALID_CHARACTER;
            }

            // Case 50: $binding (variable binding)
            50 => {
                if yytext.len() > 1 {
                    let binding_name = &yytext[1..];  // Skip the '$'
                    *yylval_param = YYSType::Literal(jv_string(binding_name));
                    return BINDING;
                }
                return INVALID_CHARACTER;
            }

            // Case 51: whitespace - skip
            51 => {
                continue;
            }

            // Case 52: invalid character
            52 => {
                return INVALID_CHARACTER;
            }

            // Case 53: scanner jammed (should never happen)
            53 => {
                eprintln!("fatal flex scanner jammed");
                return 0;
            }

            // Case 54: end of buffer (YY_END_OF_BUFFER)
            54 => {
                // Handle end of buffer
                let action = yy_get_next_buffer(yyscanner);
                match action {
                    EOB_ACT_END_OF_FILE => {
                        return 0;  // EOF
                    }
                    EOB_ACT_CONTINUE_SCAN => {
                        continue;  // More input available
                    }
                    EOB_ACT_LAST_MATCH => {
                        // Process last match then EOF
                        continue;
                    }
                    _ => return 0,
                }
            }

            // Default: unknown action, return EOF
            _ => {
                return 0;
            }
        }
    }
}

/// Helper for entering bracket states
fn enter_bracket(c: i32, currstate: i32, yyscanner: &mut YyGutsT) -> i32 {
    match c as u8 as char {
        '(' => {
            yy_push_state(1 + 2 * IN_PAREN, yyscanner);
            '(' as i32
        }
        '[' => {
            yy_push_state(1 + 2 * IN_BRACKET, yyscanner);
            '[' as i32
        }
        '{' => {
            yy_push_state(1 + 2 * IN_BRACE, yyscanner);
            '{' as i32
        }
        _ => {
            // For QQSTRING_INTERP_START
            if c == QQSTRING_INTERP_START {
                yy_push_state(1 + 2 * IN_QQINTERP, yyscanner);
                return QQSTRING_INTERP_START;
            }
            c
        }
    }
}

/// Helper for exiting bracket states
fn try_exit_bracket(c: i32, currstate: i32, yyscanner: &mut YyGutsT) -> i32 {
    let ch = c as u8 as char;
    let (expected, ret) = match currstate {
        s if s == 1 + 2 * IN_PAREN => (')', ')' as i32),
        s if s == 1 + 2 * IN_BRACKET => (']', ']' as i32),
        s if s == 1 + 2 * IN_BRACE => ('}', '}' as i32),
        s if s == 1 + 2 * IN_QQINTERP => (')', QQSTRING_INTERP_END),
        _ => return INVALID_CHARACTER,
    };

    if ch == expected {
        yy_pop_state(yyscanner);
        ret
    } else {
        INVALID_CHARACTER
    }
}
/// Lexer tables - generated by flex, extracted from lexer.c
static YY_ACCEPT: [i16; 168] = [
    0,
    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
    0,    0,    0,    0,   54,   52,   51,   51,   52,   42,
    1,   37,   37,   38,   39,   37,   37,   37,   37,   37,
   37,   41,   37,   37,   37,   37,   52,   48,   48,   48,
   48,   48,   48,   48,   48,   48,   48,   48,   48,   48,
   37,   46,   46,   44,   47,    2,    3,    2,   51,    4,
   50,   50,   31,   29,   27,   28,   35,   41,   49,   20,
   30,   41,   41,    0,   33,    5,   34,    0,   40,   48,
    0,   48,    6,   48,   48,   48,   48,   48,   48,   11,
   48,   48,   48,   48,   16,   48,   48,   48,   26,   46,
   45,   43,   45,    2,   50,    0,   50,   49,   32,   41,
    0,   41,   36,    0,   15,   48,   48,   10,   48,   48,
   17,   48,   48,   48,   48,   48,   48,   48,   21,    0,
   45,    0,   50,   48,   48,   48,   14,   13,   48,   48,
   48,   48,   48,   48,   12,   45,   50,   24,   22,   48,
   48,   48,   23,   48,   48,   45,   50,   48,    7,   48,
    9,   18,   50,   19,    8,   25,    0
];

static YY_EC: [YyChar; 256] = [
    0,    1,    1,    1,    1,    1,    1,    1,    1,    2,    3,
    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,
    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,
    1,    2,    4,    5,    6,    7,    8,    1,    1,    9,
   10,   11,   12,   13,   14,   15,   16,   17,   17,   17,
   17,   17,   17,   17,   17,   17,   17,   18,   19,   20,
   21,   22,   23,   24,   25,   25,   25,   25,   26,   25,
   25,   25,   25,   25,   25,   25,   25,   25,   25,   25,
   25,   25,   25,   25,   25,   25,   25,   25,   25,   25,
   27,   28,   29,    1,   30,    1,   31,   32,   33,   34,
   35,   36,   25,   37,   38,   25,   39,   40,   41,   42,
   43,   44,   25,   45,   46,   47,   48,   25,   25,   25,
   49,   25,   50,   51,   52,    1,    1,    1,    1,    1,
    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,
    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,
    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,
    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,
    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,
    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,
    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,
    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,
    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,
    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,
    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,
    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,
    1,    1,    1,    1,    1
];

static YY_META: [YyChar; 53] = [
    0,    1,    1,    1,    1,    2,    1,    1,    1,    1,    1,
    1,    1,    1,    1,    3,    1,    4,    5,    1,    1,
    1,    1,    1,    1,    6,    6,    1,    7,    1,    8,
    6,    6,    6,    6,    6,    6,    6,    6,    6,    6,
    6,    6,    6,    6,    6,    6,    6,    6,    6,    1,
    1,    1
];

static YY_BASE: [i16; 183] = [
    0,
    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
   50,   51,   54,   55,  331,  332,   57,   59,  309,  332,
  332,  299,  307,  332,  332,  306,  305,  332,  304,   48,
   48,   51,  303,  302,  301,  305,    0,  302,   49,   52,
   53,   54,   56,   57,   63,   62,   58,   67,   55,   69,
  298,    0,    0,  332,   79,  332,  332,   89,  100,  332,
  300,   89,  332,  332,  332,  332,  332,   94,    0,  296,
  332,   96,  106,  116,  332,  332,  332,  300,    0,  297,
  296,   76,  295,   90,   91,   98,   97,  106,  100,  294,
  108,  118,  121,  124,  293,  126,  119,  128,  332,    0,
  282,  332,  281,  332,  288,  284,  129,    0,  332,  131,
  281,  278,  332,    0,  273,  131,  132,  270,  137,  141,
  268,  143,  138,  145,  149,  146,  150,  153,  265,  161,
  242,    0,  154,  247,  157,  156,  244,  239,  161,  162,
  164,  165,  168,  169,  237,  224,  170,  232,  228,  171,
  172,  181,  209,  182,  183,  196,  192,  188,  205,  193,
  203,  202,  196,  198,  195,  173,  332,  228,  236,  239,
  245,  250,  255,  263,  271,  276,  281,  286,  288,  293,
  297,  301
];

static YY_DEF: [i16; 183] = [
  167,
    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,
  168,  168,  169,  169,  167,  167,  167,  167,  167,  167,
  167,  170,  167,  167,  167,  167,  167,  167,  167,  171,
  167,  167,  167,  167,  167,  167,  172,  173,  173,  173,
  173,  173,  173,  173,  173,  173,  173,  173,  173,  173,
  167,  174,  174,  167,  175,  167,  167,  167,  167,  167,
  176,  176,  167,  167,  167,  167,  167,  167,  177,  167,
  167,  167,  167,  167,  167,  167,  167,  167,  172,  173,
  167,  173,  173,  173,  173,  173,  173,  173,  173,  173,
  173,  173,  173,  173,  173,  173,  173,  173,  167,  174,
  167,  167,  178,  167,  176,  167,  176,  177,  167,  167,
  167,  167,  167,  179,  173,  173,  173,  173,  173,  173,
  173,  173,  173,  173,  173,  173,  173,  173,  173,  175,
  180,  170,  176,  173,  173,  173,  173,  173,  173,  173,
  173,  173,  173,  173,  173,  181,  176,  173,  173,  173,
  173,  173,  173,  173,  173,  182,  176,  173,  173,  173,
  173,  173,  176,  173,  173,  176,    0,  167,  167,  167,
  167,  167,  167,  167,  167,  167,  167,  167,  167,  167,
  167,  167
];

static YY_NXT: [i16; 385] = [
    0,   16,   17,   18,   19,   20,   21,   22,   23,   24,   25,
   26,   27,   28,   29,   30,   31,   32,   28,   28,   33,
   34,   35,   36,   37,   38,   38,   24,   16,   25,   38,
   39,   40,   41,   42,   43,   44,   38,   45,   38,   46,
   47,   38,   48,   38,   49,   38,   50,   38,   38,   24,
   51,   25,   53,   53,   54,   54,   57,   57,   59,   59,
   59,   59,   67,   70,   68,   72,   81,   73,   71,   81,
   81,   81,   81,   81,   81,   81,   74,   55,   55,   81,
   81,   58,   58,   85,   81,   74,   81,  102,   86,   96,
   82,  104,   93,   81,   83,   87,   84,   88,   90,   89,
   94,   59,   59,   91,   92,   97,  106,   81,   81,  115,
   68,   95,  110,   98,   81,   81,  104,   81,  107,   74,
   72,   74,   73,   81,  116,   81,  103,  111,   74,  111,
   74,   74,  112,  118,  119,   81,   81,  117,   81,  121,
   74,   81,  120,   81,  122,   81,  106,  110,   81,   81,
  124,  123,  125,  128,   81,   81,   74,  126,   81,  127,
   81,  135,   81,   81,  136,   74,   81,   81,  133,  167,
   81,  106,  137,   81,   81,  138,  129,  139,   81,   81,
  140,   81,   81,  142,  141,   81,   81,  106,   81,   81,
  106,  150,  149,  143,  145,  148,  147,  144,   81,   81,
   81,  155,  157,  158,  153,   81,  151,  154,  103,  106,
   81,  152,   81,  106,  160,   81,  161,  162,  159,   81,
   81,  163,   81,  130,  164,  166,   81,  165,   52,   52,
   52,   52,   52,   52,   52,   52,   56,   56,   56,   56,
   56,   56,   56,   56,   61,   81,   61,   69,   69,   81,
   69,  130,   69,   79,   81,   79,   81,   79,   80,   80,
   80,   81,   80,  100,   81,  100,  100,  100,  100,  130,
  100,  101,  101,  101,  101,  101,  101,  101,  101,  105,
  105,  105,   81,  105,  108,   81,  108,   81,  108,  131,
   81,  131,  131,  134,  112,  134,  146,  112,  146,  146,
  156,  132,  156,  156,  101,  106,  101,  101,  130,  130,
   81,   81,   81,  114,   81,  113,  109,  106,   99,   81,
   78,   77,   76,   75,   66,   65,   64,   63,   62,   60,
  167,   15,  167,  167,  167,  167,  167,  167,  167,  167,
  167,  167,  167,  167,  167,  167,  167,  167,  167,  167,
  167,  167,  167,  167,  167,  167,  167,  167,  167,  167,
  167,  167,  167,  167,  167,  167,  167,  167,  167,  167,
  167,  167,  167,  167,  167,  167,  167,  167,  167,  167,
  167,  167,  167,  167
];

static YY_CHK: [i16; 385] = [
    0,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,
    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,
    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,
    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,
    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,
    1,    1,   11,   12,   11,   12,   13,   14,   17,   17,
   18,   18,   30,   31,   30,   32,   39,   32,   31,   40,
   41,   42,   49,   43,   44,   47,   32,   11,   12,   46,
   45,   13,   14,   41,   48,   32,   50,   55,   42,   49,
   39,   58,   46,   82,   39,   43,   40,   43,   45,   44,
   47,   59,   59,   45,   45,   50,   62,   84,   85,   82,
   68,   48,   72,   50,   87,   86,   58,   89,   62,   68,
   73,   72,   73,   88,   84,   91,   55,   74,   68,   74,
   72,   73,   74,   86,   87,   92,   97,   85,   93,   88,
   73,   94,   87,   96,   89,   98,  107,  110,  116,  117,
   92,   91,   93,   97,  119,  123,  110,   94,  120,   96,
  122,  116,  124,  126,  117,  110,  125,  127,  107,  130,
  128,  133,  119,  136,  135,  120,   98,  122,  139,  140,
  123,  141,  142,  125,  124,  143,  144,  147,  150,  151,
  166,  139,  136,  126,  128,  135,  133,  127,  152,  154,
  155,  144,  147,  150,  142,  158,  140,  143,  130,  157,
  160,  141,  165,  163,  152,  164,  154,  155,  151,  162,
  161,  157,  159,  156,  158,  163,  153,  160,  168,  168,
  168,  168,  168,  168,  168,  168,  169,  169,  169,  169,
  169,  169,  169,  169,  170,  149,  170,  171,  171,  148,
  171,  146,  171,  172,  145,  172,  138,  172,  173,  173,
  173,  137,  173,  174,  134,  174,  174,  174,  174,  131,
  174,  175,  175,  175,  175,  175,  175,  175,  175,  176,
  176,  176,  129,  176,  177,  121,  177,  118,  177,  178,
  115,  178,  178,  179,  112,  179,  180,  111,  180,  180,
  181,  106,  181,  181,  182,  105,  182,  182,  103,  101,
   95,   90,   83,   81,   80,   78,   70,   61,   51,   38,
   36,   35,   34,   33,   29,   27,   26,   23,   22,   19,
   15,  167,  167,  167,  167,  167,  167,  167,  167,  167,
  167,  167,  167,  167,  167,  167,  167,  167,  167,  167,
  167,  167,  167,  167,  167,  167,  167,  167,  167,  167,
  167,  167,  167,  167,  167,  167,  167,  167,  167,  167,
  167,  167,  167,  167,  167,  167,  167,  167,  167,  167,
  167,  167,  167,  167
];
/// Buffer status constants
const YY_BUFFER_NEW: i32 = 0;
/// Try NUL transition in the DFA
pub fn yy_try_NUL_trans(
    yy_current_state: YyStateType,
    yyscanner: &mut YyscanT,
) -> YyStateType {
    let yyg = match yyscanner {
        Some(ref mut g) => g,
        None => return 0,
    };
    let yy_cp = yyg.yy_c_buf_p;
    let yy_c: YyChar = 1;
    if YY_ACCEPT[yy_current_state as usize] != 0 {
        yyg.yy_last_accepting_state = yy_current_state;
        yyg.yy_last_accepting_cpos = yy_cp;
    }
    let mut current_state = yy_current_state;
    let mut c = yy_c;
    while YY_CHK[(YY_BASE[current_state as usize] as usize) + (c as usize)] as i32
        != current_state
    {
        current_state = YY_DEF[current_state as usize] as i32;
        if current_state >= 168 {
            c = YY_META[c as usize];
        }
    }
    current_state = YY_NXT[(YY_BASE[current_state as usize] as usize) + (c as usize)]
        as i32;
    let yy_is_jam = current_state == 167;
    if yy_is_jam { 0 } else { current_state }
}
/// Get the previous DFA state
pub fn yy_get_previous_state(yyscanner: &mut YyscanT) -> YyStateType {
    let yyg = match yyscanner {
        Some(ref mut g) => g,
        None => return 0,
    };
    let mut yy_current_state = yyg.yy_start;
    let text_start = 0;
    let text_end = yyg.yy_c_buf_p;
    for yy_cp in text_start..text_end {
        let ch = if yy_cp < yyg.yytext_r.len() { yyg.yytext_r[yy_cp] } else { 0 };
        let yy_c: YyChar = if ch != 0 { YY_EC[ch as usize] } else { 1 };
        if YY_ACCEPT[yy_current_state as usize] != 0 {
            yyg.yy_last_accepting_state = yy_current_state;
            yyg.yy_last_accepting_cpos = yy_cp;
        }
        let mut c = yy_c;
        while YY_CHK[(YY_BASE[yy_current_state as usize] as usize) + (c as usize)] as i32
            != yy_current_state
        {
            yy_current_state = YY_DEF[yy_current_state as usize] as i32;
            if yy_current_state >= 168 {
                c = YY_META[c as usize];
            }
        }
        yy_current_state = YY_NXT[(YY_BASE[yy_current_state as usize] as usize)
            + (c as usize)] as i32;
    }
    yy_current_state
}
/// Internal helper to ensure buffer stack exists
fn jq_yyensure_buffer_stack_internal(yyg: &mut YyGutsT) {
    if yyg.yy_buffer_stack.is_empty() {
        yyg.yy_buffer_stack = Vec::with_capacity(1);
        yyg.yy_buffer_stack.push(None);
        yyg.yy_buffer_stack_max = 1;
        yyg.yy_buffer_stack_top = 0;
    }
}
const YY_BUFFER_NORMAL: i32 = 1;
const YY_BUFFER_EOF_PENDING: i32 = 2;
/// Return values for yy_get_next_buffer
const EOB_ACT_CONTINUE_SCAN: i32 = 0;
const EOB_ACT_END_OF_BUFFER: i32 = 1;
const EOB_ACT_LAST_MATCH: i32 = 2;
/// Read buffer size
const YY_READ_BUF_SIZE: i32 = 8192;
/// Get the next buffer content
fn yy_get_next_buffer(yyscanner: &mut YyGutsT) -> i32 {
    let buffer_stack_top = yyscanner.yy_buffer_stack_top;
    let (buf_size, fill_buffer, is_our_buffer, is_interactive, buffer_status) = {
        match yyscanner.yy_buffer_stack.get(buffer_stack_top).and_then(|b| b.as_ref()) {
            Some(b) => {
                (
                    b.yy_buf_size,
                    b.yy_fill_buffer,
                    b.yy_is_our_buffer,
                    b.yy_is_interactive,
                    b.yy_buffer_status,
                )
            }
            None => return EOB_ACT_END_OF_BUFFER,
        }
    };
    let ch_buf_len = yyscanner
        .yy_buffer_stack
        .get(buffer_stack_top)
        .and_then(|b| b.as_ref())
        .map(|b| b.yy_ch_buf.len())
        .unwrap_or(0);
    if yyscanner.yy_c_buf_p > ch_buf_len {
        yy_fatal_error(
            "fatal flex scanner internal error--end of buffer missed",
            yyscanner,
        );
        return EOB_ACT_END_OF_BUFFER;
    }
    if !fill_buffer {
        let yytext_len = yyscanner.yytext_r.len();
        if yyscanner.yy_c_buf_p.saturating_sub(yytext_len) == 1 {
            return EOB_ACT_END_OF_BUFFER;
        } else {
            return EOB_ACT_LAST_MATCH;
        }
    }
    let yytext_start = if yyscanner.yytext_r.is_empty() { 0 } else { 0 };
    let number_to_move = yyscanner
        .yy_c_buf_p
        .saturating_sub(yytext_start)
        .saturating_sub(1);
    if let Some(ref mut b) = yyscanner
        .yy_buffer_stack
        .get_mut(buffer_stack_top)
        .and_then(|b| b.as_mut())
    {
        if number_to_move > 0 && yytext_start < b.yy_ch_buf.len() {
            let src_end = yytext_start + number_to_move;
            if src_end <= b.yy_ch_buf.len() {
                for i in 0..number_to_move {
                    b.yy_ch_buf[i] = b.yy_ch_buf[yytext_start + i];
                }
            }
        }
    }
    if buffer_status == YY_BUFFER_EOF_PENDING {
        yyscanner.yy_n_chars = 0;
        if let Some(ref mut b) = yyscanner
            .yy_buffer_stack
            .get_mut(buffer_stack_top)
            .and_then(|b| b.as_mut())
        {
            b.yy_n_chars = 0;
        }
    } else {
        let mut num_to_read = (buf_size as usize)
            .saturating_sub(number_to_move)
            .saturating_sub(1);
        while num_to_read == 0 {
            if is_our_buffer {
                if let Some(ref mut b) = yyscanner
                    .yy_buffer_stack
                    .get_mut(buffer_stack_top)
                    .and_then(|b| b.as_mut())
                {
                    let new_size = if b.yy_buf_size * 2 <= 0 {
                        b.yy_buf_size + b.yy_buf_size / 8
                    } else {
                        b.yy_buf_size * 2
                    };
                    b.yy_buf_size = new_size;
                    b.yy_ch_buf.resize((new_size + 2) as usize, 0);
                    num_to_read = (new_size as usize)
                        .saturating_sub(number_to_move)
                        .saturating_sub(1);
                }
            } else {
                yy_fatal_error("fatal error - scanner input buffer overflow", yyscanner);
                return EOB_ACT_END_OF_BUFFER;
            }
        }
        if num_to_read > YY_READ_BUF_SIZE as usize {
            num_to_read = YY_READ_BUF_SIZE as usize;
        }
        let mut chars_read = 0;
        // Read from buffer's input file (yy_input_file) instead of yyin_r (which is Box<dyn Any>)
        if let Some(ref mut b) = yyscanner
            .yy_buffer_stack
            .get_mut(buffer_stack_top)
            .and_then(|b| b.as_mut())
        {
            if let Some(ref mut input) = b.yy_input_file {
                if is_interactive {
                    let mut byte = [0u8; 1];
                    while chars_read < num_to_read {
                        match input.read(&mut byte) {
                            Ok(0) => break,
                            Ok(1) => {
                                let pos = number_to_move + chars_read;
                                if pos < b.yy_ch_buf.len() {
                                    b.yy_ch_buf[pos] = byte[0];
                                }
                                chars_read += 1;
                                if byte[0] == b'\n' {
                                    break;
                                }
                            }
                            _ => break,
                        }
                    }
                } else {
                    let start = number_to_move;
                    let end = start + num_to_read;
                    if end <= b.yy_ch_buf.len() {
                        match input.read(&mut b.yy_ch_buf[start..end]) {
                            Ok(n) => chars_read = n,
                            Err(_) => {
                                // Cannot call yy_fatal_error here due to borrow issues
                                // Just set chars_read to 0 to trigger EOF handling
                                chars_read = 0;
                            }
                        }
                    }
                }
            }
        }
        yyscanner.yy_n_chars = chars_read as i32;
        if let Some(ref mut b) = yyscanner
            .yy_buffer_stack
            .get_mut(buffer_stack_top)
            .and_then(|b| b.as_mut())
        {
            b.yy_n_chars = chars_read as i32;
        }
    }
    let ret_val = if yyscanner.yy_n_chars == 0 {
        if number_to_move == 0 {
            jq_yyrestart::<std::io::Empty>(None, yyscanner);
            EOB_ACT_END_OF_BUFFER
        } else {
            if let Some(ref mut b) = yyscanner
                .yy_buffer_stack
                .get_mut(buffer_stack_top)
                .and_then(|b| b.as_mut())
            {
                b.yy_buffer_status = YY_BUFFER_EOF_PENDING;
            }
            EOB_ACT_LAST_MATCH
        }
    } else {
        EOB_ACT_CONTINUE_SCAN
    };
    let total_chars = yyscanner.yy_n_chars as usize + number_to_move;
    if let Some(ref mut b) = yyscanner
        .yy_buffer_stack
        .get_mut(buffer_stack_top)
        .and_then(|b| b.as_mut())
    {
        if total_chars > b.yy_buf_size as usize {
            let new_size = total_chars + (yyscanner.yy_n_chars as usize >> 1);
            b.yy_ch_buf.resize(new_size, 0);
            b.yy_buf_size = (new_size - 2) as i32;
        }
        let n_chars = yyscanner.yy_n_chars as usize + number_to_move;
        if n_chars < b.yy_ch_buf.len() {
            b.yy_ch_buf[n_chars] = 0;
        }
        if n_chars + 1 < b.yy_ch_buf.len() {
            b.yy_ch_buf[n_chars + 1] = 0;
        }
    }
    ret_val
}
// Token constants from parser.y (must match parser.rs)
pub const INVALID_CHARACTER: i32 = 258;
pub const IDENT: i32 = 259;
pub const FIELD: i32 = 260;
pub const BINDING: i32 = 261;
pub const LITERAL: i32 = 262;
pub const FORMAT: i32 = 263;
pub const REC: i32 = 264;
pub const SETMOD: i32 = 265;
pub const EQ: i32 = 266;
pub const NEQ: i32 = 267;
pub const DEFINEDOR: i32 = 268;
pub const AS: i32 = 269;
pub const DEF: i32 = 270;
pub const MODULE: i32 = 271;
pub const IMPORT: i32 = 272;
pub const INCLUDE: i32 = 273;
pub const IF: i32 = 274;
pub const THEN: i32 = 275;
pub const ELSE: i32 = 276;
pub const ELSE_IF: i32 = 277;
pub const REDUCE: i32 = 278;
pub const FOREACH: i32 = 279;
pub const END: i32 = 280;
pub const AND: i32 = 281;
pub const OR: i32 = 282;
pub const TRY: i32 = 283;
pub const CATCH: i32 = 284;
pub const LABEL: i32 = 285;
pub const BREAK: i32 = 286;
pub const LOC: i32 = 287;
pub const SETPIPE: i32 = 288;
pub const SETPLUS: i32 = 289;
pub const SETMINUS: i32 = 290;
pub const SETMULT: i32 = 291;
pub const SETDIV: i32 = 292;
pub const SETDEFINEDOR: i32 = 293;
pub const LESSEQ: i32 = 294;
pub const GREATEREQ: i32 = 295;
pub const ALTERNATION: i32 = 296;
pub const QQSTRING_START: i32 = 297;
pub const QQSTRING_TEXT: i32 = 298;
pub const QQSTRING_INTERP_START: i32 = 299;
pub const QQSTRING_INTERP_END: i32 = 300;
pub const QQSTRING_END: i32 = 301;

// Lexer start states
const INITIAL: i32 = 0;
const IN_PAREN: i32 = 1;
const IN_BRACKET: i32 = 2;
const IN_BRACE: i32 = 3;
const IN_QQINTERP: i32 = 4;
const IN_QQSTRING: i32 = 5;
const IN_COMMENT: i32 = 6;

// EOF state markers for flex compatibility
const YY_STATE_EOF_INITIAL: i32 = 100;
const YY_STATE_EOF_IN_COMMENT: i32 = 101;
const YY_STATE_EOF_IN_PAREN: i32 = 102;
const YY_STATE_EOF_IN_BRACKET: i32 = 103;
const YY_STATE_EOF_IN_BRACE: i32 = 104;
const YY_STATE_EOF_IN_QQINTERP: i32 = 105;
const YY_STATE_EOF_IN_QQSTRING: i32 = 106;
const YY_END_OF_BUFFER: i32 = 54;
const YY_NUM_RULES: i32 = 53;
/// Update location macro helper
fn update_location(yyg: &mut YyGutsT) {
    if let Some(ref mut lloc) = yyg.yylloc_r {
        lloc.start = yyg.yyextra_r;
        lloc.end = lloc.start + yyg.yyleng_r;
    }
    let new_extra = if let Some(ref lloc) = yyg.yylloc_r {
        lloc.end
    } else {
        yyg.yyextra_r
    };
    yyg.yyextra_r = new_extra;
}
impl YyGutsT {
    pub fn new() -> Self {
        Self::default()
    }
    /// Get the current buffer
    fn current_buffer(&self) -> Option<&YyBufferState> {
        if self.yy_buffer_stack.is_empty() {
            None
        } else {
            self.yy_buffer_stack
                .get(self.yy_buffer_stack_top)
                .and_then(|b| b.as_ref())
                .map(|b| b.as_ref())
        }
    }
    /// Get the current buffer mutably
    fn current_buffer_mut(&mut self) -> Option<&mut YyBufferState> {
        if self.yy_buffer_stack.is_empty() {
            None
        } else {
            let top = self.yy_buffer_stack_top;
            self.yy_buffer_stack
                .get_mut(top)
                .and_then(|b| b.as_mut())
                .map(|b| b.as_mut())
        }
    }
    fn get_current_buffer(&self) -> Option<&YyBufferState> {
        if self.yy_buffer_stack.is_empty() {
            None
        } else {
            self.yy_buffer_stack
                .get(self.yy_buffer_stack_top)
                .and_then(|opt| opt.as_ref().map(|b| b.as_ref()))
        }
    }
    fn get_current_buffer_mut(&mut self) -> Option<&mut YyBufferState> {
        if self.yy_buffer_stack.is_empty() {
            None
        } else {
            let top = self.yy_buffer_stack_top;
            self.yy_buffer_stack
                .get_mut(top)
                .and_then(|opt| opt.as_mut().map(|b| b.as_mut()))
        }
    }
}
impl Default for YySType {
    fn default() -> Self {
        YySType::None
    }
}
impl YySType {
    /// Set the string value (replacing set_literal since Literal variant doesn't exist)
    pub fn set_string(&mut self, s: String) {
        *self = YySType::String(s);
    }
    /// Get the string value mutably
    pub fn get_string_mut(&mut self) -> Option<&mut String> {
        match self {
            YySType::String(ref mut s) => Some(s),
            _ => None,
        }
    }
}
// LexerScanner methods are defined in types.rs or parser.rs
// The struct has `inner: Rc<RefCell<LexerScannerInner>>`, not `guts: Box<YyGutsT>`
impl Default for YyBufferState {
    fn default() -> Self {
        Self {
            yy_input_file: None,
            yy_ch_buf: Vec::new(),
            yy_buf_pos: 0,
            yy_buf_size: 0,
            yy_n_chars: 0,
            yy_is_our_buffer: false,
            yy_is_interactive: false,
            yy_at_bol: 0,
            yy_bs_lineno: 1,
            yy_bs_column: 0,
            yy_fill_buffer: false,
            yy_buffer_status: 0,
        }
    }
}
