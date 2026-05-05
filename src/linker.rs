//! Module: linker
//!
//! Contains 10 transpiled functions:
//! - default_search:15596436287665727124:./src/linker.c
//! - load_library:8768018577978306805:./src/linker.c
//! - build_lib_search_chain:18296652795391303538:./src/linker.c
//! - path_is_relative:1362990516457397603:./src/linker.c
//! - load_module_meta:11211638649970574512:./src/linker.c
//! - validate_relpath:795035018636235442:./src/linker.c
//! - jv_basename:5373460694315646083:./src/linker.c
//! - load_program:10946916831539741272:./src/linker.c
//! - find_lib:2247852630402745128:./src/linker.c
//! - process_dependencies:6190041052170502386:./src/linker.c
use std::path::Path;
use std::ffi::OsStr;
use crate::jv::{
    Jv, JvKind, jv_array, jv_array_append, jv_array_concat, jv_array_get,
    jv_array_length, jv_copy, jv_equal, jv_free, jv_get_kind, jv_invalid_get_msg,
    jv_invalid_has_msg, jv_invalid_with_msg, jv_null, jv_object, jv_object_set,
    jv_string, jv_string_fmt, jv_string_length_bytes, jv_string_split, jv_string_value,
    jv_true, jv_is_valid,
};
// Note: jv_object_get is defined locally as a stub in this file
use crate::jv_file::jv_load_file;
use crate::locfile::{locfile_init, locfile_free};
use crate::compile::{
    OP_IS_CALL_PSEUDO, block_bind_library, block_bind_self, block_drop_unreferenced,
    block_free, block_has_main, block_is_const, block_join, block_list_funcs,
    block_module_meta, block_take_imports, gen_const, gen_const_global, gen_import,
    gen_import_meta, gen_noop,
};
use crate::execute::{
    jq_get_jq_origin, jq_get_lib_dirs, jq_get_prog_origin,
    jq_report_error,
};
use crate::types::{Locfile, JqState, Block, LibLoadingState};
use crate::parser::{jq_parse, jq_parse_library};
use crate::util::{expand_path, jq_realpath};
/// Check if a path is relative (doesn't start with '/')
pub fn path_is_relative(p: Jv) -> i32 {
    let s = jv_string_value(&p);
    let res = if s.starts_with('/') { 0 } else { 1 };
    jv_free(p);
    res
}
/// Get the basename of a path (filename without directory)
pub fn jv_basename(path: Jv) -> Jv {
    let path_str = jv_string_value(&path);
    let basename = Path::new(path_str).file_name().and_then(OsStr::to_str).unwrap_or("").to_string();
    jv_free(path);
    jv_string(&basename)
}
/// Validate a relative path for module imports
fn validate_relpath(name: Jv) -> Jv {
    let s = jv_string_value(&name);
    if s.contains('\\') {
        let res = jv_invalid_with_msg(jv_string(&format!(
            "Modules must be named by relative paths using '/', not '\\' ({})",
            s
        )));
        jv_free(name);
        return res;
    }
    let components = jv_string_split(jv_copy(&name), jv_string("/"));
    let len = jv_array_length(&components);
    for i in 0..len {
        let x = jv_array_get(jv_copy(&components), i);
        let x_str = jv_string_value(&x);
        if x_str == ".." {
            jv_free(x);
            jv_free(components);
                let res = jv_invalid_with_msg(jv_string(&format!(
                    "Relative paths to modules may not traverse to parent directories ({})",
                    s
                )));
            jv_free(name);
            return res;
        }
        if i > 0 {
            let prev = jv_array_get(jv_copy(&components), i - 1);
            if jv_equal(&x, &prev) {
                jv_free(prev);
                jv_free(x);
                jv_free(components);
                let name_str = jv_string_value(&name);
                let res = jv_invalid_with_msg(jv_string(&format!(
                    "module names must not have equal consecutive components: {}",
                    name_str
                )));
                jv_free(name);
                return res;
            }
            jv_free(prev);
        }
        jv_free(x);
    }
    jv_free(components);
    name
}
/// Get the default search paths for libraries
fn default_search<T>(jq: &mut JqState<T>, value: Jv) -> Jv {
    if !jv_is_valid(&value) {
        jv_free(value);
        return jv_array_concat(
            jv_array_append(jv_array(), jv_string(".")),
            jq_get_lib_dirs(jq),
        );
    }
    if jv_get_kind(&value) != JvKind::Array {
        return jv_array_append(jv_array(), value);
    }
    value
}
/// Find a library file in the search paths
pub fn find_lib<T>(
    jq: &mut JqState<T>,
    rel_path: Jv,
    search: Jv,
    suffix: &str,
    jq_origin: Jv,
    lib_origin: Jv,
) -> Jv {
    if !jv_is_valid(&rel_path) {
        jv_free(search);
        jv_free(jq_origin);
        jv_free(lib_origin);
        return rel_path;
    }
    if jv_get_kind(&rel_path) != JvKind::String {
        jv_free(rel_path);
        jv_free(search);
        jv_free(jq_origin);
        jv_free(lib_origin);
        return jv_invalid_with_msg(jv_string("Module path must be a string"));
    }
    if jv_get_kind(&search) != JvKind::Array {
        jv_free(rel_path);
        jv_free(search);
        jv_free(jq_origin);
        jv_free(lib_origin);
        return jv_invalid_with_msg(jv_string("Module search path must be an array"));
    }
    let search = build_lib_search_chain(jq, search, jq_origin, lib_origin);
    let err = jv_array_get(jv_copy(&search), 1);
    let search = jv_array_get(search, 0);
    let bname = jv_basename(jv_copy(&rel_path));
    let bname_str = jv_string_value(&bname);
    let rel_path_str = jv_string_value(&rel_path);
    let len = jv_array_length(&search);
    for i in 0..len {
        let spath = jv_array_get(jv_copy(&search), i);
        if jv_get_kind(&spath) == JvKind::Null {
            jv_free(spath);
            break;
        }
        if jv_get_kind(&spath) != JvKind::String {
            jv_free(spath);
            continue;
        }
        let spath_str = jv_string_value(&spath);
        if spath_str.is_empty() {
            jv_free(spath);
            continue;
        }
        let test_path_str = format!("{}/{}{}", spath_str, rel_path_str, suffix);
        let mut testpath = jq_realpath(jv_string(&test_path_str));
        if !Path::new(&jv_string_value(&testpath)).exists() {
            jv_free(testpath);
            let test_path_str = format!(
                "{}/{}/jq/main{}", spath_str, rel_path_str, suffix
            );
            testpath = jq_realpath(jv_string(&test_path_str));
        }
        if !Path::new(&jv_string_value(&testpath)).exists() {
            jv_free(testpath);
            let test_path_str = format!(
                "{}/{}/{}{}", spath_str, rel_path_str, bname_str, suffix
            );
            testpath = jq_realpath(jv_string(&test_path_str));
        }
        if Path::new(&jv_string_value(&testpath)).exists() {
            jv_free(err);
            jv_free(rel_path);
            jv_free(search);
            jv_free(bname);
            jv_free(spath);
            return testpath;
        }
        jv_free(testpath);
        jv_free(spath);
    }
    let output;
    if !jv_is_valid(&err) {
        let err_msg = jv_invalid_get_msg(err);
        let err_str = jv_string_value(&err_msg);
        output = jv_invalid_with_msg(jv_string(&format!(
            "module not found: {} ({})",
            rel_path_str, err_str
        )));
        jv_free(err_msg);
    } else {
        jv_free(err);
        output = jv_invalid_with_msg(jv_string(&format!("module not found: {}", rel_path_str)));
    }
    jv_free(rel_path);
    jv_free(search);
    jv_free(bname);
    output
}
/// Build the library search chain by expanding paths
pub fn build_lib_search_chain<T>(
    _jq: &mut JqState<T>,
    search_path: Jv,
    jq_origin: Jv,
    lib_origin: Jv,
) -> Jv {
    assert!(jv_get_kind(& search_path) == JvKind::Array);
    let mut expanded = jv_array();
    let mut err = jv_null();
    let len = jv_array_length(&search_path);
    for i in 0..len {
        let mut path = jv_array_get(jv_copy(&search_path), i);
        if jv_get_kind(&path) != JvKind::String {
            jv_free(path);
            continue;
        }
        path = expand_path(path);
        if !jv_is_valid(&path) {
            err = path;
            continue;
        }
        let path_str = jv_string_value(&path);
        let expanded_elt;
        if path_str == "." {
            expanded_elt = jv_copy(&path);
        } else if path_str.starts_with("$ORIGIN/") {
            let origin_str = jv_string_value(&jq_origin);
            let suffix = &path_str["$ORIGIN/".len()..];
            expanded_elt = jv_string(&format!("{}/{}", origin_str, suffix));
        } else if jv_get_kind(&lib_origin) == JvKind::String
            && path_is_relative(jv_copy(&path)) != 0
        {
            let lib_origin_str = jv_string_value(&lib_origin);
            expanded_elt = jv_string(&format!("{}/{}", lib_origin_str, path_str));
        } else {
            expanded_elt = path;
            path = jv_null();
        }
        expanded = jv_array_append(expanded, expanded_elt);
        if jv_get_kind(&path) != JvKind::Null {
            jv_free(path);
        }
    }
    jv_free(jq_origin);
    jv_free(lib_origin);
    jv_free(search_path);
    let result = jv_array();
    let result = jv_array_append(result, expanded);
    jv_array_append(result, err)
}
/// Process dependencies from import statements
pub fn process_dependencies<T>(
    jq: &mut JqState<T>,
    jq_origin: Jv,
    lib_origin: Jv,
    src_block: &mut Block,
    lib_state: &mut LibLoadingState,
) -> i32 {
    let deps = block_take_imports(src_block);
    let mut bk = std::mem::take(src_block);
    let mut nerrors = 0;
    let len = jv_array_length(&deps);
    for i in (0..len).rev() {
        let dep = jv_array_get(jv_copy(&deps), i);
        let is_data = jv_get_kind(&jv_object_get(jv_copy(&dep), jv_string("is_data")))
            == JvKind::True;
        let v = jv_object_get(jv_copy(&dep), jv_string("raw"));
        let raw = jv_get_kind(&v) == JvKind::True;
        jv_free(v);
        let optional = jv_get_kind(&jv_object_get(jv_copy(&dep), jv_string("optional")))
            == JvKind::True;
        let relpath = validate_relpath(
            jv_object_get(jv_copy(&dep), jv_string("relpath")),
        );
        let as_jv = jv_object_get(jv_copy(&dep), jv_string("as"));
        assert!(! jv_is_valid(& as_jv) || jv_get_kind(& as_jv) == JvKind::String);
        let as_str = if jv_get_kind(&as_jv) == JvKind::String {
            Some(jv_string_value(&as_jv).to_string())
        } else {
            None
        };
        let search = default_search(jq, jv_object_get(dep, jv_string("search")));
        let suffix = if is_data { ".json" } else { ".jq" };
        let resolved = find_lib(
            jq,
            relpath,
            search,
            suffix,
            jv_copy(&jq_origin),
            jv_copy(&lib_origin),
        );
        if !jv_is_valid(&resolved) {
            jv_free(as_jv);
            if optional {
                jv_free(resolved);
                continue;
            }
            let emsg = jv_invalid_get_msg(resolved);
            jq_report_error(
                jq,
                jv_string(&format!("jq: error: {}\n", jv_string_value(& emsg))),
            );
            jv_free(emsg);
            jv_free(deps);
            jv_free(jq_origin);
            jv_free(lib_origin);
            return 1;
        }
        if is_data {
            let mut dep_def_block = Block::default();
            nerrors
                += load_library(
                    jq,
                    resolved,
                    is_data as i32,
                    raw as i32,
                    optional as i32,
                    as_str.as_deref(),
                    &mut dep_def_block,
                    lib_state,
                );
            if nerrors == 0 {
                let dep_def_block_copy = dep_def_block.clone();
                bk = block_bind_library(
                    dep_def_block,
                    bk,
                    OP_IS_CALL_PSEUDO,
                    as_str.as_deref(),
                );
                bk = block_bind_library(
                    dep_def_block_copy,
                    bk,
                    OP_IS_CALL_PSEUDO,
                    None,
                );
            }
        } else {
            let resolved_str = jv_string_value(&resolved);
            let mut state_idx = None;
            for idx in 0..lib_state.ct as usize {
                if lib_state.names[idx] == resolved_str {
                    state_idx = Some(idx);
                    break;
                }
            }
            if let Some(idx) = state_idx {
                jv_free(resolved);
                let cached_block = lib_state.defs[idx].clone();
                bk = block_bind_library(
                    cached_block,
                    bk,
                    OP_IS_CALL_PSEUDO,
                    as_str.as_deref(),
                );
            } else {
                let mut dep_def_block = gen_noop();
                nerrors
                    += load_library(
                        jq,
                        resolved,
                        is_data as i32,
                        raw as i32,
                        optional as i32,
                        as_str.as_deref(),
                        &mut dep_def_block,
                        lib_state,
                    );
                if nerrors == 0 {
                    bk = block_bind_library(
                        dep_def_block,
                        bk,
                        OP_IS_CALL_PSEUDO,
                        as_str.as_deref(),
                    );
                }
            }
        }
        jv_free(as_jv);
    }
    jv_free(lib_origin);
    jv_free(jq_origin);
    jv_free(deps);
    *src_block = bk;
    nerrors
}
/// Load a library from a file path
fn load_data_file(filename: &str) -> Jv {
    match std::fs::read_to_string(filename) {
        Ok(contents) => {
            let value = crate::jv_parse::jv_parse(&contents);
            if jv_is_valid(&value) {
                jv_array_append(jv_array(), value)
            } else {
                value
            }
        }
        Err(e) => jv_invalid_with_msg(jv_string(&format!("Could not open {}: {}", filename, e))),
    }
}

/// Load a library from a file path
fn load_library<T>(
    jq: &mut JqState<T>,
    lib_path: Jv,
    is_data: i32,
    raw: i32,
    optional: i32,
    as_name: Option<&str>,
    out_block: &mut Block,
    lib_state: &mut LibLoadingState,
) -> i32 {
    let mut nerrors = 0;
    let program: Block;
    let data = if is_data != 0 && raw == 0 {
        load_data_file(jv_string_value(&lib_path))
    } else {
        jv_load_file(jv_string_value(&lib_path), true)
    };
    if !jv_is_valid(&data) {
        program = gen_noop();
        if optional == 0 {
            let err_data = if jv_invalid_has_msg(jv_copy(&data)) != 0 {
                jv_invalid_get_msg(jv_copy(&data))
            } else {
                jv_string("unknown error")
            };
            let lib_path_str = jv_string_value(&lib_path);
            let err_data_str = jv_string_value(&err_data);
            jq_report_error(
                jq,
                jv_string(&format!(
                    "jq: error loading data file {}: {}\n",
                    lib_path_str, err_data_str
                )),
            );
            jv_free(err_data);
            nerrors += 1;
        }
    } else if is_data != 0 {
        program = gen_const_global(jv_copy(&data), as_name.unwrap_or(""));
    } else {
        let src = locfile_init::<T>(
            None,
            jv_string_value(&lib_path),
            jv_string_value(&data),
            jv_string_length_bytes(&data),
        );
        let mut lib_program = gen_noop();
        nerrors += jq_parse_library::<T>(&mut src.borrow_mut(), &mut lib_program);
        locfile_free(&src);
        if nerrors == 0 {
            let lib_path_str = jv_string_value(&lib_path);
            let lib_dir = Path::new(lib_path_str)
                .parent()
                .and_then(|p| p.to_str())
                .unwrap_or(".");
            nerrors
                += process_dependencies(
                    jq,
                    jq_get_jq_origin(jq),
                    jv_string(lib_dir),
                    &mut lib_program,
                    lib_state,
                );
            lib_program = block_bind_self(lib_program, OP_IS_CALL_PSEUDO);
        }
        program = lib_program;
    }
    let _state_idx = lib_state.ct as usize;
    lib_state.ct += 1;
    lib_state.names.push(jv_string_value(&lib_path).to_string());
    lib_state.defs.push(program.clone());
    *out_block = program;
    jv_free(lib_path);
    jv_free(data);
    nerrors
}
/// Load and parse a jq program from source
pub fn load_program<T>(
    jq: &mut JqState<T>,
    src: &mut Locfile<T>,
    out_block: &mut Block,
) -> i32 {
    let mut nerrors = 0;
    let mut lib_state = LibLoadingState::default();
    let mut program = gen_noop();
    nerrors = jq_parse(src, &mut program);
    if nerrors != 0 {
        return nerrors;
    }
    if !block_has_main(&program) {
        jq_report_error(
            jq,
            jv_string("jq: error: Top-level program not given (try \".\")"),
        );
        block_free(program);
        return 1;
    }
    if let Ok(home) = std::env::var("HOME") {
        let meta = jv_object_set(
            jv_object_set(jv_object(), jv_string("optional"), jv_true()),
            jv_string("search"),
            jv_string(&home),
        );
        let import = gen_import_meta(gen_import("", None, 0), gen_const(meta));
        program = block_join(import, program);
    }
    nerrors = process_dependencies(
        jq,
        jq_get_jq_origin(jq),
        jq_get_prog_origin(jq),
        &mut program,
        &mut lib_state,
    );
    let mut libs = gen_noop();
    for i in 0..lib_state.ct as usize {
        let def = std::mem::take(&mut lib_state.defs[i]);
        if nerrors == 0 && block_is_const(&def) == 0 {
            libs = block_join(libs, def);
        } else {
            block_free(def);
        }
    }
    if nerrors != 0 {
        block_free(program);
    } else {
        *out_block = block_drop_unreferenced(block_join(libs, program));
    }
    nerrors
}
/// Load module metadata from a relative path
pub fn load_module_meta<T>(jq: &mut JqState<T>, mod_relpath: Jv) -> Jv {
    // Get values before the mutable borrow for find_lib
    let lib_dirs = jq_get_lib_dirs(jq);
    let jq_origin = jq_get_jq_origin(jq);
    let lib_path = find_lib(
        jq,
        validate_relpath(mod_relpath),
        lib_dirs,
        ".jq",
        jq_origin,
        jv_null(),
    );
    if !jv_is_valid(&lib_path) {
        return lib_path;
    }
    let mut meta = jv_null();
    let data = jv_load_file(jv_string_value(&lib_path), true);
    if jv_is_valid(&data) {
        let mut program = gen_noop();
        let src = locfile_init::<T>(
            None,
            jv_string_value(&lib_path),
            jv_string_value(&data),
            jv_string_length_bytes(&data),
        );
        let nerrors = jq_parse_library::<T>(&mut src.borrow_mut(), &mut program);
        if nerrors == 0 {
            meta = block_module_meta(&program);
            if jv_get_kind(&meta) == JvKind::Null {
                meta = jv_object();
            }
            meta = jv_object_set(
                meta,
                jv_string("deps"),
                block_take_imports(&mut program),
            );
            meta = jv_object_set(meta, jv_string("defs"), block_list_funcs(std::mem::take(&mut program), 0));
        }
        locfile_free(&src);
        block_free(program);
    }
    jv_free(lib_path);
    jv_free(data);
    meta
}
fn jv_object_get(obj: Jv, key: Jv) -> Jv {
    use crate::jv::jv_object_get as jv_obj_get;
    if crate::jv::jv_object_has_key(&obj, &key) {
        let result = jv_obj_get(&obj, key);
        jv_free(obj);
        result
    } else {
        jv_free(obj);
        jv_free(key);
        Jv::invalid()
    }
}
impl Default for LibLoadingState {
    fn default() -> Self {
        Self {
            names: Vec::new(),
            defs: Vec::new(),
            ct: 0,
        }
    }
}
/// Get the dirname of a path (part before the last '/')
fn jv_dirname(name: Jv) -> Jv {
    let s = jv_string_value(&name);
    if let Some(pos) = s.rfind('/') {
        let res = jv_string(&s[..pos]);
        jv_free(name);
        res
    } else {
        jv_free(name);
        jv_string(".")
    }
}
