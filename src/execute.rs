//! Module: execute
//!
//! Contains 59 transpiled functions:
//! - stack_save:8626404832995464570:./src/execute.c
//! - align_round_up:9727113335816096859:./src/execute.c
//! - jq_get_input_cb:17947184970076626083:./src/execute.c
//! - path_append:9503547520681085602:./src/execute.c
//! - jq_halt:2475621139556795524:./src/execute.c
//! - jq_set_attrs:12037154435134197511:./src/execute.c
//! - stack_restore:17584660248401016146:./src/execute.c
//! - jq_get_prog_origin:6741701908280704971:./src/execute.c
//! - stack_block_next:402661876393537026:./src/execute.c
//! - jq_get_error_message:12446610662032935884:./src/execute.c
//! - stack_push:10873297261595477634:./src/execute.c
//! - _jq_path_append:2020692005862222393:./src/execute.c
//! - frame_get_level:13062733179999402069:./src/execute.c
//! - path_intact:7860956186951725436:./src/execute.c
//! - priv_fwrite:5762849748582323993:./src/execute.c
//! - frame_push:16197419980367406370:./src/execute.c
//! - stack_init:9705703242587607101:./src/execute.c
//! - set_error:17869686878088443779:./src/execute.c
//! - stack_reset:6809220776526140583:./src/execute.c
//! - jq_get_jq_origin:7686615720083679321:./src/execute.c
//! - jq_get_attr:16075171066991329163:./src/execute.c
//! - jq_halted:3649489340416321008:./src/execute.c
//! - args2obj:12293679776949877594:./src/execute.c
//! - jq_set_nomem_handler:9215837898523496802:./src/execute.c
//! - stack_pop_will_free:472524999524718633:./src/execute.c
//! - jq_set_attr:1919651177518093639:./src/execute.c
//! - stack_pop:11577569423640935673:./src/execute.c
//! - make_closure:4503105054010658888:./src/execute.c
//! - jq_init:6269076948399490986:./src/execute.c
//! - jq_get_stderr_cb:18396473574722213546:./src/execute.c
//! - stack_popn:8679339336362018926:./src/execute.c
//! - jq_set_debug_cb:18048491396630180774:./src/execute.c
//! - jq_get_error_cb:5106702439240976474:./src/execute.c
//! - jq_set_stderr_cb:12801517292208311569:./src/execute.c
//! - jq_get_exit_code:13819526638477694041:./src/execute.c
//! - jq_teardown:1079479029867450957:./src/execute.c
//! - optimize_code:8147106947360583278:./src/execute.c
//! - jq_set_error_cb:2213694162219208076:./src/execute.c
//! - jq_compile:1295495835109507668:./src/execute.c
//! - frame_local_var:16784293840187802395:./src/execute.c
//! - stack_get_pos:9889430765500165337:./src/execute.c
//! - jq_format_error:16544492440937359302:./src/execute.c
//! - default_err_cb:4775165338007341330:./src/execute.c
//! - jq_set_input_cb:10484753659679126981:./src/execute.c
//! - stack_push_block:17408752543657309216:./src/execute.c
//! - jq_start:6327647443670381717:./src/execute.c
//! - tail_call_analyze:18313246504659453549:./src/execute.c
//! - optimize:1273501910818977337:./src/execute.c
//! - jq_report_error:7401358645559290913:./src/execute.c
//! - ret_follows:8000504243957259081:./src/execute.c
//! - frame_pop:10373073981211018121:./src/execute.c
//! - frame_size:14139548852446161485:./src/execute.c
//! - jq_dump_disassembly:16690812856563521806:./src/execute.c
//! - frame_current:14186103824046633318:./src/execute.c
//! - jq_next:13006700536635786755:./src/execute.c
//! - jq_get_lib_dirs:557010409671394661:./src/execute.c
//! - jq_reset:16360113730805257480:./src/execute.c
//! - jq_compile_args:2275069765301462280:./src/execute.c
//! - jq_get_debug_cb:17914488979065195518:./src/execute.c

use crate::types::*;
use crate::types::{Stack, StackPtr};
use std::mem;
use std::io::Write;
use crate::bytecode::{
    Bytecode, bytecode_free,
};
// Block type mismatch is now fixed - using unified Block from compile.rs
use crate::parser::jq_parse;
use crate::compile::{compile, count_cfunctions, jv_mem_alloc, jv_mem_calloc};
use crate::types::Block;
use crate::jv::jv_number_value;
use crate::jv_aux::jv_keys;
use crate::builtin::builtins_bind;
// Note: jv_free and jv_get_kind are defined locally in this file
// Note: opcode_describe may not be exported or have different signature
// Note: frame_size is defined locally in this file

// Type aliases for lowercase C-style names used in stack functions
type stack = Stack;
type stack_ptr = StackPtr;

/// Trait for types that can provide mutable access to JqState
/// This allows functions to work with both `&mut JqState<T>` and `&mut Option<Box<JqState<T>>>`
pub trait JqStateAccess<T> {
    fn get_jq_state(&mut self) -> Option<&mut JqState<T>>;
}

impl<T> JqStateAccess<T> for JqState<T> {
    fn get_jq_state(&mut self) -> Option<&mut JqState<T>> {
        Some(self)
    }
}

impl<T> JqStateAccess<T> for Option<Box<JqState<T>>> {
    fn get_jq_state(&mut self) -> Option<&mut JqState<T>> {
        self.as_mut().map(|b| b.as_mut())
    }
}
/// Check if stack pop will free memory
pub fn stack_pop_will_free<T>(jq: &JqState<T>) -> bool {
    crate::exec_stack::stack_pop_will_free(&jq.stk, jq.stk_top) != 0
}
/// Get the exit code from jq state
pub fn jq_get_exit_code<T>(jq: &JqState<T>) -> Jv {
    jq.exit_code.clone()
}
/// Halt jq execution with given exit code and error message
pub fn jq_halt<T>(jq: &mut JqState<T>, exit_code: Jv, error_message: Jv) {
    assert!(! jq.halted, "jq already halted");
    jq.halted = true;
    jq.exit_code = exit_code;
    jq.error_message = error_message;
}
/// Forkpoint structure for backtracking - stored on the fork stack
#[repr(C)]
struct ForkpointData {
    saved_data_stack: StackPtr,
    saved_curr_frame: StackPtr,
    path_len: i32,
    subexp_nest: i32,
    return_address_offset: usize,
    // Note: value_at_path is stored separately because it's a Jv
}

/// Save stack state for backtracking (fork point)
/// retaddr is the pc offset to return to when backtracking
pub fn stack_save<T>(jq: &mut JqState<T>, retaddr_offset: Option<usize>, sp: StackPos) {
    // Push forkpoint onto the fork stack
    let fork_size = std::mem::size_of::<ForkpointData>() + std::mem::size_of::<Jv>();
    jq.fork_top = stack_push_block(&mut jq.stk, jq.fork_top, fork_size);

    let fork_block = stack_block(&mut jq.stk, jq.fork_top);
    if !fork_block.is_null() {
        // Calculate path_len
        let path_len = if jv_get_kind(&jq.path) == JvKind::Array {
            jv_array_length(&jq.path)
        } else {
            0
        };

        unsafe {
            // Store the forkpoint data
            let fork_data = fork_block as *mut ForkpointData;
            (*fork_data).saved_data_stack = jq.stk_top;
            (*fork_data).saved_curr_frame = jq.curr_frame;
            (*fork_data).path_len = path_len;
            (*fork_data).subexp_nest = jq.subexp_nest;
            (*fork_data).return_address_offset = retaddr_offset.unwrap_or(0);

            // Store value_at_path after the ForkpointData
            let value_ptr = (fork_data as *mut u8).add(std::mem::size_of::<ForkpointData>()) as *mut Jv;
            std::ptr::write(value_ptr, jv_copy(&jq.value_at_path));
        }
    }

    // Restore stack position to saved position
    jq.stk_top = sp.saved_data_stack;
    jq.curr_frame = sp.saved_limit;
}

/// Restore stack state from forkpoint (for backtracking)
/// Returns the pc offset to return to, or None if no more forkpoints
fn stack_restore_impl<T>(jq: &mut JqState<T>) -> Option<usize> {
    // Pop values and frames until we reach the fork point
    // C: while (!stack_pop_will_free(&jq->stk, jq->fork_top)) { ... }
    while crate::exec_stack::stack_pop_will_free(&jq.stk, jq.fork_top) == 0 {
        if crate::exec_stack::stack_pop_will_free(&jq.stk, jq.stk_top) != 0 {
            // Pop and free a value from the data stack
            if let Some(val) = stack_pop(jq) {
                jv_free(val);
            } else {
                break;
            }
        } else if crate::exec_stack::stack_pop_will_free(&jq.stk, jq.curr_frame) != 0 {
            // Pop a frame
            frame_pop(jq);
        } else {
            break;
        }
    }

    // C: if (jq->fork_top == 0) { return 0; }
    if jq.fork_top == 0 {
        return None;
    }

    let fork_size = std::mem::size_of::<ForkpointData>() + std::mem::size_of::<Jv>();
    let fork_block = stack_block(&mut jq.stk, jq.fork_top);

    if fork_block.is_null() {
        return None;
    }

    let (retaddr_offset, path_len, subexp_nest, value_at_path) = unsafe {
        let fork_data = fork_block as *const ForkpointData;
        let retaddr = (*fork_data).return_address_offset;
        let path_len = (*fork_data).path_len;
        let subexp_nest = (*fork_data).subexp_nest;

        // Restore stk_top and curr_frame
        jq.stk_top = (*fork_data).saved_data_stack;
        jq.curr_frame = (*fork_data).saved_curr_frame;

        // Read value_at_path
        let value_ptr = (fork_data as *const u8).add(std::mem::size_of::<ForkpointData>()) as *const Jv;
        let value_at_path = std::ptr::read(value_ptr);

        (retaddr, path_len, subexp_nest, value_at_path)
    };

    // Truncate path if it's an array
    if jv_get_kind(&jq.path) == JvKind::Array && path_len >= 0 {
        let path = std::mem::replace(&mut jq.path, jv_null());
        jq.path = jv_array_slice(path, 0, path_len);
    }

    // Restore value_at_path
    jv_free(std::mem::replace(&mut jq.value_at_path, value_at_path));
    jq.subexp_nest = subexp_nest;

    // Pop the forkpoint from fork stack
    jq.fork_top = stack_pop_block(&mut jq.stk, jq.fork_top, fork_size);

    Some(retaddr_offset)
}
/// Format an error message for display
pub fn jq_format_error(msg: Jv) -> Jv {
    match msg.get_kind() {
        JvKind::Null => {
            eprintln!("jq: error: out of memory");
            Jv::null()
        }
        JvKind::Invalid => {
            // JqState doesn't have invalid_has_msg method - check if it has a message
            // For now, just print a generic error
            eprintln!("jq: error: out of memory");
            Jv::null()
        }
        JvKind::String => msg,
        _ => {
            // Jv doesn't have dump_string method - just format a generic error
            Jv::string(&format!("jq: error: (value)"))
        }
    }
}
/// Pop the current frame
/// C: static void frame_pop(struct jq_state* jq)
pub fn frame_pop<T>(jq: &mut JqState<T>) {
    // C: assert(jq->curr_frame);
    if jq.curr_frame == 0 {
        return;
    }

    // C: struct frame* fp = frame_current(jq);
    let (frame_sz, nlocals) = if let Some(frame) = frame_current(jq) {
        let sz = if let Some(ref bc) = frame.bc {
            frame_size(bc.as_ref())
        } else {
            std::mem::size_of::<Frame>()
        };
        let nl = frame.bc.as_ref().map(|bc| bc.nlocals).unwrap_or(0);
        (sz, nl)
    } else {
        (std::mem::size_of::<Frame>(), 0)
    };

    // C: if (stack_pop_will_free(&jq->stk, jq->curr_frame)) {
    //        int nlocals = fp->bc->nlocals;
    //        for (int i=0; i<nlocals; i++) {
    //            jv_free(*frame_local_var(jq, i, 0));
    //        }
    //    }
    if crate::exec_stack::stack_pop_will_free(&jq.stk, jq.curr_frame) != 0 {
        for i in 0..nlocals {
            if let Some(var) = frame_local_var(jq, i as i32, 0) {
                jv_free(std::mem::replace(var, jv_null()));
            }
        }
    }

    // C: jq->curr_frame = stack_pop_block(&jq->stk, jq->curr_frame, frame_size(fp->bc));
    jq.curr_frame = stack_pop_block(&mut jq.stk, jq.curr_frame, frame_sz as usize);
}
/// Get the program origin attribute
pub fn jq_get_prog_origin<T>(jq: &JqState<T>) -> Jv {
    jq_get_attr(jq, Jv::string("PROGRAM_ORIGIN"))
}
/// Get an attribute from jq state
pub fn jq_get_attr<T>(jq: &JqState<T>, attr: Jv) -> Jv {
    if jq.attrs.get_kind() == JvKind::Object {
        jv_object_get(&jq.attrs, attr)
    } else {
        Jv::null()
    }
}
/// Optimize bytecode (recursive optimization of subfunctions)
pub fn optimize(bc: &mut Bytecode) -> &mut Bytecode {
    for i in 0..bc.nsubfunctions as usize {
        if i < bc.subfunctions.len() {
            optimize(&mut bc.subfunctions[i]);
        }
    }
    optimize_code(bc)
}
/// Optimize bytecode by converting calls to tail calls where possible
pub fn optimize_code(bc: &mut Bytecode) -> &mut Bytecode {
    let mut pc_offset = 0;
    while pc_offset < bc.code.len() {
        match bc.code[pc_offset] {
            CALL_JQ => {
                bc.code[pc_offset] = tail_call_analyze(&bc.code[pc_offset..]);
            }
            _ => {}
        }
        pc_offset += bytecode_operation_length_at(&bc.code, pc_offset);
    }
    bc
}
/// Reset the jq state
pub fn jq_reset<T>(jq: &mut JqState<T>) {
    // Pop all forkpoints by calling stack_restore until it returns None
    while stack_restore_impl(jq).is_some() {}

    assert!(jq.stk_top == 0, "stk_top should be 0 after reset");
    assert!(jq.fork_top == 0, "fork_top should be 0 after reset");
    assert!(jq.curr_frame == 0, "curr_frame should be 0 after reset");

    stack_reset(&mut jq.stk);

    // Free and reset error
    jv_free(std::mem::replace(&mut jq.error, jv_null()));

    jq.halted = false;
    jv_free(std::mem::replace(&mut jq.exit_code, jv_invalid()));
    jv_free(std::mem::replace(&mut jq.error_message, jv_invalid()));

    // Reset path tracking
    if jv_get_kind(&jq.path) != JvKind::Invalid {
        jv_free(std::mem::replace(&mut jq.path, jv_null()));
    }
    jv_free(std::mem::replace(&mut jq.value_at_path, jv_null()));
    jq.subexp_nest = 0;
}
/// Report an error through the error callback
pub fn jq_report_error<T>(jq: &mut JqState<T>, value: Jv) {
    if let Some(ref err_cb) = jq.err_cb {
        if let Some(ref mut data) = jq.err_cb_data {
            err_cb(data, value);
        }
    }
}
/// Calculate frame size based on bytecode
pub fn frame_size(bc: &Bytecode) -> usize {
    let base_size = std::mem::size_of::<Frame>();
    let entries_size = (bc.nlocals + bc.nclosures) as usize
        * std::mem::size_of::<FrameEntry>();
    base_size + entries_size
}
/// Reset stack
pub fn stack_reset(s: &mut Stack) {
    assert!(s.limit == 0, "stack freed while not empty");
    // Stack doesn't have mem field - just reinitialize
    stack_init(s);
}
/// Restore stack state and return the saved address
/// C: uint16_t* stack_restore(jq_state *jq)
pub fn stack_restore<T>(jq: &mut JqState<T>) -> Option<usize> {
    stack_restore_impl(jq)
}
/// Get current frame
pub fn frame_current<T>(jq: &JqState<T>) -> Option<&Frame> {
    if jq.curr_frame == 0 || jq.stk.mem_end.is_null() {
        return None;
    }
    // C: struct frame* fp = stack_block(&jq->stk, jq->curr_frame);
    let block = unsafe { jq.stk.mem_end.offset(jq.curr_frame as isize) as *const Frame };
    unsafe { block.as_ref() }
}
/// Get mutable current frame
pub fn frame_current_mut<T>(jq: &mut JqState<T>) -> Option<&mut Frame> {
    if jq.curr_frame == 0 || jq.stk.mem_end.is_null() {
        return None;
    }
    // C: struct frame* fp = stack_block(&jq->stk, jq->curr_frame);
    let block = unsafe { jq.stk.mem_end.offset(jq.curr_frame as isize) as *mut Frame };
    unsafe { block.as_mut() }
}
/// Get a local variable from a frame at given level
/// C: static jv* frame_local_var(struct jq_state* jq, int var, int level)
pub fn frame_local_var<T>(jq: &mut JqState<T>, var: i32, level: i32) -> Option<&mut Jv> {
    // C: struct frame* fr = stack_block(&jq->stk, frame_get_level(jq, level));
    let fr_ptr = frame_get_level(jq, level);
    let frame_ptr = unsafe { jq.stk.mem_end.offset(fr_ptr as isize) as *mut Frame };
    let frame = unsafe { &mut *frame_ptr };

    // C: assert(var >= 0);
    if var < 0 {
        return None;
    }

    // C: return &fr->entries[fr->bc->nclosures + var].localvar;
    let nclosures = frame.bc.as_ref().map(|bc| bc.nclosures).unwrap_or(0) as usize;
    let var_idx = nclosures + var as usize;

    if var_idx < frame.entries.len() {
        match &mut frame.entries[var_idx] {
            FrameEntry::LocalVar(jv) => Some(jv),
            _ => None,
        }
    } else {
        None
    }
}
/// Get stack position for save/restore
pub fn stack_get_pos<T>(jq: &JqState<T>) -> StackPos {
    StackPos {
        saved_data_stack: jq.stk_top,
        saved_limit: jq.curr_frame,
    }
}
/// Get the library directories from jq state
///
/// Returns the JQ_LIBRARY_PATH attribute if valid, otherwise returns an empty array
pub fn jq_get_lib_dirs<T>(jq: &mut JqState<T>) -> Jv {
    let lib_dirs = jq_get_attr(jq, jv_string("JQ_LIBRARY_PATH"));
    if jv_is_valid(&lib_dirs) {
        lib_dirs
    } else {
        jv_free(lib_dirs);
        // Return empty array
        Jv {
            kind_flags: JvKind::Array as u8,
            pad_: 0,
            offset: 0,
            size: 0,
            u: 0,
        }
    }
}
/// Get error message from jq state
pub fn jq_get_error_message<T>(jq: &JqState<T>) -> Jv {
    jq.error_message.clone()
}
/// Set an attribute on jq state
pub fn jq_set_attr<T>(jq: &mut JqState<T>, attr: Jv, val: Jv) {
    if jq.attrs.get_kind() != JvKind::Object {
        jq.attrs = jv_object();
    }
    jq.attrs = jv_object_set(jq.attrs.clone(), attr, val);
}
/// Set the error callback
pub fn jq_set_error_cb<T>(
    jq: &mut JqState<T>,
    cb: Option<JqMsgCb<T>>,
    data: Option<Box<T>>,
) {
    match cb {
        None => {
            jq.err_cb = Some(default_err_cb);
            jq.err_cb_data = None;
        }
        Some(callback) => {
            jq.err_cb = Some(callback);
            jq.err_cb_data = data;
        }
    }
}
/// Get error callback and data from jq state
///
/// Returns the error callback function and associated data through output parameters
pub fn jq_get_error_cb<T>(jq: &JqState<T>) -> (Option<JqMsgCb<T>>, Option<&T>) {
    let cb = jq.err_cb;
    let data = jq.err_cb_data.as_ref().map(|b| b.as_ref());
    (cb, data)
}
/// Set debug callback
pub fn jq_set_debug_cb<T>(
    jq: &mut JqState<T>,
    cb: Option<fn(&mut T, Jv)>,
    data: Option<Box<T>>,
) {
    jq.debug_cb = cb;
    jq.debug_cb_data = data;
}
/// Get debug callback
pub fn jq_get_debug_cb<T>(jq: &JqState<T>) -> (Option<fn(&mut T, Jv)>, Option<&T>) {
    (jq.debug_cb, jq.debug_cb_data.as_deref())
}
/// Set input callback
pub fn jq_set_input_cb<T>(
    jq: &mut JqState<T>,
    cb: Option<fn(&mut JqState<T>, &mut T) -> Jv>,
    data: Option<Box<T>>,
) {
    jq.input_cb = cb;
    jq.input_cb_data = data;
}
/// Set the no-memory handler for jq state
///
/// Sets both the global jv no-memory handler and stores it in the jq state
pub fn jq_set_nomem_handler<T>(
    jq: &mut JqState<T>,
    nomem_handler: Option<JqNomemHandler<T>>,
    data: Option<Box<T>>,
) {
    jq.nomem_handler = nomem_handler;
    jq.nomem_handler_data = data;
}
/// Set error on jq state
pub fn set_error<T>(jq: &mut JqState<T>, value: Jv) {
    jv_free(mem::take(&mut jq.error_message));
    jq.error_message = value;
}
/// Push a value onto the jq stack
pub fn stack_push<T>(jq: &mut JqState<T>, val: Jv) {
    assert!(jv_is_valid(&val), "jv_is_valid(val)");
    jq.stk_top = stack_push_block(&mut jq.stk, jq.stk_top, std::mem::size_of::<Jv>());
    let block = stack_block(&mut jq.stk, jq.stk_top);
    if !block.is_null() {
        unsafe {
            let sval = block as *mut Jv;
            std::ptr::write(sval, val);
        }
    }
}
/// Pop a value from the jq stack
pub fn stack_pop<T>(jq: &mut JqState<T>) -> Option<Jv> {
    // C has no empty check - stk_top==0 means empty, negative values are valid
    if jq.stk_top == 0 {
        return None;
    }
    let block = stack_block(&mut jq.stk, jq.stk_top);
    if block.is_null() {
        return None;
    }
    let val = unsafe { std::ptr::read(block as *const Jv) };
    let will_free = crate::exec_stack::stack_pop_will_free(&jq.stk, jq.stk_top) != 0;
    let result = if will_free { val } else { jv_copy(&val) };
    jq.stk_top = stack_pop_block(
        &mut jq.stk,
        jq.stk_top,
        std::mem::size_of::<Jv>(),
    );
    assert!(jv_is_valid(&result), "jv_is_valid(val)");
    Some(result)
}
/// Pop a value from the jq data stack
/// Like stack_pop(), but assert !stack_pop_will_free() and replace with jv_null() on stack
pub fn stack_popn<T>(jq: &mut JqState<T>) -> Jv {
    let sval_ptr = stack_block(&mut jq.stk, jq.stk_top);
    let val = if !sval_ptr.is_null() {
        let val = unsafe { std::ptr::read(sval_ptr as *const Jv) };
        let will_free = crate::exec_stack::stack_pop_will_free(&jq.stk, jq.stk_top) != 0;
        if !will_free {
            // Replace with null on stack when not freeing
            unsafe { std::ptr::write(sval_ptr as *mut Jv, jv_null()) };
        }
        val
    } else {
        jv_null()
    };
    jq.stk_top = stack_pop_block(&mut jq.stk, jq.stk_top, std::mem::size_of::<Jv>());
    assert!(jv_is_valid(&val), "jv_is_valid(val)");
    val
}
/// Check if the current path is intact (unchanged)
pub fn path_intact<T>(jq: &JqState<T>, curr: Jv) -> bool {
    if jq.subexp_nest == 0 && jv_get_kind(&jq.path) == JvKind::Array {
        jv_identical(curr, jq.value_at_path.clone())
    } else {
        true
    }
}
/// Append to path
pub fn path_append<T>(jq: &mut JqState<T>, component: Jv, value_at_path: Jv) {
    if jq.subexp_nest == 0 && jv_get_kind(&jq.path) == JvKind::Array {
        let path = mem::take(&mut jq.path);
        jq.path = path.array_append(component);
        jv_free(mem::take(&mut jq.value_at_path));
        jq.value_at_path = value_at_path;
    } else {
        jv_free(component);
        jv_free(value_at_path);
    }
}
/// Check if a RET instruction follows (possibly after JUMPs)
pub fn ret_follows(pc: &[u16], offset: usize) -> bool {
    if offset >= pc.len() {
        return false;
    }
    if pc[offset] == RET {
        return true;
    }
    if pc[offset] != JUMP {
        return false;
    }
    if offset + 1 >= pc.len() {
        return false;
    }
    let jump_offset = pc[offset + 1] as usize;
    let new_offset = offset + 2 + jump_offset;
    ret_follows(pc, new_offset)
}
/// Analyze a call instruction to determine if it can be a tail call
///
/// Returns TAIL_CALL_JQ if the call can be optimized to a tail call,
/// otherwise returns CALL_JQ
pub fn tail_call_analyze(pc: &[u16]) -> u16 {
    if pc.is_empty() || pc[0] != CALL_JQ {
        return CALL_JQ;
    }
    let mut idx = 1;
    if idx >= pc.len() {
        return CALL_JQ;
    }
    let mut nclosures = pc[idx] as i32 + 1;
    idx += 1;
    while nclosures > 0 {
        if idx >= pc.len() || pc[idx] == 0 {
            return CALL_JQ;
        }
        idx += 1;
        nclosures -= 1;
        idx += 1;
    }
    if idx < pc.len() && ret_follows(&pc[idx..], 0) { TAIL_CALL_JQ } else { CALL_JQ }
}
/// Convert args array to object
///
/// If args is already an object, returns it unchanged.
/// If args is an array of {name, value} objects, converts to a single object.
fn args2obj(args: Jv) -> Jv {
    if jv_get_kind(&args) == JvKind::Object {
        return args;
    }
    assert!(jv_get_kind(&args) == JvKind::Array, "jv_get_kind(args) == JV_KIND_ARRAY");
    let mut r = jv_object();
    let kk = jv_string("name");
    let vk = jv_string("value");
    let len = jv_array_length(&args);
    for i in 0..len {
        let v = jv_array_get(jv_copy(&args), i);
        if jv_get_kind(&v) != JvKind::Invalid {
            let key = jv_object_get(&v, jv_copy(&kk));
            let val = jv_object_get(&v, jv_copy(&vk));
            r = jv_object_set(r, key, val);
        }
        jv_free(v);
    }
    jv_free(args);
    jv_free(kk);
    jv_free(vk);
    r
}
/// Get frame at a specific level
/// C: static stack_ptr frame_get_level(struct jq_state* jq, int level)
pub fn frame_get_level<T>(jq: &JqState<T>, level: i32) -> StackPtr {
    // C: stack_ptr fr = jq->curr_frame;
    let mut fr = jq.curr_frame;
    // C: for (int i=0; i<level; i++) {
    for _ in 0..level {
        // C: struct frame* fp = stack_block(&jq->stk, fr);
        // Uses same pattern as frame_current()
        let fp = unsafe { jq.stk.mem_end.offset(fr as isize) as *const Frame };
        let frame = unsafe { &*fp };
        // C: fr = fp->env;
        fr = frame.env;
    }
    // C: return fr;
    fr
}
/// Dump disassembly for debugging
pub fn jq_dump_disassembly<T>(jq: &JqState<T>, indent: i32) {
    if let Some(ref bc) = jq.bc {
        use crate::bytecode::dump_disassembly;
        dump_disassembly(indent, bc);
    }
}
// Cannot use .max() in const, so use a const fn approach
const ALIGN_F64: usize = std::mem::align_of::<f64>();
const SIZE_USIZE: usize = std::mem::size_of::<usize>();
const ALIGNMENT: usize = if ALIGN_F64 > SIZE_USIZE { ALIGN_F64 } else { SIZE_USIZE };
/// Round up a size value to the nearest alignment boundary
///
/// # Arguments
/// * `sz` - The size to align
///
/// # Returns
/// The size rounded up to the nearest multiple of ALIGNMENT
pub fn align_round_up(sz: usize) -> usize {
    let alignment = ALIGNMENT as usize;
    ((sz + alignment - 1) / alignment) * alignment
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_jq_state_creation() {
        let jq: JqState<()> = JqState::new();
        assert!(! jq_halted(& jq));
    }
    #[test]
    fn test_stack_operations() {
        let mut stack = Stack { mem_end: std::ptr::null_mut(), bound: 0, limit: 0 };
        stack_init(&mut stack);
        assert_eq!(stack.limit, 0);
        // Per C code: s->bound = ALIGNMENT
        assert_eq!(stack.bound, ALIGNMENT as i32);
    }
    #[test]
    fn test_jv_validity() {
        let jv = Jv::default();
        assert!(! jv_is_valid(& jv));
        let valid_jv = Jv {
            kind_flags: JvKind::Null as u8,
            ..Default::default()
        };
        assert!(jv_is_valid(& valid_jv));
    }
    #[test]
    fn test_frame_size_calculation() {
        let bc = Bytecode {
            code: vec![],
            codelen: 0,
            nlocals: 2,
            nclosures: 3,
            constants: Jv::default(),
            globals: None,
            subfunctions: vec![],
            nsubfunctions: 0,
            parent: None,
            debuginfo: Jv::default(),
        };
        let size = frame_size(&bc);
        assert!(size > mem::size_of::< Frame > ());
    }
}
/// Check if jv is valid (standalone function for compatibility)
pub fn jv_is_valid(x: &Jv) -> bool {
    x.is_valid()
}
/// Free a jv value
pub fn jv_free(v: Jv) {
    drop(v);
}
/// Copy a jv value
pub fn jv_copy(v: &Jv) -> Jv {
    v.clone()
}
fn jv_get_kind(v: &Jv) -> JvKind {
    unsafe { std::mem::transmute(v.kind_flags & 0x0F) }
}
/// Check if two jv values are identical
pub fn jv_identical(a: Jv, b: Jv) -> bool {
    // Jv doesn't have identical method, compare kind_flags and size
    a.kind_flags == b.kind_flags && a.size == b.size && a.u == b.u
}
/// Initialize a stack structure
///
/// Sets the stack to its initial empty state with proper alignment.
///
/// # Arguments
/// * `s` - Mutable reference to the stack to initialize
pub fn stack_init(s: &mut Stack) {
    s.mem_end = std::ptr::null_mut();
    s.bound = ALIGNMENT as i32;
    s.limit = 0;
}
/// Reallocate stack memory to accommodate more data
pub fn stack_reallocate(s: &mut Stack, sz: usize) {
    // C: int old_mem_length = -(s->bound) + ALIGNMENT;
    // bound starts at ALIGNMENT, becomes negative after first alloc
    let old_mem_length = ((-s.bound) + ALIGNMENT as i32) as usize;
    let new_mem_length = align_round_up((old_mem_length + sz + 256) * 2);

    unsafe {
        let layout = std::alloc::Layout::from_size_align_unchecked(new_mem_length, ALIGNMENT);
        let new_mem = std::alloc::alloc_zeroed(layout);
        if new_mem.is_null() {
            std::alloc::handle_alloc_error(layout);
        }

        if !s.mem_end.is_null() && old_mem_length > ALIGNMENT {
            let old_mem_start = s.mem_end.sub(old_mem_length) as *const u8;
            let dest = new_mem.add(new_mem_length - old_mem_length);
            std::ptr::copy_nonoverlapping(old_mem_start, dest, old_mem_length);

            let old_layout = std::alloc::Layout::from_size_align_unchecked(old_mem_length, ALIGNMENT);
            std::alloc::dealloc(s.mem_end.sub(old_mem_length) as *mut u8, old_layout);
        }

        s.mem_end = new_mem.add(new_mem_length) as *mut i8;
        s.bound = -((new_mem_length - ALIGNMENT) as i32);
    }
}
/// Push a block onto the stack
///
/// Allocates space for a block of the given size and returns the new stack pointer.
/// If there isn't enough space, the stack is reallocated.
pub fn stack_push_block(s: &mut Stack, p: StackPtr, sz: usize) -> StackPtr {
    let alloc_sz = align_round_up(sz) as i32 + ALIGNMENT as i32;
    let r = s.limit - alloc_sz;
    if r < s.bound {
        stack_reallocate(s, alloc_sz as usize);
    }
    s.limit = r;
    // stack_block_next returns a mutable reference, not Option
    let next_ptr = stack_block_next(s, r);
    *next_ptr = p;
    r
}
/// Pop a block from the stack
/// Returns the "next" block pointer (the block that was pushed before this one)
pub fn stack_pop_block(s: &mut Stack, p: StackPtr, sz: usize) -> StackPtr {
    let r = *stack_block_next(s, p);
    // C: if (p == s->limit) { s->limit += alloc_sz; }
    if p == s.limit {
        let alloc_sz = align_round_up(sz) as i32 + ALIGNMENT as i32;
        s.limit += alloc_sz;
    }
    r
}
/// Helper function that returns raw pointer to block data.
/// This mirrors the C function stack_block which returns void*.
/// C: return (void*)(s->mem_end + p);  // p is negative
fn stack_block(s: &mut stack, p: stack_ptr) -> *mut u8 {
    unsafe { s.mem_end.offset(p as isize) as *mut u8 }
}
/// Get pointer to the "next" field of a stack block.
///
/// In the C implementation, stack blocks store a `stack_ptr` at offset -1
/// (just before the block data) to form a linked list of blocks.
/// This function returns a mutable reference to that "next" pointer.
///
/// # Safety
/// This function uses unsafe code because it performs pointer arithmetic
/// to access the stack_ptr stored just before the block data.
pub fn stack_block_next(s: &mut stack, p: stack_ptr) -> &mut stack_ptr {
    let block_ptr = stack_block(s, p);
    unsafe {
        let stack_ptr_array = block_ptr as *mut stack_ptr;
        &mut *stack_ptr_array.offset(-1)
    }
}
/// Check if jq has halted
pub fn jq_halted<T>(jq: &JqState<T>) -> bool {
    jq.halted
}
/// Append to jq path
pub fn _jq_path_append<T>(jq: &mut JqState<T>, v: Jv, p: Jv, value_at_path: Jv) -> Jv {
    if jq.subexp_nest != 0 || jv_get_kind(&jq.path) != JvKind::Array
        || !jv_is_valid(&value_at_path)
    {
        jv_free(v);
        jv_free(p);
        return value_at_path;
    }
    if !jv_identical(v, jv_copy(&jq.value_at_path)) {
        jv_free(p);
        return value_at_path;
    }
    if jv_get_kind(&p) == JvKind::Array {
        let path = mem::take(&mut jq.path);
        // Use array_append for each element since array_concat doesn't exist
        jq.path = jv_array_append(path, p);
    } else {
        let path = mem::take(&mut jq.path);
        jq.path = jv_array_append(path, p);
    }
    jv_free(mem::take(&mut jq.value_at_path));
    jq.value_at_path = value_at_path.clone();
    value_at_path
}
/// Default error callback
pub fn default_err_cb<T>(data: &mut T, msg: Jv) {
    let formatted = jq_format_error(msg);
    eprintln!("{}", formatted.string_value().unwrap_or("<error>"));
    jv_free(formatted);
    let _ = data;
}
/// Set stderr callback
pub fn jq_set_stderr_cb<T>(jq: &mut JqState<T>, cb: JqMsgCb<T>, data: Box<T>) {
    jq.stderr_cb = Some(cb);
    jq.stderr_cb_data = Some(data);
}
/// Set attributes
pub fn jq_set_attrs<T>(jq: &mut JqState<T>, attrs: Jv) {
    assert!(jv_get_kind(& attrs) == JvKind::Object, "attrs must be an object");
    jv_free(mem::take(&mut jq.attrs));
    jq.attrs = attrs;
}
/// Get input callback
pub fn jq_get_input_cb<'a, T>(
    jq: &'a JqState<T>,
    cb: &mut Option<JqInputCb<T>>,
    data: &mut Option<&'a T>,
) {
    *cb = jq.input_cb;
    *data = jq.input_cb_data.as_deref();
}
/// Make a closure from bytecode
/// C: static struct closure make_closure(struct jq_state* jq, uint16_t* pc)
pub fn make_closure<T>(jq: &JqState<T>, pc: &[u16], pc_offset: &mut usize) -> Closure {
    // C: uint16_t level = *pc++;
    let level = pc[*pc_offset] as i32;
    *pc_offset += 1;
    // C: uint16_t idx = *pc++;
    let idx = pc[*pc_offset];
    *pc_offset += 1;
    // C: stack_ptr fridx = frame_get_level(jq, level);
    let fridx = frame_get_level(jq, level);
    // C: struct frame* fr = stack_block(&jq->stk, fridx);
    let fr_ptr = unsafe { jq.stk.mem_end.offset(fridx as isize) as *const Frame };
    let fr = unsafe { &*fr_ptr };

    // C: if (idx & ARG_NEWCLOSURE)
    if idx & 0x1000 != 0 {
        // C: int subfn_idx = idx & ~ARG_NEWCLOSURE;
        let subfn_idx = (idx & !0x1000) as usize;
        let debug = std::env::var("DEBUG_EXEC").is_ok();
        if debug {
            eprintln!("make_closure: subfunction reference, subfn_idx={}", subfn_idx);
        }
        // C: assert(subfn_idx < fr->bc->nsubfunctions);
        // C: struct closure cl = {fr->bc->subfunctions[subfn_idx], fridx};
        if let Some(ref bc) = fr.bc {
            if debug {
                eprintln!("make_closure: bc.nsubfunctions={} bc.subfunctions.len={}", bc.nsubfunctions, bc.subfunctions.len());
            }
            assert!(subfn_idx < bc.nsubfunctions as usize, "subfn_idx out of bounds");
            if subfn_idx < bc.subfunctions.len() {
                return Closure {
                    bc: Some(bc.subfunctions[subfn_idx].clone()),
                    env: fridx,
                };
            }
        } else if debug {
            eprintln!("make_closure: fr.bc is None");
        }
        Closure { bc: None, env: 0 }
    } else {
        // C: int closure = idx;
        // C: return fr->entries[closure].closure;
        let closure_idx = idx as usize;
        if let Some(ref bc) = fr.bc {
            assert!(closure_idx < bc.nclosures as usize, "closure out of bounds");
        }
        if closure_idx < fr.entries.len() {
            if let FrameEntry::Closure(ref cl) = fr.entries[closure_idx] {
                return cl.clone();
            }
        }
        Closure { bc: None, env: 0 }
    }
}
// ============================================================================
// OPCODE CONSTANTS - Must match C opcode_list.h order exactly!
// ============================================================================
const LOADK: u16 = 0;
const DUP: u16 = 1;
const DUPN: u16 = 2;
const DUP2: u16 = 3;
const PUSHK_UNDER: u16 = 4;
const POP: u16 = 5;
const LOADV: u16 = 6;
const LOADVN: u16 = 7;
const STOREV: u16 = 8;
const STORE_GLOBAL: u16 = 9;
const INDEX: u16 = 10;
const INDEX_OPT: u16 = 11;
const EACH: u16 = 12;
const EACH_OPT: u16 = 13;
const FORK: u16 = 14;
const TRY_BEGIN: u16 = 15;
const TRY_END: u16 = 16;
const JUMP: u16 = 17;
const JUMP_F: u16 = 18;
const BACKTRACK: u16 = 19;
const APPEND: u16 = 20;
const INSERT: u16 = 21;
const RANGE: u16 = 22;
const SUBEXP_BEGIN: u16 = 23;
const SUBEXP_END: u16 = 24;
const PATH_BEGIN: u16 = 25;
const PATH_END: u16 = 26;
const CALL_BUILTIN: u16 = 27;
const CALL_JQ: u16 = 28;
const RET: u16 = 29;
const TAIL_CALL_JQ: u16 = 30;
const CLOSURE_PARAM: u16 = 31;
const CLOSURE_REF: u16 = 32;
const CLOSURE_CREATE: u16 = 33;
const CLOSURE_CREATE_C: u16 = 34;
const TOP: u16 = 35;
const CLOSURE_PARAM_REGULAR: u16 = 36;
const DEPS: u16 = 37;
const MODULEMETA: u16 = 38;
const GENLABEL: u16 = 39;
const DESTRUCTURE_ALT: u16 = 40;
const STOREVN: u16 = 41;
const ERRORK: u16 = 42;
const NUM_OPCODES: u16 = 43;

/// Macro to compute backtracking opcode: ON_BACKTRACK(op) = op + NUM_OPCODES
const fn on_backtrack(op: u16) -> u16 {
    op + NUM_OPCODES
}

/// ARG_NEWCLOSURE flag for closure references
const ARG_NEWCLOSURE: u16 = 0x1000;

/// Maximum C function arguments
const MAX_CFUNCTION_ARGS: usize = 10;
/// Initialize a new jq state
pub fn jq_init<T: Default>() -> Option<Box<JqState<T>>> {
    let mut jq = Box::new(JqState {
        err_cb: Some(default_err_cb),
        err_cb_data: None,
        error_cb: None,
        error_cb_data: None,
        input_cb: None,
        input_cb_data: None,
        debug_cb: None,
        debug_cb_data: None,
        stderr_cb: None,
        stderr_cb_data: None,
        nomem_handler: None,
        nomem_handler_data: None,
        halted: false,
        exit_code: jv_invalid(),
        error_message: jv_invalid(),
        error: jv_null(),  // Current error for try/catch
        bc: None,
        stk: Stack { mem_end: std::ptr::null_mut(), bound: ALIGNMENT as i32, limit: 0 },
        curr_frame: 0,
        stk_top: 0,        // Stack top pointer
        fork_top: 0,       // Fork point top pointer
        path: jv_null(),
        value_at_path: jv_null(),
        subexp_nest: 0,
        debug_trace_enabled: 0,
        initial_execution: false,
        next_label: 0,
        attrs: jv_object(),
        frames: Vec::new(),
        _phantom: std::marker::PhantomData,
    });
    stack_init(&mut jq.stk);
    Some(jq)
}
/// Compile a jq program string
/// Returns 1 on success (non-zero), 0 on failure
/// Works with both `&mut JqState<T>` and `&mut Option<Box<JqState<T>>>`
pub fn jq_compile<T, J: JqStateAccess<T>>(jq: &mut J, program: &str) -> i32 {
    if jq_compile_args_impl(jq, program, jv_null()) { 1 } else { 0 }
}
/// Compile a jq program with arguments
///
/// Compiles the given jq program string with the provided arguments.
/// Returns true if compilation succeeded, false otherwise.
/// Works with both `&mut JqState<T>` and `&mut Option<Box<JqState<T>>>`
pub fn jq_compile_args<T, J: JqStateAccess<T>>(jq: &mut J, _str: &str, args: Jv) -> bool {
    jq_compile_args_impl(jq, _str, args)
}

fn jq_compile_args_impl<T, J: JqStateAccess<T>>(jq: &mut J, program_str: &str, args: Jv) -> bool {
    let debug = std::env::var("DEBUG_COMPILE").is_ok();
    if debug { eprintln!("DEBUG: jq_compile_args_impl called with program={:?}", program_str); }

    let jq_inner = match jq.get_jq_state() {
        Some(jq_state) => jq_state,
        None => {
            if debug { eprintln!("DEBUG: jq.get_jq_state() returned None"); }
            jv_free(args);
            return false;
        }
    };

    let args_kind = jv_get_kind(&args);
    if debug { eprintln!("DEBUG: args_kind={:?}", args_kind); }
    assert!(
        args_kind == JvKind::Array || args_kind == JvKind::Object || args_kind == JvKind::Null,
        "jv_get_kind(args) == JV_KIND_ARRAY || jv_get_kind(args) == JV_KIND_OBJECT || JV_KIND_NULL"
    );

    // Reset state
    jq_reset(jq_inner);
    if let Some(bc) = jq_inner.bc.take() {
        bytecode_free(Some(bc));
    }

    // Create Locfile for tracking source locations
    // Note: compile() expects Locfile<()>, not Locfile<T>
    let mut linemap = vec![0i32];
    for (i, c) in program_str.char_indices() {
        if c == '\n' {
            linemap.push((i + 1) as i32);
        }
    }
    // Add end of data marker (needed for locfile_line_length)
    linemap.push(program_str.len() as i32);
    let nlines = linemap.len() as i32 - 1; // nlines doesn't count the end marker
    let mut locations = Locfile::<()> {
        fname: "<program>".to_string(),
        data: program_str.to_string(),
        length: program_str.len() as i32,
        linemap,
        nlines,
        error: None,
        jq: None,
        refct: 1,
    };

    // Parse the program
    let debug = std::env::var("DEBUG_COMPILE").is_ok();
    let mut program = Block::default();
    if debug { eprintln!("DEBUG: Parsing program: {:?}", program_str); }
    let parse_errors = jq_parse(&mut locations, &mut program);

    if parse_errors > 0 {
        if debug { eprintln!("DEBUG: Parse failed with {} errors", parse_errors); }
        let msg = Jv::string(&format!("jq: {} parse errors", parse_errors));
        jq_report_error(jq_inner, msg);
        jv_free(args);
        return false;
    }

    // Bind builtins to the program - matches C's builtins_bind call in execute.c:1247
    if debug { eprintln!("DEBUG: Binding builtins to program"); }
    let program = builtins_bind(jq_inner, program);

    // Compile the parsed program with builtins bound
    let mut bc = Bytecode::default();
    // Initialize bc.globals similar to block_compile() in compile.c
    let ncfunc = count_cfunctions(&program) as usize;
    bc.globals = Some(jv_mem_alloc::<SymbolTable>());
    if let Some(ref mut globals) = bc.globals {
        globals.ncfunctions = 0;
        globals.cfunctions = jv_mem_calloc(ncfunc);
        globals.cfunc_names = Jv::array();
    }
    let mut env: Option<Jv> = Some(Jv::object());
    if debug { eprintln!("DEBUG: Compiling parsed program, ncfunc={}", ncfunc); }
    let compile_errors = compile(&mut bc, program, &mut locations, args, &mut env);

    if compile_errors > 0 {
        if debug { eprintln!("DEBUG: Compile failed with {} errors", compile_errors); }
        let msg = Jv::string(&format!("jq: {} compile errors", compile_errors));
        jq_report_error(jq_inner, msg);
        return false;
    }
    if debug {
        eprintln!("DEBUG: Compilation successful, bytecode len={}", bc.code.len());
        eprint!("DEBUG: bytecode = [");
        for (i, &op) in bc.code.iter().enumerate() {
            if i > 0 { eprint!(", "); }
            eprint!("{}", op);
        }
        eprintln!("]");
        eprintln!("DEBUG: bc.constants.len={}", bc.constants.array_length());
        for i in 0..bc.constants.array_length() {
            let c = crate::jv::jv_array_get(jv_copy(&bc.constants), i);
            eprintln!("  bc.constants[{}] kind={:?}", i, jv_get_kind(&c));
        }
    }

    // Store the compiled bytecode
    jq_inner.bc = Some(Box::new(bc));
    if debug {
        if let Some(ref stored_bc) = jq_inner.bc {
            eprintln!("DEBUG: after store, jq_inner.bc.constants.len={}", stored_bc.constants.array_length());
            for i in 0..stored_bc.constants.array_length() {
                let c = crate::jv::jv_array_get(jv_copy(&stored_bc.constants), i);
                eprintln!("  stored_bc.constants[{}] kind={:?}", i, jv_get_kind(&c));
            }
        }
    }
    jq_inner.bc.is_some()
}
/// Set the current error (for try/catch)
fn set_error_jq<T>(jq: &mut JqState<T>, value: Jv) {
    jv_free(std::mem::replace(&mut jq.error, value));
}

/// Execute next step of jq program
/// This is the full bytecode interpreter matching C execute.c:jq_next
/// Works with both `&mut JqState<T>` and `&mut Option<Box<JqState<T>>>`
pub fn jq_next<T, J: JqStateAccess<T>>(jq: &mut J) -> Jv {
    let jq_inner = match jq.get_jq_state() {
        Some(jq_state) => jq_state,
        None => return Jv::invalid(),
    };

    // Restore from fork point to get the pc offset
    let mut pc_offset = match stack_restore_impl(jq_inner) {
        Some(offset) => offset,
        None => return Jv::invalid(),
    };

    let mut backtracking = !jq_inner.initial_execution;
    jq_inner.initial_execution = false;

    // Ensure error is null at start
    assert!(jv_get_kind(&jq_inner.error) == JvKind::Null, "error should be null");

    // eprintln!("DEBUG jq_next: entering loop, halted={}, bc.is_some={}", jq_inner.halted, jq_inner.bc.is_some());
    // Main execution loop
    loop {
        if jq_inner.halted {
            // eprintln!("DEBUG jq_next: halted=true, returning invalid");
            return Jv::invalid();
        }

        // Get the bytecode from the current frame
        let bc = match frame_current(jq_inner) {
            Some(frame) => match &frame.bc {
                Some(bc) => bc.clone(),
                None => {
                    // eprintln!("DEBUG jq_next: frame.bc is None");
                    return Jv::invalid();
                }
            },
            None => {
                // eprintln!("DEBUG jq_next: frame_current returned None, curr_frame={}", jq_inner.curr_frame);
                return Jv::invalid();
            }
        };

        if pc_offset >= bc.code.len() {
            return Jv::invalid();
        }

        let mut opcode = bc.code[pc_offset];
        if std::env::var("DEBUG_EXEC").is_ok() {
            eprintln!("EXEC: pc={}, opcode={}, bc.code.len={}", pc_offset, opcode, bc.code.len());
        }
        let raising = !jv_is_valid(&jq_inner.error);

        if backtracking {
            opcode = on_backtrack(opcode);
            // eprintln!("DEBUG jq_next: backtracking, new opcode={}", opcode);
            backtracking = false;
        }
        pc_offset += 1;

        match opcode {
            TOP => {
                // No-op
            }

            ERRORK => {
                let const_idx = bc.code[pc_offset] as i32;
                pc_offset += 1;
                let v = jv_array_get_bc(&bc.constants, const_idx);
                set_error_jq(jq_inner, jv_invalid_with_msg(v));
                // Backtrack
                match stack_restore_impl(jq_inner) {
                    Some(offset) => {
                        pc_offset = offset;
                        backtracking = true;
                    }
                    None => {
                        if !jv_is_valid(&jq_inner.error) {
                            let error = std::mem::replace(&mut jq_inner.error, jv_null());
                            return error;
                        }
                        return Jv::invalid();
                    }
                }
            }

            LOADK => {
                let debug = std::env::var("DEBUG_EXEC").is_ok();
                let const_idx = bc.code[pc_offset] as i32;
                pc_offset += 1;
                if debug {
                    eprintln!("EXEC LOADK: const_idx={} constants.len={}", const_idx, bc.constants.array_length());
                    for i in 0..bc.constants.array_length() {
                        let c = jv_array_get_bc(&bc.constants, i);
                        eprintln!("  constants[{}] kind={:?}", i, jv_get_kind(&c));
                    }
                }
                let v = jv_array_get_bc(&bc.constants, const_idx);
                if debug { eprintln!("EXEC LOADK: v.kind={:?} v.is_valid={}", jv_get_kind(&v), jv_is_valid(&v)); }
                assert!(jv_is_valid(&v), "constant must be valid");
                // Pop old value, push new
                if let Some(old) = stack_pop(jq_inner) {
                    if debug { eprintln!("EXEC LOADK: popped old.kind={:?}", jv_get_kind(&old)); }
                    jv_free(old);
                }
                stack_push(jq_inner, v);
                if debug { eprintln!("EXEC LOADK: pushed constant"); }
            }

            GENLABEL => {
                let label = jq_inner.next_label;
                jq_inner.next_label += 1;
                let label_jv = jv_object_set(
                    jv_object(),
                    Jv::string("__jq"),
                    jv_number(label as f64),
                );
                stack_push(jq_inner, label_jv);
            }

            DUP => {
                if let Some(v) = stack_pop(jq_inner) {
                    stack_push(jq_inner, jv_copy(&v));
                    stack_push(jq_inner, v);
                }
            }

            DUPN => {
                let v = stack_popn(jq_inner);
                stack_push(jq_inner, jv_copy(&v));
                stack_push(jq_inner, v);
            }

            DUP2 => {
                if let Some(keep) = stack_pop(jq_inner) {
                    if let Some(v) = stack_pop(jq_inner) {
                        stack_push(jq_inner, jv_copy(&v));
                        stack_push(jq_inner, keep);
                        stack_push(jq_inner, v);
                    }
                }
            }

            SUBEXP_BEGIN => {
                if let Some(v) = stack_pop(jq_inner) {
                    stack_push(jq_inner, jv_copy(&v));
                    stack_push(jq_inner, v);
                    jq_inner.subexp_nest += 1;
                }
            }

            SUBEXP_END => {
                assert!(jq_inner.subexp_nest > 0, "subexp_nest must be > 0");
                jq_inner.subexp_nest -= 1;
                if let Some(a) = stack_pop(jq_inner) {
                    if let Some(b) = stack_pop(jq_inner) {
                        stack_push(jq_inner, a);
                        stack_push(jq_inner, b);
                    }
                }
            }

            PUSHK_UNDER => {
                let debug = std::env::var("DEBUG_EXEC").is_ok();
                let const_idx = bc.code[pc_offset] as i32;
                pc_offset += 1;
                if debug {
                    eprintln!("EXEC PUSHK_UNDER: const_idx={} bc.constants.len={}",
                             const_idx, bc.constants.array_length());
                }
                let v = jv_array_get_bc(&bc.constants, const_idx);
                if debug {
                    eprintln!("EXEC PUSHK_UNDER: v.kind={:?} v.is_valid={}", jv_get_kind(&v), jv_is_valid(&v));
                    if jv_get_kind(&v) == JvKind::String {
                        eprintln!("EXEC PUSHK_UNDER: string value=\"{}\"", crate::jv::jv_string_value(&v));
                    }
                }
                if !jv_is_valid(&v) {
                    if debug { eprintln!("EXEC PUSHK_UNDER: constant is invalid!"); }
                    stack_push(jq_inner, jv_null());
                } else {
                    if let Some(v2) = stack_pop(jq_inner) {
                        stack_push(jq_inner, v);
                        stack_push(jq_inner, v2);
                    }
                }
            }

            POP => {
                if let Some(v) = stack_pop(jq_inner) {
                    jv_free(v);
                }
            }

            APPEND => {
                if let Some(v) = stack_pop(jq_inner) {
                    let level = bc.code[pc_offset] as i32;
                    pc_offset += 1;
                    let vidx = bc.code[pc_offset] as i32;
                    pc_offset += 1;
                    if let Some(var) = frame_local_var(jq_inner, vidx, level) {
                        assert!(jv_get_kind(var) == JvKind::Array, "var must be array");
                        let arr = std::mem::replace(var, jv_null());
                        *var = jv_array_append(arr, v);
                    }
                }
            }

            INSERT => {
                if let Some(stktop) = stack_pop(jq_inner) {
                    if let Some(v) = stack_pop(jq_inner) {
                        if let Some(k) = stack_pop(jq_inner) {
                            if let Some(objv) = stack_pop(jq_inner) {
                                assert!(jv_get_kind(&objv) == JvKind::Object, "must be object");
                                if jv_get_kind(&k) == JvKind::String {
                                    let new_obj = jv_object_set(objv, k, v);
                                    stack_push(jq_inner, new_obj);
                                    stack_push(jq_inner, stktop);
                                } else {
                                    // Invalid key type - set error and backtrack
                                    set_error_jq(jq_inner, jv_invalid_with_msg(
                                        Jv::string("Cannot use non-string as object key"),
                                    ));
                                    jv_free(stktop);
                                    jv_free(v);
                                    jv_free(k);
                                    jv_free(objv);
                                    // Backtrack
                                    match stack_restore_impl(jq_inner) {
                                        Some(offset) => {
                                            pc_offset = offset;
                                            backtracking = true;
                                        }
                                        None => {
                                            if !jv_is_valid(&jq_inner.error) {
                                                let error = std::mem::replace(&mut jq_inner.error, jv_null());
                                                return error;
                                            }
                                            return Jv::invalid();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            LOADV => {
                let level = bc.code[pc_offset] as i32;
                pc_offset += 1;
                let v = bc.code[pc_offset] as i32;
                pc_offset += 1;
                // First copy the value from the variable
                let var_copy = frame_local_var(jq_inner, v, level).map(|v| jv_copy(v));
                // Then do stack operations
                if let Some(copy) = var_copy {
                    if let Some(old) = stack_pop(jq_inner) {
                        jv_free(old);
                    }
                    stack_push(jq_inner, copy);
                }
            }

            LOADVN => {
                let level = bc.code[pc_offset] as i32;
                pc_offset += 1;
                let v = bc.code[pc_offset] as i32;
                pc_offset += 1;
                // First take the value from the variable (replacing with null)
                let val = frame_local_var(jq_inner, v, level).map(|v| std::mem::replace(v, jv_null()));
                // Then do stack operations
                if let Some(val) = val {
                    let _ = stack_popn(jq_inner);
                    stack_push(jq_inner, val);
                }
            }

            STOREV => {
                let level = bc.code[pc_offset] as i32;
                pc_offset += 1;
                let v = bc.code[pc_offset] as i32;
                pc_offset += 1;
                if let Some(val) = stack_pop(jq_inner) {
                    if let Some(var) = frame_local_var(jq_inner, v, level) {
                        jv_free(std::mem::replace(var, val));
                    }
                }
            }

            STOREVN => {
                // Save state for backtracking first
                let spos = stack_get_pos(jq_inner);
                stack_save(jq_inner, Some(pc_offset - 1), spos);

                let level = bc.code[pc_offset] as i32;
                pc_offset += 1;
                let v = bc.code[pc_offset] as i32;
                pc_offset += 1;
                if let Some(val) = stack_pop(jq_inner) {
                    if let Some(var) = frame_local_var(jq_inner, v, level) {
                        jv_free(std::mem::replace(var, val));
                    }
                }
            }

            op if op == on_backtrack(STOREVN) => {
                let level = bc.code[pc_offset] as i32;
                pc_offset += 1;
                let v = bc.code[pc_offset] as i32;
                pc_offset += 1;
                if let Some(var) = frame_local_var(jq_inner, v, level) {
                    jv_free(std::mem::replace(var, jv_null()));
                }
                // Backtrack
                match stack_restore_impl(jq_inner) {
                    Some(offset) => {
                        pc_offset = offset;
                        backtracking = true;
                    }
                    None => {
                        if !jv_is_valid(&jq_inner.error) {
                            let error = std::mem::replace(&mut jq_inner.error, jv_null());
                            return error;
                        }
                        return Jv::invalid();
                    }
                }
            }

            INDEX | INDEX_OPT => {
                let debug = std::env::var("DEBUG_EXEC").is_ok();
                if debug { eprintln!("EXEC INDEX: stack before pop"); }
                if let Some(t) = stack_pop(jq_inner) {
                    if debug { eprintln!("EXEC INDEX: t.kind={:?}", jv_get_kind(&t)); }
                    if let Some(k) = stack_pop(jq_inner) {
                        if debug {
                            eprintln!("EXEC INDEX: k.kind={:?}", jv_get_kind(&k));
                            if jv_get_kind(&k) == JvKind::String {
                                eprintln!("EXEC INDEX: k.string=\"{}\"", crate::jv::jv_string_value(&k));
                            }
                        }
                        // Check path integrity
                        if !path_intact(jq_inner, jv_copy(&t)) {
                            if debug { eprintln!("EXEC INDEX: path_intact failed"); }
                            set_error_jq(jq_inner, jv_invalid_with_msg(
                                Jv::string("Invalid path expression"),
                            ));
                            // Backtrack
                            match stack_restore_impl(jq_inner) {
                                Some(offset) => {
                                    pc_offset = offset;
                                    backtracking = true;
                                    continue;
                                }
                                None => {
                                    if !jv_is_valid(&jq_inner.error) {
                                        let error = std::mem::replace(&mut jq_inner.error, jv_null());
                                        return error;
                                    }
                                    return Jv::invalid();
                                }
                            }
                        }
                        let v = jv_get(t, jv_copy(&k));
                        if debug { eprintln!("EXEC INDEX: v.kind={:?} v.is_valid={}", jv_get_kind(&v), jv_is_valid(&v)); }
                        if jv_is_valid(&v) {
                            path_append(jq_inner, k, jv_copy(&v));
                            stack_push(jq_inner, v);
                            if debug { eprintln!("EXEC INDEX: pushed result"); }
                        } else {
                            if debug { eprintln!("EXEC INDEX: v is invalid, backtracking"); }
                            jv_free(k);
                            if opcode == INDEX {
                                set_error_jq(jq_inner, v);
                            } else {
                                jv_free(v);
                            }
                            // Backtrack
                            match stack_restore_impl(jq_inner) {
                                Some(offset) => {
                                    pc_offset = offset;
                                    backtracking = true;
                                }
                                None => {
                                    if !jv_is_valid(&jq_inner.error) {
                                        let error = std::mem::replace(&mut jq_inner.error, jv_null());
                                        return error;
                                    }
                                    return Jv::invalid();
                                }
                            }
                        }
                    } else {
                        if debug { eprintln!("EXEC INDEX: k pop failed"); }
                    }
                } else {
                    if debug { eprintln!("EXEC INDEX: t pop failed"); }
                }
            }

            JUMP => {
                let offset = bc.code[pc_offset] as usize;
                pc_offset += 1;
                pc_offset += offset;
            }

            JUMP_F => {
                let offset = bc.code[pc_offset] as usize;
                pc_offset += 1;
                if let Some(t) = stack_pop(jq_inner) {
                    let kind = jv_get_kind(&t);
                    if kind == JvKind::False || kind == JvKind::Null {
                        pc_offset += offset;
                    }
                    stack_push(jq_inner, t);
                }
            }

            BACKTRACK => {
                // Backtrack
                match stack_restore_impl(jq_inner) {
                    Some(offset) => {
                        pc_offset = offset;
                        backtracking = true;
                    }
                    None => {
                        if !jv_is_valid(&jq_inner.error) {
                            let error = std::mem::replace(&mut jq_inner.error, jv_null());
                            return error;
                        }
                        return Jv::invalid();
                    }
                }
            }

            FORK | DESTRUCTURE_ALT => {
                let spos = stack_get_pos(jq_inner);
                stack_save(jq_inner, Some(pc_offset - 1), spos);
                pc_offset += 1; // Skip offset this time
            }

            op if op == on_backtrack(FORK) => {
                if raising {
                    // Backtrack
                    match stack_restore_impl(jq_inner) {
                        Some(offset) => {
                            pc_offset = offset;
                            backtracking = true;
                        }
                        None => {
                            if !jv_is_valid(&jq_inner.error) {
                                let error = std::mem::replace(&mut jq_inner.error, jv_null());
                                return error;
                            }
                            return Jv::invalid();
                        }
                    }
                } else {
                    let offset = bc.code[pc_offset] as usize;
                    pc_offset += 1;
                    pc_offset += offset;
                }
            }

            op if op == on_backtrack(DESTRUCTURE_ALT) => {
                if jv_is_valid(&jq_inner.error) {
                    // Backtrack more
                    if let Some(v) = stack_pop(jq_inner) {
                        jv_free(v);
                    }
                    match stack_restore_impl(jq_inner) {
                        Some(offset) => {
                            pc_offset = offset;
                            backtracking = true;
                        }
                        None => {
                            if !jv_is_valid(&jq_inner.error) {
                                let error = std::mem::replace(&mut jq_inner.error, jv_null());
                                return error;
                            }
                            return Jv::invalid();
                        }
                    }
                } else {
                    // Exception caught - clear error
                    jv_free(std::mem::replace(&mut jq_inner.error, jv_null()));
                    let offset = bc.code[pc_offset] as usize;
                    pc_offset += 1;
                    pc_offset += offset;
                }
            }

            TRY_BEGIN => {
                let spos = stack_get_pos(jq_inner);
                stack_save(jq_inner, Some(pc_offset - 1), spos);
                pc_offset += 1; // Skip handler offset
            }

            TRY_END => {
                let spos = stack_get_pos(jq_inner);
                stack_save(jq_inner, Some(pc_offset - 1), spos);
            }

            op if op == on_backtrack(TRY_BEGIN) => {
                if !raising {
                    // EXP backtracked, backtrack more
                    if let Some(v) = stack_pop(jq_inner) {
                        jv_free(v);
                    }
                    match stack_restore_impl(jq_inner) {
                        Some(offset) => {
                            pc_offset = offset;
                            backtracking = true;
                        }
                        None => {
                            if !jv_is_valid(&jq_inner.error) {
                                let error = std::mem::replace(&mut jq_inner.error, jv_null());
                                return error;
                            }
                            return Jv::invalid();
                        }
                    }
                } else {
                    // Error caught - jump to handler
                    let offset = bc.code[pc_offset] as usize;
                    pc_offset += 1;
                    if let Some(v) = stack_pop(jq_inner) {
                        jv_free(v);
                    }
                    // Push error message
                    let err_msg = jv_invalid_get_msg(std::mem::replace(&mut jq_inner.error, jv_null()));
                    stack_push(jq_inner, err_msg);
                    pc_offset += offset;
                }
            }

            op if op == on_backtrack(TRY_END) => {
                if raising {
                    // Wrap error so TRY_BEGIN doesn't catch it
                    let err = jv_copy(&jq_inner.error);
                    set_error_jq(jq_inner, jv_invalid_with_msg(err));
                }
                match stack_restore_impl(jq_inner) {
                    Some(offset) => {
                        pc_offset = offset;
                        backtracking = true;
                    }
                    None => {
                        if !jv_is_valid(&jq_inner.error) {
                            let error = std::mem::replace(&mut jq_inner.error, jv_null());
                            return error;
                        }
                        return Jv::invalid();
                    }
                }
            }

            RET => {
                let debug = std::env::var("DEBUG_EXEC").is_ok();
                if debug { eprintln!("EXEC RET: about to pop, stk_top={}", jq_inner.stk_top); }
                if let Some(value) = stack_pop(jq_inner) {
                    if debug { eprintln!("EXEC RET: popped value kind={:?}", jv_get_kind(&value)); }
                    // Check if this is a top-level return
                    let frame = frame_current(jq_inner);
                    let is_top_level = match frame {
                        Some(f) => {
                            if debug { eprintln!("EXEC RET: frame.retaddr_offset={}, frame.env={}", f.retaddr_offset, f.env); }
                            f.retaddr_offset == 0 && f.env == -1
                        }
                        None => {
                            if debug { eprintln!("EXEC RET: frame is None"); }
                            true
                        }
                    };
                    if debug { eprintln!("EXEC RET: is_top_level={}", is_top_level); }

                    if is_top_level {
                        // Top-level return - yield this value
                        let spos = stack_get_pos(jq_inner);
                        stack_push(jq_inner, jv_null());
                        stack_save(jq_inner, Some(pc_offset - 1), spos);
                        if debug { eprintln!("EXEC RET: returning value kind={:?}", jv_get_kind(&value)); }
                        return value;
                    } else {
                        // Function return
                        let retaddr = match frame_current(jq_inner) {
                            Some(f) => f.retaddr_offset,
                            None => 0,
                        };
                        pc_offset = retaddr;
                        frame_pop(jq_inner);
                        stack_push(jq_inner, value);
                    }
                } else {
                    // eprintln!("DEBUG RET: stack_pop returned None!");
                }
            }

            op if op == on_backtrack(RET) => {
                // Resumed after top-level return, backtrack
                match stack_restore_impl(jq_inner) {
                    Some(offset) => {
                        pc_offset = offset;
                        backtracking = true;
                    }
                    None => {
                        if !jv_is_valid(&jq_inner.error) {
                            let error = std::mem::replace(&mut jq_inner.error, jv_null());
                            return error;
                        }
                        return Jv::invalid();
                    }
                }
            }

            CALL_JQ | TAIL_CALL_JQ => {
                let debug_call = std::env::var("DEBUG_EXEC").is_ok();
                if debug_call {
                    eprintln!("EXEC CALL_JQ: starting, pc_offset={}", pc_offset);
                }
                if let Some(input) = stack_pop(jq_inner) {
                    let nclosures = bc.code[pc_offset] as usize;
                    pc_offset += 1;
                    if debug_call {
                        eprintln!("EXEC CALL_JQ: nclosures={}", nclosures);
                    }

                    // Get callee closure
                    let cl = make_closure_from_pc(jq_inner, &bc.code, &mut pc_offset);
                    if debug_call {
                        eprintln!("EXEC CALL_JQ: closure bc.is_some()={}", cl.bc.is_some());
                        if let Some(ref bc) = cl.bc {
                            eprintln!("EXEC CALL_JQ: closure bc.codelen={} constants.len={}", bc.codelen, bc.constants.array_length());
                        }
                    }

                    // Calculate return address
                    let retaddr = pc_offset + nclosures * 2;

                    if opcode == TAIL_CALL_JQ {
                        // For tail calls, use the current frame's return info
                        frame_pop(jq_inner);
                    }

                    // Push new frame
                    let _new_frame = frame_push(jq_inner, cl, nclosures as i32);

                    // Set return address on the new frame
                    if let Some(frame) = frame_current_mut(jq_inner) {
                        frame.retaddr_offset = retaddr;
                    }

                    // Push input and reset pc to start of new function
                    stack_push(jq_inner, input);
                    pc_offset = 0;
                }
            }

            // EACH and EACH_OPT - iterate over arrays/objects
            EACH | EACH_OPT => {
                let debug = std::env::var("DEBUG_EXEC").is_ok();
                if debug { eprintln!("EXEC EACH: starting iteration"); }
                if let Some(container) = stack_pop(jq_inner) {
                    // Check path integrity
                    if !path_intact(jq_inner, jv_copy(&container)) {
                        let msg = Jv::string("Invalid path expression near attempt to iterate");
                        set_error_jq(jq_inner, jv_invalid_with_msg(msg));
                        jv_free(container);
                        match stack_restore_impl(jq_inner) {
                            Some(offset) => {
                                pc_offset = offset;
                                backtracking = true;
                                continue;
                            }
                            None => return Jv::invalid(),
                        }
                    }
                    // Push container and initial index (-1) for first iteration
                    stack_push(jq_inner, container);
                    stack_push(jq_inner, Jv::number(-1.0));
                    // Fall through to backtrack handler by setting backtracking
                    backtracking = true;
                    // Don't advance pc_offset - we want to re-execute this opcode in backtrack mode
                    pc_offset -= 1;
                }
            }

            // ON_BACKTRACK(EACH) and ON_BACKTRACK(EACH_OPT) - continue iteration
            on_bt_each if on_bt_each == on_backtrack(EACH) || on_bt_each == on_backtrack(EACH_OPT) => {
                let debug = std::env::var("DEBUG_EXEC").is_ok();
                if debug { eprintln!("EXEC EACH backtrack: continuing iteration"); }

                let idx_jv = stack_pop(jq_inner).unwrap_or_else(|| Jv::number(0.0));
                let container = stack_pop(jq_inner).unwrap_or_else(Jv::null);
                let mut idx = jv_number_value(&idx_jv) as i32;

                let container_kind = jv_get_kind(&container);
                let (keep_going, is_last, key, value) = if container_kind == JvKind::Array {
                    // For arrays
                    if opcode == on_backtrack(EACH) || opcode == on_backtrack(EACH_OPT) {
                        idx = if idx < 0 { 0 } else { idx + 1 };
                    }
                    let len = jv_array_length(&container);
                    let keep = idx < len;
                    let last = idx == len - 1;
                    if keep {
                        let k = Jv::number(idx as f64);
                        let v = jv_array_get(jv_copy(&container), idx);
                        (keep, last, k, v)
                    } else {
                        (false, false, Jv::null(), Jv::null())
                    }
                } else if container_kind == JvKind::Object {
                    // For objects - use object iteration
                    if opcode == on_backtrack(EACH) || opcode == on_backtrack(EACH_OPT) {
                        idx = if idx < 0 { 0 } else { idx + 1 };
                    }
                    let keys = jv_keys(jv_copy(&container));
                    let len = jv_array_length(&keys);
                    let keep = idx < len;
                    let last = idx == len - 1;
                    if keep {
                        let k = jv_array_get(jv_copy(&keys), idx);
                        let v = jv_object_get(&container, jv_copy(&k));
                        jv_free(keys);
                        (keep, last, k, v)
                    } else {
                        jv_free(keys);
                        (false, false, Jv::null(), Jv::null())
                    }
                } else {
                    // Not iterable
                    if on_bt_each == on_backtrack(EACH) {
                        let msg = format!("Cannot iterate over {}",
                            match container_kind {
                                JvKind::Null => "null",
                                JvKind::Number => "number",
                                JvKind::String => "string",
                                _ => "value",
                            });
                        set_error_jq(jq_inner, jv_invalid_with_msg(Jv::string(&msg)));
                    }
                    (false, false, Jv::null(), Jv::null())
                };

                if !keep_going {
                    jv_free(container);
                    match stack_restore_impl(jq_inner) {
                        Some(offset) => {
                            pc_offset = offset;
                            backtracking = true;
                            continue;
                        }
                        None => return Jv::invalid(),
                    }
                } else if is_last {
                    // Last element - no need for backtrack point
                    jv_free(container);
                    path_append(jq_inner, key, jv_copy(&value));
                    stack_push(jq_inner, value);
                } else {
                    // More elements - save state for next iteration
                    let spos = stack_get_pos(jq_inner);
                    stack_push(jq_inner, container);
                    stack_push(jq_inner, Jv::number(idx as f64));
                    stack_save(jq_inner, Some(pc_offset - 1), spos);
                    path_append(jq_inner, key, jv_copy(&value));
                    stack_push(jq_inner, value);
                }
            }

            CALL_BUILTIN => {
                let nargs = bc.code[pc_offset] as i32;
                pc_offset += 1;
                let cfunc_idx = bc.code[pc_offset] as usize;
                pc_offset += 1;

                // Pop arguments from stack
                let mut args: Vec<Jv> = Vec::with_capacity(nargs as usize);
                for _ in 0..nargs {
                    if let Some(arg) = stack_pop(jq_inner) {
                        args.push(arg);
                    }
                }

                // Get function name from cfunc_names
                let func_name = if let Some(ref bc_box) = jq_inner.bc {
                    if let Some(ref globals) = bc_box.globals {
                        let name = crate::jv::jv_array_get(globals.cfunc_names.copy(), cfunc_idx as i32);
                        if name.get_kind() == JvKind::String {
                            crate::jv::jv_string_value(&name).to_string()
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };

                // Dispatch to builtin function
                let result = call_builtin(jq_inner, &func_name, args);

                if jv_is_valid(&result) {
                    stack_push(jq_inner, result);
                } else if crate::jv::jv_invalid_has_msg(jv_copy(&result)) != 0 {
                    set_error_jq(jq_inner, result);
                    match stack_restore_impl(jq_inner) {
                        Some(offset) => {
                            pc_offset = offset;
                            backtracking = true;
                        }
                        None => return Jv::invalid(),
                    }
                } else {
                    jv_free(result);
                    match stack_restore_impl(jq_inner) {
                        Some(offset) => {
                            pc_offset = offset;
                            backtracking = true;
                        }
                        None => return Jv::invalid(),
                    }
                }
            }

            RANGE => {
                // RANGE opcode: iterate from current value up to limit
                // Read the variable binding (level and var index)
                let var_idx = bc.code[pc_offset] as i32;
                pc_offset += 1;

                // Get current counter value from variable using frame_local_var
                let counter = if let Some(v) = frame_local_var(jq_inner, var_idx, 0) {
                    jv_copy(v)
                } else {
                    Jv::number(0.0)
                };

                let idx = jv_number_value(&counter) as i32;
                jv_free(counter);

                // Get limit from stack
                if let Some(limit_jv) = stack_pop(jq_inner) {
                    let limit = jv_number_value(&limit_jv) as i32;

                    if idx < limit {
                        // Save state for next iteration
                        let spos = stack_get_pos(jq_inner);
                        stack_save(jq_inner, Some(pc_offset - 2), spos);

                        // Push limit back and push current value
                        stack_push(jq_inner, limit_jv);
                        stack_push(jq_inner, Jv::number(idx as f64));

                        // Increment counter in frame
                        if let Some(v) = frame_local_var(jq_inner, var_idx, 0) {
                            let old = std::mem::replace(v, Jv::number((idx + 1) as f64));
                            jv_free(old);
                        }
                    } else {
                        jv_free(limit_jv);
                        match stack_restore_impl(jq_inner) {
                            Some(offset) => {
                                pc_offset = offset;
                                backtracking = true;
                            }
                            None => return Jv::invalid(),
                        }
                    }
                }
            }

            op if op == on_backtrack(RANGE) => {
                // Continue range iteration - same logic as RANGE
                let var_idx = bc.code[pc_offset] as i32;
                pc_offset += 1;

                let counter = if let Some(v) = frame_local_var(jq_inner, var_idx, 0) {
                    jv_copy(v)
                } else {
                    Jv::number(0.0)
                };

                let idx = jv_number_value(&counter) as i32;
                jv_free(counter);

                if let Some(limit_jv) = stack_pop(jq_inner) {
                    let limit = jv_number_value(&limit_jv) as i32;

                    if idx < limit {
                        let spos = stack_get_pos(jq_inner);
                        stack_save(jq_inner, Some(pc_offset - 2), spos);
                        stack_push(jq_inner, limit_jv);
                        stack_push(jq_inner, Jv::number(idx as f64));

                        if let Some(v) = frame_local_var(jq_inner, var_idx, 0) {
                            let old = std::mem::replace(v, Jv::number((idx + 1) as f64));
                            jv_free(old);
                        }
                    } else {
                        jv_free(limit_jv);
                        match stack_restore_impl(jq_inner) {
                            Some(offset) => {
                                pc_offset = offset;
                                backtracking = true;
                            }
                            None => return Jv::invalid(),
                        }
                    }
                }
            }

            PATH_BEGIN => {
                // C: jv v = stack_pop(jq);
                if let Some(v) = stack_pop(jq_inner) {
                    // C: stack_push(jq, jq->path);
                    let old_path = std::mem::replace(&mut jq_inner.path, jv_null());
                    stack_push(jq_inner, old_path);

                    // C: stack_save(jq, pc - 1, stack_get_pos(jq));
                    let spos = stack_get_pos(jq_inner);
                    stack_save(jq_inner, Some(pc_offset - 1), spos);

                    // C: stack_push(jq, jv_number(jq->subexp_nest));
                    stack_push(jq_inner, Jv::number(jq_inner.subexp_nest as f64));

                    // C: stack_push(jq, jq->value_at_path);
                    let old_value_at_path = std::mem::replace(&mut jq_inner.value_at_path, jv_null());
                    stack_push(jq_inner, old_value_at_path);

                    // C: stack_push(jq, jv_copy(v));
                    stack_push(jq_inner, jv_copy(&v));

                    // C: jq->path = jv_array();
                    jq_inner.path = jv_array();

                    // C: jq->value_at_path = v;
                    jq_inner.value_at_path = v;

                    // C: jq->subexp_nest = 0;
                    jq_inner.subexp_nest = 0;
                }
            }

            PATH_END => {
                // C: jv v = stack_pop(jq);
                if let Some(v) = stack_pop(jq_inner) {
                    // C: if (!path_intact(jq, jv_copy(v))) { ... error ... }
                    if !path_intact(jq_inner, jv_copy(&v)) {
                        set_error_jq(jq_inner, jv_invalid_with_msg(
                            Jv::string("Invalid path expression with result"),
                        ));
                        jv_free(v);
                        match stack_restore_impl(jq_inner) {
                            Some(offset) => {
                                pc_offset = offset;
                                backtracking = true;
                                continue;
                            }
                            None => {
                                if !jv_is_valid(&jq_inner.error) {
                                    let error = std::mem::replace(&mut jq_inner.error, jv_null());
                                    return error;
                                }
                                return Jv::invalid();
                            }
                        }
                    }
                    // C: jv_free(v); // discard value, only keep path
                    jv_free(v);

                    // C: jv old_value_at_path = stack_pop(jq);
                    let old_value_at_path = stack_pop(jq_inner).unwrap_or_else(jv_null);

                    // C: int old_subexp_nest = (int)jv_number_value(stack_pop(jq));
                    let old_subexp_nest_jv = stack_pop(jq_inner).unwrap_or_else(|| Jv::number(0.0));
                    let old_subexp_nest = jv_number_value(&old_subexp_nest_jv) as i32;
                    jv_free(old_subexp_nest_jv);

                    // C: jv path = jq->path;
                    // C: jq->path = stack_pop(jq);
                    let path = std::mem::replace(&mut jq_inner.path, jv_null());
                    jq_inner.path = stack_pop(jq_inner).unwrap_or_else(jv_null);

                    // C: struct stack_pos spos = stack_get_pos(jq);
                    // C: stack_push(jq, jv_copy(path));
                    // C: stack_save(jq, pc - 1, spos);
                    let spos = stack_get_pos(jq_inner);
                    stack_push(jq_inner, jv_copy(&path));
                    stack_save(jq_inner, Some(pc_offset - 1), spos);

                    // C: stack_push(jq, path);
                    stack_push(jq_inner, path);

                    // C: jq->subexp_nest = old_subexp_nest;
                    jq_inner.subexp_nest = old_subexp_nest;

                    // C: jv_free(jq->value_at_path);
                    // C: jq->value_at_path = old_value_at_path;
                    jv_free(std::mem::replace(&mut jq_inner.value_at_path, old_value_at_path));
                }
            }

            // ON_BACKTRACK(PATH_BEGIN) and ON_BACKTRACK(PATH_END)
            op if op == on_backtrack(PATH_BEGIN) || op == on_backtrack(PATH_END) => {
                // C: jv_free(jq->path);
                // C: jq->path = stack_pop(jq);
                // C: goto do_backtrack;
                jv_free(std::mem::replace(&mut jq_inner.path, jv_null()));
                jq_inner.path = stack_pop(jq_inner).unwrap_or_else(jv_null);
                match stack_restore_impl(jq_inner) {
                    Some(offset) => {
                        pc_offset = offset;
                        backtracking = true;
                    }
                    None => {
                        if !jv_is_valid(&jq_inner.error) {
                            let error = std::mem::replace(&mut jq_inner.error, jv_null());
                            return error;
                        }
                        return Jv::invalid();
                    }
                }
            }

            STORE_GLOBAL => {
                let _kidx = bc.code[pc_offset];
                pc_offset += 1;
                if let Some(val) = stack_pop(jq_inner) {
                    jv_free(val);
                }
            }

            CLOSURE_CREATE | CLOSURE_CREATE_C => {
                pc_offset += 1;
            }

            CLOSURE_PARAM | CLOSURE_PARAM_REGULAR => {
                pc_offset += 1;
            }

            CLOSURE_REF => {
                pc_offset += 2;
            }

            DEPS => {
                // Module dependencies - no-op at runtime
            }

            MODULEMETA => {
                if let Some(name) = stack_pop(jq_inner) {
                    jv_free(name);
                    stack_push(jq_inner, Jv::null());
                }
            }

            _ => {
                // Unknown opcode
                eprintln!("Unknown opcode: {}", opcode);
                return Jv::invalid();
            }
        }
    }
}
// ============================================================================
// BUILTIN FUNCTION DISPATCHER
// ============================================================================

fn call_builtin<T>(jq: &mut JqState<T>, name: &str, mut args: Vec<Jv>) -> Jv {
    use crate::jv_aux::{jv_keys, jv_keys_unsorted, jv_has, jv_get, jv_set, jv_getpath, jv_setpath, jv_delpaths, jv_sort, jv_group};
    use crate::jv::{jv_array_length, jv_string_length_bytes, jv_object_length, jv_get_kind, jv_kind_name};

    // args[0] is the input (top of stack when popped)
    let input = if !args.is_empty() { args.remove(0) } else { Jv::null() };

    match name {
        "keys" => {
            if input.get_kind() == JvKind::Object || input.get_kind() == JvKind::Array {
                jv_keys(input)
            } else {
                crate::builtin::type_error(input, "has no keys")
            }
        }
        "keys_unsorted" => {
            if input.get_kind() == JvKind::Object || input.get_kind() == JvKind::Array {
                jv_keys_unsorted(input)
            } else {
                crate::builtin::type_error(input, "has no keys")
            }
        }
        "length" => {
            match input.get_kind() {
                JvKind::Array => {
                    let len = jv_array_length(&input);
                    jv_free(input);
                    Jv::number(len as f64)
                }
                JvKind::Object => {
                    let len = jv_object_length(&input);
                    jv_free(input);
                    Jv::number(len as f64)
                }
                JvKind::String => {
                    let len = jv_string_length_bytes(&input);
                    jv_free(input);
                    Jv::number(len as f64)
                }
                JvKind::Null => {
                    jv_free(input);
                    Jv::number(0.0)
                }
                JvKind::Number => {
                    let n = input.number_value().abs();
                    jv_free(input);
                    Jv::number(n)
                }
                _ => crate::builtin::type_error(input, "has no length")
            }
        }
        "type" => {
            let kind = input.get_kind();
            jv_free(input);
            Jv::string(jv_kind_name(kind))
        }
        "has" => {
            let key = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            // jv_has takes ownership and returns a Jv (true/false)
            jv_has(input, key)
        }
        "contains" => {
            let other = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            let result = crate::jv::jv_contains(jv_copy(&input), jv_copy(&other));
            jv_free(input);
            jv_free(other);
            if result != 0 { Jv::jv_true() } else { Jv::jv_false() }
        }
        "sort" => {
            if input.get_kind() == JvKind::Array {
                crate::builtin::f_sort(jq, input)
            } else {
                crate::builtin::type_error(input, "cannot be sorted, as it is not an array")
            }
        }
        "reverse" => {
            if input.get_kind() == JvKind::Array {
                let len = jv_array_length(&input);
                let mut result = crate::jv::jv_array();
                for i in (0..len).rev() {
                    let elem = crate::jv::jv_array_get(jv_copy(&input), i);
                    result = crate::jv::jv_array_append(result, elem);
                }
                jv_free(input);
                result
            } else {
                crate::builtin::type_error(input, "cannot be reversed (not an array)")
            }
        }
        "add" => {
            if input.get_kind() == JvKind::Array {
                crate::builtin::f_add(jq, input)
            } else {
                crate::builtin::type_error(input, "cannot be added (not an array)")
            }
        }
        "floor" => {
            if input.get_kind() == JvKind::Number {
                let v = input.number_value().floor();
                jv_free(input);
                Jv::number(v)
            } else {
                crate::builtin::type_error(input, "cannot floor non-number")
            }
        }
        "ceil" => {
            if input.get_kind() == JvKind::Number {
                let v = input.number_value().ceil();
                jv_free(input);
                Jv::number(v)
            } else {
                crate::builtin::type_error(input, "cannot ceil non-number")
            }
        }
        "round" => {
            if input.get_kind() == JvKind::Number {
                let v = input.number_value().round();
                jv_free(input);
                Jv::number(v)
            } else {
                crate::builtin::type_error(input, "cannot round non-number")
            }
        }
        "sqrt" => {
            if input.get_kind() == JvKind::Number {
                let v = input.number_value().sqrt();
                jv_free(input);
                Jv::number(v)
            } else {
                crate::builtin::type_error(input, "cannot sqrt non-number")
            }
        }
        "tonumber" => {
            crate::builtin::f_tonumber(jq, input)
        }
        "tostring" => {
            crate::builtin::f_tostring(jq, input)
        }
        "tojson" | "tojsonstream" => {
            crate::builtin::f_dump(jq, input)
        }
        "fromjson" => {
            crate::builtin::f_json_parse(jq, input)
        }
        "not" => {
            let is_false = match input.get_kind() {
                JvKind::False | JvKind::Null => true,
                _ => false,
            };
            jv_free(input);
            if is_false { Jv::jv_true() } else { Jv::jv_false() }
        }
        "null" => {
            jv_free(input);
            Jv::null()
        }
        "true" => {
            jv_free(input);
            Jv::jv_true()
        }
        "false" => {
            jv_free(input);
            Jv::jv_false()
        }
        "empty" => {
            jv_free(input);
            Jv::invalid()
        }
        "error" => {
            crate::builtin::f_error(jq, input)
        }
        "first" => {
            if input.get_kind() == JvKind::Array && jv_array_length(&input) > 0 {
                crate::jv::jv_array_get(input, 0)
            } else {
                jv_free(input);
                Jv::invalid()
            }
        }
        "last" => {
            if input.get_kind() == JvKind::Array {
                let len = jv_array_length(&input);
                if len > 0 {
                    crate::jv::jv_array_get(input, len - 1)
                } else {
                    jv_free(input);
                    Jv::invalid()
                }
            } else {
                jv_free(input);
                Jv::invalid()
            }
        }
        "nth" => {
            let n = if !args.is_empty() { args.remove(0) } else { Jv::number(0.0) };
            if input.get_kind() == JvKind::Array && n.get_kind() == JvKind::Number {
                let idx = n.number_value() as i32;
                jv_free(n);
                crate::jv::jv_array_get(input, idx)
            } else {
                jv_free(input);
                jv_free(n);
                Jv::invalid()
            }
        }
        "min" => crate::builtin::f_min(jq, input),
        "max" => crate::builtin::f_max(jq, input),
        "unique" => crate::builtin::f_unique(jq, input),
        "flatten" => {
            let depth = if !args.is_empty() { args.remove(0) } else { Jv::number(-1.0) };
            crate::builtin::f_flatten(jq, input, depth)
        }
        "getpath" => {
            let path = if !args.is_empty() { args.remove(0) } else { Jv::array() };
            jv_getpath(input, path)
        }
        "setpath" => {
            let path = if !args.is_empty() { args.remove(0) } else { Jv::array() };
            let value = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            jv_setpath(input, path, value)
        }
        "delpaths" => {
            let paths = if !args.is_empty() { args.remove(0) } else { Jv::array() };
            jv_delpaths(input, paths)
        }
        "group_by" | "_group_by_impl" => {
            let keys = if !args.is_empty() { args.remove(0) } else { Jv::array() };
            if input.get_kind() == JvKind::Array && keys.get_kind() == JvKind::Array {
                jv_group(input, keys)
            } else {
                crate::builtin::type_error2(input, keys, "cannot be grouped")
            }
        }
        "sort_by" | "_sort_by_impl" => {
            let keys = if !args.is_empty() { args.remove(0) } else { Jv::array() };
            if input.get_kind() == JvKind::Array && keys.get_kind() == JvKind::Array {
                jv_sort(input, keys)
            } else {
                crate::builtin::type_error2(input, keys, "cannot be sorted")
            }
        }
        "splits" | "split" => {
            let sep = if !args.is_empty() { args.remove(0) } else { Jv::string("") };
            crate::builtin::f_string_split(jq, input, sep)
        }
        "startswith" => {
            let s = if !args.is_empty() { args.remove(0) } else { Jv::string("") };
            crate::builtin::f_startswith(jq, input, s)
        }
        "endswith" => {
            let s = if !args.is_empty() { args.remove(0) } else { Jv::string("") };
            crate::builtin::f_endswith(jq, input, s)
        }
        "ltrimstr" => {
            let s = if !args.is_empty() { args.remove(0) } else { Jv::string("") };
            crate::builtin::f_ltrimstr(jq, input, s)
        }
        "rtrimstr" => {
            let s = if !args.is_empty() { args.remove(0) } else { Jv::string("") };
            crate::builtin::f_rtrimstr(jq, input, s)
        }
        "ascii_downcase" => crate::builtin::f_ascii_downcase(jq, input),
        "ascii_upcase" => crate::builtin::f_ascii_upcase(jq, input),
        "explode" => crate::builtin::f_string_explode(jq, input),
        "implode" => crate::builtin::f_string_implode(jq, input),
        "env" => crate::builtin::f_env(jq, input),
        "now" => {
            jv_free(input);
            crate::builtin::f_now(jq, Jv::null())
        }
        "debug" => crate::builtin::f_debug(jq, input),
        "input" => crate::builtin::f_input(jq, input),
        "inputs" => crate::builtin::f_input(jq, input), // TODO: proper inputs
        "infinite" => {
            jv_free(input);
            Jv::number(f64::INFINITY)
        }
        "nan" => {
            jv_free(input);
            Jv::number(f64::NAN)
        }
        "isinfinite" => {
            let result = if input.get_kind() == JvKind::Number {
                input.number_value().is_infinite()
            } else {
                false
            };
            jv_free(input);
            if result { Jv::jv_true() } else { Jv::jv_false() }
        }
        "isnan" => {
            let result = if input.get_kind() == JvKind::Number {
                input.number_value().is_nan()
            } else {
                false
            };
            jv_free(input);
            if result { Jv::jv_true() } else { Jv::jv_false() }
        }
        "isnormal" => {
            let result = if input.get_kind() == JvKind::Number {
                input.number_value().is_normal()
            } else {
                false
            };
            jv_free(input);
            if result { Jv::jv_true() } else { Jv::jv_false() }
        }
        "isfinite" => {
            let result = if input.get_kind() == JvKind::Number {
                input.number_value().is_finite()
            } else {
                false
            };
            jv_free(input);
            if result { Jv::jv_true() } else { Jv::jv_false() }
        }
        "values" => {
            // Filter out null values
            match input.get_kind() {
                JvKind::Null => {
                    jv_free(input);
                    Jv::invalid() // backtrack
                }
                _ => input
            }
        }
        "nulls" => {
            match input.get_kind() {
                JvKind::Null => input,
                _ => {
                    jv_free(input);
                    Jv::invalid()
                }
            }
        }
        "booleans" => {
            match input.get_kind() {
                JvKind::True | JvKind::False => input,
                _ => {
                    jv_free(input);
                    Jv::invalid()
                }
            }
        }
        "numbers" => {
            match input.get_kind() {
                JvKind::Number => input,
                _ => {
                    jv_free(input);
                    Jv::invalid()
                }
            }
        }
        "strings" => {
            match input.get_kind() {
                JvKind::String => input,
                _ => {
                    jv_free(input);
                    Jv::invalid()
                }
            }
        }
        "arrays" => {
            match input.get_kind() {
                JvKind::Array => input,
                _ => {
                    jv_free(input);
                    Jv::invalid()
                }
            }
        }
        "objects" => {
            match input.get_kind() {
                JvKind::Object => input,
                _ => {
                    jv_free(input);
                    Jv::invalid()
                }
            }
        }
        "iterables" => {
            match input.get_kind() {
                JvKind::Array | JvKind::Object => input,
                _ => {
                    jv_free(input);
                    Jv::invalid()
                }
            }
        }
        "scalars" => {
            match input.get_kind() {
                JvKind::Array | JvKind::Object => {
                    jv_free(input);
                    Jv::invalid()
                }
                _ => input
            }
        }
        "builtins" => {
            jv_free(input);
            // Return list of builtin names
            let mut arr = crate::jv::jv_array();
            for name in ["keys", "keys_unsorted", "length", "type", "has", "contains",
                         "sort", "reverse", "add", "floor", "ceil", "round", "sqrt",
                         "tonumber", "tostring", "tojson", "fromjson", "not", "null",
                         "true", "false", "empty", "error", "first", "last", "min", "max",
                         "unique", "flatten", "getpath", "setpath", "delpaths",
                         "split", "startswith", "endswith", "ltrimstr", "rtrimstr",
                         "ascii_downcase", "ascii_upcase", "explode", "implode",
                         "env", "now", "debug", "input", "infinite", "nan",
                         "isinfinite", "isnan", "isnormal", "isfinite",
                         "values", "nulls", "booleans", "numbers", "strings", "arrays", "objects",
                         "iterables", "scalars", "builtins"].iter() {
                arr = crate::jv::jv_array_append(arr, Jv::string(name));
            }
            arr
        }
        // Binary arithmetic operators
        // C: These are implemented as cfunctions with nargs=3 (input, a, b)
        "_plus" => {
            let a = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            let b = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            jv_free(input);
            crate::builtin::binop_plus(a, b)
        }
        "_minus" => {
            let a = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            let b = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            jv_free(input);
            crate::builtin::binop_minus(a, b)
        }
        "_multiply" => {
            let a = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            let b = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            jv_free(input);
            crate::builtin::binop_multiply(a, b)
        }
        "_divide" => {
            let a = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            let b = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            jv_free(input);
            crate::builtin::binop_divide(a, b)
        }
        "_mod" => {
            let a = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            let b = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            jv_free(input);
            crate::builtin::binop_mod(a, b)
        }
        "_equal" => {
            let a = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            let b = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            jv_free(input);
            crate::builtin::binop_equal(a, b)
        }
        "_notequal" => {
            let a = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            let b = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            jv_free(input);
            crate::builtin::binop_notequal(a, b)
        }
        "_less" => {
            let a = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            let b = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            jv_free(input);
            crate::builtin::binop_less(a, b)
        }
        "_greater" => {
            let a = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            let b = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            jv_free(input);
            crate::builtin::binop_greater(a, b)
        }
        "_lesseq" => {
            let a = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            let b = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            jv_free(input);
            crate::builtin::binop_lesseq(a, b)
        }
        "_greatereq" => {
            let a = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            let b = if !args.is_empty() { args.remove(0) } else { Jv::null() };
            jv_free(input);
            crate::builtin::binop_greatereq(a, b)
        }
        "_negate" => {
            // Unary negation - negates the input directly (nargs=1 means just input)
            if input.get_kind() == JvKind::Number {
                let ret = Jv::number(-input.number_value());
                jv_free(input);
                ret
            } else {
                crate::builtin::type_error(input, "cannot be negated")
            }
        }
        _ => {
            // Unknown builtin - return input unchanged (passthrough)
            // This handles user-defined functions that get called as builtins
            input
        }
    }
}

// ============================================================================
// JV HELPER FUNCTIONS
// ============================================================================

fn jv_null() -> Jv {
    crate::jv::jv_null()
}

fn jv_invalid() -> Jv {
    crate::jv::Jv::invalid()
}

fn jv_object() -> Jv {
    crate::jv::jv_object()
}

fn jv_string(s: &str) -> Jv {
    crate::jv::Jv::string(s)
}

fn jv_number(n: f64) -> Jv {
    crate::jv::jv_number(n)
}

fn jv_array() -> Jv {
    crate::jv::jv_array()
}

fn jv_array_length(v: &Jv) -> i32 {
    crate::jv::jv_array_length(v)
}

fn jv_array_append(arr: Jv, val: Jv) -> Jv {
    crate::jv::jv_array_append(arr, val)
}

fn jv_array_get(arr: Jv, idx: i32) -> Jv {
    crate::jv::jv_array_get(arr, idx)
}

/// Get element from bytecode constants array
fn jv_array_get_bc(arr: &Jv, idx: i32) -> Jv {
    crate::jv::jv_array_get(jv_copy(arr), idx)
}

fn jv_array_slice(arr: Jv, start: i32, end: i32) -> Jv {
    crate::jv::jv_array_slice(arr, start, end)
}

fn jv_object_get(obj: &Jv, key: Jv) -> Jv {
    crate::jv::jv_object_get(obj, key)
}

fn jv_object_set(obj: Jv, key: Jv, val: Jv) -> Jv {
    crate::jv::jv_object_set(obj, key, val)
}

fn jv_object_merge(a: Jv, b: Jv) -> Jv {
    crate::jv::jv_object_merge(a, b)
}

/// Get a value by key/index - implements jv_get from C
fn jv_get(container: Jv, key: Jv) -> Jv {
    match jv_get_kind(&container) {
        JvKind::Object => {
            if jv_get_kind(&key) == JvKind::String {
                let result = jv_object_get(&container, key);
                jv_free(container);
                result
            } else {
                jv_free(container);
                jv_free(key);
                jv_invalid()
            }
        }
        JvKind::Array => {
            if jv_get_kind(&key) == JvKind::Number {
                let idx = jv_number_value(&key) as i32;
                jv_free(key);
                let result = jv_array_get(container, idx);
                if jv_get_kind(&result) == JvKind::Invalid {
                    jv_invalid()
                } else {
                    result
                }
            } else {
                jv_free(container);
                jv_free(key);
                jv_invalid()
            }
        }
        _ => {
            jv_free(container);
            jv_free(key);
            jv_invalid()
        }
    }
}

/// Create an invalid jv with a message
fn jv_invalid_with_msg(msg: Jv) -> Jv {
    // Simplified - real implementation would store the message
    Jv {
        kind_flags: JvKind::Invalid as u8,
        pad_: 0,
        offset: 0,
        size: 0,
        u: 0,
    }
}

/// Get the message from an invalid jv
fn jv_invalid_get_msg(v: Jv) -> Jv {
    // Simplified - real implementation would retrieve stored message
    jv_null()
}

/// Make closure from bytecode pc
/// C: static struct closure make_closure(struct jq_state* jq, uint16_t* pc)
fn make_closure_from_pc<T>(jq: &JqState<T>, code: &[u16], pc_offset: &mut usize) -> Closure {
    let debug = std::env::var("DEBUG_EXEC").is_ok();
    // C: uint16_t level = *pc++;
    let level = code[*pc_offset] as i32;
    *pc_offset += 1;
    // C: uint16_t idx = *pc++;
    let idx = code[*pc_offset];
    *pc_offset += 1;
    if debug {
        eprintln!("make_closure_from_pc: level={} idx={} (0x{:x})", level, idx, idx);
    }

    // C: stack_ptr fridx = frame_get_level(jq, level);
    let fridx = frame_get_level(jq, level);
    // C: struct frame* fr = stack_block(&jq->stk, fridx);
    let fr_ptr = unsafe { jq.stk.mem_end.offset(fridx as isize) as *const Frame };
    let fr = unsafe { &*fr_ptr };

    // C: if (idx & ARG_NEWCLOSURE)
    if idx & ARG_NEWCLOSURE != 0 {
        // C: int subfn_idx = idx & ~ARG_NEWCLOSURE;
        let subfn_idx = (idx & !ARG_NEWCLOSURE) as usize;
        if debug {
            eprintln!("make_closure_from_pc: new closure, subfn_idx={}", subfn_idx);
        }
        // C: assert(subfn_idx < fr->bc->nsubfunctions);
        // C: struct closure cl = {fr->bc->subfunctions[subfn_idx], fridx};
        if let Some(ref bc) = fr.bc {
            if debug {
                eprintln!("make_closure_from_pc: bc.nsubfunctions={} bc.subfunctions.len={}", bc.nsubfunctions, bc.subfunctions.len());
            }
            if subfn_idx < bc.subfunctions.len() {
                return Closure {
                    bc: Some(bc.subfunctions[subfn_idx].clone()),
                    env: fridx,
                };
            }
        } else if debug {
            eprintln!("make_closure_from_pc: fr.bc is None");
        }
        Closure { bc: None, env: 0 }
    } else {
        // C: int closure = idx;
        // C: return fr->entries[closure].closure;
        let closure_idx = idx as usize;
        if closure_idx < fr.entries.len() {
            if let FrameEntry::Closure(ref cl) = fr.entries[closure_idx] {
                return cl.clone();
            }
        }
        Closure { bc: None, env: 0 }
    }
}
fn stack_block_get_jv(s: &Stack, ptr: StackPtr) -> Option<Jv> {
    if ptr <= 0 || s.mem_end.is_null() {
        return None;
    }
    // Return a null value as placeholder - actual implementation would read from stack memory
    Some(jv_null())
}
fn stack_block_set_jv(s: &mut Stack, ptr: StackPtr, val: Jv) {}
fn bytecode_operation_length_at(code: &[u16], offset: usize) -> usize {
    if offset >= code.len() {
        return 1;
    }
    2
}
/// JQ debug trace flag
pub const JQ_DEBUG_TRACE_ALL: i32 = 1;
/// Get mutable frame at a specific level
/// Similar to frame_get_level but returns mutable reference
pub fn frame_get_level_mut<T>(jq: &mut JqState<T>, level: i32) -> Option<&mut Frame> {
    if level < 0 {
        return None;
    }
    // Get frame stack pointer using same logic as frame_get_level
    let mut fr = jq.curr_frame;
    for _ in 0..level {
        let fp = unsafe { jq.stk.mem_end.offset(fr as isize) as *const Frame };
        let frame = unsafe { &*fp };
        fr = frame.env;
    }
    // Return mutable reference to frame at fr
    let frame_ptr = unsafe { jq.stk.mem_end.offset(fr as isize) as *mut Frame };
    unsafe { frame_ptr.as_mut() }
}
/// Push a new frame
pub fn frame_push<T>(jq: &mut JqState<T>, cl: Closure, _nargs: i32) -> StackPtr {
    let bc = cl.bc.clone();
    let env = cl.env;
    let frame_sz = if let Some(ref bc_ref) = bc {
        frame_size(bc_ref.as_ref())
    } else {
        std::mem::size_of::<Frame>()
    };
    let old_frame = jq.curr_frame;
    jq.curr_frame = stack_push_block(&mut jq.stk, jq.curr_frame, frame_sz);
    let block = stack_block(&mut jq.stk, jq.curr_frame);
    if !block.is_null() {
        let frame = unsafe { &mut *(block as *mut Frame) };
        frame.bc = bc;
        frame.env = env;
        frame.retdata = old_frame;
        frame.retaddr_offset = 0;
        if let Some(ref bc_ref) = frame.bc {
            let num_entries = (bc_ref.nclosures + bc_ref.nlocals) as usize;
            frame.entries = Vec::with_capacity(num_entries);
            for _ in 0..num_entries {
                frame.entries.push(FrameEntry::LocalVar(Jv::null()));
            }
        }
    }
    jq.curr_frame
}
/// Get stderr callback and data
pub fn jq_get_stderr_cb<T>(jq: &JqState<T>) -> (Option<JqMsgCb<T>>, Option<&T>) {
    (jq.stderr_cb, jq.stderr_cb_data.as_deref())
}
/// Get jq origin attribute
pub fn jq_get_jq_origin<T>(jq: &JqState<T>) -> Jv {
    jq_get_attr(jq, Jv::string("JQ_ORIGIN"))
}
/// Set the no-memory handler
pub fn jv_nomem_handler<T>(_handler: Option<JqNomemHandler<T>>, _data: Option<&T>) {}
/// Start jq execution with an input value
/// Works with both `&mut JqState<T>` and `&mut Option<Box<JqState<T>>>`
pub fn jq_start<T, J: JqStateAccess<T>>(jq: &mut J, input: Jv, flags: i32) {
    let jq_inner = match jq.get_jq_state() {
        Some(jq_state) => jq_state,
        None => return,
    };

    jq_reset(jq_inner);

    // Set debug trace flags
    jq_inner.debug_trace_enabled = flags;

    let top = Closure {
        bc: jq_inner.bc.clone(),
        env: -1,
    };

    // Push the initial frame
    let _frame_ptr = frame_push(jq_inner, top, 0);

    // Push the input value
    stack_push(jq_inner, input);

    // Save initial state for backtracking (pc offset 0 = start of bytecode)
    if jq_inner.bc.is_some() {
        let pos = stack_get_pos(jq_inner);
        stack_save(jq_inner, Some(0), pos);
    }

    // Mark this as the initial execution (not backtracking)
    jq_inner.initial_execution = true;
}
/// Teardown jq state and free resources
pub fn jq_teardown<T>(jq: &mut Option<Box<JqState<T>>>) {
    let old_jq = match jq.take() {
        Some(j) => j,
        None => return,
    };
    let mut old_jq = *old_jq;
    jq_reset(&mut old_jq);
    old_jq.bc = None;
    jv_free(old_jq.attrs);
}
/// Private fwrite implementation for tty-aware output
pub fn priv_fwrite<W: Write>(s: &[u8], fout: &mut W, _is_tty: bool) {
    let _ = fout.write_all(s);
}
/// Debug trace flags
pub const JQ_DEBUG_TRACE_DETAIL: i32 = 2;
/// Get a const reference to stack block
fn stack_block_const(s: &Stack, p: StackPtr) -> Option<*const u8> {
    if p <= 0 {
        return None;
    }
    // Stack uses mem_end pointer, compute offset
    if s.mem_end.is_null() {
        return None;
    }
    Some(unsafe { s.mem_end.offset(-(p as isize)) as *const u8 })
}

// Note: Stack, Frame, FrameEntry, and Closure types are defined in types.rs
// These impl blocks are removed to avoid conflicts with those definitions

/// Helper function to create a new JqState
fn create_jq_state<T>() -> JqState<T> {
    JqState {
        err_cb: None,
        err_cb_data: None,
        error_cb: None,
        error_cb_data: None,
        input_cb: None,
        input_cb_data: None,
        debug_cb: None,
        debug_cb_data: None,
        stderr_cb: None,
        stderr_cb_data: None,
        nomem_handler: None,
        nomem_handler_data: None,
        halted: false,
        exit_code: Jv::invalid(),
        error_message: Jv::invalid(),
        error: Jv::null(),
        bc: None,
        stk: Stack { mem_end: std::ptr::null_mut(), bound: ALIGNMENT as i32, limit: 0 },
        curr_frame: 0,
        stk_top: 0,
        fork_top: 0,
        path: Jv::invalid(),
        value_at_path: Jv::invalid(),
        subexp_nest: 0,
        debug_trace_enabled: 0,
        initial_execution: false,
        next_label: 0,
        attrs: Jv::invalid(),
        frames: Vec::new(),
        _phantom: std::marker::PhantomData,
    }
}
