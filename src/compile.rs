//! Module: compile
//!
//! Contains 84 transpiled functions:
//! - gen_function:15770830560644642917:./src/compile.c
//! - gen_op_unbound:12689198608725190964:./src/compile.c
//! - expand_call_arglist:8299073433263651435:./src/compile.c
//! - inst_block:14941879183898116289:./src/compile.c
//! - block_bind:6369352317476858403:./src/compile.c
//! - gen_try:10306184440755735993:./src/compile.c
//! - block_has_only_binders:6512264591220377869:./src/compile.c
//! - gen_error:4900797317882405536:./src/compile.c
//! - make_env:13162544385125938279:./src/compile.c
//! - gen_op_bound:16989292312276341534:./src/compile.c
//! - block_is_single:13584206563529267670:./src/compile.c
//! - block_compile:2774222247891135653:./src/compile.c
//! - block_take_imports:16316593046370622043:./src/compile.c
//! - gen_const_object:12430364926993445164:./src/compile.c
//! - gen_subexp:12252552778466157012:./src/compile.c
//! - block_mark_referenced:3388070595868749876:./src/compile.c
//! - block_get_unbound_vars:15540261773993203023:./src/compile.c
//! - gen_dictpair:617656683882977260:./src/compile.c
//! - gen_const_global:9508174081940745072:./src/compile.c
//! - nesting_level:7549708933280759385:./src/compile.c
//! - block_has_only_binders_and_imports:13591695882328181893:./src/compile.c
//! - gen_const:7152438802591446249:./src/compile.c
//! - block_join:1657253047175936669:./src/compile.c
//! - gen_const_array:5374950559355066492:./src/compile.c
//! - block_take:17514348921725574084:./src/compile.c
//! - gen_collect:9491006960698658330:./src/compile.c
//! - block_is_const:6041122288214742168:./src/compile.c
//! - gen_wildvar_binding:5979935852685963132:./src/compile.c
//! - gen_op_targetlater:13837680318522160919:./src/compile.c
//! - block_bind_subblock_inner:9931894544489267810:./src/compile.c
//! - block_count_actuals:761322909611365104:./src/compile.c
//! - inst_free:5815254911737140153:./src/compile.c
//! - gen_array_matcher:5026329949225736392:./src/compile.c
//! - gen_import:2678088091415267427:./src/compile.c
//! - gen_location:9910572531633905607:./src/compile.c
//! - block_list_funcs:17274916514054446697:./src/compile.c
//! - inst_new:1001158171169224866:./src/compile.c
//! - gen_noop:4432520510543284842:./src/compile.c
//! - inst_is_binder:14073250669767675226:./src/compile.c
//! - block_drop_unreferenced:16430253341906680862:./src/compile.c
//! - gen_import_meta:11064355544484932277:./src/compile.c
//! - block_is_noop:2457778412937784089:./src/compile.c
//! - gen_cbinding:2931537461215640969:./src/compile.c
//! - bind_matcher:11680519694114948305:./src/compile.c
//! - block_const_kind:8222538425397257410:./src/compile.c
//! - gen_destructure:3231316923019533527:./src/compile.c
//! - gen_module:14023265560859808942:./src/compile.c
//! - gen_var_binding:3125693778417095453:./src/compile.c
//! - block_bind_each:12828490085031471958:./src/compile.c
//! - block_bind_subblock:3874115998918039051:./src/compile.c
//! - block_is_funcdef:2599751628656261259:./src/compile.c
//! - gen_op_simple:4199444842969483484:./src/compile.c
//! - gen_param:8299582542071576613:./src/compile.c
//! - gen_both:3026040034991793084:./src/compile.c
//! - gen_call:11087105749328165625:./src/compile.c
//! - gen_destructure_alt:5823585134158500798:./src/compile.c
//! - block_module_meta:12053496709470416566:./src/compile.c
//! - gen_foreach:17657186446354168751:./src/compile.c
//! - block_take_last:5760801037458803721:./src/compile.c
//! - gen_label:7460005076524413621:./src/compile.c
//! - block_has_main:17682257798532086817:./src/compile.c
//! - gen_lambda:16405718835130366356:./src/compile.c
//! - block_bind_self:7242683726786302567:./src/compile.c
//! - bind_alternation_matchers:7177411194245512156:./src/compile.c
//! - gen_reduce:15795173649898728514:./src/compile.c
//! - gen_or:2321340866541518284:./src/compile.c
//! - inst_join:15165427579958610426:./src/compile.c
//! - gen_definedor:8634581243819335243:./src/compile.c
//! - gen_op_pushk_under:15406523952478080884:./src/compile.c
//! - block_free:9445400545857179516:./src/compile.c
//! - compile:7890493684884009041:./src/compile.c
//! - gen_object_matcher:5109443577987652022:./src/compile.c
//! - gen_condbranch:17123727241846176772:./src/compile.c
//! - gen_op_var_fresh:6047340946932327564:./src/compile.c
//! - gen_op_target:5933796885115739878:./src/compile.c
//! - inst_set_target:15948241514102350179:./src/compile.c
//! - gen_cond:18013315145700506248:./src/compile.c
//! - block_bind_referenced:10968548954273338752:./src/compile.c
//! - block_const:4711001174850245653:./src/compile.c
//! - block_append:1149291636794736549:./src/compile.c
//! - block_bind_library:2174792191771518565:./src/compile.c
//! - count_cfunctions:2760200080338759519:./src/compile.c
//! - gen_param_regular:1505997278419016416:./src/compile.c
//! - gen_and:13993577812804826075:./src/compile.c

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::ptr;
use crate::bytecode::{
    Bytecode, OpcodeDescription, Cfunction, SymbolTable,
    bytecode_free, bytecode_operation_length, dump_disassembly, dump_operation, getlevel,
    symbol_table_free,
};
// Note: opcode_describe is defined in this file
use crate::locfile::{locfile_free, locfile_retain};
// Note: UNKNOWN_LOCATION is defined locally in this file
// Note: locfile_locate is defined in this file
use crate::jv::{
    Jv, JvKind, jv_is_valid, jv_object_get, jv_object_iter, jv_object_iter_key,
    jv_object_iter_next, jv_object_iter_valid, jv_object_merge,
};
use crate::jv_aux::jv_keys_unsorted;
use crate::types::{
    Locfile, Location, CompileBlock, CompileInst,
    ImmediateValue, Op, InstBlock, Immediate,
};

/// Count instructions in a block for debug purposes
fn count_block_instructions(b: &Block) -> usize {
    let mut count = 0;
    let mut curr = b.first.as_ref().map(|b| b.as_ref() as *const Inst);
    while let Some(ptr) = curr {
        count += 1;
        unsafe {
            curr = (*ptr).next.as_ref().map(|b| b.as_ref() as *const Inst);
        }
    }
    count
}

/// Dump block instructions for debug purposes
fn dump_block(b: &Block, prefix: &str) {
    let mut curr = b.first.as_ref().map(|b| b.as_ref() as *const Inst);
    let mut idx = 0;
    while let Some(ptr) = curr {
        unsafe {
            let inst = &*ptr;
            let op_name = opcode_describe(inst.op as u16).name;
            eprintln!("{}: inst[{}] @ {:?} op={} ({}) symbol={:?} bound_by={:?}",
                     prefix, idx, ptr, inst.op, op_name, inst.symbol, inst.bound_by);
            idx += 1;
            curr = inst.next.as_ref().map(|b| b.as_ref() as *const Inst);
        }
    }
}

fn object_has_key(object: &Jv, key: &str) -> bool {
    let mut iter = jv_object_iter(object);
    while jv_object_iter_valid(object, iter) {
        let object_key = jv_object_iter_key(object, iter);
        if object_key.get_kind() == JvKind::String
            && crate::jv::jv_string_value(&object_key) == key
        {
            return true;
        }
        iter = jv_object_iter_next(object, iter);
    }
    false
}

/// Local OpcodeDesc for this module
#[derive(Debug, Clone)]
pub struct OpcodeDesc {
    pub op: u16,
    pub name: &'static str,
    pub flags: i32,
    pub length: i32,
}

/// CfuncRef for c function references
#[derive(Debug, Clone, Copy)]
pub struct CfuncRef {
    pub index: usize,
    pub nargs: i32,
}

/// Opcode values matching C opcode_list.h order exactly
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum CompileOpcode {
    LOADK = 0,
    DUP = 1,
    DUPN = 2,
    DUP2 = 3,
    PUSHK_UNDER = 4,
    POP = 5,
    LOADV = 6,
    LOADVN = 7,
    STOREV = 8,
    STORE_GLOBAL = 9,
    INDEX = 10,
    INDEX_OPT = 11,
    EACH = 12,
    EACH_OPT = 13,
    FORK = 14,
    TRY_BEGIN = 15,
    TRY_END = 16,
    JUMP = 17,
    JUMP_F = 18,
    BACKTRACK = 19,
    APPEND = 20,
    INSERT = 21,
    RANGE = 22,
    SUBEXP_BEGIN = 23,
    SUBEXP_END = 24,
    PATH_BEGIN = 25,
    PATH_END = 26,
    CALL_BUILTIN = 27,
    CALL_JQ = 28,
    RET = 29,
    TAIL_CALL_JQ = 30,
    CLOSURE_PARAM = 31,
    CLOSURE_REF = 32,
    CLOSURE_CREATE = 33,
    CLOSURE_CREATE_C = 34,
    TOP = 35,
    CLOSURE_PARAM_REGULAR = 36,
    DEPS = 37,
    MODULEMETA = 38,
    GENLABEL = 39,
    DESTRUCTURE_ALT = 40,
    STOREVN = 41,
    ERRORK = 42,
}

/// Local InstImmediate enum with all needed variants (shadows types::InstImmediate)
#[derive(Debug, Clone)]
pub enum InstImmediate {
    None,
    Constant(Jv),
    IntVal(i32),
    Target(Option<*mut Inst>),
    Cfunc(Box<CfuncRef>),
}

/// Type alias for Opcode
pub type Opcode = u16;

/// Local Inst struct for compilation (shadows types::Inst)
#[derive(Debug)]
pub struct Inst {
    pub next: Option<Box<Inst>>,
    pub prev: Option<*mut Inst>,
    pub op: Opcode,
    pub bytecode_pos: i32,
    pub bound_by: Option<*mut Inst>,
    pub symbol: Option<String>,
    pub any_unbound: i32,
    pub referenced: bool,
    pub nformals: i32,
    pub nactuals: i32,
    pub subfn: Block,
    pub arglist: Block,
    pub source: Location,
    pub locfile: Option<Box<Locfile>>,
    pub imm: InstImmediate,
    pub target: Option<*mut Inst>,
    pub compiled: Option<*mut Bytecode>,
}

impl Inst {
    /// Raw clone that copies structure without remapping pointers.
    /// Used internally by Block::clone to build complete tree before remapping.
    fn clone_raw(&self) -> Self {
        Self {
            next: self.next.as_ref().map(|n| Box::new(n.clone_raw())),
            prev: self.prev,
            op: self.op,
            bytecode_pos: self.bytecode_pos,
            bound_by: self.bound_by,
            symbol: self.symbol.clone(),
            any_unbound: self.any_unbound,
            referenced: self.referenced,
            nformals: self.nformals,
            nactuals: self.nactuals,
            subfn: self.subfn.clone_raw(),
            arglist: self.arglist.clone_raw(),
            source: self.source.clone(),
            locfile: None,
            imm: self.imm.clone(),
            target: self.target,
            compiled: self.compiled,
        }
    }
}

impl Clone for Inst {
    fn clone(&self) -> Self {
        // NOTE: This uses clone_raw. Block::clone handles remapping
        // after building a complete ptr_map of the entire tree.
        self.clone_raw()
    }
}

impl Default for Inst {
    fn default() -> Self {
        Self {
            next: None,
            prev: None,
            op: DUP,
            bytecode_pos: -1,
            bound_by: None,
            symbol: None,
            any_unbound: 0,
            referenced: false,
            nformals: -1,
            nactuals: -1,
            subfn: Block { first: None, last: None },
            arglist: Block { first: None, last: None },
            source: Location { start: -1, end: -1 },
            locfile: None,
            imm: InstImmediate::None,
            target: None,
            compiled: None,
        }
    }
}

///// Local Block struct for compilation (shadows types::Block)
#[derive(Debug, Default)]
pub struct Block {
    pub first: Option<Box<Inst>>,
    pub last: Option<*mut Inst>,
}

impl Block {
    /// Raw clone that copies structure without remapping pointers.
    /// Used internally by clone() and Inst::clone_raw().
    fn clone_raw(&self) -> Self {
        let first = self.first.as_ref().map(|f| Box::new(f.clone_raw()));
        let last = if first.is_some() {
            let mut curr = first.as_ref().map(|b| b.as_ref() as *const Inst as *mut Inst);
            let mut last_ptr = curr;
            while let Some(ptr) = curr {
                last_ptr = curr;
                unsafe {
                    curr = (*ptr).next.as_ref().map(|b| b.as_ref() as *const Inst as *mut Inst);
                }
            }
            last_ptr
        } else {
            None
        };
        Block { first, last }
    }
}

impl Clone for Block {
    fn clone(&self) -> Self {
        use std::collections::HashMap;

        // Phase 1: Raw clone - copies structure without remapping pointers
        let first = self.first.as_ref().map(|f| Box::new(f.clone_raw()));

        // Phase 2: Build COMPLETE ptr_map by recursively walking BOTH old and new trees
        // This includes the main list AND all nested subfn AND all nested arglist
        let mut ptr_map: HashMap<*const Inst, *mut Inst> = HashMap::new();

        fn collect_mappings(
            old_opt: &Option<Box<Inst>>,
            new_opt: &Option<Box<Inst>>,
            ptr_map: &mut HashMap<*const Inst, *mut Inst>,
            debug: bool,
        ) {
            let (Some(ref old_first), Some(ref new_first)) = (old_opt, new_opt) else {
                return;
            };
            let mut old_curr: Option<&Inst> = Some(old_first.as_ref());
            let mut new_curr: Option<&Inst> = Some(new_first.as_ref());

            while let (Some(old_inst), Some(new_inst)) = (old_curr, new_curr) {
                // Add this instruction to the map
                let old_ptr = old_inst as *const Inst;
                let new_ptr = new_inst as *const Inst as *mut Inst;
                ptr_map.insert(old_ptr, new_ptr);

                if debug && (old_inst.op == CLOSURE_CREATE || old_inst.op == CLOSURE_PARAM) {
                    eprintln!("  collect_mappings: op={} {:?} -> {:?} symbol={:?}",
                        old_inst.op, old_ptr, new_ptr, old_inst.symbol);
                }

                // Recurse into subfn
                collect_mappings(&old_inst.subfn.first, &new_inst.subfn.first, ptr_map, debug);

                // Recurse into arglist
                collect_mappings(&old_inst.arglist.first, &new_inst.arglist.first, ptr_map, debug);

                // Move to next instruction in list
                old_curr = old_inst.next.as_ref().map(|b| b.as_ref());
                new_curr = new_inst.next.as_ref().map(|b| b.as_ref());
            }
        }

        let debug = std::env::var("DEBUG_COMPILE").is_ok();
        if debug { eprintln!("Block::clone - collecting mappings..."); }
        collect_mappings(&self.first, &first, &mut ptr_map, debug);
        if debug { eprintln!("Block::clone - collected {} mappings", ptr_map.len()); }

        // Phase 3: Remap ALL pointers in the ENTIRE new tree (main + subfn + arglist)
        fn remap_pointers(new_opt: &Option<Box<Inst>>, ptr_map: &HashMap<*const Inst, *mut Inst>, debug: bool) {
            let Some(ref new_first) = new_opt else {
                return;
            };
            let mut curr: Option<*mut Inst> = Some(new_first.as_ref() as *const Inst as *mut Inst);

            while let Some(ptr) = curr {
                unsafe {
                    let inst = &mut *ptr;

                    // Remap bound_by pointer
                    if let Some(old_bound) = inst.bound_by {
                        // CLOSURE_CREATE and CLOSURE_PARAM must ALWAYS be self-bound
                        if inst.op == CLOSURE_CREATE || inst.op == CLOSURE_PARAM {
                            let sym = inst.symbol.as_deref().unwrap_or("");
                            if debug && sym == "recurse" {
                                eprintln!("  REMAP: recurse CLOSURE_CREATE at {:?} setting to self (was {:?})",
                                    ptr, old_bound);
                            }
                            inst.bound_by = Some(ptr);
                        } else if let Some(&new_bound) = ptr_map.get(&(old_bound as *const Inst)) {
                            if debug {
                                eprintln!("  REMAP: op={} symbol={:?} bound_by {:?} -> {:?}",
                                    inst.op, inst.symbol, old_bound, new_bound);
                            }
                            inst.bound_by = Some(new_bound);
                        } else {
                            // Not in map - this could be an external reference or a bug
                            if debug {
                                eprintln!("  REMAP WARNING: op={} symbol={:?} bound_by {:?} NOT IN MAP",
                                    inst.op, inst.symbol, old_bound);
                            }
                        }
                    }

                    // Remap prev pointer
                    if let Some(old_prev) = inst.prev {
                        if let Some(&new_prev) = ptr_map.get(&(old_prev as *const Inst)) {
                            inst.prev = Some(new_prev);
                        }
                    }

                    // Remap imm.target if it's a Target variant
                    if let InstImmediate::Target(Some(old_target)) = inst.imm {
                        if let Some(&new_target) = ptr_map.get(&(old_target as *const Inst)) {
                            inst.imm = InstImmediate::Target(Some(new_target));
                        }
                    }

                    // Recurse into subfn
                    remap_pointers(&inst.subfn.first, ptr_map, debug);

                    // Recurse into arglist
                    remap_pointers(&inst.arglist.first, ptr_map, debug);

                    curr = inst.next.as_ref().map(|b| b.as_ref() as *const Inst as *mut Inst);
                }
            }
        }

        remap_pointers(&first, &ptr_map, debug);

        // Debug: verify CLOSURE_CREATE self-bindings after remap
        if std::env::var("DEBUG_COMPILE").is_ok() {
            fn check_bindings(opt: &Option<Box<Inst>>, ptr_map: &HashMap<*const Inst, *mut Inst>) {
                let Some(ref first) = opt else { return };
                let mut curr = Some(first.as_ref());
                while let Some(inst) = curr {
                    if inst.op == 33 { // CLOSURE_CREATE
                        let self_ptr = inst as *const Inst;
                        if let Some(bound) = inst.bound_by {
                            if bound as *const Inst != self_ptr {
                                let bound_in_keys = ptr_map.contains_key(&(bound as *const Inst));
                                let mapped_to = ptr_map.get(&(bound as *const Inst));
                                eprintln!("CLONE DEBUG: CLOSURE_CREATE not self-bound!");
                                eprintln!("  self={:?} bound_by={:?} symbol={:?}", self_ptr, bound, inst.symbol);
                                eprintln!("  bound_by in ptr_map keys: {}, maps to: {:?}", bound_in_keys, mapped_to);
                            }
                        }
                    }
                    check_bindings(&inst.subfn.first, ptr_map);
                    check_bindings(&inst.arglist.first, ptr_map);
                    curr = inst.next.as_ref().map(|b| b.as_ref());
                }
            }
            check_bindings(&first, &ptr_map);
        }

        // Find the last instruction in the cloned list
        let last = if first.is_some() {
            let mut curr = first.as_ref().map(|b| b.as_ref() as *const Inst as *mut Inst);
            let mut last_ptr = curr;
            while let Some(ptr) = curr {
                last_ptr = curr;
                unsafe {
                    curr = (*ptr).next.as_ref().map(|b| b.as_ref() as *const Inst as *mut Inst);
                }
            }
            last_ptr
        } else {
            None
        };

        Block { first, last }
    }
}

// Note: Block, CompileBlock, CompileInst, CompileOpcode, InstImmediate, Op, Inst, and all
// block_*, gen_*, bind_*, count_* functions are defined in this file
/// Opcode flags - matching C jq bytecode.h values exactly
pub const OP_HAS_CONSTANT: i32 = 2;
pub const OP_HAS_VARIABLE: i32 = 4;
pub const OP_HAS_BRANCH: i32 = 8;
pub const OP_IS_CALL_PSEUDO: i32 = 16;
pub const OP_HAS_CFUNC: i32 = 32;
pub const OP_HAS_UFUNC: i32 = 64;
pub const OP_HAS_BINDING: i32 = 1024;
/// Create a new instruction with the given opcode
fn inst_new(op: Opcode) -> Box<Inst> {
    Box::new(Inst {
        next: None,
        prev: None,
        op,
        bytecode_pos: -1,
        bound_by: None,
        symbol: None,
        any_unbound: 0,
        referenced: false,
        nformals: -1,
        nactuals: -1,
        subfn: gen_noop(),
        arglist: gen_noop(),
        source: UNKNOWN_LOCATION,
        locfile: None,
        imm: InstImmediate::None,
        target: None,
        compiled: None,
    })
}
/// Create a block from a single instruction
pub fn inst_block(inst: Box<Inst>) -> Block {
    let raw_ptr = Box::into_raw(inst);
    Block {
        first: Some(unsafe { Box::from_raw(raw_ptr) }),
        last: Some(raw_ptr),
    }
}
/// Join two instructions together
fn inst_join(a: &mut Inst, b: &mut Inst) {
    assert!(a.next.is_none(), "a->next must be None");
    assert!(b.prev.is_none(), "b->prev must be None");
    let a_ptr = a as *mut Inst;
    a.next = Some(Box::new(std::mem::take(b)));
    if let Some(ref mut next) = a.next {
        next.prev = Some(a_ptr);
    }
}
/// Join two blocks together
pub fn block_join(mut a: Block, mut b: Block) -> Block {
    let debug_join = std::env::var("DEBUG_PARSER").is_ok();
    if debug_join {
        eprintln!("block_join: a.first={} a.last={} b.first={} b.last={}",
                 a.first.is_some(), a.last.is_some(), b.first.is_some(), b.last.is_some());
    }
    if a.first.is_none() {
        if debug_join { eprintln!("block_join: returning b (a.first is none)"); }
        return b;
    }
    if b.first.is_none() {
        if debug_join { eprintln!("block_join: returning a (b.first is none)"); }
        return a;
    }
    if let (Some(ref mut a_last_ptr), Some(ref mut b_first)) = (a.last, &mut b.first) {
        unsafe {
            if let Some(a_last) = a_last_ptr.as_mut() {
                if debug_join { eprintln!("block_join: linking a_last -> b_first"); }
                b_first.prev = Some(a_last);
                a_last.next = Some(std::mem::take(b_first));
            } else {
                if debug_join { eprintln!("block_join: a_last_ptr.as_mut() returned None!"); }
            }
        }
    } else {
        if debug_join { eprintln!("block_join: pattern match failed! a.last.is_some()={}", a.last.is_some()); }
    }
    Block {
        first: a.first,
        last: b.last,
    }
}
/// Check if a block is a no-op (empty)
pub fn block_is_noop(b: &Block) -> bool {
    b.first.is_none() && b.last.is_none()
}
/// Check if block is a constant
pub fn block_is_const(b: &Block) -> i32 {
    if block_is_single(b) {
        if let Some(ref first) = b.first {
            if first.op == LOADK || first.op == PUSHK_UNDER
            {
                return 1;
            }
        }
    }
    0
}
/// Get the constant value from a const block
pub fn block_const(b: &Block) -> Jv {
    assert!(block_is_const(b) != 0, "block_is_const(b)");
    if let Some(ref first) = b.first {
        if let InstImmediate::Constant(ref c) = first.imm {
            return c.clone();
        }
    }
    Jv::null()
}
/// Generate a no-op block (empty block)
pub fn gen_noop() -> Block {
    Block { first: None, last: None }
}
pub fn gen_const(v: Jv) -> Block {
    let mut i = inst_new(LOADK);
    i.imm = InstImmediate::Constant(v);
    inst_block(i)
}
/// Generate an error instruction with a constant
pub fn gen_error(constant: Jv) -> Block {
    let mut i = inst_new(ERRORK);
    i.imm = InstImmediate::Constant(constant);
    inst_block(i)
}
/// Generate a simple opcode instruction
pub fn gen_op_simple(op: Opcode) -> Block {
    let i = inst_new(op);
    inst_block(i)
}
/// Generate an unbound operation with a symbol name
pub fn gen_op_unbound(op: Opcode, name: &str) -> Block {
    let flags = get_opcode_flags_u16(op);
    assert!(flags & OP_HAS_BINDING != 0, "opcode_describe(op)->flags & OP_HAS_BINDING");
    let mut i = inst_new(op);
    i.symbol = Some(name.to_string());
    i.any_unbound = 1;
    inst_block(i)
}
/// Generate an op bound to a binder
pub fn gen_op_bound(op: Opcode, binder: &Block) -> Block {
    assert!(block_is_single(binder), "block_is_single(binder)");
    let binder_first = binder.first.as_ref().expect("binder must have first");
    let symbol = binder_first.symbol.as_ref().expect("symbol must exist");
    let mut b = gen_op_unbound(op, symbol);
    if let Some(ref mut first) = b.first {
        first.bound_by = Some(binder_first.as_ref() as *const Inst as *mut Inst);
        first.any_unbound = 0;
    }
    b
}
/// Generate an op bound to a binder via raw pointer
/// Used when the binder Block will be moved later but we need to bind to it now
fn gen_op_bound_to_ptr(op: Opcode, binder_ptr: Option<*mut Inst>) -> Block {
    let binder_ptr = binder_ptr.expect("binder_ptr must be Some");
    let symbol = unsafe { (*binder_ptr).symbol.as_ref().expect("symbol must exist").clone() };
    let mut b = gen_op_unbound(op, &symbol);
    if let Some(ref mut first) = b.first {
        first.bound_by = Some(binder_ptr);
        first.any_unbound = 0;
    }
    b
}
/// Generate a fresh variable operation
pub fn gen_op_var_fresh(op: Opcode, name: &str) -> Block {
    let flags = get_opcode_flags_u16(op);
    assert!(
        flags & OP_HAS_VARIABLE != 0, "opcode_describe(op)->flags & OP_HAS_VARIABLE"
    );
    let mut b = gen_op_unbound(op, name);
    if let Some(ref mut first) = b.first {
        let ptr: *mut Inst = first.as_mut();
        first.bound_by = Some(ptr);
    }
    b
}
/// Generate an instruction with a branch target
/// Note: Takes &Block to avoid consuming the target - caller must keep it alive
pub fn gen_op_target(op: Opcode, target: &Block) -> Block {
    let op_desc = opcode_describe(op);
    assert!(
        (op_desc.flags & OP_HAS_BRANCH) != 0, "opcode_describe(op).flags & OP_HAS_BRANCH"
    );
    assert!(target.last.is_some(), "target.last");
    let mut inst = inst_new(op);
    inst.imm = InstImmediate::Target(target.last);
    inst_block(inst)
}
/// Generate a function call
pub fn gen_call(name: &str, args: Block) -> Block {
    let mut b = gen_op_unbound(CALL_JQ, name);
    let nactuals = block_count_actuals(&args);
    if let Some(ref mut first) = b.first {
        first.arglist = args;
        first.nactuals = nactuals;
    }
    b
}
/// Generate a lambda expression
pub fn gen_lambda(body: Block) -> Block {
    // In C, this is: return gen_function("@lambda", gen_noop(), body);
    gen_function("@lambda", gen_noop(), body)
}
/// Generate a subexpression block
pub fn gen_subexp(a: Block) -> Block {
    if block_is_noop(&a) {
        return gen_op_simple(DUP);
    }
    if block_is_single(&a) {
        if let Some(ref first) = a.first {
            if first.op == LOADK {
                let c = block_const(&a);
                block_free(a);
                return gen_op_pushk_under(c);
            }
        }
    }
    let begin = gen_op_simple(SUBEXP_BEGIN);
    let end = gen_op_simple(SUBEXP_END);
    let joined1 = block_join(begin, a);
    block_join(joined1, end)
}
/// Bind a subblock within another block - matches C block_bind_subblock
fn block_bind_subblock(binder: &mut Block, body: &mut Block, bindflags: i32, break_distance: i32) -> i32 {
    // Matches C: block_bind_subblock calls block_bind_subblock_inner
    let mut any_unbound = 0i32;
    block_bind_subblock_inner(&mut any_unbound, binder, body, bindflags, break_distance)
}
/// Generate destructure
pub fn gen_destructure(var: Block, matchers: Block, body: Block) -> Block {
    let mut top = gen_noop();
    let mut body = body;
    if let Some(ref first) = body.first {
        if first.op == TOP {
            if let Some(inst) = block_take(&mut body) {
                top = inst_block(Box::new(inst));
            }
        }
    }
    let mut var = var;
    if let Some(ref first) = matchers.first {
        if first.op == DESTRUCTURE_ALT {
            block_append(&mut var, gen_op_simple(DUP));
        } else {
            top = block_join(top, gen_op_simple(DUP));
        }
    } else {
        top = block_join(top, gen_op_simple(DUP));
    }
    block_join(
        block_join(block_join(top, gen_subexp(var)), gen_op_simple(POP)),
        bind_alternation_matchers(matchers, body),
    )
}
/// Generate variable binding
pub fn gen_var_binding(var: Block, name: &str, body: Block) -> Block {
    gen_destructure(var, gen_op_unbound(STOREV, name), body)
}
/// Generate a wildcard variable binding
pub fn gen_wildvar_binding(var: Block, name: &str, body: Block) -> Block {
    let dup = gen_op_simple_internal(DUP);
    let joined1 = block_join_internal(dup, var);
    let storev = gen_op_unbound_internal(STOREV, name);
    let bound_body = block_bind_internal(
        storev,
        body,
        OP_HAS_VARIABLE | OP_BIND_WILDCARD,
    );
    block_join_internal(joined1, bound_body)
}
/// Generate a conditional expression
pub fn gen_cond(cond: Block, iftrue: Block, iffalse: Block) -> Block {
    let dup = gen_op_simple(DUP);
    let subexp = gen_subexp(cond);
    let pop = gen_op_simple(POP);
    let cond_part = block_join(subexp, pop);
    let cond_with_dup = block_join(dup, cond_part);
    let true_branch = block_join(gen_op_simple(POP), iftrue);
    let false_branch = block_join(gen_op_simple(POP), iffalse);
    let branches = gen_condbranch(true_branch, false_branch);
    block_join(cond_with_dup, branches)
}
/// Generate a try-catch expression
pub fn gen_try(exp: Block, mut handler: Block) -> Block {
    if block_is_noop(&handler) {
        let dup = gen_op_simple(DUP);
        let pop = gen_op_simple(POP);
        handler = block_join(dup, pop);
    }
    let jump = gen_op_target(JUMP, &handler);
    let try_begin = gen_op_target(TRY_BEGIN, &jump);
    let try_end = gen_op_simple(TRY_END);
    let try_body = block_join(try_begin, exp);
    let try_with_end = block_join(try_body, try_end);
    let try_with_jump = block_join(try_with_end, jump);
    block_join(try_with_jump, handler)
}
/// Generate a function definition
pub fn gen_function(name: &str, mut formals: Block, mut body: Block) -> Block {
    let mut i = inst_new(CLOSURE_CREATE);
    let mut nformals = 0;
    let mut current = formals.last;
    while let Some(curr_ptr) = current {
        unsafe {
            let curr = &mut *curr_ptr;
            nformals += 1;
            curr.nformals = 0;
            if curr.op == CLOSURE_PARAM_REGULAR {
                curr.op = CLOSURE_PARAM;
                if let Some(ref symbol) = curr.symbol.clone() {
                    body = gen_var_binding(gen_call(symbol, gen_noop()), symbol, body);
                }
            }
            let mut formal_block = Block {
                first: None,
                last: Some(curr_ptr),
            };
            block_bind_subblock(
                &mut formal_block,
                &mut body,
                OP_IS_CALL_PSEUDO | OP_HAS_BINDING,
                0,
            );
            current = curr.prev;
        }
    }
    i.subfn = body;
    i.symbol = Some(name.to_string());
    i.any_unbound = -1;
    i.nformals = nformals;
    i.arglist = formals;
    let mut b = inst_block(i);

    // Bind the CLOSURE_CREATE to itself and bind references in subfn/arglist.
    // We use a raw pointer approach to avoid cloning (which would create dangling pointers
    // when the clone is dropped).
    if let Some(ref mut first) = b.first {
        let binder_ptr = first.as_mut() as *mut Inst;
        let binder_symbol = first.symbol.clone().unwrap_or_default();
        let binder_nformals = first.nformals;

        // Set self-binding: the CLOSURE_CREATE binds to itself
        first.bound_by = Some(binder_ptr);

        if std::env::var("DEBUG_COMPILE").is_ok() {
            eprintln!("gen_function({:?}): set bound_by={:?} on instruction at {:?}",
                name, binder_ptr, binder_ptr);
        }

        // Bind unbound references in subfn and arglist to this binder
        let bindflags = OP_IS_CALL_PSEUDO | OP_HAS_BINDING;
        block_bind_to_binder_ptr(
            &mut first.subfn,
            binder_ptr,
            &binder_symbol,
            binder_nformals,
            bindflags,
        );
        block_bind_to_binder_ptr(
            &mut first.arglist,
            binder_ptr,
            &binder_symbol,
            binder_nformals,
            bindflags,
        );
    }
    b
}
/// Generate a label expression
pub fn gen_label(label: &str, exp: Block) -> Block {
    let cond = gen_call(
        "_equal",
        block_join(gen_lambda(gen_noop()), gen_lambda(gen_op_unbound(LOADV, label))),
    );
    // Use STOREV as the variable binding operation for labels
    // GENLABEL is not a standard opcode - use gen_var_binding pattern instead
    gen_var_binding(
        gen_op_simple(DUP),
        label,
        block_join(
            gen_op_simple(POP),
            gen_try(
                exp,
                gen_cond(
                    cond,
                    gen_op_simple(BACKTRACK),
                    gen_call("error", gen_noop()),
                ),
            ),
        ),
    )
}
/// Generate a reduce expression
pub fn gen_reduce(source: Block, matcher: Block, init: Block, body: Block) -> Block {
    let res_var = gen_op_var_fresh(STOREV, "reduce");
    let inner_body = block_join(
        block_join(gen_op_bound(LOADVN, &res_var), body),
        gen_op_bound(STOREV, &res_var),
    );
    let loop_block = block_join(
        block_join(
            block_join(gen_op_simple(DUPN), source),
            bind_alternation_matchers(matcher, inner_body),
        ),
        gen_op_simple(BACKTRACK),
    );
    block_join(
        block_join(
            block_join(
                block_join(block_join(gen_op_simple(DUP), init), res_var.clone()),
                gen_op_target(FORK, &loop_block),
            ),
            loop_block,
        ),
        gen_op_bound(LOADVN, &res_var),
    )
}
/// Generate an array matcher
pub fn gen_array_matcher(left: Block, curr: Block) -> Block {
    let index: i32;
    if block_is_noop(&left) {
        index = 0;
    } else {
        if let Some(ref first) = left.first {
            assert!(first.op == DUP, "left.first->op == DUP");
            assert!(first.next.is_some(), "left.first->next != NULL");
            if let Some(ref next) = first.next {
                let i = if next.op == PUSHK_UNDER {
                    next
                } else {
                    assert!(
                        next.op == SUBEXP_BEGIN,
                        "left.first->next->op == SUBEXP_BEGIN"
                    );
                    if let Some(ref next_next) = next.next {
                        assert!(
                            next_next.op == LOADK,
                            "left.first->next->next->op == LOADK"
                        );
                        next_next
                    } else {
                        panic!("Expected LOADK instruction");
                    }
                };
                if let InstImmediate::Constant(ref c) = i.imm {
                    index = 1 + jv_number_value(c) as i32;
                } else {
                    index = 1;
                }
            } else {
                index = 1;
            }
        } else {
            index = 0;
        }
    }
    block_join(
        block_join(
            block_join(
                block_join(
                    gen_op_simple(DUP),
                    gen_subexp(gen_const(jv_number(index as f64))),
                ),
                gen_op_simple(INDEX),
            ),
            curr,
        ),
        left,
    )
}
/// Calculate nesting level between bytecode and target instruction
fn nesting_level(_bc: &mut Bytecode, _target: *mut Inst) -> u16 {
    // The Inst struct doesn't have a 'compiled' field in this implementation.
    // This function calculates the nesting level by traversing the bytecode parent chain.
    // Since Bytecode.parent is Box<Bytecode>, not Rc<RefCell<>>, we'd need to track depth differently.
    // For now, return 0 as a placeholder.
    0
}
fn jv_number(n: f64) -> Jv {
    Jv::number(n)
}
fn jv_number_value(jv: &Jv) -> f64 {
    jv.number_value()
}
fn jv_copy(jv: &Jv) -> Jv {
    jv.clone()
}
/// Bind alternation matchers to a body
pub fn bind_alternation_matchers(
    matchers: Block,
    body: Block,
) -> Block {
    let mut preamble = gen_noop();
    let mut altmatchers = gen_noop();
    let mut mb = gen_noop();
    let mut final_matcher = matchers;
    while let Some(ref first) = final_matcher.first {
        if first.op == DESTRUCTURE_ALT {
            if let Some(taken) = block_take(&mut final_matcher) {
                block_append(&mut altmatchers, inst_block(Box::new(taken)));
            } else {
                break;
            }
        } else {
            break;
        }
    }
    if altmatchers.first.is_none() {
        return bind_matcher_local(final_matcher, body);
    }
    let mut all_vars = jv_object();
    block_get_unbound_vars_local(&altmatchers, &mut all_vars);
    block_get_unbound_vars_local(&final_matcher, &mut all_vars);
    for key in jv_object_keys(&all_vars) {
        let key_str = jv_string_value(&key);
        let dup = gen_op_simple(DUP);
        let null_const = gen_const(jv_null());
        let store = gen_op_unbound(STOREV, &key_str);
        preamble = block_join(preamble, dup);
        preamble = block_join(preamble, null_const);
        preamble = block_join(preamble, store);
        jv_free(key);
    }
    jv_free(all_vars);
    let mut current = altmatchers.first.as_ref();
    while let Some(inst) = current {
        let submatcher = inst.subfn.clone();
        let jump = gen_op_target(JUMP, &final_matcher);
        let submatcher = block_join(submatcher, jump);
        let destruct_alt = gen_op_target(DESTRUCTURE_ALT, &submatcher);
        mb = block_join(mb, destruct_alt);
        mb = block_join(mb, submatcher);
        current = inst.next.as_ref();
    }
    block_free(altmatchers);
    let combined = block_join(mb, final_matcher);
    let combined = block_join(combined, body);
    bind_matcher_local(preamble, combined)
}

/// Bind a matcher block to a body (local version using Block type)
/// This matches C's bind_matcher: for each STOREV/STOREVN that's unbound,
/// bind it to the body with OP_HAS_VARIABLE
fn bind_matcher_local(mut matcher: Block, mut body: Block) -> Block {
    // Iterate through matcher looking for unbound STOREV/STOREVN
    let mut current_ptr: Option<*mut Inst> = matcher.first.as_mut().map(|b| b.as_mut() as *mut Inst);
    while let Some(ptr) = current_ptr {
        let inst = unsafe { &mut *ptr };
        if (inst.op == STOREV || inst.op == STOREVN) && inst.bound_by.is_none() {
            // Create a reference-only binder block (first: None, last: Some(ptr))
            // This avoids ownership issues while allowing binding
            let mut binder = Block {
                first: None,
                last: Some(ptr),
            };
            block_bind_subblock(&mut binder, &mut body, OP_HAS_VARIABLE, 0);
        }
        current_ptr = inst.next.as_mut().map(|b| b.as_mut() as *mut Inst);
    }
    block_join(matcher, body)
}

/// Get unbound variables from a block (local version using Block type)
fn block_get_unbound_vars_local(_b: &Block, _vars: &mut Jv) {
    // Stub implementation - iterate through block and collect unbound variable names
}

pub const UNKNOWN_LOCATION: Location = Location { start: -1, end: -1 };
/// Free an instruction and its resources
fn inst_free(mut i: Box<Inst>) {
    block_free(std::mem::take(&mut i.subfn));
    block_free(std::mem::take(&mut i.arglist));
    // Drop the locfile Box - Rust handles cleanup automatically
    i.locfile.take();
    if let InstImmediate::Constant(constant) = std::mem::take(&mut i.imm) {
        constant.free();
    }
}
/// Check if instruction is a binder with given flags
fn inst_is_binder(i: &Inst, bindflags: i32) -> bool {
    let op_flags = get_opcode_flags_u16(i.op);
    !((op_flags & bindflags) != bindflags && i.op != MODULEMETA)
}
/// Get opcode flags (placeholder - should integrate with bytecode module)
fn get_opcode_flags(op: CompileOpcode) -> i32 {
    match op {
        CompileOpcode::CLOSURE_PARAM => OP_HAS_BINDING | OP_HAS_VARIABLE,
        CompileOpcode::CLOSURE_CREATE => OP_IS_CALL_PSEUDO | OP_HAS_BINDING,
        CompileOpcode::CLOSURE_CREATE_C => OP_IS_CALL_PSEUDO | OP_HAS_BINDING,
        CompileOpcode::JUMP => OP_HAS_BRANCH,
        CompileOpcode::JUMP_F => OP_HAS_BRANCH,
        CompileOpcode::TRY_BEGIN => OP_HAS_BRANCH,
        CompileOpcode::FORK => OP_HAS_BRANCH,
        CompileOpcode::DESTRUCTURE_ALT => OP_HAS_BRANCH,
        CompileOpcode::CALL_JQ => OP_HAS_UFUNC | OP_HAS_BINDING | OP_IS_CALL_PSEUDO,
        _ => 0,
    }
}
/// Get opcode flags from u16 opcode
fn get_opcode_flags_u16(op: Opcode) -> i32 {
    match op {
        CLOSURE_PARAM | CLOSURE_PARAM_REGULAR => OP_IS_CALL_PSEUDO | OP_HAS_BINDING,
        CLOSURE_CREATE => OP_IS_CALL_PSEUDO | OP_HAS_BINDING,
        CLOSURE_CREATE_C => OP_IS_CALL_PSEUDO | OP_HAS_BINDING,
        CLOSURE_REF => OP_IS_CALL_PSEUDO | OP_HAS_BINDING,
        JUMP => OP_HAS_BRANCH,
        JUMP_F => OP_HAS_BRANCH,
        TRY_BEGIN => OP_HAS_BRANCH,
        FORK => OP_HAS_BRANCH,
        DESTRUCTURE_ALT => OP_HAS_BRANCH,
        LOADV | LOADVN | STOREV | STOREVN => OP_HAS_BINDING | OP_HAS_VARIABLE,
        APPEND => OP_HAS_BINDING,
        CALL_JQ => OP_HAS_UFUNC | OP_HAS_BINDING | OP_IS_CALL_PSEUDO,
        _ => 0,
    }
}
/// Generate a parameter binding
pub fn gen_param(name: &str) -> Block {
    gen_op_unbound(CLOSURE_PARAM, name)
}
/// Generate conditional branch
pub fn gen_condbranch(iftrue: Block, iffalse: Block) -> Block {
    // Matches C: iftrue = BLOCK(iftrue, gen_op_target(JUMP, iffalse));
    //            return BLOCK(gen_op_target(JUMP_F, iftrue), iftrue, iffalse);
    //
    // The JUMP at end of iftrue skips over iffalse (jumps to end of iffalse)
    // The JUMP_F at start skips over iftrue if condition is false (jumps to end of iftrue)
    //
    // IMPORTANT: Don't use .clone() - it creates new instructions and the target
    // pointers would point to the cloned (then dropped) instructions, causing
    // invalid bytecode_pos during compilation.

    // Step 1: Get pointer to iffalse's last instruction (before consuming iffalse)
    let iffalse_last = iffalse.last;

    // Step 2: Create JUMP instruction targeting end of iffalse
    let mut jump_inst = inst_new(JUMP);
    jump_inst.imm = InstImmediate::Target(iffalse_last);
    let jump_block = inst_block(jump_inst);

    // Step 3: Join iftrue with the JUMP
    let iftrue_with_jump = block_join(iftrue, jump_block);

    // Step 4: Get pointer to end of iftrue (after the JUMP was appended)
    let iftrue_last = iftrue_with_jump.last;

    // Step 5: Create JUMP_F instruction targeting end of iftrue
    let mut jumpf_inst = inst_new(JUMP_F);
    jumpf_inst.imm = InstImmediate::Target(iftrue_last);
    let jumpf_block = inst_block(jumpf_inst);

    // Step 6: Join everything: [JUMP_F] [iftrue] [JUMP] [iffalse]
    block_join(jumpf_block, block_join(iftrue_with_jump, iffalse))
}
/// Free a block and all its instructions
pub fn block_free(mut b: Block) {
    let mut current = b.first.take();
    while let Some(inst) = current {
        let next = inst.next.clone();
        inst_free(inst);
        current = next;
    }
}
/// Count C functions in a block (recursive)
pub fn count_cfunctions(b: &Block) -> i32 {
    let mut n = 0;
    let mut current = &b.first;
    while let Some(ref inst) = current {
        if inst.op == CLOSURE_CREATE_C {
            n += 1;
        }
        n += count_cfunctions(&inst.subfn);
        current = &inst.next;
    }
    n
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gen_noop() {
        let b = gen_noop();
        assert!(block_is_noop(& b));
    }
    #[test]
    fn test_gen_op_simple() {
        let b = gen_op_simple(DUP);
        assert!(! block_is_noop(& b));
    }
    #[test]
    fn test_block_join() {
        let a = gen_op_simple(DUP);
        let b = gen_op_simple(POP);
        let joined = block_join(a, b);
        assert!(! block_is_noop(& joined));
    }
    #[test]
    fn test_count_cfunctions_empty() {
        let b = gen_noop();
        assert_eq!(count_cfunctions(& b), 0);
    }
}
pub const LOADV: Opcode = CompileOpcode::LOADV as u16;
/// Opcode constants (u16 values matching CompileOpcode discriminants)
pub const DUP: Opcode = CompileOpcode::DUP as u16;
pub const POP: Opcode = CompileOpcode::POP as u16;
pub const FORK: Opcode = CompileOpcode::FORK as u16;
pub const JUMP: Opcode = CompileOpcode::JUMP as u16;
pub const JUMP_F: Opcode = CompileOpcode::JUMP_F as u16;
pub const BACKTRACK: Opcode = CompileOpcode::BACKTRACK as u16;
pub const APPEND: Opcode = CompileOpcode::APPEND as u16;
pub const CALL_JQ: Opcode = CompileOpcode::CALL_JQ as u16;
pub const CALL_JQ_CONST: Opcode = CompileOpcode::CALL_JQ as u16;
pub const CALL_BUILTIN: Opcode = CompileOpcode::CALL_BUILTIN as u16;
pub const CLOSURE_CREATE: Opcode = CompileOpcode::CLOSURE_CREATE as u16;
pub const CLOSURE_PARAM: Opcode = CompileOpcode::CLOSURE_PARAM as u16;
pub const CLOSURE_PARAM_CONST: Opcode = CompileOpcode::CLOSURE_PARAM as u16;
pub const CLOSURE_REF: Opcode = CompileOpcode::CLOSURE_REF as u16;
pub const CLOSURE_CREATE_C: Opcode = CompileOpcode::CLOSURE_CREATE_C as u16;
pub const STORE_GLOBAL: Opcode = CompileOpcode::STORE_GLOBAL as u16;
pub const RET: Opcode = CompileOpcode::RET as u16;
pub const DESTRUCTURE_ALT: Opcode = CompileOpcode::DESTRUCTURE_ALT as u16;
pub const LOADVN: Opcode = CompileOpcode::LOADVN as u16;
pub const LOADK: Opcode = CompileOpcode::LOADK as u16;
pub const ERRORK: Opcode = CompileOpcode::ERRORK as u16;
pub const SUBEXP_BEGIN: Opcode = CompileOpcode::SUBEXP_BEGIN as u16;
pub const SUBEXP_END: Opcode = CompileOpcode::SUBEXP_END as u16;
pub const TRY_BEGIN: Opcode = CompileOpcode::TRY_BEGIN as u16;
pub const TRY_END: Opcode = CompileOpcode::TRY_END as u16;
pub const TOP: Opcode = CompileOpcode::TOP as u16;
pub const DEPS: Opcode = CompileOpcode::DEPS as u16;
pub const STOREV: Opcode = CompileOpcode::STOREV as u16;
pub const STOREVN: Opcode = CompileOpcode::STOREVN as u16;
pub const DUPN: Opcode = CompileOpcode::DUPN as u16;
pub const CLOSURE_PARAM_REGULAR: Opcode = CompileOpcode::CLOSURE_PARAM_REGULAR as u16;
pub const INSERT: Opcode = CompileOpcode::INSERT as u16;
pub const DUP2: Opcode = CompileOpcode::DUP2 as u16;
pub const EACH: Opcode = CompileOpcode::EACH as u16;
pub const EACH_OPT: Opcode = CompileOpcode::EACH_OPT as u16;
pub const INDEX: Opcode = CompileOpcode::INDEX as u16;
pub const INDEX_OPT: Opcode = CompileOpcode::INDEX_OPT as u16;
pub const RANGE: Opcode = CompileOpcode::RANGE as u16;
pub const PATH_BEGIN: Opcode = CompileOpcode::PATH_BEGIN as u16;
pub const PATH_END: Opcode = CompileOpcode::PATH_END as u16;
pub const GENLABEL: Opcode = CompileOpcode::GENLABEL as u16;
pub const MODULEMETA: Opcode = CompileOpcode::MODULEMETA as u16;
pub const PUSHK_UNDER: Opcode = CompileOpcode::PUSHK_UNDER as u16;
pub const TAIL_CALL_JQ: Opcode = CompileOpcode::TAIL_CALL_JQ as u16;
/// Get opcode description - matches C bytecode.c exactly
/// Types from C: NONE=0,1  CONSTANT=OP_HAS_CONSTANT,2  VARIABLE=(OP_HAS_VARIABLE|OP_HAS_BINDING),3
///               GLOBAL=(OP_HAS_CONSTANT|OP_HAS_VARIABLE|OP_HAS_BINDING|OP_IS_CALL_PSEUDO),4
///               BRANCH=OP_HAS_BRANCH,2  CFUNC=(OP_HAS_CFUNC|OP_HAS_BINDING),3
///               UFUNC=(OP_HAS_UFUNC|OP_HAS_BINDING|OP_IS_CALL_PSEUDO),4
///               DEFINITION=(OP_IS_CALL_PSEUDO|OP_HAS_BINDING),0  CLOSURE_REF_IMM=same,2
pub fn opcode_describe(op: Opcode) -> OpcodeDesc {
    match op {
        // CONSTANT type: OP_HAS_CONSTANT, length 2
        LOADK => OpcodeDesc { op, name: "LOADK", flags: OP_HAS_CONSTANT, length: 2 },
        PUSHK_UNDER => OpcodeDesc { op, name: "PUSHK_UNDER", flags: OP_HAS_CONSTANT, length: 2 },
        DEPS => OpcodeDesc { op, name: "DEPS", flags: OP_HAS_CONSTANT, length: 2 },
        MODULEMETA => OpcodeDesc { op, name: "MODULEMETA", flags: OP_HAS_CONSTANT, length: 2 },
        ERRORK => OpcodeDesc { op, name: "ERRORK", flags: OP_HAS_CONSTANT, length: 2 },

        // VARIABLE type: OP_HAS_VARIABLE | OP_HAS_BINDING, length 3
        LOADV => OpcodeDesc { op, name: "LOADV", flags: OP_HAS_VARIABLE | OP_HAS_BINDING, length: 3 },
        LOADVN => OpcodeDesc { op, name: "LOADVN", flags: OP_HAS_VARIABLE | OP_HAS_BINDING, length: 3 },
        STOREV => OpcodeDesc { op, name: "STOREV", flags: OP_HAS_VARIABLE | OP_HAS_BINDING, length: 3 },
        STOREVN => OpcodeDesc { op, name: "STOREVN", flags: OP_HAS_VARIABLE | OP_HAS_BINDING, length: 3 },
        APPEND => OpcodeDesc { op, name: "APPEND", flags: OP_HAS_VARIABLE | OP_HAS_BINDING, length: 3 },
        RANGE => OpcodeDesc { op, name: "RANGE", flags: OP_HAS_VARIABLE | OP_HAS_BINDING, length: 3 },

        // GLOBAL type: OP_HAS_CONSTANT | OP_HAS_VARIABLE | OP_HAS_BINDING | OP_IS_CALL_PSEUDO, length 4
        STORE_GLOBAL => OpcodeDesc { op, name: "STORE_GLOBAL", flags: OP_HAS_CONSTANT | OP_HAS_VARIABLE | OP_HAS_BINDING | OP_IS_CALL_PSEUDO, length: 4 },

        // BRANCH type: OP_HAS_BRANCH, length 2
        FORK => OpcodeDesc { op, name: "FORK", flags: OP_HAS_BRANCH, length: 2 },
        TRY_BEGIN => OpcodeDesc { op, name: "TRY_BEGIN", flags: OP_HAS_BRANCH, length: 2 },
        JUMP => OpcodeDesc { op, name: "JUMP", flags: OP_HAS_BRANCH, length: 2 },
        JUMP_F => OpcodeDesc { op, name: "JUMP_F", flags: OP_HAS_BRANCH, length: 2 },
        DESTRUCTURE_ALT => OpcodeDesc { op, name: "DESTRUCTURE_ALT", flags: OP_HAS_BRANCH, length: 2 },

        // CFUNC type: OP_HAS_CFUNC | OP_HAS_BINDING, length 3
        CALL_BUILTIN => OpcodeDesc { op, name: "CALL_BUILTIN", flags: OP_HAS_CFUNC | OP_HAS_BINDING, length: 3 },

        // UFUNC type: OP_HAS_UFUNC | OP_HAS_BINDING | OP_IS_CALL_PSEUDO, length 4
        CALL_JQ => OpcodeDesc { op, name: "CALL_JQ", flags: OP_HAS_UFUNC | OP_HAS_BINDING | OP_IS_CALL_PSEUDO, length: 4 },
        TAIL_CALL_JQ => OpcodeDesc { op, name: "TAIL_CALL_JQ", flags: OP_HAS_UFUNC | OP_HAS_BINDING | OP_IS_CALL_PSEUDO, length: 4 },

        // DEFINITION type: OP_IS_CALL_PSEUDO | OP_HAS_BINDING, length 0
        CLOSURE_PARAM => OpcodeDesc { op, name: "CLOSURE_PARAM", flags: OP_IS_CALL_PSEUDO | OP_HAS_BINDING, length: 0 },
        CLOSURE_CREATE => OpcodeDesc { op, name: "CLOSURE_CREATE", flags: OP_IS_CALL_PSEUDO | OP_HAS_BINDING, length: 0 },
        CLOSURE_CREATE_C => OpcodeDesc { op, name: "CLOSURE_CREATE_C", flags: OP_IS_CALL_PSEUDO | OP_HAS_BINDING, length: 0 },
        CLOSURE_PARAM_REGULAR => OpcodeDesc { op, name: "CLOSURE_PARAM_REGULAR", flags: OP_IS_CALL_PSEUDO | OP_HAS_BINDING, length: 0 },

        // CLOSURE_REF_IMM type: OP_IS_CALL_PSEUDO | OP_HAS_BINDING, length 2
        CLOSURE_REF => OpcodeDesc { op, name: "CLOSURE_REF", flags: OP_IS_CALL_PSEUDO | OP_HAS_BINDING, length: 2 },

        // NONE type: 0, length 1
        DUP => OpcodeDesc { op, name: "DUP", flags: 0, length: 1 },
        DUPN => OpcodeDesc { op, name: "DUPN", flags: 0, length: 1 },
        DUP2 => OpcodeDesc { op, name: "DUP2", flags: 0, length: 1 },
        POP => OpcodeDesc { op, name: "POP", flags: 0, length: 1 },
        INDEX => OpcodeDesc { op, name: "INDEX", flags: 0, length: 1 },
        INDEX_OPT => OpcodeDesc { op, name: "INDEX_OPT", flags: 0, length: 1 },
        EACH => OpcodeDesc { op, name: "EACH", flags: 0, length: 1 },
        EACH_OPT => OpcodeDesc { op, name: "EACH_OPT", flags: 0, length: 1 },
        TRY_END => OpcodeDesc { op, name: "TRY_END", flags: 0, length: 1 },
        BACKTRACK => OpcodeDesc { op, name: "BACKTRACK", flags: 0, length: 1 },
        INSERT => OpcodeDesc { op, name: "INSERT", flags: 0, length: 1 },
        SUBEXP_BEGIN => OpcodeDesc { op, name: "SUBEXP_BEGIN", flags: 0, length: 1 },
        SUBEXP_END => OpcodeDesc { op, name: "SUBEXP_END", flags: 0, length: 1 },
        PATH_BEGIN => OpcodeDesc { op, name: "PATH_BEGIN", flags: 0, length: 1 },
        PATH_END => OpcodeDesc { op, name: "PATH_END", flags: 0, length: 1 },
        RET => OpcodeDesc { op, name: "RET", flags: 0, length: 1 },
        TOP => OpcodeDesc { op, name: "TOP", flags: 0, length: 1 },
        GENLABEL => OpcodeDesc { op, name: "GENLABEL", flags: 0, length: 1 },

        _ => OpcodeDesc { op, name: "UNKNOWN", flags: 0, length: 1 },
    }
}
pub fn locfile_locate(locfile: &Option<Box<Locfile>>, source: &Location, fmt: &str) {
    if let Some(lf) = locfile {
        lf.locate(source, fmt, &[]);
    }
}
pub fn locfile_locate_with_args(
    locfile: &Option<Box<Locfile>>,
    source: &Location,
    fmt: &str,
    args: &[&str],
) {
    if let Some(lf) = locfile {
        lf.locate(source, fmt, args);
    }
}
/// Check if block is a single instruction
/// Returns true if:
/// - first and last both point to the same instruction (owned block), OR
/// - first is None but last is Some (reference-only block used for binding)
pub fn block_is_single(b: &Block) -> bool {
    if let Some(ref first) = b.first {
        if let Some(last_ptr) = b.last {
            let first_ptr = &**first as *const Inst;
            return first_ptr == last_ptr;
        }
    } else if b.last.is_some() {
        // Reference-only block: first is None but last points to an instruction
        // This is used when we need to bind without taking ownership
        return true;
    }
    false
}
/// Append block b to block a
pub fn block_append(a: &mut Block, b: Block) {
    if b.first.is_none() {
        return;
    }
    if a.first.is_none() {
        a.first = b.first;
        a.last = b.last;
    } else {
        if let Some(ref mut last) = a.first {
            let mut current = last.as_mut();
            while current.next.is_some() {
                current = current.next.as_mut().unwrap();
            }
            if let Some(b_first) = b.first {
                let raw_current = current as *mut Inst;
                let mut b_first_boxed = b_first;
                b_first_boxed.prev = Some(raw_current);
                current.next = Some(b_first_boxed);
            }
        }
        a.last = b.last;
    }
}
/// Take first instruction from block
pub fn block_take(b: &mut Block) -> Option<Inst> {
    if b.first.is_none() {
        return None;
    }
    let mut first = b.first.take()?;
    if first.next.is_some() {
        let mut next = first.next.take()?;
        next.prev = None;
        b.first = Some(next);
    } else {
        b.first = None;
        b.last = None;
    }
    Some(*first)
}
/// Take the last instruction from a block, preserving its heap location
/// Returns Box<Inst> to avoid moving the instruction to a new heap address
/// which would invalidate bound_by pointers
pub fn block_take_last(b: &mut Block) -> Option<Box<Inst>> {
    if b.first.is_none() {
        return None;
    }
    if block_is_single(b) {
        return b.first.take();
    }
    let mut current = &mut b.first;
    while let Some(ref mut node) = current {
        if node.next.is_some() {
            let next_is_last = {
                if let Some(ref next) = node.next { next.next.is_none() } else { false }
            };
            if next_is_last {
                let last = node.next.take();
                b.last = Some(&mut **node as *mut Inst);
                return last;
            }
        }
        current = &mut current.as_mut().unwrap().next;
    }
    None
}
/// Check if block has only binder instructions
pub fn block_has_only_binders(binders: &Block, mut bindflags: i32) -> bool {
    bindflags |= OP_HAS_BINDING;
    bindflags &= !OP_BIND_WILDCARD;
    let mut current = &binders.first;
    while let Some(ref inst) = current {
        let flags = get_opcode_flags_u16(inst.op);
        if (flags & bindflags) != bindflags && inst.op != MODULEMETA {
            return false;
        }
        current = &inst.next;
    }
    true
}
/// Generate AND operation
pub fn gen_and(a: Block, b: Block) -> Block {
    block_join(
        block_join(gen_op_simple(DUP), a),
        gen_condbranch(
            block_join(
                block_join(gen_op_simple(POP), b),
                gen_condbranch(gen_const(Jv::jv_true()), gen_const(Jv::jv_false())),
            ),
            block_join(gen_op_simple(POP), gen_const(Jv::jv_false())),
        ),
    )
}
/// Make environment from system environment variables
fn make_env(env: Jv) -> Jv {
    if jv_is_valid(&env) {
        return env.clone();
    }
    let mut r = Jv::object();
    for (key, value) in env::vars() {
        r = r.object_set(Jv::string(&key), Jv::string(&value));
    }
    r
}
/// Expand call argument list
pub fn expand_call_arglist(b: &mut Block, args: Jv, env: &mut Jv) -> i32 {
    let debug = std::env::var("DEBUG_COMPILE").is_ok();
    let mut errors = 0;
    let mut ret = gen_noop();
    // Use block_take_ptr to keep instructions in their original Box memory locations,
    // preserving the validity of bound_by raw pointers (matches C behavior)
    while let Some(mut curr_box) = block_take_ptr(b) {
        let curr = &mut *curr_box;
        let desc = opcode_describe(curr.op);
        if debug { eprintln!("  expand_call_arglist: op={} name={} flags={} symbol={:?} bound_by={:?}", curr.op, desc.name, desc.flags, curr.symbol, curr.bound_by.is_some()); }
        if desc.flags & OP_HAS_BINDING != 0 {
            if curr.bound_by.is_none() && curr.op == LOADV {
                if let Some(ref sym) = curr.symbol {
                    if sym == "ENV" {
                        curr.op = LOADK;
                        let new_env = make_env(env.clone());
                        curr.imm = InstImmediate::Constant(new_env.clone());
                        *env = new_env;
                    } else if object_has_key(&args, sym) {
                        curr.op = LOADK;
                        curr.imm = InstImmediate::Constant(
                            jv_object_get(&args, Jv::string(sym)),
                        );
                    } else {
                        let sym_bytes = sym.as_bytes();
                        if sym_bytes.len() == 2 && sym_bytes[0] == b'*'
                            && sym_bytes[1] >= b'1' && sym_bytes[1] <= b'3'
                        {
                            locfile_locate(
                                &curr.locfile,
                                &curr.source,
                                "jq: error: break used outside labeled control structure",
                            );
                        } else {
                            locfile_locate(
                                &curr.locfile,
                                &curr.source,
                                &format!("jq: error: ${} is not defined", sym),
                            );
                        }
                        errors += 1;
                        ret = block_join(ret, inst_block(curr_box));
                        continue;
                    }
                }
            } else if curr.bound_by.is_none() {
                if let Some(ref sym) = curr.symbol {
                    if debug { eprintln!("  ERROR: {}/{} is not defined (op={})", sym, curr.nactuals, curr.op); }
                    // Always print error even if locfile is None
                    eprintln!("jq: error: {}/{} is not defined", sym, curr.nactuals);
                } else {
                    // No symbol - report the opcode so we can debug
                    let op_name = opcode_describe(curr.op).name;
                    if debug { eprintln!("  ERROR: unbound {} instruction (op={})", op_name, curr.op); }
                    // Always print error even if locfile is None
                    eprintln!("jq: error: unbound {} instruction", op_name);
                }
                if debug { eprintln!("  ERROR (no symbol): bound_by=None, op={}", curr.op); }
                errors += 1;
                ret = block_join(ret, inst_block(curr_box));
                continue;
            }
        }
        let mut prelude = gen_noop();
        if curr.op == CALL_JQ {
            let mut actual_args = 0;
            let desired_args;
            if let Some(bound_by_ptr) = curr.bound_by {
                let bound_by_op = unsafe { (*bound_by_ptr).op };
                if bound_by_op == CLOSURE_CREATE || bound_by_op == CLOSURE_PARAM {
                    let mut callargs = gen_noop();
                    while let Some(i_box) = block_take_ptr(&mut curr.arglist) {
                        let i = &*i_box;
                        assert!(
                            opcode_describe(i.op).flags & OP_IS_CALL_PSEUDO != 0
                        );
                        if i.op == CLOSURE_REF {
                            block_append(&mut callargs, inst_block(i_box));
                        } else if i.op == CLOSURE_CREATE {
                            // IMPORTANT: In C, block is just {first,last} pointers - passing by value
                            // shares the same instruction memory. In Rust, we must NOT clone because
                            // the CLOSURE_REF must point to the actual instruction that goes to prelude.
                            // Append the ORIGINAL instruction to prelude (not a clone).
                            let b_inst = inst_block(i_box);
                            block_append(&mut prelude, b_inst);
                            // Now create CLOSURE_REF bound to the instruction we just appended.
                            // Get a reference to the last instruction in prelude (what we just added).
                            let binder_ptr = prelude.last.expect("prelude should have last after append");
                            block_append(
                                &mut callargs,
                                gen_op_bound_to_ptr(CLOSURE_REF, Some(binder_ptr)),
                            );
                        } else {
                            panic!("Unknown type of parameter");
                        }
                        actual_args += 1;
                    }
                    curr.imm = InstImmediate::IntVal(actual_args);
                    curr.arglist = callargs;
                    desired_args = if bound_by_op == CLOSURE_CREATE {
                        let mut count = 0;
                        unsafe {
                            let mut param = (*bound_by_ptr).arglist.first.as_ref();
                            while let Some(p) = param {
                                assert!(p.op == CLOSURE_PARAM);
                                count += 1;
                                param = p.next.as_ref();
                            }
                        }
                        count
                    } else {
                        0
                    };
                } else if bound_by_op == CLOSURE_CREATE_C {
                    while let Some(mut i_box) = block_take_ptr(&mut curr.arglist) {
                        let i = &mut *i_box;
                        assert!(i.op == CLOSURE_CREATE);
                        let mut body = std::mem::take(&mut i.subfn);
                        i.subfn = gen_noop();
                        inst_free(i_box);
                        errors += expand_call_arglist(&mut body, args.clone(), env);
                        prelude = block_join(gen_subexp(body), prelude);
                        actual_args += 1;
                    }
                    assert!(curr.op == CALL_JQ);
                    curr.op = CALL_BUILTIN;
                    curr.imm = InstImmediate::IntVal(actual_args + 1);
                    assert!(bound_by_op == CLOSURE_CREATE_C);
                    // Use nformals which was set to nargs-1 in gen_cbinding
                    // This avoids reading from imm which may have been overwritten to IntVal(idx)
                    desired_args = unsafe { (*bound_by_ptr).nformals };
                    assert!(curr.arglist.first.is_none());
                } else {
                    if debug {
                        eprintln!("DEBUG: Unknown function type - bound_by_op={}, CLOSURE_CREATE={}, CLOSURE_PARAM={}, CLOSURE_CREATE_C={}, symbol={:?}",
                            bound_by_op, CLOSURE_CREATE, CLOSURE_PARAM, CLOSURE_CREATE_C, curr.symbol);
                    }
                    panic!("Unknown function type");
                }
                assert!(actual_args == desired_args);
            }
        }
        ret = block_join(block_join(ret, prelude), inst_block(curr_box));
    }
    *b = ret;
    errors
}
/// Bind referenced instructions
pub fn block_bind_referenced(
    mut binder: Block,
    mut body: Block,
    mut bindflags: i32,
) -> Block {
    assert!(block_has_only_binders(& binder, bindflags));
    bindflags |= OP_HAS_BINDING as i32;
    while let Some(curr) = block_take_last(&mut binder) {
        let mut b = inst_block(curr);
        if block_bind_subblock(&mut b, &mut body, bindflags, 0) == 0 {
            block_free(b);
        } else {
            body = block_join(b, body);
        }
    }
    body
}
/// Generate define-or operation
pub fn gen_definedor(a: Block, b: Block) -> Block {
    let found_var = gen_op_var_fresh(STOREV, "found");
    let init = block_join(
        block_join(gen_op_simple(DUP), gen_const(Jv::jv_false())),
        found_var.clone(),
    );
    let backtrack = gen_op_simple(BACKTRACK);
    let tail = block_join(
        block_join(
            block_join(
                block_join(
                    block_join(
                        gen_op_simple(DUP),
                        gen_op_bound(LOADV, &found_var),
                    ),
                    gen_op_target(JUMP_F, &backtrack),
                ),
                backtrack.clone(),
            ),
            gen_op_simple(POP),
        ),
        b,
    );
    let if_notfound = gen_op_simple(BACKTRACK);
    let if_found = block_join(
        block_join(
            block_join(gen_op_simple(DUP), gen_const(Jv::jv_true())),
            gen_op_bound(STOREV, &found_var),
        ),
        gen_op_target(JUMP, &tail),
    );
    block_join(
        block_join(
            block_join(
                block_join(
                    block_join(
                        block_join(init, gen_op_target(FORK, &if_notfound)),
                        a,
                    ),
                    gen_op_target(JUMP_F, &if_found),
                ),
                if_found,
            ),
            if_notfound,
        ),
        tail,
    )
}
/// Generate a constant array if all elements are constant
fn gen_const_array(expr: &Block) -> Option<Block> {
    let mut all_const = true;
    let mut commas = 0;
    let mut normal = true;
    let mut a = jv_array();
    let mut current: Option<&Inst> = expr.first.as_ref().map(|b| b.as_ref());
    while let Some(inst) = current {
        if inst.op == FORK {
            commas += 1;
            // C: if (i->imm.target == NULL || i->imm.target->op != JUMP || jv_array_length(jv_copy(a)) > 0)
            let target_invalid = inst.target.is_none() ||
                inst.target.map_or(true, |t| unsafe { (*t).op != JUMP });
            if target_invalid || jv_array_length(jv_copy(&a)) > 0 {
                normal = false;
                break;
            }
        } else if all_const && inst.op == LOADK {
            if let Some(ref next) = inst.next {
                if next.op != JUMP {
                    normal = false;
                    break;
                }
            }
            if let InstImmediate::Constant(ref constant) = inst.imm {
                a = jv_array_append(a, jv_copy(constant));
            }
        } else if inst.op != JUMP {
            all_const = false;
        }
        current = inst.next.as_ref().map(|b| b.as_ref());
    }
    let last_is_loadk = expr.last.map_or(true, |_| true);
    if all_const && normal && last_is_loadk && jv_array_length(jv_copy(&a)) == commas + 1 {
        return Some(gen_const(a));
    }
    drop(a);
    None
}
/// Generate collect operation
pub fn gen_collect(expr: Block) -> Block {
    if let Some(const_array) = gen_const_array(&expr) {
        return const_array;
    }
    // Matches C: block array_var = gen_op_var_fresh(STOREV, "collect");
    //            block c = BLOCK(gen_op_simple(DUP), gen_const(jv_array()), array_var);
    //            block tail = BLOCK(gen_op_bound(APPEND, array_var), gen_op_simple(BACKTRACK));
    //            return BLOCK(c, gen_op_target(FORK, tail), expr, tail, gen_op_bound(LOADVN, array_var));
    // In C, array_var is used by value (shared pointers) everywhere.
    // In Rust, we must use pointers to avoid cloning which breaks bound_by.
    let array_var = gen_op_var_fresh(STOREV, "collect");
    let array_var_ptr = array_var.first.as_ref().map(|b| b.as_ref() as *const Inst as *mut Inst);

    // Create append and backtrack instructions, bound to array_var via pointer
    let append = gen_op_bound_to_ptr(APPEND, array_var_ptr);
    let tail = block_join(append, gen_op_simple(BACKTRACK));

    // Create loadvn instruction, bound to array_var via pointer
    let loadvn = gen_op_bound_to_ptr(LOADVN, array_var_ptr);

    // Build: c = [DUP, const_array, array_var]
    let c = block_join(
        block_join(gen_op_simple(DUP), gen_const(jv_array())),
        array_var,  // array_var moved here, not cloned
    );

    // Build: [c, FORK->tail, expr, tail, loadvn]
    block_join(
        block_join(
            block_join(block_join(c, gen_op_target(FORK, &tail)), expr),
            tail,
        ),
        loadvn,
    )
}
/// Bind each binder in a block to a body - matches C block_bind_each
fn block_bind_each(binder: &Block, body: &mut Block, bindflags: i32) -> i32 {
    if !block_has_only_binders(binder, bindflags) {
        return 0;
    }
    let bindflags = bindflags | OP_HAS_BINDING;
    let mut nrefs = 0;
    let mut curr_ptr = binder.first.as_ref().map(|b| b.as_ref() as *const Inst);
    while let Some(ptr) = curr_ptr {
        unsafe {
            let curr = &*ptr;
            let mut inst_blk = Block {
                first: None,
                last: Some(ptr as *mut Inst),
            };
            nrefs += block_bind_subblock(&mut inst_blk, body, bindflags, 0);
            curr_ptr = curr.next.as_ref().map(|b| b.as_ref() as *const Inst);
        }
    }
    nrefs
}
/// Inner function for binding subblocks - matches C block_bind_subblock_inner
pub fn block_bind_subblock_inner(
    any_unbound: &mut i32,
    binder: &mut Block,
    body: &mut Block,
    bindflags: i32,
    mut break_distance: i32,
) -> i32 {
    if !block_is_single(binder) {
        return 0;
    }

    // Extract all needed values from binder in a limited scope
    // Handle both owned blocks (first is Some) and reference-only blocks (first is None, last is Some)
    let (binder_ptr, binder_symbol, binder_nformals) = {
        let binder_inst: &mut Inst = if let Some(ref mut f) = binder.first {
            f.as_mut()
        } else if let Some(last_ptr) = binder.last {
            unsafe { &mut *last_ptr }
        } else {
            return 0;
        };
        let binder_op_flags = get_opcode_flags_u16(binder_inst.op);
        if (binder_op_flags & bindflags) != (bindflags & !OP_BIND_WILDCARD) {
            return 0;
        }
        let symbol = match binder_inst.symbol.as_ref() {
            Some(s) => s.clone(),
            None => return 0,
        };
        if break_distance < 0 {
            return 0;
        }
        let nformals = binder_inst.nformals;
        let ptr = binder_inst as *mut Inst;

        // C line 314: binder.first->bound_by = binder.first
        // The binder binds to itself - this marks it as a definition
        if binder_inst.bound_by.is_none() {
            binder_inst.bound_by = Some(ptr);
        }

        (ptr, symbol, nformals)
    };
    // binder borrow is now released

    let mut nrefs = 0;
    let mut current = body.first.as_mut();
    while let Some(inst) = current {
        if inst.any_unbound == 0 {
            current = inst.next.as_mut();
            continue;
        }
        let flags = get_opcode_flags_u16(inst.op);
        let debug_bind = std::env::var("DEBUG_BIND").is_ok();
        if debug_bind && inst.symbol.as_ref().map_or(false, |s| s == "f") {
            eprintln!("  bind_subblock_inner: inst op={} sym={:?} bound={:?} nact={} vs binder={:?} nformal={}",
                inst.op, inst.symbol, inst.bound_by.is_some(), inst.nactuals, binder_symbol, binder_nformals);
        }
        if (flags & bindflags) == (bindflags & !OP_BIND_WILDCARD)
            && inst.bound_by.is_none() && inst.symbol.is_some()
        {
            let inst_symbol = inst.symbol.as_ref().unwrap();
            let matches = inst_symbol == &binder_symbol
                || ((bindflags & OP_BIND_WILDCARD) != 0 && inst_symbol.starts_with('*')
                    && break_distance <= 3 && inst_symbol.len() == 2
                    && inst_symbol.chars().nth(1)
                        == Some((b'1' + break_distance as u8) as char));
            if debug_bind && inst_symbol == "f" {
                eprintln!("    matches={} arity_ok={}", matches, inst.nactuals == -1 || inst.nactuals == binder_nformals);
            }
            if matches && (inst.nactuals == -1 || inst.nactuals == binder_nformals) {
                if debug_bind && inst_symbol == "f" {
                    eprintln!("    BINDING f to {:?}", binder_ptr);
                }
                inst.bound_by = Some(binder_ptr);
                nrefs += 1;
            }
        } else if (flags & bindflags) == (bindflags & !OP_BIND_WILDCARD)
            && inst.bound_by.is_some() && binder_symbol.starts_with("*anonlabel")
            && inst.symbol.as_ref().map_or(false, |s| s.starts_with("*anonlabel"))
        {
            break_distance += 1;
        }
        inst.any_unbound = if inst.symbol.is_some() && inst.bound_by.is_none() {
            1
        } else {
            0
        };
        nrefs
            += block_bind_subblock_inner(
                &mut inst.any_unbound,
                binder,
                &mut inst.subfn,
                bindflags,
                break_distance,
            );
        nrefs
            += block_bind_subblock_inner(
                &mut inst.any_unbound,
                binder,
                &mut inst.arglist,
                bindflags,
                break_distance,
            );
        if inst.any_unbound != 0 {
            *any_unbound = 1;
        }
        current = inst.next.as_mut();
    }
    nrefs
}

/// Bind all matching unbound instructions in body to the given binder pointer.
/// This avoids the need to clone when binding a block to itself (like in gen_function).
/// Unlike block_bind_subblock_inner, this takes binder info as values rather than a Block ref.
fn block_bind_to_binder_ptr(
    body: &mut Block,
    binder_ptr: *mut Inst,
    binder_symbol: &str,
    binder_nformals: i32,
    bindflags: i32,
) -> i32 {
    let mut nrefs = 0;
    let mut current = body.first.as_mut();
    while let Some(inst) = current {
        if inst.any_unbound == 0 {
            current = inst.next.as_mut();
            continue;
        }
        let flags = get_opcode_flags_u16(inst.op);
        if (flags & bindflags) == (bindflags & !OP_BIND_WILDCARD)
            && inst.bound_by.is_none()
            && inst.symbol.is_some()
        {
            let inst_symbol = inst.symbol.as_ref().unwrap();
            if inst_symbol == binder_symbol
                && (inst.nactuals == -1 || inst.nactuals == binder_nformals)
            {
                inst.bound_by = Some(binder_ptr);
                nrefs += 1;
            }
        }
        inst.any_unbound = if inst.symbol.is_some() && inst.bound_by.is_none() {
            1
        } else {
            0
        };
        // Recurse into subfn and arglist
        nrefs += block_bind_to_binder_ptr(
            &mut inst.subfn,
            binder_ptr,
            binder_symbol,
            binder_nformals,
            bindflags,
        );
        nrefs += block_bind_to_binder_ptr(
            &mut inst.arglist,
            binder_ptr,
            binder_symbol,
            binder_nformals,
            bindflags,
        );
        current = inst.next.as_mut();
    }
    nrefs
}

/// Get the constant kind from a block
pub fn block_const_kind(b: &Block) -> JvKind {
    if block_is_const(b) == 0 {
        return JvKind::Invalid;
    }
    if let Some(ref first) = b.first {
        if let InstImmediate::Constant(ref c) = first.imm {
            return jv_get_kind(c);
        }
    }
    JvKind::Invalid
}
/// Get the kind of a JV value
pub fn jv_get_kind(jv: &Jv) -> JvKind {
    jv.get_kind()
}
fn gen_op_targetlater(op: Opcode) -> Block {
    let mut inst = inst_new(op);
    inst.imm = InstImmediate::Target(None);
    inst_block(inst)
}
/// Bind a binder block to a body block - matches C block_bind
pub fn block_bind(binder: Block, mut body: Block, bindflags: i32) -> Block {
    block_bind_each(&binder, &mut body, bindflags);
    block_join(binder, body)
}
/// Generate an object matcher for pattern matching
pub fn gen_object_matcher(name: Block, curr: Block) -> Block {
    let dup_block = gen_op_simple(DUP);
    let subexp_block = gen_subexp(name);
    let index_block = gen_op_simple(INDEX);
    let joined1 = block_join(dup_block, subexp_block);
    let joined2 = block_join(joined1, index_block);
    block_join(joined2, curr)
}
/// Check if a block is a function definition
pub fn block_is_funcdef(b: &Block) -> i32 {
    if let Some(ref first) = b.first {
        if first.op == CLOSURE_CREATE {
            return 1;
        }
    }
    0
}
/// Generate a module with metadata
pub fn gen_module(metadata: Block) -> Block {
    assert!(
        block_is_const(&metadata) != 0 && block_const_kind(&metadata) == JvKind::Object,
        "block_is_const(metadata) && block_const_kind(metadata) == JV_KIND_OBJECT"
    );
    let mut inst = inst_new(MODULEMETA);
    let constant = block_const(&metadata);
    let final_constant = if jv_get_kind(&constant) != JvKind::Object {
        Jv::object().object_set(Jv::string("metadata"), constant)
    } else {
        constant
    };
    inst.imm = InstImmediate::Constant(final_constant);
    block_free(metadata);
    inst_block(inst)
}
/// Generate a push constant under operation
pub fn gen_op_pushk_under(constant: Jv) -> Block {
    let desc = opcode_describe(PUSHK_UNDER);
    assert!(
        desc.flags & OP_HAS_CONSTANT != 0,
        "opcode_describe(PUSHK_UNDER)->flags & OP_HAS_CONSTANT"
    );
    let mut inst = inst_new(PUSHK_UNDER);
    inst.imm = InstImmediate::Constant(constant);
    inst_block(inst)
}
/// Count the number of actual parameters in a block
pub fn block_count_actuals(b: &Block) -> i32 {
    let mut args = 0;
    let mut current: Option<&Inst> = b.first.as_ref().map(|b| b.as_ref());
    while let Some(inst) = current {
        match inst.op {
            op if op == CLOSURE_CREATE || op == CLOSURE_PARAM
                || op == CLOSURE_CREATE_C => {
                args += 1;
            }
            _ => {
                // Skip non-closure ops
            }
        }
        current = inst.next.as_ref().map(|b| b.as_ref());
    }
    args
}
/// Opcode flags for binding (must match C's bytecode.h: OP_BIND_WILDCARD = 2048)
const OP_BIND_WILDCARD: i32 = 2048;
/// Helper function to set object key
pub fn jv_object_set(mut obj: Jv, key: Jv, value: Jv) -> Jv {
    obj
}
/// Memory allocation placeholder
pub fn jv_mem_alloc<T: Default>() -> Box<T> {
    Box::new(T::default())
}
pub fn jv_mem_calloc<T: Default + Clone>(count: usize) -> Vec<T> {
    vec![T::default(); count]
}
/// Compile a block of instructions into bytecode
pub fn block_compile(
    b: Block,
    out: &mut Option<Box<Bytecode>>,
    lf: &mut Locfile,
    args: Jv,
) -> i32 {
    let mut bc = jv_mem_alloc::<Bytecode>();
    bc.parent = None;
    bc.nclosures = 0;
    bc.globals = Some(jv_mem_alloc::<SymbolTable>());
    let ncfunc = count_cfunctions_internal(&b);
    if let Some(ref mut globals) = bc.globals {
        globals.ncfunctions = 0;
        globals.cfunctions = jv_mem_calloc(ncfunc);
        globals.cfunc_names = Jv::array();
    }
    bc.debuginfo = jv_object_set(Jv::object(), Jv::string("name"), Jv::null());
    let mut env = Jv::invalid();
    let nerrors = compile_internal(&mut bc, b, lf, args.clone(), &mut env);
    args.free();
    env.free();
    if let Some(ref globals) = bc.globals {
        assert!(
            globals.ncfunctions == ncfunc as i32, "bc->globals->ncfunctions == ncfunc"
        );
    }
    if nerrors > 0 {
        bytecode_free_internal(bc);
        *out = None;
    } else {
        *out = Some(bc);
    }
    nerrors
}
/// Internal compile function placeholder
fn compile_internal(
    bc: &mut Box<Bytecode>,
    b: Block,
    lf: &mut Locfile,
    args: Jv,
    env: &mut Jv,
) -> i32 {
    0
}
/// Internal bytecode free function
fn bytecode_free_internal(bc: Box<Bytecode>) {
    drop(bc);
}
/// Count C functions in a block
fn count_cfunctions_internal(b: &Block) -> usize {
    let mut count = 0;
    let mut current: Option<&Inst> = b.first.as_ref().map(|b| b.as_ref());
    while let Some(inst) = current {
        if inst.op == CLOSURE_CREATE_C {
            count += 1;
        }
        current = inst.next.as_ref().map(|b| b.as_ref());
    }
    count
}
/// Generate a simple opcode instruction
fn gen_op_simple_internal(op: Opcode) -> Block {
    inst_block(inst_new(op))
}
/// Generate an unbound opcode instruction
fn gen_op_unbound_internal(op: Opcode, name: &str) -> Block {
    let mut inst = inst_new(op);
    inst.symbol = Some(name.to_string());
    inst.any_unbound = 1;
    inst_block(inst)
}
/// Join two blocks together
fn block_join_internal(a: Block, b: Block) -> Block {
    block_join(a, b)
}
/// Bind instructions in a block
fn block_bind_internal(binder: Block, body: Block, _bindflags: i32) -> Block {
    block_join_internal(binder, body)
}
/// Generate C function bindings
pub fn gen_cbinding(
    cfunctions: &[Cfunction],
    ncfunctions: i32,
    mut code: Block,
) -> Block {
    for cfunc in 0..ncfunctions as usize {
        let mut inst = inst_new(CLOSURE_CREATE_C);
        inst.imm = InstImmediate::Cfunc(Box::new(CfuncRef { index: cfunc, nargs: cfunctions[cfunc].nargs }));
        if let Some(ref name) = cfunctions[cfunc].name {
            inst.symbol = Some(name.clone());
        }
        inst.nformals = cfunctions[cfunc].nargs - 1;
        inst.any_unbound = 0;
        code = block_join_internal(inst_block(inst), code);
    }
    code
}
/// Generate a constant global binding
pub fn gen_const_global(constant: Jv, name: &str) -> Block {
    let flags = get_opcode_flags_u16(STORE_GLOBAL);
    let expected = OP_HAS_CONSTANT | OP_HAS_VARIABLE | OP_HAS_BINDING;
    assert!((flags & expected) == expected, "STORE_GLOBAL must have correct flags");
    let mut inst = inst_new(STORE_GLOBAL);
    inst.imm = InstImmediate::Constant(constant);
    inst.symbol = Some(name.to_string());
    inst.any_unbound = 0;
    inst_block(inst)
}
/// Check if block is a no-op
fn block_is_noop_internal(b: &Block) -> bool {
    b.first.is_none()
}
/// Check if block has exactly one instruction
fn block_is_single_internal(b: &Block) -> bool {
    if let Some(ref first) = b.first { first.next.is_none() } else { false }
}
/// Get constant from block
fn block_const_internal(b: &Block) -> Jv {
    if let Some(ref first) = b.first {
        if let InstImmediate::Constant(ref c) = first.imm {
            return c.clone();
        }
    }
    Jv::invalid()
}
/// Free a block
fn block_free_internal(b: Block) {
    drop(b);
}
/// Generate pushk_under operation
fn gen_op_pushk_under_internal(c: Jv) -> Block {
    let mut inst = inst_new(LOADK);
    inst.imm = InstImmediate::Constant(c);
    inst_block(inst)
}
/// Bind a matcher block to a body
/// This matches C's bind_matcher: for each STOREV/STOREVN that's unbound,
/// bind it to the body with OP_HAS_VARIABLE
pub fn bind_matcher(mut matcher: Block, mut body: Block) -> Block {
    // Iterate through matcher looking for unbound STOREV/STOREVN
    let mut current_ptr: Option<*mut Inst> = matcher.first.as_mut().map(|b| b.as_mut() as *mut Inst);
    while let Some(ptr) = current_ptr {
        let inst = unsafe { &mut *ptr };
        if (inst.op == STOREV || inst.op == STOREVN) && inst.bound_by.is_none() {
            // Create a reference-only binder block (first: None, last: Some(ptr))
            let mut binder = Block {
                first: None,
                last: Some(ptr),
            };
            block_bind_subblock(&mut binder, &mut body, OP_HAS_VARIABLE, 0);
        }
        current_ptr = inst.next.as_mut().map(|b| b.as_mut() as *mut Inst);
    }
    block_join_internal(matcher, body)
}
/// Drop unreferenced instructions from a block
pub fn block_drop_unreferenced(mut body: Block) -> Block {
    block_mark_referenced_internal(&mut body);
    let mut refd = gen_noop_internal();
    while let Some(curr) = block_take_internal(&mut body) {
        let is_self_bound = if let Some(bound_by) = curr.bound_by {
            std::ptr::eq(bound_by, curr.as_ref() as *const Inst as *mut Inst)
        } else {
            false
        };
        if is_self_bound && !curr.referenced {
            inst_free(curr);
        } else {
            refd = block_join_internal(refd, inst_block(curr));
        }
    }
    refd
}
/// Mark referenced instructions
fn block_mark_referenced_internal(body: &mut Block) {
    let mut current: Option<&Inst> = body.first.as_ref().map(|b| b.as_ref());
    while let Some(inst) = current {
        current = inst.next.as_ref().map(|b| b.as_ref());
    }
}
/// Take an instruction from a block
fn block_take_internal(body: &mut Block) -> Option<Box<Inst>> {
    body.first.take()
}
/// Generate a no-op block
fn gen_noop_internal() -> Block {
    Block::default()
}
/// Generate destructure alternative
pub fn gen_destructure_alt(mut matcher: Block) -> Block {
    let mut current_ptr = matcher.first.as_mut().map(|b| b.as_mut() as *mut Inst);
    while let Some(ptr) = current_ptr {
        unsafe {
            let inst = &mut *ptr;
            if inst.op == STOREV {
                inst.op = STOREVN;
            }
            current_ptr = inst.next.as_mut().map(|b| b.as_mut() as *mut Inst);
        }
    }
    let mut inst = inst_new(DESTRUCTURE_ALT);
    inst.subfn = Block::default();
    inst_block(inst)
}
/// Compile a block into bytecode
pub fn compile(
    bc: &mut Bytecode,
    mut b: Block,
    lf: &mut Locfile,
    args: Jv,
    env: &mut Option<Jv>,
) -> i32 {
    let debug_compile = std::env::var("DEBUG_COMPILE").is_ok();
    if debug_compile {
        eprintln!("COMPILE: Block has {} instructions", count_block_instructions(&b));
        dump_block(&b, "COMPILE");
    }
    let mut errors = 0;
    let mut pos = 0i32;
    let mut var_frame_idx = 0i32;
    bc.nsubfunctions = 0;
    if let Some(ref mut env_jv) = env {
        errors += expand_call_arglist(&mut b, args.clone(), env_jv);
    }
    b = block_join(b, gen_op_simple(RET));
    let mut localnames = Jv::array();
    let mut curr_ptr = b.first.as_mut().map(|b| b.as_mut() as *mut Inst);
    while let Some(ptr) = curr_ptr {
        unsafe {
            let curr = &mut *ptr;
            if curr.next.is_none() {
                if let Some(last_ptr) = b.last {
                    assert!(ptr == last_ptr, "curr == b.last");
                }
            }
            let op_desc = opcode_describe(curr.op as u16);
            let mut length = op_desc.length;
            if curr.op == CALL_JQ {
                let mut arg_ptr = curr
                    .arglist
                    .first
                    .as_ref()
                    .map(|b| b.as_ref() as *const Inst);
                while let Some(arg_p) = arg_ptr {
                    length += 2;
                    arg_ptr = (*arg_p).next.as_ref().map(|b| b.as_ref() as *const Inst);
                }
            }
            pos += length;
            curr.bytecode_pos = pos;
            curr.compiled = Some(bc as *mut Bytecode);
            assert!(
                curr.op != CLOSURE_REF && curr.op != CLOSURE_PARAM,
                "curr.op != CLOSURE_REF && curr.op != CLOSURE_PARAM"
            );
            let op_flags = op_desc.flags;
            if (op_flags & OP_HAS_VARIABLE) != 0 {
                if let Some(bound_ptr) = curr.bound_by {
                    if bound_ptr == ptr {
                        curr.imm = InstImmediate::IntVal(var_frame_idx);
                        var_frame_idx += 1;
                        if let Some(ref sym) = curr.symbol {
                            localnames = localnames.array_append(Jv::string(sym));
                        }
                    }
                }
            }
            if curr.op == CLOSURE_CREATE {
                if let Some(bound_ptr) = curr.bound_by {
                    if bound_ptr != ptr && debug_compile {
                        eprintln!("CLOSURE_CREATE mismatch: self={:?} bound_by={:?} symbol={:?}",
                            ptr, bound_ptr, curr.symbol);
                    }
                    assert!(bound_ptr == ptr, "curr.bound_by == curr");
                }
                curr.imm = InstImmediate::IntVal(bc.nsubfunctions);
                bc.nsubfunctions += 1;
            }
            if curr.op == CLOSURE_CREATE_C {
                if let Some(bound_ptr) = curr.bound_by {
                    assert!(bound_ptr == ptr, "curr.bound_by == curr");
                }
                if let Some(ref mut globals) = bc.globals {
                    let idx = globals.ncfunctions;
                    globals.ncfunctions += 1;
                    if let Some(ref sym) = curr.symbol {
                        globals.cfunc_names = globals
                            .cfunc_names
                            .clone()
                            .array_append(Jv::string(sym));
                    }
                    if let InstImmediate::Cfunc(ref cfunc_ref) = curr.imm {
                        // CfuncRef contains index into cfunctions - no copy needed
                        // The Cfunction already exists in the globals.cfunctions array
                        let _ = cfunc_ref; // Used for validation only
                    }
                    curr.imm = InstImmediate::IntVal(idx);
                }
            }
            curr_ptr = curr.next.as_mut().map(|b| b.as_mut() as *mut Inst);
        }
    }
    if pos > 0xFFFF {
        // Create a Location for error reporting
        eprintln!("jq: error: function compiled to {} bytes which is too long", pos);
        if debug_compile { eprintln!("  ERROR: bytecode too long: pos={}", pos); }
        errors += 1;
    }
    bc.codelen = pos;
    bc.debuginfo = bc.debuginfo.clone().object_set(Jv::string("locals"), localnames);
    if bc.nsubfunctions > 0 && errors == 0 {
        bc.subfunctions = Vec::with_capacity(bc.nsubfunctions as usize);
        for _ in 0..bc.nsubfunctions {
            bc.subfunctions.push(Box::new(Bytecode::default()));
        }
        let mut curr_ptr = b.first.as_mut().map(|b| b.as_mut() as *mut Inst);
        while let Some(ptr) = curr_ptr {
            unsafe {
                let curr = &mut *ptr;
                if curr.op == CLOSURE_CREATE {
                    if let InstImmediate::IntVal(idx) = curr.imm {
                        let subfn = &mut bc.subfunctions[idx as usize];
                        subfn.globals = bc.globals.clone();
                        // Note: parent cannot be a Box pointing back to bc (ownership cycle)
                        subfn.parent = None;
                        subfn.nclosures = 0;
                        subfn.debuginfo = if let Some(ref sym) = curr.symbol {
                            Jv::object()
                                .object_set(Jv::string("name"), Jv::string(sym))
                        } else {
                            Jv::object()
                                .object_set(Jv::string("name"), Jv::null())
                        };
                        let mut params = Jv::array();
                        let mut param_ptr = curr
                            .arglist
                            .first
                            .as_mut()
                            .map(|b| b.as_mut() as *mut Inst);
                        while let Some(param_p) = param_ptr {
                            let param = &mut *param_p;
                            assert!(
                                param.op == CLOSURE_PARAM,
                                "param.op == CLOSURE_PARAM"
                            );
                            if let Some(bound_ptr) = param.bound_by {
                                assert!(bound_ptr == param_p, "param.bound_by == param");
                            }
                            param.imm = InstImmediate::IntVal(subfn.nclosures);
                            subfn.nclosures += 1;
                            param.compiled = Some(subfn.as_mut() as *mut Bytecode);
                            if let Some(ref sym) = param.symbol {
                                params = params.array_append(Jv::string(sym));
                            }
                            param_ptr = param
                                .next
                                .as_mut()
                                .map(|b| b.as_mut() as *mut Inst);
                        }
                        subfn.debuginfo = subfn
                            .debuginfo
                            .clone()
                            .object_set(Jv::string("params"), params);
                        let subfn_block = std::mem::take(&mut curr.subfn);
                        errors
                            += compile(
                                subfn.as_mut(),
                                subfn_block,
                                lf,
                                args.clone(),
                                env,
                            );
                        curr.subfn = gen_noop();
                    }
                }
                curr_ptr = curr.next.as_mut().map(|b| b.as_mut() as *mut Inst);
            }
        }
    } else {
        bc.nsubfunctions = 0;
        bc.subfunctions = Vec::new();
    }
    let mut code = vec![0u16; bc.codelen as usize];
    pos = 0;
    let mut constant_pool = Jv::array();
    let mut maxvar: i32 = -1;
    if errors == 0 {
        let mut curr_ptr = b.first.as_ref().map(|b| b.as_ref() as *const Inst);
        while let Some(ptr) = curr_ptr {
            unsafe {
                let curr = &*ptr;
                let op_desc = opcode_describe(curr.op as u16);
                if op_desc.length == 0 {
                    curr_ptr = curr.next.as_ref().map(|b| b.as_ref() as *const Inst);
                    continue;
                }
                code[pos as usize] = curr.op as u16;
                pos += 1;
                assert!(
                    curr.op != CLOSURE_REF && curr.op != CLOSURE_PARAM,
                    "curr.op != CLOSURE_REF && curr.op != CLOSURE_PARAM"
                );
                if curr.op == CALL_BUILTIN {
                    if let Some(bound_ptr) = curr.bound_by {
                        assert!(
                            (* bound_ptr).op == CLOSURE_CREATE_C,
                            "curr.bound_by.op == CLOSURE_CREATE_C"
                        );
                    }
                    assert!(curr.arglist.first.is_none(), "!curr.arglist.first");
                    if let InstImmediate::IntVal(intval) = curr.imm {
                        code[pos as usize] = intval as u16;
                        pos += 1;
                    }
                    if let Some(bound_ptr) = curr.bound_by {
                        if debug_compile {
                            eprintln!("  CODEGEN CALL_BUILTIN: bound_ptr.imm = {:?}", (*bound_ptr).imm);
                        }
                        if let InstImmediate::IntVal(intval) = (*bound_ptr).imm {
                            code[pos as usize] = intval as u16;
                            pos += 1;
                        } else {
                            if debug_compile {
                                eprintln!("  CODEGEN CALL_BUILTIN: ERROR - bound_ptr.imm is not IntVal!");
                            }
                        }
                    }
                } else if curr.op == CALL_JQ {
                    if let Some(bound_ptr) = curr.bound_by {
                        assert!(
                            (* bound_ptr).op == CLOSURE_CREATE || (*
                            bound_ptr).op == CLOSURE_PARAM,
                            "curr.bound_by.op == CLOSURE_CREATE || curr.bound_by.op == CLOSURE_PARAM"
                        );
                        if let InstImmediate::IntVal(intval) = curr.imm {
                            code[pos as usize] = intval as u16;
                            pos += 1;
                        }
                        let level = nesting_level(bc, bound_ptr);
                        code[pos as usize] = level as u16;
                        pos += 1;
                        if let InstImmediate::IntVal(intval) = (*bound_ptr).imm {
                            let flag = if (*bound_ptr).op == CLOSURE_CREATE {
                                0x1000
                            } else {
                                0
                            };
                            code[pos as usize] = (intval as u16) | flag;
                            pos += 1;
                        }
                        let mut arg_ptr = curr
                            .arglist
                            .first
                            .as_ref()
                            .map(|b| b.as_ref() as *const Inst);
                        while let Some(arg_p) = arg_ptr {
                            let arg = &*arg_p;
                            assert!(
                                arg.op == CLOSURE_REF,
                                "arg.op == CLOSURE_REF"
                            );
                            if let Some(arg_bound_ptr) = arg.bound_by {
                                assert!(
                                    (* arg_bound_ptr).op == CLOSURE_CREATE,
                                    "arg.bound_by.op == CLOSURE_CREATE"
                                );
                                let level = nesting_level(bc, arg_bound_ptr);
                                code[pos as usize] = level as u16;
                                pos += 1;
                                if let InstImmediate::IntVal(intval) = (*arg_bound_ptr).imm
                                {
                                    code[pos as usize] = (intval as u16) | 0x1000;
                                    pos += 1;
                                }
                            }
                            arg_ptr = arg
                                .next
                                .as_ref()
                                .map(|b| b.as_ref() as *const Inst);
                        }
                    }
                } else if (op_desc.flags & OP_HAS_CONSTANT) != 0
                    && (op_desc.flags & OP_HAS_VARIABLE) != 0
                {
                    let pool_idx = constant_pool.clone().array_length();
                    code[pos as usize] = pool_idx as u16;
                    pos += 1;
                    if let InstImmediate::Constant(ref c) = curr.imm {
                        constant_pool = constant_pool.array_append(c.clone());
                    }
                    if let Some(bound_ptr) = curr.bound_by {
                        let level = nesting_level(bc, bound_ptr);
                        code[pos as usize] = level as u16;
                        pos += 1;
                        if let InstImmediate::IntVal(intval) = (*bound_ptr).imm {
                            let var = intval as u16;
                            code[pos as usize] = var;
                            pos += 1;
                            if intval > maxvar {
                                maxvar = intval;
                            }
                        }
                    }
                } else if (op_desc.flags & OP_HAS_CONSTANT) != 0 {
                    let pool_idx = constant_pool.clone().array_length();
                    code[pos as usize] = pool_idx as u16;
                    pos += 1;
                    if let InstImmediate::Constant(ref c) = curr.imm {
                        if debug_compile {
                            eprintln!("  COMPILE: adding constant kind={:?} to pool at idx={}", jv_get_kind(c), pool_idx);
                            eprintln!("  COMPILE: before append pool.size={}", constant_pool.size);
                        }
                        constant_pool = constant_pool.array_append(c.clone());
                        if debug_compile {
                            eprintln!("  COMPILE: after append pool.size={}", constant_pool.size);
                        }
                    } else {
                        if debug_compile { eprintln!("  COMPILE: OP_HAS_CONSTANT but imm is not Constant, adding null"); }
                        constant_pool = constant_pool.array_append(Jv::null());
                    }
                } else if (op_desc.flags & OP_HAS_VARIABLE) != 0 {
                    if let Some(bound_ptr) = curr.bound_by {
                        let level = nesting_level(bc, bound_ptr);
                        code[pos as usize] = level as u16;
                        pos += 1;
                        if let InstImmediate::IntVal(intval) = (*bound_ptr).imm {
                            let var = intval as u16;
                            code[pos as usize] = var;
                            pos += 1;
                            if intval > maxvar {
                                maxvar = intval;
                            }
                        }
                    }
                } else if (op_desc.flags & OP_HAS_BRANCH) != 0 {
                    if let InstImmediate::Target(Some(target_ptr)) = curr.imm {
                        let target = &*target_ptr;
                        assert!(
                            target.bytecode_pos != - 1,
                            "curr.imm.target.bytecode_pos != -1"
                        );
                        assert!(
                            target.bytecode_pos > pos,
                            "curr.imm.target.bytecode_pos > pos"
                        );
                        code[pos as usize] = (target.bytecode_pos - (pos + 1)) as u16;
                        pos += 1;
                    }
                } else if op_desc.length > 1 {
                    panic!("codegen not implemented for op={} name={} flags={} length={}", curr.op, op_desc.name, op_desc.flags, op_desc.length);
                }
                curr_ptr = curr.next.as_ref().map(|b| b.as_ref() as *const Inst);
            }
        }
    }
    if debug_compile {
        eprintln!("  COMPILE: final constant_pool.size={} array_length={}", constant_pool.size, constant_pool.array_length());
        for i in 0..constant_pool.array_length() {
            let copy = jv_copy(&constant_pool);
            eprintln!("    about to get idx={} from copy.size={}", i, copy.size);
            let c = crate::jv::jv_array_get(copy, i);
            eprintln!("    constant_pool[{}] kind={:?}", i, jv_get_kind(&c));
        }
    }
    bc.constants = constant_pool;
    bc.nlocals = maxvar + 2;
    bc.code = code;
    block_free(b);
    errors
}
/// List functions in a block
pub fn block_list_funcs(body: Block, omit_underscores: i32) -> Jv {
    let mut funcs = Jv::object();
    let mut curr_ptr = body.first.as_ref().map(|b| b.as_ref() as *const Inst);
    while let Some(ptr) = curr_ptr {
        unsafe {
            let pos = &*ptr;
            if pos.op == CLOSURE_CREATE
                || pos.op == CLOSURE_CREATE_C
            {
                if let Some(ref symbol) = pos.symbol {
                    if omit_underscores == 0 || !symbol.starts_with('_') {
                        let key = format!("{}/{}", symbol, pos.nformals);
                        funcs = funcs.object_set(Jv::string(&key), Jv::null());
                    }
                }
            }
            curr_ptr = pos.next.as_ref().map(|b| b.as_ref() as *const Inst);
        }
    }
    jv_keys_unsorted(funcs)
}
/// Generate location information for a block
pub fn gen_location(loc: Location, l: &mut Locfile, mut b: Block) -> Block {
    let mut curr_ptr = b.first.as_mut().map(|b| b.as_mut() as *mut Inst);
    while let Some(ptr) = curr_ptr {
        unsafe {
            let i = &mut *ptr;
            if i.source.start == UNKNOWN_LOCATION.start
                && i.source.end == UNKNOWN_LOCATION.end
            {
                i.source = loc;
                // Note: locfile reference not stored in transpiled version
                i.locfile = None;
            }
            curr_ptr = i.next.as_mut().map(|b| b.as_mut() as *mut Inst);
        }
    }
    b
}
/// Check if block has only binders and imports
pub fn block_has_only_binders_and_imports(binders: Block, bindflags: i32) -> i32 {
    let bindflags = bindflags | OP_HAS_BINDING;
    let mut curr_ptr = binders.first.as_ref().map(|b| b.as_ref() as *const Inst);
    while let Some(ptr) = curr_ptr {
        unsafe {
            let curr = &*ptr;
            let op_desc = opcode_describe(curr.op as u16);
            if (op_desc.flags & bindflags) != bindflags && curr.op != DEPS
                && curr.op != MODULEMETA
            {
                return 0;
            }
            curr_ptr = curr.next.as_ref().map(|b| b.as_ref() as *const Inst);
        }
    }
    1
}
/// Set the target of a branch instruction
pub fn inst_set_target(b: &mut Block, target: &Block) {
    assert!(block_is_single(b), "block_is_single(b)");
    if let Some(ref mut first) = b.first {
        let op_desc = opcode_describe(first.op as u16);
        assert!(
            (op_desc.flags & OP_HAS_BRANCH) != 0,
            "opcode_describe(b.first.op).flags & OP_HAS_BRANCH"
        );
        assert!(target.last.is_some(), "target.last");
        first.imm = InstImmediate::Target(target.last);
    }
}
/// Generate an import instruction
pub fn gen_import(name: &str, as_name: Option<&str>, is_data: i32) -> Block {
    let mut inst = inst_new(DEPS);
    let mut meta = Jv::object();
    if let Some(alias) = as_name {
        meta = jv_object_set(meta, jv_string("as"), jv_string(alias));
    }
    meta = jv_object_set(
        meta,
        jv_string("is_data"),
        if is_data != 0 { jv_true() } else { jv_false() },
    );
    meta = jv_object_set(meta, jv_string("relpath"), jv_string(name));
    inst.imm = InstImmediate::Constant(meta);
    inst_block(inst)
}
/// Take imports from a block
pub fn block_take_imports(body: &mut Block) -> Jv {
    let mut imports = jv_array();
    if let Some(ref first) = body.first {
        if first.op == TOP {
            if let Some(ref next) = first.next {
                assert!(
                    next.op != MODULEMETA && next.op != DEPS, "unexpected structure"
                );
            }
        }
    }
    while let Some(ref first) = body.first {
        if first.op != MODULEMETA && first.op != DEPS {
            break;
        }
        let dep = block_take(body);
        if let Some(dep_inst) = dep {
            if dep_inst.op == DEPS {
                if let InstImmediate::Constant(ref constant) = dep_inst.imm {
                    imports = jv_array_append(imports, jv_copy(constant));
                }
            }
            inst_free(Box::new(dep_inst));
        }
    }
    imports
}
/// Generate a regular parameter
pub fn gen_param_regular(name: &str) -> Block {
    gen_op_unbound(CLOSURE_PARAM_REGULAR, name)
}
/// Generate an OR expression
pub fn gen_or(a: Block, b: Block) -> Block {
    block_join(
        block_join(gen_op_simple(DUP), a),
        gen_condbranch(
            block_join(gen_op_simple(POP), gen_const(jv_true())),
            block_join(
                block_join(gen_op_simple(POP), b),
                gen_condbranch(gen_const(jv_true()), gen_const(jv_false())),
            ),
        ),
    )
}
/// Get unbound variables from a block
pub fn block_get_unbound_vars(b: &Block, vars: &mut Jv) {
    if jv_get_kind(vars) != JvKind::Object {
        return;
    }
    let mut current: Option<&Inst> = b.first.as_ref().map(|b| b.as_ref());
    while let Some(inst) = current {
        if inst.subfn.first.is_some() {
            block_get_unbound_vars(&inst.subfn, vars);
            current = inst.next.as_ref().map(|b| b.as_ref());
            continue;
        }
        if (inst.op == STOREV || inst.op == STOREVN)
            && inst.bound_by.is_none()
        {
            if let Some(ref symbol) = inst.symbol {
                *vars = jv_object_set(vars.clone(), jv_string(symbol), jv_true());
            }
        }
        current = inst.next.as_ref().map(|b| b.as_ref());
    }
}
/// Check if block has a main (TOP) instruction
pub fn block_has_main(top: &Block) -> bool {
    let mut current: Option<&Inst> = top.first.as_ref().map(|b| b.as_ref());
    while let Some(inst) = current {
        if inst.op == TOP {
            return true;
        }
        current = inst.next.as_ref().map(|b| b.as_ref());
    }
    false
}
fn jv_string(s: &str) -> Jv {
    crate::jv::Jv::string(s)
}
fn jv_true() -> Jv {
    crate::jv::jv_true()
}
fn jv_false() -> Jv {
    crate::jv::jv_false()
}
fn jv_array() -> Jv {
    crate::jv::jv_array()
}
fn jv_array_append(arr: Jv, value: Jv) -> Jv {
    crate::jv::jv_array_append(arr, value)
}
fn jv_array_length(arr: Jv) -> i32 {
    arr.size
}
/// Generate a foreach construct
pub fn gen_foreach(
    source: Block,
    matcher: Block,
    init: Block,
    update: Block,
    extract: Block,
) -> Block {
    // Create variables for multiple references
    let state_var = gen_op_var_fresh(STOREV, "foreach");
    let state_var_ref1 = gen_op_unbound(LOADVN, "foreach");
    let state_var_ref2 = gen_op_unbound(STOREV, "foreach");
    let output = gen_op_targetlater(JUMP);
    // Save pointer to output instruction BEFORE joining it (matches gen_both pattern)
    let output_ptr = output.first.as_ref().map(|b| b.as_ref() as *const Inst as *mut Inst);

    // Build loop_inner
    let inner1 = block_join(state_var_ref1, update);
    let inner2 = block_join(inner1, gen_op_simple(DUP));
    let inner3 = block_join(inner2, state_var_ref2);
    let inner4 = block_join(inner3, extract);
    let loop_inner = block_join(inner4, output);

    // Build loop_block
    let loop1 = block_join(gen_op_simple(DUPN), source);
    let loop_block = block_join(loop1, bind_alternation_matchers(matcher, loop_inner));

    // Build foreach
    let foreach1 = block_join(gen_op_simple(DUP), init);
    let foreach2 = block_join(foreach1, state_var);
    let foreach3 = block_join(foreach2, gen_op_target(FORK, &loop_block));
    let foreach4 = block_join(foreach3, loop_block);
    let foreach = block_join(foreach4, gen_op_simple(BACKTRACK));

    // C: inst_set_target(output, foreach); // make that JUMP go past the BACKTRACK
    if let Some(ptr) = output_ptr {
        unsafe {
            (*ptr).imm = InstImmediate::Target(foreach.last);
        }
    }
    foreach
}
/// Take first instruction from block (returns raw pointer for compatibility)
pub fn block_take_ptr(b: &mut Block) -> Option<Box<Inst>> {
    if b.first.is_none() {
        return None;
    }
    let mut first = b.first.take()?;
    if first.next.is_some() {
        b.first = first.next.take();
        if let Some(ref mut new_first) = b.first {
            new_first.prev = None;
        }
    } else {
        b.first = None;
        b.last = None;
    }
    Some(first)
}
/// Generate import metadata
pub fn gen_import_meta(import: Block, metadata: Block) -> Block {
    assert!(
        block_is_single(& import) && import.first.as_ref().map_or(false, | i | i.op ==
        DEPS), "block_is_single(import) && import.first->op == DEPS"
    );
    assert!(
        block_is_const(&metadata) != 0 && block_const_kind(&metadata) ==
        JvKind::Object,
        "block_is_const(metadata) && block_const_kind(metadata) == JV_KIND_OBJECT"
    );
    let mut result = import;
    if let Some(ref mut inst) = result.first {
        let metadata_const = block_const(&metadata);
        if let InstImmediate::Constant(ref existing) = inst.imm {
            // Merge metadata into existing constant
            let merged = jv_object_merge(metadata_const.clone(), existing.clone());
            inst.imm = InstImmediate::Constant(merged);
        } else {
            inst.imm = InstImmediate::Constant(metadata_const);
        }
    }
    block_free(metadata);
    result
}
/// Generate a dictionary pair
pub fn gen_dictpair(k: Block, v: Block) -> Block {
    block_join(block_join(gen_subexp(k), gen_subexp(v)), gen_op_simple(INSERT))
}
/// Generate a constant object from expression
pub fn gen_const_object(expr: Block) -> Block {
    let mut is_const = true;
    let mut o = Jv::object();
    let mut k = Jv::null();
    let mut v = Jv::null();
    let mut current = expr.first.as_ref();
    while let Some(inst) = current {
        if inst.op == PUSHK_UNDER {
            if let InstImmediate::Constant(ref c) = inst.imm {
                k = c.copy();
            }
            current = inst.next.as_ref();
            if current.is_none() {
                is_const = false;
                break;
            }
        } else if inst.op != SUBEXP_BEGIN {
            is_const = false;
            break;
        } else {
            let next1 = inst.next.as_ref();
            if next1.is_none() || next1.unwrap().op != LOADK {
                is_const = false;
                break;
            }
            let next2 = next1.unwrap().next.as_ref();
            if next2.is_none() || next2.unwrap().op != SUBEXP_END {
                is_const = false;
                break;
            }
            if let InstImmediate::Constant(ref c) = next1.unwrap().imm {
                k = c.copy();
            }
            current = next2.unwrap().next.as_ref();
            if current.is_none() {
                is_const = false;
                break;
            }
        }
        let inst = current.unwrap();
        if inst.op == PUSHK_UNDER {
            if let InstImmediate::Constant(ref c) = inst.imm {
                v = c.copy();
            }
            current = inst.next.as_ref();
        } else if inst.op != SUBEXP_BEGIN {
            is_const = false;
            break;
        } else {
            let next1 = inst.next.as_ref();
            if next1.is_none() || next1.unwrap().op != LOADK {
                is_const = false;
                break;
            }
            let next2 = next1.unwrap().next.as_ref();
            if next2.is_none() || next2.unwrap().op != SUBEXP_END {
                is_const = false;
                break;
            }
            if let InstImmediate::Constant(ref c) = next1.unwrap().imm {
                v = c.copy();
            }
            current = next2.unwrap().next.as_ref();
        }
        if current.is_none() || current.unwrap().op != INSERT {
            is_const = false;
            break;
        }
        if k.get_kind() != JvKind::String {
            is_const = false;
            break;
        }
        o = o.object_set(k, v);
        k = Jv::null();
        v = Jv::null();
        current = current.unwrap().next.as_ref();
    }
    if !is_const {
        o.free();
        k.free();
        v.free();
        return Block::new();
    }
    block_free(expr);
    gen_const(o)
}
/// Block bind self
pub fn block_bind_self(binder: Block, bindflags: i32) -> Block {
    assert!(
        block_has_only_binders(& binder, bindflags),
        "block_has_only_binders(binder, bindflags)"
    );
    let bindflags = bindflags | OP_HAS_BINDING;
    let mut body = gen_noop();
    let mut binder = binder;
    while let Some(curr) = block_take_last(&mut binder) {
        let mut b = inst_block(curr);
        // Perform binding using the inner function
        let mut any_unbound = 0i32;
        let _nrefs = block_bind_subblock_inner(&mut any_unbound, &mut b, &mut body, bindflags, 0);
        body = block_join(b, body);
    }
    body
}
/// Block bind library
pub fn block_bind_library(
    binder: Block,
    body: Block,
    bindflags: i32,
    libname: Option<&str>,
) -> Block {
    let bindflags = bindflags | OP_HAS_BINDING;
    let matchname = match libname {
        Some(name) if !name.is_empty() => format!("{}::", name),
        _ => String::new(),
    };
    assert!(
        block_has_only_binders(& binder, bindflags),
        "block_has_only_binders(binder, bindflags)"
    );
    let mut result_body = body;
    let mut current = binder.last;
    while let Some(curr_ptr) = current {
        let curr = unsafe { &mut *curr_ptr };
        let mut bindflags2 = bindflags;
        let has_var_or_const = false;
        if has_var_or_const {
            bindflags2 = OP_HAS_VARIABLE | OP_HAS_BINDING;
        }
        let original_symbol = curr.symbol.clone();
        if let Some(ref sym) = original_symbol {
            let temp_name = format!("{}{}", matchname, sym);
            curr.symbol = Some(temp_name);
            // Create a temporary block wrapper for the current instruction
            let mut inst_blk = Block {
                first: None,  // We don't own this instruction
                last: Some(curr_ptr),
            };
            let mut any_unbound = 0i32;
            let _nrefs = block_bind_subblock_inner(&mut any_unbound, &mut inst_blk, &mut result_body, bindflags2, 0);
            curr.symbol = original_symbol;
        }
        current = curr.prev;
    }
    result_body
}
/// Mark all referenced instructions in a block
pub fn block_mark_referenced(body: Block) {
    let mut saw_top = false;
    let mut current = body.last;
    while let Some(inst_ptr) = current {
        let inst = unsafe { &mut *inst_ptr };
        if saw_top {
            if let Some(bound_ptr) = inst.bound_by {
                if bound_ptr == inst_ptr && !inst.referenced {
                    current = inst.prev;
                    continue;
                }
            }
        }
        if inst.op == TOP {
            saw_top = true;
        }
        if let Some(bound_ptr) = inst.bound_by {
            unsafe {
                (*bound_ptr).referenced = true;
            }
        }
        block_mark_referenced(inst.arglist.clone());
        block_mark_referenced(inst.subfn.clone());
        current = inst.prev;
    }
}
/// Get module metadata from a block
pub fn block_module_meta(b: &Block) -> Jv {
    if let Some(ref first) = b.first {
        if first.op == MODULEMETA {
            if let InstImmediate::Constant(ref c) = first.imm {
                return jv_copy(c);
            }
        }
    }
    jv_null()
}
/// Generate a block that tries both alternatives
pub fn gen_both(a: Block, b: Block) -> Block {
    // Matches C: block jump = gen_op_targetlater(JUMP);
    //            block fork = gen_op_target(FORK, jump);
    //            block c = BLOCK(fork, a, jump, b);
    //            inst_set_target(jump, c);
    let jump = gen_op_targetlater(JUMP);
    // Save pointer to jump instruction BEFORE joining it
    let jump_ptr = jump.first.as_ref().map(|b| b.as_ref() as *const Inst as *mut Inst);
    let fork = gen_op_target(FORK, &jump);  // Pass reference, jump stays alive
    let mut c = block_join(fork, a);
    c = block_join(c, jump);  // jump is now part of c
    c = block_join(c, b);
    // Set jump's target to the end of c (after b)
    // Matches C: b.first->imm.target = target.last (compile.c:210)
    if let Some(ptr) = jump_ptr {
        unsafe {
            (*ptr).imm = InstImmediate::Target(c.last);
        }
    }
    c
}
fn jv_null() -> Jv {
    crate::jv::jv_null()
}
fn jv_object() -> Jv {
    crate::jv::jv_object()
}
fn jv_free(v: Jv) {
    crate::jv::jv_free(v)
}
fn jv_object_keys(obj: &Jv) -> Vec<Jv> {
    let mut keys = Vec::new();
    let mut iter = crate::jv::jv_object_iter(obj);
    while crate::jv::jv_object_iter_valid(obj, iter) {
        keys.push(crate::jv::jv_object_iter_key(obj, iter));
        iter = crate::jv::jv_object_iter_next(obj, iter);
    }
    keys
}
fn jv_string_value(v: &Jv) -> String {
    crate::jv::jv_string_value(v).to_string()
}
impl Block {
    pub fn new() -> Self {
        Block { first: None, last: None }
    }
    pub fn is_empty(&self) -> bool {
        self.first.is_none()
    }
    /// Check if block is a no-op (empty)
    pub fn is_noop(&self) -> bool {
        self.first.is_none()
    }
    pub fn from_inst(inst: Box<Inst>) -> Self {
        let raw_ptr = Box::into_raw(inst);
        Block {
            first: Some(unsafe { Box::from_raw(raw_ptr) }),
            last: Some(raw_ptr),
        }
    }
}
impl Default for Immediate {
    fn default() -> Self {
        Immediate::None
    }
}
impl CompileBlock {
    pub fn new() -> Self {
        CompileBlock {
            first: None,
            last: None,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.first.is_none()
    }
}
impl Default for ImmediateValue {
    fn default() -> Self {
        ImmediateValue::None
    }
}
impl Default for InstImmediate {
    fn default() -> Self {
        InstImmediate::None
    }
}
impl Locfile {
    pub fn locate(&self, source: &Location, fmt: &str, args: &[&str]) {
        let mut msg = fmt.to_string();
        for arg in args {
            if let Some(pos) = msg.find("%s") {
                msg.replace_range(pos..pos + 2, arg);
            } else if let Some(pos) = msg.find("%d") {
                msg.replace_range(pos..pos + 2, arg);
            }
        }
        eprintln!("{}: {} (at {}:{})", self.fname, msg, source.start, source.end);
    }
}
// Note: InstBlock from types has a different structure than local Block
// Methods for types::InstBlock removed - use Block instead
impl Inst {
    pub fn new(op: CompileOpcode) -> Self {
        Inst {
            op: op as Opcode,
            next: None,
            prev: None,
            imm: InstImmediate::None,
            symbol: None,
            nactuals: -1,
            nformals: -1,
            any_unbound: 0,
            bound_by: None,
            arglist: Block::default(),
            subfn: Block::default(),
            source: Location::default(),
            locfile: None,
            referenced: false,
            bytecode_pos: -1,
            target: None,
            compiled: None,
        }
    }
    fn imm_target_is_none_or_not_jump(&self) -> bool {
        match &self.imm {
            InstImmediate::Target(target) => {
                target.is_none() || target.map_or(true, |_t| false)
            }
            _ => true,
        }
    }
    fn imm_target_is_none_or_not_loadk(&self) -> bool {
        match &self.imm {
            InstImmediate::Target(target) => {
                target.is_none() || target.map_or(true, |_t| false)
            }
            _ => true,
        }
    }
}
