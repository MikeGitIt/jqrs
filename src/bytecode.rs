//! Module: bytecode
//!
//! Contains 8 transpiled functions:
//! - bytecode_operation_length:2760889882685720955:./src/bytecode.c
//! - dump_operation:16231873656675787963:./src/bytecode.c
//! - dump_code:12638258843262288919:./src/bytecode.c
//! - opcode_describe:1841883757805999828:./src/bytecode.c
//! - getlevel:5855220464582707951:./src/bytecode.c
//! - symbol_table_free:7667964176926874911:./src/bytecode.c
//! - bytecode_free:41937767254399033:./src/bytecode.c
//! - dump_disassembly:9522031834458701226:./src/bytecode.c
use crate::types::*;
use crate::jv::{jv_object_get, jv_array_get, jv_free};
use crate::jv_print::jv_dump;
// Re-export commonly used types for other modules
pub use crate::types::{Bytecode, Opcode, OpcodeDescription, Cfunction, SymbolTable};
/// Flags for opcode descriptions - matching C jq bytecode.h
pub const OP_HAS_CONSTANT: i32 = 2;
pub const OP_HAS_VARIABLE: i32 = 4;
pub const OP_HAS_BRANCH: i32 = 8;
pub const OP_HAS_BINDING: i32 = 1024;
/// Number of opcodes in the system
pub const NUM_OPCODES: usize = 256;
/// Invalid opcode description for error handling
static INVALID_OPCODE_DESCRIPTION: OpcodeDescription = OpcodeDescription {
    op: Opcode::Invalid,
    name: "INVALID",
    flags: 0,
    length: 1,
    stack_in: 0,
    stack_out: 0,
};
/// Opcode descriptions table - ALL opcodes matching C jq opcode order
lazy_static::lazy_static! {
    static ref OPCODE_DESCRIPTIONS: Vec<OpcodeDescription> = {
        let mut descs = vec![OpcodeDescription::default(); NUM_OPCODES];
        // Opcode order from compile.rs CompileOpcode enum
        descs[0] = OpcodeDescription { op: Opcode::Loadk, name: "LOADK", flags: OP_HAS_CONSTANT, length: 2, stack_in: 1, stack_out: 1 };
        descs[1] = OpcodeDescription { op: Opcode::Dup, name: "DUP", flags: 0, length: 1, stack_in: 1, stack_out: 2 };
        descs[2] = OpcodeDescription { op: Opcode::Dupn, name: "DUPN", flags: 0, length: 1, stack_in: 1, stack_out: 2 };
        descs[3] = OpcodeDescription { op: Opcode::Dup2, name: "DUP2", flags: 0, length: 1, stack_in: 2, stack_out: 4 };
        descs[4] = OpcodeDescription { op: Opcode::PushkUnder, name: "PUSHK_UNDER", flags: OP_HAS_CONSTANT, length: 2, stack_in: 1, stack_out: 2 };
        descs[5] = OpcodeDescription { op: Opcode::Pop, name: "POP", flags: 0, length: 1, stack_in: 1, stack_out: 0 };
        descs[6] = OpcodeDescription { op: Opcode::Loadv, name: "LOADV", flags: OP_HAS_VARIABLE | OP_HAS_BINDING, length: 3, stack_in: 0, stack_out: 1 };
        descs[7] = OpcodeDescription { op: Opcode::Loadvn, name: "LOADVN", flags: OP_HAS_VARIABLE | OP_HAS_BINDING, length: 3, stack_in: 0, stack_out: 1 };
        descs[8] = OpcodeDescription { op: Opcode::Storev, name: "STOREV", flags: OP_HAS_VARIABLE | OP_HAS_BINDING, length: 3, stack_in: 1, stack_out: 0 };
        descs[9] = OpcodeDescription { op: Opcode::StoreGlobal, name: "STORE_GLOBAL", flags: OP_HAS_CONSTANT | OP_HAS_VARIABLE | OP_HAS_BINDING, length: 4, stack_in: 1, stack_out: 0 };
        descs[10] = OpcodeDescription { op: Opcode::Index, name: "INDEX", flags: 0, length: 1, stack_in: 2, stack_out: 1 };
        descs[11] = OpcodeDescription { op: Opcode::IndexOpt, name: "INDEX_OPT", flags: 0, length: 1, stack_in: 2, stack_out: 1 };
        descs[12] = OpcodeDescription { op: Opcode::Each, name: "EACH", flags: 0, length: 1, stack_in: 1, stack_out: 1 };
        descs[13] = OpcodeDescription { op: Opcode::EachOpt, name: "EACH_OPT", flags: 0, length: 1, stack_in: 1, stack_out: 1 };
        descs[14] = OpcodeDescription { op: Opcode::Fork, name: "FORK", flags: OP_HAS_BRANCH, length: 2, stack_in: 0, stack_out: 0 };
        descs[15] = OpcodeDescription { op: Opcode::TryBegin, name: "TRY_BEGIN", flags: OP_HAS_BRANCH, length: 2, stack_in: 0, stack_out: 0 };
        descs[16] = OpcodeDescription { op: Opcode::TryEnd, name: "TRY_END", flags: 0, length: 1, stack_in: 0, stack_out: 0 };
        descs[17] = OpcodeDescription { op: Opcode::Jump, name: "JUMP", flags: OP_HAS_BRANCH, length: 2, stack_in: 0, stack_out: 0 };
        descs[18] = OpcodeDescription { op: Opcode::JumpF, name: "JUMP_F", flags: OP_HAS_BRANCH, length: 2, stack_in: 1, stack_out: 0 };
        descs[19] = OpcodeDescription { op: Opcode::Backtrack, name: "BACKTRACK", flags: 0, length: 1, stack_in: 0, stack_out: 0 };
        descs[20] = OpcodeDescription { op: Opcode::Append, name: "APPEND", flags: OP_HAS_BINDING, length: 2, stack_in: 1, stack_out: 0 };
        descs[21] = OpcodeDescription { op: Opcode::Insert, name: "INSERT", flags: 0, length: 1, stack_in: 4, stack_out: 2 };
        descs[22] = OpcodeDescription { op: Opcode::Range, name: "RANGE", flags: OP_HAS_BRANCH, length: 2, stack_in: 1, stack_out: 1 };
        descs[23] = OpcodeDescription { op: Opcode::SubexpBegin, name: "SUBEXP_BEGIN", flags: 0, length: 1, stack_in: 1, stack_out: 2 };
        descs[24] = OpcodeDescription { op: Opcode::SubexpEnd, name: "SUBEXP_END", flags: 0, length: 1, stack_in: 2, stack_out: 1 };
        descs[25] = OpcodeDescription { op: Opcode::PathBegin, name: "PATH_BEGIN", flags: 0, length: 1, stack_in: 0, stack_out: 0 };
        descs[26] = OpcodeDescription { op: Opcode::PathEnd, name: "PATH_END", flags: 0, length: 1, stack_in: 0, stack_out: 1 };
        descs[27] = OpcodeDescription { op: Opcode::CallBuiltin, name: "CALL_BUILTIN", flags: 0, length: 3, stack_in: 0, stack_out: 1 };
        descs[28] = OpcodeDescription { op: Opcode::CallJq, name: "CALL_JQ", flags: 0, length: 4, stack_in: 0, stack_out: 1 };
        descs[29] = OpcodeDescription { op: Opcode::Ret, name: "RET", flags: 0, length: 1, stack_in: 1, stack_out: 1 };
        descs[30] = OpcodeDescription { op: Opcode::TailCallJq, name: "TAIL_CALL_JQ", flags: 0, length: 4, stack_in: 0, stack_out: 1 };
        descs[31] = OpcodeDescription { op: Opcode::ClosureParam, name: "CLOSURE_PARAM", flags: OP_HAS_BINDING, length: 2, stack_in: 0, stack_out: 0 };
        descs[32] = OpcodeDescription { op: Opcode::ClosureRef, name: "CLOSURE_REF", flags: OP_HAS_BINDING, length: 2, stack_in: 0, stack_out: 0 };
        descs[33] = OpcodeDescription { op: Opcode::ClosureCreate, name: "CLOSURE_CREATE", flags: 0, length: 3, stack_in: 0, stack_out: 0 };
        descs[34] = OpcodeDescription { op: Opcode::ClosureCreateC, name: "CLOSURE_CREATE_C", flags: OP_HAS_BINDING, length: 3, stack_in: 0, stack_out: 0 };
        descs[35] = OpcodeDescription { op: Opcode::Top, name: "TOP", flags: 0, length: 1, stack_in: 0, stack_out: 0 };
        descs[36] = OpcodeDescription { op: Opcode::ClosureParamRegular, name: "CLOSURE_PARAM_REGULAR", flags: OP_HAS_BINDING, length: 2, stack_in: 0, stack_out: 0 };
        descs[37] = OpcodeDescription { op: Opcode::Deps, name: "DEPS", flags: 0, length: 1, stack_in: 0, stack_out: 0 };
        descs[38] = OpcodeDescription { op: Opcode::Modulemeta, name: "MODULEMETA", flags: 0, length: 1, stack_in: 0, stack_out: 1 };
        descs[39] = OpcodeDescription { op: Opcode::Genlabel, name: "GENLABEL", flags: 0, length: 1, stack_in: 0, stack_out: 1 };
        descs[40] = OpcodeDescription { op: Opcode::DestructureAlt, name: "DESTRUCTURE_ALT", flags: OP_HAS_BRANCH, length: 2, stack_in: 0, stack_out: 0 };
        descs[41] = OpcodeDescription { op: Opcode::Storevn, name: "STOREVN", flags: OP_HAS_VARIABLE | OP_HAS_BINDING, length: 3, stack_in: 1, stack_out: 1 };
        descs[42] = OpcodeDescription { op: Opcode::Errork, name: "ERRORK", flags: OP_HAS_CONSTANT, length: 2, stack_in: 0, stack_out: 0 };
        descs
    };
}
/// Get the bytecode at a specific level in the hierarchy
pub fn getlevel(bc: &Bytecode, mut level: u16) -> &Bytecode {
    let mut current = bc;
    while level > 0 {
        if let Some(ref parent) = current.parent {
            current = parent.as_ref();
        } else {
            break;
        }
        level -= 1;
    }
    current
}
/// Standalone function matching C API signature
/// Frees all resources associated with a symbol table.
pub fn symbol_table_free(syms: Box<SymbolTable>) {
    let syms = *syms;
    // cfunctions is a Vec which will be dropped automatically
    drop(syms.cfunctions);
    jv_free(syms.cfunc_names);
}
/// Get the description for an opcode
pub fn opcode_describe(op: u16) -> &'static OpcodeDescription {
    if (op as usize) < NUM_OPCODES {
        &OPCODE_DESCRIPTIONS[op as usize]
    } else {
        &INVALID_OPCODE_DESCRIPTION
    }
}
/// Calculate the length of a bytecode operation
pub fn bytecode_operation_length(codeptr: &[u16]) -> i32 {
    if codeptr.is_empty() {
        return 1;
    }
    let opcode = codeptr[0];
    let mut length = opcode_describe(opcode).length;
    if opcode == Opcode::CallJq as u16 || opcode == Opcode::TailCallJq as u16 {
        if codeptr.len() > 1 {
            length += (codeptr[1] as i32) * 2;
        }
    }
    length
}
/// Dump a single bytecode operation
pub fn dump_operation(bc: &Bytecode, code_offset: usize) {
    if code_offset >= bc.code.len() {
        return;
    }
    let mut pc = code_offset;
    print!("{:04} ", pc);
    let op = opcode_describe(bc.code[pc]);
    pc += 1;
    print!("{}", op.name);
    if op.length > 1 && pc < bc.code.len() {
        let imm = bc.code[pc] as i32;
        pc += 1;
        if op.op == Opcode::CallJq || op.op == Opcode::TailCallJq {
            for _ in 0..=imm {
                if pc + 1 >= bc.code.len() {
                    break;
                }
                let level = bc.code[pc];
                pc += 1;
                let mut idx = bc.code[pc];
                pc += 1;
                let level_bc = getlevel(bc, level);
                let name = if idx & 0x1000 != 0 {
                    idx &= !0x1000;
                    if (idx as usize) < level_bc.subfunctions.len() {
                        let debuginfo_copy = level_bc.subfunctions[idx as usize].debuginfo.copy();
                        jv_object_get(&debuginfo_copy, Jv::string("name"))
                    } else {
                        Jv::new()
                    }
                } else {
                    let debuginfo_copy = level_bc.debuginfo.copy();
                    let params = jv_object_get(&debuginfo_copy, Jv::string("params"));
                    jv_array_get(params, idx as i32)
                };
                print!(" {}:{}", name.string_value().unwrap_or("<unknown>"), idx);
                name.free();
                if level > 0 {
                    print!("^{}", level);
                }
            }
        } else if op.op == Opcode::CallBuiltin {
            if pc < bc.code.len() {
                let func = bc.code[pc] as i32;
                pc += 1;
                if let Some(ref globals) = &bc.globals {
                    let name = jv_array_get(globals.cfunc_names.copy(), func);
                    print!(" {}", name.string_value().unwrap_or("<unknown>"));
                    name.free();
                }
            }
        } else if op.flags & OP_HAS_BRANCH != 0 {
            print!(" {:04}", pc as i32 + imm);
        } else if op.flags & OP_HAS_CONSTANT != 0 {
            print!(" ");
            jv_dump(jv_array_get(bc.constants.copy(), imm), 0);
        } else if op.flags & OP_HAS_VARIABLE != 0 {
            if pc < bc.code.len() {
                let v = bc.code[pc];
                pc += 1;
                let level_bc = getlevel(bc, imm as u16);
                let debuginfo_copy = level_bc.debuginfo.copy();
                let locals = jv_object_get(&debuginfo_copy, Jv::string("locals"));
                let name = jv_array_get(locals, v as i32);
                print!(" ${}:{}", name.string_value().unwrap_or("<unknown>"), v);
                name.free();
                if imm > 0 {
                    print!("^{}", imm);
                }
            }
        } else {
            print!(" {}", imm);
        }
    }
}
/// Dump bytecode instructions with indentation
fn dump_code(indent: i32, bc: &Bytecode) {
    let mut pc = 0usize;
    while (pc as i32) < bc.codelen {
        print!("{:width$}", "", width = indent as usize);
        dump_operation(bc, pc);
        println!();
        pc += bytecode_operation_length(&bc.code[pc..]) as usize;
    }
}
/// Dump full disassembly of bytecode including subfunctions
pub fn dump_disassembly(indent: i32, bc: &Bytecode) {
    if bc.nclosures > 0 {
        print!("{:width$}[params: ", "", width = indent as usize);
        let debuginfo_copy = bc.debuginfo.copy();
        let params = jv_object_get(&debuginfo_copy, Jv::string("params"));
        for i in 0..bc.nclosures {
            if i > 0 {
                print!(", ");
            }
            let name = jv_array_get(params.copy(), i);
            print!("{}", name.string_value().unwrap_or("<unknown>"));
            name.free();
        }
        params.free();
        println!("]");
    }
    dump_code(indent, bc);
    for i in 0..bc.nsubfunctions as usize {
        if i < bc.subfunctions.len() {
            let subfn = &bc.subfunctions[i];
            let debuginfo_copy = subfn.debuginfo.copy();
            let name = jv_object_get(&debuginfo_copy, Jv::string("name"));
            println!(
                "{:width$}{}:{}:", "", name.string_value().unwrap_or("<unknown>"), i, width = indent as usize
            );
            name.free();
            dump_disassembly(indent + 2, subfn);
        }
    }
}
/// Free bytecode and all associated resources
pub fn bytecode_free(bc: Option<Box<Bytecode>>) {
    if let Some(mut bytecode) = bc {
        bytecode.constants.free();
        for subfn in bytecode.subfunctions.drain(..) {
            bytecode_free(Some(subfn));
        }
        if bytecode.parent.is_none() {
            if let Some(globals) = bytecode.globals.take() {
                symbol_table_free(globals);
            }
        }
        bytecode.debuginfo.free();
    }
}
/// Check if a jv value is valid
pub fn jv_is_valid(x: &Jv) -> bool {
    x.kind_flags != 0
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_getlevel_zero() {
        let bc = Bytecode::default();
        let result = getlevel(&bc, 0);
        assert_eq!(result.codelen, bc.codelen);
    }
    #[test]
    fn test_getlevel_with_parent() {
        let mut parent = Bytecode::default();
        parent.codelen = 42;
        let mut child = Bytecode::default();
        child.codelen = 10;
        child.parent = Some(Box::new(parent));
        let result = getlevel(&child, 1);
        assert_eq!(result.codelen, 42);
    }
    #[test]
    fn test_getlevel_exceeds_depth() {
        let bc = Bytecode::default();
        let result = getlevel(&bc, 5);
        assert_eq!(result.codelen, bc.codelen);
    }
    #[test]
    fn test_symbol_table_free() {
        let syms = Box::new(SymbolTable {
            cfunctions: vec![Cfunction::default()],
            ncfunctions: 1,
            cfunc_names: Jv::default(),
        });
        symbol_table_free(syms);
    }
    #[test]
    fn test_symbol_table_free_opt_some() {
        let syms = Some(Box::new(SymbolTable::default()));
        symbol_table_free_opt(syms);
    }
    #[test]
    fn test_symbol_table_free_opt_none() {
        let syms: Option<Box<SymbolTable>> = None;
        symbol_table_free_opt(syms);
    }
}
fn jv_mem_free<T>(_ptr: T) {}
/// Alternative symbol_table_free that takes Option<Box<SymbolTable>>
/// for cases where the symbol table might be null
pub fn symbol_table_free_opt(syms: Option<Box<SymbolTable>>) {
    if let Some(syms) = syms {
        symbol_table_free(syms);
    }
}
impl Default for Cfunction {
    fn default() -> Self {
        Self {
            fptr: None,
            name: None,
            nargs: 0,
        }
    }
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            cfunctions: Vec::new(),
            ncfunctions: 0,
            cfunc_names: Jv::new(),
        }
    }
    /// Free all resources associated with the symbol table.
    /// In Rust, this is typically handled by Drop, but we provide an explicit
    /// method for API compatibility.
    pub fn symbol_table_free(self) {
        jv_free(self.cfunc_names);
    }
}
impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        match value {
            0 => Opcode::CallJq,
            1 => Opcode::TailCallJq,
            2 => Opcode::CallBuiltin,
            _ => Opcode::Invalid,
        }
    }
}
impl Default for Bytecode {
    fn default() -> Self {
        Self::new()
    }
}
impl Bytecode {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            codelen: 0,
            nlocals: 0,
            nclosures: 0,
            constants: crate::jv::jv_array(),
            globals: None,
            subfunctions: Vec::new(),
            nsubfunctions: 0,
            parent: None,
            parent_ptr: None,
            debuginfo: crate::jv::jv_object(),
        }
    }
    /// Traverse up the bytecode parent chain by the specified number of levels.
    /// Returns a reference to the bytecode at that level.
    ///
    /// # Arguments
    /// * `level` - Number of parent levels to traverse
    ///
    /// # Returns
    /// Reference to the bytecode at the specified level, or self if level is 0
    pub fn getlevel(&self, mut level: i32) -> &Bytecode {
        let mut current = self;
        while level > 0 {
            if let Some(ref parent) = current.parent {
                current = parent.as_ref();
                level -= 1;
            } else {
                break;
            }
        }
        current
    }
    /// Mutable version of getlevel for cases where mutation is needed
    pub fn getlevel_mut(&mut self, mut level: i32) -> &mut Bytecode {
        if level <= 0 {
            return self;
        }
        let mut current = self;
        while level > 0 {
            if current.parent.is_some() {
                current = current.parent.as_mut().unwrap().as_mut();
                level -= 1;
            } else {
                break;
            }
        }
        current
    }
}
