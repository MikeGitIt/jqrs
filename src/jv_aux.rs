//! Module: jv_aux
//!
//! Contains 19 transpiled functions:
//! - jv_has:14436608940595815116:./src/jv_aux.c
//! - jv_keys_unsorted:4648212728920375683:./src/jv_aux.c
//! - sort_items:10433524878758585710:./src/jv_aux.c
//! - jv_cmp:10720017219002167313:./src/jv_aux.c
//! - sort_cmp:2214236636998602789:./src/jv_aux.c
//! - jv_group:1650194907073300883:./src/jv_aux.c
//! - jv_setpath:17442096123013353038:./src/jv_aux.c
//! - jv_delpaths:13237450901658517611:./src/jv_aux.c
//! - jv_keys:17522807267547667769:./src/jv_aux.c
//! - jv_number_get_value_and_consume:13490370844426867593:./src/jv_aux.c
//! - delpaths_sorted:12203267546879748066:./src/jv_aux.c
//! - jv_get:5498610809406452644:./src/jv_aux.c
//! - jv_sort:1538461067543493388:./src/jv_aux.c
//! - jv_set:12357725007649289471:./src/jv_aux.c
//! - jv_getpath:7699321837383283480:./src/jv_aux.c
//! - jv_dels:530579538121973539:./src/jv_aux.c
//! - string_cmp:16035778265772709412:./src/jv_aux.c
//! - jv_is_valid:7339449971924854417:./src/jv_aux.c
//! - parse_slice:11537782206179489912:./src/jv_aux.c

use std::cmp::Ordering;
use crate::jv::{
    jv_bool, jv_false, jv_object, jv_object_delete, jv_object_set, Jv, JvKind,
    jvp_number_cmp,
};
// Note: Most jv_* functions are defined locally as stubs in this file
// Note: jv_cmp, jv_group, jv_has, jv_keys_unsorted, jv_set, jv_getpath are defined in this file
use crate::types::*;
/// Checks if a jv value is valid (not JV_KIND_INVALID)
///
/// This is a fundamental validity check used throughout the jq codebase
/// to verify that jv values are in a usable state.
#[inline]
pub fn jv_is_valid(x: &Jv) -> bool {
    let kind = jv_get_kind(x);
    kind != JvKind::Invalid
}
/// Parse a slice specification and extract start/end indices
///
/// # Arguments
/// * `j` - The array or string to slice
/// * `slice` - Object containing start/end specifications
/// * `pstart` - Output for start index
/// * `pend` - Output for end index
///
/// # Returns
/// jv_true() on success, error JV on failure
fn parse_slice(j: Jv, slice: Jv, pstart: &mut i32, pend: &mut i32) -> Jv {
    let mut start_jv = jv_object_get(jv_copy(&slice), jv_string("start"));
    let mut end_jv = jv_object_get(slice, jv_string("end"));
    if jv_get_kind(&start_jv) == JvKind::Null {
        jv_free(start_jv);
        start_jv = jv_number(0.0);
    }
    let len: i32;
    if jv_get_kind(&j) == JvKind::Array {
        len = jv_array_length(j);
    } else if jv_get_kind(&j) == JvKind::String {
        len = jv_string_length_codepoints(j);
    } else {
        jv_free(j);
        jv_free(start_jv);
        jv_free(end_jv);
        return jv_invalid_with_msg(jv_string("Only arrays and strings can be sliced"));
    }
    if jv_get_kind(&end_jv) == JvKind::Null {
        jv_free(end_jv);
        end_jv = jv_number(len as f64);
    }
    if jv_get_kind(&start_jv) != JvKind::Number || jv_get_kind(&end_jv) != JvKind::Number
    {
        jv_free(start_jv);
        jv_free(end_jv);
        return jv_invalid_with_msg(
            jv_string("Array/string slice indices must be integers"),
        );
    }
    let mut dstart = jv_number_value(&start_jv);
    let mut dend = jv_number_value(&end_jv);
    jv_free(start_jv);
    jv_free(end_jv);
    if dstart.is_nan() {
        dstart = 0.0;
    }
    if dstart < 0.0 {
        dstart += len as f64;
    }
    if dstart < 0.0 {
        dstart = 0.0;
    }
    if dstart > len as f64 {
        dstart = len as f64;
    }
    let start = if dstart > i32::MAX as f64 { i32::MAX } else { dstart as i32 };
    if dend.is_nan() {
        dend = len as f64;
    }
    if dend < 0.0 {
        dend += len as f64;
    }
    if dend < 0.0 {
        dend = start as f64;
    }
    let mut end = if dend > i32::MAX as f64 { i32::MAX } else { dend as i32 };
    if end > len {
        end = len;
    }
    if end < len && (end as f64) < dend {
        end += 1;
    }
    if end < start {
        end = start;
    }
    assert!(0 <= start && start <= end && end <= len);
    *pstart = start;
    *pend = end;
    jv_true()
}
/// Get a value from a JV structure by key
///
/// # Arguments
/// * `t` - The JV value to index into
/// * `k` - The key or index
///
/// # Returns
/// The value at the specified key/index
pub fn jv_get(t: Jv, k: Jv) -> Jv {
    let v: Jv;
    if jv_get_kind(&t) == JvKind::Object && jv_get_kind(&k) == JvKind::String {
        v = jv_object_get(t, k);
        if !jv_is_valid(&v) {
            jv_free(v);
            return jv_null();
        }
        return v;
    } else if jv_get_kind(&t) == JvKind::Array && jv_get_kind(&k) == JvKind::Number {
        if jvp_number_is_nan(&k) {
            jv_free(t);
            return jv_null();
        } else {
            let mut didx = jv_number_value(&k);
            if jvp_number_is_nan(&k) {
                return jv_null();
            } else {
                if didx < i32::MIN as f64 {
                    didx = i32::MIN as f64;
                }
                if didx > i32::MAX as f64 {
                    didx = i32::MAX as f64;
                }
                let mut idx = didx as i32;
                if idx < 0 {
                    idx += jv_array_length(jv_copy(&t));
                }
                v = jv_array_get(t, idx);
                if !jv_is_valid(&v) {
                    jv_free(v);
                    return jv_null();
                }
            }
        }
        jv_free(k);
        return v;
    } else if jv_get_kind(&t) == JvKind::Array && jv_get_kind(&k) == JvKind::Object {
        let mut start: i32 = 0;
        let mut end: i32 = 0;
        let e = parse_slice(jv_copy(&t), k, &mut start, &mut end);
        if jv_get_kind(&e) == JvKind::True {
            return jv_array_slice(t, start, end);
        } else {
            jv_free(t);
            return e;
        }
    } else if jv_get_kind(&t) == JvKind::String && jv_get_kind(&k) == JvKind::Object {
        let mut start: i32 = 0;
        let mut end: i32 = 0;
        let e = parse_slice(jv_copy(&t), k, &mut start, &mut end);
        if jv_get_kind(&e) == JvKind::True {
            return jv_string_slice(t, start, end);
        } else {
            jv_free(t);
            return e;
        }
    } else if jv_get_kind(&t) == JvKind::Array && jv_get_kind(&k) == JvKind::Array {
        return jv_array_indexes(t, k);
    } else if jv_get_kind(&t) == JvKind::Null
        && (jv_get_kind(&k) == JvKind::String || jv_get_kind(&k) == JvKind::Number
            || jv_get_kind(&k) == JvKind::Object)
    {
        jv_free(t);
        jv_free(k);
        return jv_null();
    } else {
        let t_kind = jv_get_kind(&t);
        let k_kind = jv_get_kind(&k);
        let v = if k_kind == JvKind::String && jv_string_length_bytes(jv_copy(&k)) < 30 {
            jv_invalid_with_msg(
                jv_string_fmt(
                    &format!("Cannot index {} with string", jv_kind_name(t_kind)),
                    jv_string_value(&k),
                ),
            )
        } else {
            jv_invalid_with_msg(
                jv_string_fmt(
                    &format!(
                        "Cannot index {} with {}", jv_kind_name(t_kind),
                        jv_kind_name(k_kind)
                    ),
                    "",
                ),
            )
        };
        jv_free(t);
        jv_free(k);
        return v;
    }
}
/// Set a value in a jv container (array or object)
pub fn jv_set(mut t: Jv, k: Jv, v: Jv) -> Jv {
    if !jv_is_valid(&v) {
        jv_free(t);
        jv_free(k);
        return v;
    }
    let isnull = jv_get_kind(&t) == JvKind::Null;
    let kind_k = jv_get_kind(&k);
    let kind_t = jv_get_kind(&t);
    if kind_k == JvKind::String && (kind_t == JvKind::Object || isnull) {
        if isnull {
            jv_free(t);
            t = jv_object();
        }
        return jv_object_set(t, k, v);
    }
    if kind_k == JvKind::Number && (kind_t == JvKind::Array || isnull) {
        if jvp_number_is_nan(&k) {
            jv_free(t);
            jv_free(k);
            return jv_invalid_with_msg(
                jv_string("Cannot set array element at NaN index"),
            );
        }
        let mut didx = jv_number_value(&k);
        if didx < i32::MIN as f64 {
            didx = i32::MIN as f64;
        }
        if didx > i32::MAX as f64 {
            didx = i32::MAX as f64;
        }
        if isnull {
            jv_free(t);
            t = jv_array();
        }
        let result = jv_array_set(t, didx as i32, v);
        jv_free(k);
        return result;
    }
    if kind_k == JvKind::Object && (kind_t == JvKind::Array || isnull) {
        if isnull {
            jv_free(t);
            t = jv_array();
        }
        let mut start = 0i32;
        let mut end = 0i32;
        let e = parse_slice(jv_copy(&t), k, &mut start, &mut end);
        if jv_get_kind(&e) == JvKind::True {
            jv_free(e);
            if jv_get_kind(&v) == JvKind::Array {
                let array_len = jv_array_length(jv_copy(&t));
                assert!(0 <= start && start <= end && end <= array_len);
                let slice_len = end - start;
                let insert_len = jv_array_length(jv_copy(&v));
                if slice_len < insert_len {
                    let shift = insert_len - slice_len;
                    for i in (end..array_len).rev() {
                        let elem = jv_array_get(jv_copy(&t), i);
                        t = jv_array_set(t, i + shift, elem);
                    }
                } else if slice_len > insert_len {
                    let shift = slice_len - insert_len;
                    for i in end..array_len {
                        let elem = jv_array_get(jv_copy(&t), i);
                        t = jv_array_set(t, i - shift, elem);
                    }
                    t = jv_array_slice(t, 0, array_len - shift);
                }
                for i in 0..insert_len {
                    let elem = jv_array_get(jv_copy(&v), i);
                    t = jv_array_set(t, start + i, elem);
                }
                jv_free(v);
                return t;
            } else {
                jv_free(t);
                jv_free(v);
                return jv_invalid_with_msg(
                    jv_string_fmt(
                        "A slice of an array can only be assigned another array",
                        "",
                    ),
                );
            }
        } else {
            jv_free(t);
            jv_free(v);
            return e;
        }
    }
    if kind_k == JvKind::Object && kind_t == JvKind::String {
        jv_free(t);
        jv_free(k);
        jv_free(v);
        return jv_invalid_with_msg(jv_string_fmt("Cannot update string slices", ""));
    }
    let err = jv_invalid_with_msg(
        jv_string_fmt(
            &format!(
                "Cannot update field at {} index of {}", jv_kind_name(kind_k),
                jv_kind_name(kind_t)
            ),
            "",
        ),
    );
    jv_free(t);
    jv_free(k);
    jv_free(v);
    err
}
/// Compare two strings for sorting
fn string_cmp(a: &Jv, b: &Jv) -> i32 {
    let lena = jv_string_length_bytes(jv_copy(a));
    let lenb = jv_string_length_bytes(jv_copy(b));
    let minlen = lena.min(lenb) as usize;
    let str_a = jv_string_value(a);
    let str_b = jv_string_value(b);
    let bytes_a = str_a.as_bytes();
    let bytes_b = str_b.as_bytes();
    let cmp_len = minlen.min(bytes_a.len()).min(bytes_b.len());
    let r = bytes_a[..cmp_len].cmp(&bytes_b[..cmp_len]);
    match r {
        Ordering::Equal => lena - lenb,
        Ordering::Less => -1,
        Ordering::Greater => 1,
    }
}
/// Compare two jv values
pub fn jv_cmp(a: Jv, b: Jv) -> i32 {
    let kind_a = jv_get_kind(&a);
    let kind_b = jv_get_kind(&b);
    if kind_a != kind_b {
        let r = (kind_a as i32) - (kind_b as i32);
        jv_free(a);
        jv_free(b);
        return r;
    }
    let r = match kind_a {
        JvKind::Null | JvKind::False | JvKind::True => 0,
        JvKind::Number => {
            if jvp_number_is_nan(&a) {
                let result = jv_cmp(jv_null(), jv_copy(&b));
                jv_free(a);
                jv_free(b);
                return result;
            } else if jvp_number_is_nan(&b) {
                let result = jv_cmp(jv_copy(&a), jv_null());
                jv_free(a);
                jv_free(b);
                return result;
            } else {
                jvp_number_cmp(&a, &b)
            }
        }
        JvKind::String => string_cmp(&a, &b),
        JvKind::Array => {
            let mut r = 0;
            let mut i = 0;
            loop {
                let a_done = i >= jv_array_length(jv_copy(&a));
                let b_done = i >= jv_array_length(jv_copy(&b));
                if a_done || b_done {
                    r = (b_done as i32) - (a_done as i32);
                    break;
                }
                let xa = jv_array_get(jv_copy(&a), i);
                let xb = jv_array_get(jv_copy(&b), i);
                r = jv_cmp(xa, xb);
                if r != 0 {
                    break;
                }
                i += 1;
            }
            r
        }
        JvKind::Object => {
            let keys_a = jv_keys(jv_copy(&a));
            let keys_b = jv_keys(jv_copy(&b));
            let mut r = jv_cmp(jv_copy(&keys_a), keys_b);
            if r == 0 {
                let len = jv_array_length(jv_copy(&keys_a));
                for i in 0..len {
                    let key = jv_array_get(jv_copy(&keys_a), i);
                    let xa = jv_object_get(jv_copy(&a), jv_copy(&key));
                    let xb = jv_object_get(jv_copy(&b), key);
                    r = jv_cmp(xa, xb);
                    if r != 0 {
                        break;
                    }
                }
            }
            jv_free(keys_a);
            r
        }
        _ => {
            panic!("invalid kind passed to jv_cmp");
        }
    };
    jv_free(a);
    jv_free(b);
    r
}
/// Get the numeric value from a jv and consume it
fn jv_number_get_value_and_consume(number: Jv) -> f64 {
    let value = jv_number_value(&number);
    jv_free(number);
    value
}
/// Delete elements from a jv container
fn jv_dels(mut t: Jv, keys: Jv) -> Jv {
    assert!(jv_get_kind(& keys) == JvKind::Array);
    assert!(jv_is_valid(& t));
    let kind_t = jv_get_kind(&t);
    if kind_t == JvKind::Null || jv_array_length(jv_copy(&keys)) == 0
    {} else if kind_t == JvKind::Array {
        let mut neg_keys = jv_array();
        let mut nonneg_keys = jv_array();
        let mut new_array = jv_array();
        let mut starts = jv_array();
        let mut ends = jv_array();
        let keys_len = jv_array_length(jv_copy(&keys));
        let mut error_occurred = false;
        for i in 0..keys_len {
            if error_occurred {
                break;
            }
            let key = jv_array_get(jv_copy(&keys), i);
            let kind_key = jv_get_kind(&key);
            if kind_key == JvKind::Number {
                if jv_number_value(&key) < 0.0 {
                    neg_keys = jv_array_append(neg_keys, key);
                } else {
                    nonneg_keys = jv_array_append(nonneg_keys, key);
                }
            } else if kind_key == JvKind::Object {
                let mut start = 0i32;
                let mut end = 0i32;
                let e = parse_slice(jv_copy(&t), key, &mut start, &mut end);
                if jv_get_kind(&e) == JvKind::True {
                    jv_free(e);
                    starts = jv_array_append(starts, jv_number(start as f64));
                    ends = jv_array_append(ends, jv_number(end as f64));
                } else {
                    jv_free(new_array);
                    new_array = e;
                    error_occurred = true;
                }
            } else {
                jv_free(new_array);
                new_array = jv_invalid_with_msg(
                    jv_string_fmt(
                        &format!(
                            "Cannot delete {} element of array", jv_kind_name(kind_key)
                        ),
                        "",
                    ),
                );
                jv_free(key);
                error_occurred = true;
            }
        }
        if !error_occurred {
            let mut neg_idx = 0;
            let mut nonneg_idx = 0;
            let len = jv_array_length(jv_copy(&t));
            for i in 0..len {
                let mut del = false;
                while neg_idx < jv_array_length(jv_copy(&neg_keys)) {
                    let delidx = len
                        + jv_number_get_value_and_consume(
                            jv_array_get(jv_copy(&neg_keys), neg_idx),
                        ) as i32;
                    if i == delidx {
                        del = true;
                    }
                    if i < delidx {
                        break;
                    }
                    neg_idx += 1;
                }
                while nonneg_idx < jv_array_length(jv_copy(&nonneg_keys)) {
                    let delidx = jv_number_get_value_and_consume(
                        jv_array_get(jv_copy(&nonneg_keys), nonneg_idx),
                    ) as i32;
                    if i == delidx {
                        del = true;
                    }
                    if i < delidx {
                        break;
                    }
                    nonneg_idx += 1;
                }
                let starts_len = jv_array_length(jv_copy(&starts));
                for sidx in 0..starts_len {
                    if del {
                        break;
                    }
                    let start_val = jv_number_get_value_and_consume(
                        jv_array_get(jv_copy(&starts), sidx),
                    ) as i32;
                    let end_val = jv_number_get_value_and_consume(
                        jv_array_get(jv_copy(&ends), sidx),
                    ) as i32;
                    if start_val <= i && i < end_val {
                        del = true;
                    }
                }
                if !del {
                    new_array = jv_array_append(new_array, jv_array_get(jv_copy(&t), i));
                }
            }
        }
        jv_free(neg_keys);
        jv_free(nonneg_keys);
        jv_free(starts);
        jv_free(ends);
        jv_free(t);
        t = new_array;
    } else if kind_t == JvKind::Object {
        let keys_len = jv_array_length(jv_copy(&keys));
        for i in 0..keys_len {
            let k = jv_array_get(jv_copy(&keys), i);
            if jv_get_kind(&k) != JvKind::String {
                jv_free(t);
                t = jv_invalid_with_msg(
                    jv_string_fmt(
                        &format!(
                            "Cannot delete {} field of object",
                            jv_kind_name(jv_get_kind(&k))
                        ),
                        "",
                    ),
                );
                jv_free(k);
                break;
            }
            t = jv_object_delete(t, k);
        }
    } else {
        let err = jv_invalid_with_msg(
            jv_string_fmt(&format!("Cannot delete fields from {}", jv_kind_name(kind_t)), ""),
        );
        jv_free(t);
        t = err;
    }
    jv_free(keys);
    t
}
/// Comparison function for sort entries
fn sort_cmp(a: &SortEntry, b: &SortEntry) -> Ordering {
    let r = jv_cmp(jv_copy(&a.key), jv_copy(&b.key));
    if r != 0 {
        if r < 0 { Ordering::Less } else { Ordering::Greater }
    } else {
        a.index.cmp(&b.index)
    }
}
/// Get keys from a jv object or array (unsorted for objects)
pub fn jv_keys_unsorted(x: Jv) -> Jv {
    if jv_get_kind(&x) != JvKind::Object {
        return jv_keys(x);
    }
    let len = jv_object_length(jv_copy(&x));
    let mut answer = jv_array_sized(len);
    let mut iter = jv_object_iter(&x);
    while jv_object_iter_valid(&x, iter) {
        let key = jv_object_iter_key(&x, iter);
        let value = jv_object_iter_value(&x, iter);
        answer = jv_array_append(answer, key);
        jv_free(value);
        iter = jv_object_iter_next(&x, iter);
    }
    jv_free(x);
    answer
}
/// Get sorted keys from a jv object or array indices
pub fn jv_keys(x: Jv) -> Jv {
    let kind = jv_get_kind(&x);
    if kind == JvKind::Object {
        let len = jv_object_length(jv_copy(&x));
        let mut keys: Vec<Jv> = Vec::with_capacity(len as usize);
        let mut iter = jv_object_iter(&x);
        while jv_object_iter_valid(&x, iter) {
            let key = jv_object_iter_key(&x, iter);
            let value = jv_object_iter_value(&x, iter);
            keys.push(key);
            jv_free(value);
            iter = jv_object_iter_next(&x, iter);
        }
        jv_free(x);
        keys.sort_by(|a, b| {
            let r = string_cmp(a, b);
            if r < 0 {
                Ordering::Less
            } else if r > 0 {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        let mut result = jv_array_sized(len);
        for key in keys {
            result = jv_array_append(result, key);
        }
        result
    } else if kind == JvKind::Array {
        let len = jv_array_length(jv_copy(&x));
        jv_free(x);
        let mut result = jv_array_sized(len);
        for i in 0..len {
            result = jv_array_append(result, jv_number(i as f64));
        }
        result
    } else {
        jv_free(x);
        jv_invalid_with_msg(
            jv_string_fmt(&format!("{} has no keys", jv_kind_name(kind)), ""),
        )
    }
}
/// Sort items by keys and return sorted entries
///
/// # Arguments
/// * `objects` - Array of objects to sort
/// * `keys` - Array of keys corresponding to objects
///
/// # Returns
/// Vector of SortEntry containing sorted items
fn sort_items(objects: Jv, keys: Jv) -> Vec<SortEntry> {
    assert!(jv_get_kind(& objects) == JvKind::Array);
    assert!(jv_get_kind(& keys) == JvKind::Array);
    assert!(jv_array_length(jv_copy(& objects)) == jv_array_length(jv_copy(& keys)));
    let n = jv_array_length(jv_copy(&objects)) as usize;
    let mut entries: Vec<SortEntry> = Vec::with_capacity(n);
    for i in 0..n {
        entries
            .push(SortEntry {
                object: jv_array_get(jv_copy(&objects), i as i32),
                key: jv_array_get(jv_copy(&keys), i as i32),
                index: i as i32,
            });
    }
    jv_free(objects);
    jv_free(keys);
    entries.sort_by(sort_cmp);
    entries
}
/// Group objects by keys
pub fn jv_group(objects: Jv, keys: Jv) -> Jv {
    assert!(jv_get_kind(& objects) == JvKind::Array);
    assert!(jv_get_kind(& keys) == JvKind::Array);
    assert!(jv_array_length(jv_copy(& objects)) == jv_array_length(jv_copy(& keys)));
    let n = jv_array_length(jv_copy(&objects));
    let mut entries = sort_items(objects, keys);
    let mut ret = jv_array();
    if n > 0 {
        let mut curr_key = std::mem::replace(&mut entries[0].key, jv_null());
        let first_obj = std::mem::replace(&mut entries[0].object, jv_null());
        let mut group = jv_array_append(jv_array(), first_obj);
        for i in 1..(n as usize) {
            let entry_key = std::mem::replace(&mut entries[i].key, jv_null());
            let entry_obj = std::mem::replace(&mut entries[i].object, jv_null());
            if jv_equal(jv_copy(&curr_key), jv_copy(&entry_key)) {
                jv_free(entry_key);
            } else {
                jv_free(curr_key);
                curr_key = entry_key;
                ret = jv_array_append(ret, group);
                group = jv_array();
            }
            group = jv_array_append(group, entry_obj);
        }
        jv_free(curr_key);
        ret = jv_array_append(ret, group);
    }
    ret
}
/// Check if a container has a given key
pub fn jv_has(t: Jv, k: Jv) -> Jv {
    assert!(jv_is_valid(& t));
    assert!(jv_is_valid(& k));
    let kind_t = jv_get_kind(&t);
    let kind_k = jv_get_kind(&k);
    if kind_t == JvKind::Null {
        jv_free(t);
        jv_free(k);
        return jv_false();
    }
    if kind_t == JvKind::Object && kind_k == JvKind::String {
        let elem = jv_object_get(t, k);
        let ret = jv_bool(jv_is_valid(&elem));
        jv_free(elem);
        return ret;
    }
    if kind_t == JvKind::Array && kind_k == JvKind::Number {
        if jvp_number_is_nan(&k) {
            jv_free(t);
            jv_free(k);
            return jv_false();
        }
        let elem = jv_array_get(t, jv_number_value(&k) as i32);
        let ret = jv_bool(jv_is_valid(&elem));
        jv_free(elem);
        jv_free(k);
        return ret;
    }
    let ret = jv_invalid_with_msg(
        jv_string_fmt(
            &format!(
                "Cannot check whether {} has a {} key", jv_kind_name(kind_t),
                jv_kind_name(kind_k)
            ),
            "",
        ),
    );
    jv_free(t);
    jv_free(k);
    ret
}
/// Get a value at a path in a nested structure
pub fn jv_getpath(root: Jv, path: Jv) -> Jv {
    if jv_get_kind(&path) != JvKind::Array {
        jv_free(root);
        jv_free(path);
        return jv_invalid_with_msg(jv_string("Path must be specified as an array"));
    }
    if !jv_is_valid(&root) {
        jv_free(path);
        return root;
    }
    let path_len = jv_array_length(jv_copy(&path));
    if path_len == 0 {
        jv_free(path);
        return root;
    }
    let pathcurr = jv_array_get(jv_copy(&path), 0);
    let pathrest = jv_array_slice(path, 1, path_len);
    jv_getpath(jv_get(root, pathcurr), pathrest)
}
/// Set a value at a path in a JSON structure
///
/// # Arguments
/// * `root` - The root JV value to modify
/// * `path` - Array representing the path
/// * `value` - The value to set at the path
///
/// # Returns
/// Modified JV value with the path set
pub fn jv_setpath(root: Jv, path: Jv, value: Jv) -> Jv {
    if jv_get_kind(&path) != JvKind::Array {
        jv_free(value);
        jv_free(root);
        jv_free(path);
        return jv_invalid_with_msg(jv_string("Path must be specified as an array"));
    }
    if !jv_is_valid(&root) {
        jv_free(value);
        jv_free(path);
        return root;
    }
    if jv_array_length(jv_copy(&path)) == 0 {
        jv_free(path);
        jv_free(root);
        return value;
    }
    let pathcurr = jv_array_get(jv_copy(&path), 0);
    let path_len = jv_array_length(jv_copy(&path));
    let pathrest = jv_array_slice(path, 1, path_len);
    if jv_get_kind(&pathcurr) == JvKind::Object {
        let subpath = jv_get(jv_copy(&root), jv_copy(&pathcurr));
        let new_value = jv_setpath(subpath, pathrest, value);
        return jv_set(root, pathcurr, new_value);
    }
    let subroot = jv_get(jv_copy(&root), jv_copy(&pathcurr));
    if !jv_is_valid(&subroot) {
        jv_free(root);
        jv_free(pathcurr);
        jv_free(pathrest);
        jv_free(value);
        return subroot;
    }
    let root = jv_set(root, jv_copy(&pathcurr), jv_null());
    if !jv_is_valid(&root) {
        jv_free(subroot);
        jv_free(pathcurr);
        jv_free(pathrest);
        jv_free(value);
        return root;
    }
    jv_set(root, pathcurr, jv_setpath(subroot, pathrest, value))
}
/// Delete paths from a sorted paths array (internal helper)
fn delpaths_sorted(mut object: Jv, paths: Jv, start: i32) -> Jv {
    let mut delkeys = jv_array();
    let paths_len = jv_array_length(jv_copy(&paths));
    let mut i = 0;
    while i < paths_len {
        let mut j = i;
        let path_i = jv_array_get(jv_copy(&paths), i);
        assert!(jv_array_length(jv_copy(& path_i)) > start);
        let delkey = jv_array_length(jv_copy(&path_i)) == start + 1;
        let key = jv_array_get(path_i, start);
        while j < paths_len {
            let path_j = jv_array_get(jv_copy(&paths), j);
            let key_j = jv_array_get(path_j, start);
            if !jv_equal(jv_copy(&key), key_j) {
                break;
            }
            j += 1;
        }
        if delkey {
            delkeys = jv_array_append(delkeys, key);
        } else {
            let subobject = jv_get(jv_copy(&object), jv_copy(&key));
            if !jv_is_valid(&subobject) {
                jv_free(key);
                jv_free(object);
                object = subobject;
                break;
            } else if jv_get_kind(&subobject) == JvKind::Null {
                jv_free(key);
                jv_free(subobject);
            } else {
                let newsubobject = delpaths_sorted(
                    subobject,
                    jv_array_slice(jv_copy(&paths), i, j),
                    start + 1,
                );
                if !jv_is_valid(&newsubobject) {
                    jv_free(key);
                    jv_free(object);
                    object = newsubobject;
                    break;
                }
                object = jv_set(object, key, newsubobject);
            }
            if !jv_is_valid(&object) {
                break;
            }
        }
        i = j;
    }
    jv_free(paths);
    if jv_is_valid(&object) {
        object = jv_dels(object, delkeys);
    } else {
        jv_free(delkeys);
    }
    object
}
/// Delete multiple paths from an object
///
/// # Arguments
/// * `object` - The object to modify
/// * `paths` - Array of paths to delete
///
/// # Returns
/// Object with paths deleted
pub fn jv_delpaths(object: Jv, paths: Jv) -> Jv {
    if jv_get_kind(&paths) != JvKind::Array {
        jv_free(object);
        jv_free(paths);
        return jv_invalid_with_msg(jv_string("Paths must be specified as an array"));
    }
    let paths = jv_sort(jv_copy(&paths), jv_copy(&paths));
    let paths_len = jv_array_length(jv_copy(&paths));
    for i in 0..paths_len {
        let elem = jv_array_get(jv_copy(&paths), i);
        if jv_get_kind(&elem) != JvKind::Array {
            jv_free(object);
            jv_free(paths);
            let elem_kind = jv_get_kind(&elem);
            let err = jv_invalid_with_msg(
                jv_string_fmt(
                    "Path must be specified as array, not",
                    jv_kind_name(elem_kind),
                ),
            );
            jv_free(elem);
            return err;
        }
        jv_free(elem);
    }
    if jv_array_length(jv_copy(&paths)) == 0 {
        jv_free(paths);
        return object;
    }
    let first_path = jv_array_get(jv_copy(&paths), 0);
    if jv_array_length(first_path) == 0 {
        jv_free(paths);
        jv_free(object);
        return jv_null();
    }
    delpaths_sorted(object, paths, 0)
}
/// Sort an array by keys
///
/// # Arguments
/// * `objects` - Array of objects to sort
/// * `keys` - Array of keys to sort by
///
/// # Returns
/// Sorted array
pub fn jv_sort(objects: Jv, keys: Jv) -> Jv {
    assert!(jv_get_kind(& objects) == JvKind::Array);
    assert!(jv_get_kind(& keys) == JvKind::Array);
    assert!(jv_array_length(jv_copy(& objects)) == jv_array_length(jv_copy(& keys)));
    let n = jv_array_length(jv_copy(&objects));
    let entries = sort_items(objects, keys);
    let mut ret = jv_array();
    for (i, entry) in entries.into_iter().enumerate() {
        jv_free(entry.key);
        ret = jv_array_set(ret, i as i32, entry.object);
    }
    ret
}
/// Get the kind of a jv value
///
/// The kind is stored in the kind_flags field, masked appropriately
#[inline]
fn jv_get_kind(x: &Jv) -> JvKind {
    let kind_value = x.kind_flags & 0x0F;
    match kind_value {
        0 => JvKind::Invalid,
        1 => JvKind::Null,
        2 => JvKind::False,
        3 => JvKind::True,
        4 => JvKind::Number,
        5 => JvKind::String,
        6 => JvKind::Array,
        7 => JvKind::Object,
        _ => JvKind::Invalid,
    }
}
fn jv_copy(v: &Jv) -> Jv {
    v.copy()
}
fn jv_free(v: Jv) {
    v.free();
}
fn jv_null() -> Jv {
    Jv::null()
}
fn jv_true() -> Jv {
    Jv::jv_true()
}
fn jv_invalid() -> Jv {
    Jv::invalid()
}
fn jv_invalid_with_msg(msg: Jv) -> Jv {
    Jv::invalid_with_msg(msg)
}
fn jv_string(s: &str) -> Jv {
    Jv::string(s)
}
fn jv_string_fmt(fmt: &str, arg: &str) -> Jv {
    if arg.is_empty() {
        Jv::string(fmt)
    } else {
        Jv::string_fmt(fmt, &[arg])
    }
}
fn jv_number(n: f64) -> Jv {
    Jv::number(n)
}
fn jv_array() -> Jv {
    Jv::array()
}
fn jv_array_sized(n: i32) -> Jv {
    Jv::array_sized(n)
}
fn jv_array_length(v: Jv) -> i32 {
    crate::jv::jv_array_length(&v)
}
fn jv_array_get(arr: Jv, idx: i32) -> Jv {
    crate::jv::jv_array_get(arr, idx)
}
fn jv_array_set(arr: Jv, idx: i32, val: Jv) -> Jv {
    crate::jv::jv_array_set(arr, idx, val)
}
fn jv_array_append(arr: Jv, val: Jv) -> Jv {
    crate::jv::jv_array_append(arr, val)
}
fn jv_array_slice(arr: Jv, start: i32, end: i32) -> Jv {
    crate::jv::jv_array_slice(arr, start, end)
}
fn jv_array_indexes(arr: Jv, indexes: Jv) -> Jv {
    crate::jv::jv_array_indexes(arr, indexes)
}
fn jv_object_get(obj: Jv, key: Jv) -> Jv {
    crate::jv::jv_object_get(&obj, key)
}
fn jv_object_length(obj: Jv) -> i32 {
    crate::jv::jv_object_length(&obj)
}
fn jv_object_iter(obj: &Jv) -> i32 {
    crate::jv::jv_object_iter(obj)
}
fn jv_object_iter_valid(obj: &Jv, iter: i32) -> bool {
    crate::jv::jv_object_iter_valid(obj, iter)
}
fn jv_object_iter_next(obj: &Jv, iter: i32) -> i32 {
    crate::jv::jv_object_iter_next(obj, iter)
}
fn jv_object_iter_key(obj: &Jv, iter: i32) -> Jv {
    crate::jv::jv_object_iter_key(obj, iter)
}
fn jv_object_iter_value(obj: &Jv, iter: i32) -> Jv {
    crate::jv::jv_object_iter_value(obj, iter)
}
fn jv_string_length_bytes(s: Jv) -> i32 {
    crate::jv::jv_string_length_bytes(&s)
}
fn jv_string_length_codepoints(s: Jv) -> i32 {
    crate::jv::jv_string_length_codepoints(s)
}
fn jv_string_value(s: &Jv) -> &str {
    crate::jv::jv_string_value(s)
}
fn jv_string_slice(s: Jv, start: i32, end: i32) -> Jv {
    crate::jv::jv_string_slice(s, start, end)
}
fn jv_number_value(n: &Jv) -> f64 {
    f64::from_bits(n.u)
}
fn jvp_number_is_nan(n: &Jv) -> bool {
    jv_number_value(n).is_nan()
}
fn jv_equal(a: Jv, b: Jv) -> bool {
    crate::jv::jv_equal(&a, &b)
}
fn jv_kind_name(kind: JvKind) -> &'static str {
    match kind {
        JvKind::Invalid => "invalid",
        JvKind::Null => "null",
        JvKind::False => "false",
        JvKind::True => "true",
        JvKind::Number => "number",
        JvKind::String => "string",
        JvKind::Array => "array",
        JvKind::Object => "object",
    }
}
fn jv_mem_calloc<T: Default + Clone>(n: usize) -> Vec<T> {
    vec![T::default(); n]
}
fn jv_mem_free<T>(_v: Vec<T>) {}
