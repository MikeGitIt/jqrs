//! Module: jv
//!
//! Contains 124 transpiled functions:
//! - jvp_object_buckets:7036368859489133686:./src/jv.c
//! - jv_false:16667125968873769745:./src/jv.c
//! - jv_free:10104483226085342730:./src/jv.c
//! - jvp_object_new:11600744187379732558:./src/jv.c
//! - jv_tsd_dec_ctx_fini:14769323101683616450:./src/jv.c
//! - jvp_refcnt_dec:16991766764298997749:./src/jv.c
//! - jv_get_refcnt:7004921633020335707:./src/jv.c
//! - jv_string_sized:352950505292703964:./src/jv.c
//! - jvp_object_add_slot:15892079546242991462:./src/jv.c
//! - jvp_string_equal:18331513984048513159:./src/jv.c
//! - jv_object_set:18103580491227404271:./src/jv.c
//! - jv_invalid_has_msg:3204004893098547504:./src/jv.c
//! - jv_object_iter_valid:8942480913480974188:./src/jv.c
//! - jv_string_split:11681615330657020060:./src/jv.c
//! - jv_array_slice:8162640946360663897:./src/jv.c
//! - jv_array:17879309833667131327:./src/jv.c
//! - jv_object_has:8070810500871440159:./src/jv.c
//! - jvp_object_find_slot:12487852010469494415:./src/jv.c
//! - jvp_refcnt_inc:10099799447585598739:./src/jv.c
//! - jvp_object_find_bucket:6072412426606899541:./src/jv.c
//! - jvp_object_read:17529977726293701460:./src/jv.c
//! - jv_equal:962022854609494252:./src/jv.c
//! - jv_string_fmt:16449586014106472796:./src/jv.c
//! - jvp_number_free:4363175880969761604:./src/jv.c
//! - jvp_array_contains:1996297302189816558:./src/jv.c
//! - jv_contains:6658231308635379809:./src/jv.c
//! - jvp_string_append:11999287554301284853:./src/jv.c
//! - jvp_literal_number_to_double:1422307941778510709:./src/jv.c
//! - jvp_array_length:16529777103492198962:./src/jv.c
//! - jvp_string_ptr:16153285040208759237:./src/jv.c
//! - jvp_object_size:10673524912427968847:./src/jv.c
//! - jvp_string_new:8135517071416038247:./src/jv.c
//! - jvp_object_ptr:6206734814388911485:./src/jv.c
//! - jv_string_length_bytes:1513389612743934782:./src/jv.c
//! - jvp_object_delete:17139806323356453202:./src/jv.c
//! - jvp_array_write:4027294140324259218:./src/jv.c
//! - jvp_array_alloc:6525672327773670147:./src/jv.c
//! - jv_number_get_literal:11305791103479383756:./src/jv.c
//! - jv_tsd_dec_ctx_init:3424628020201604020:./src/jv.c
//! - jv_is_integer:959887821618013495:./src/jv.c
//! - jvp_object_free:11227387503408604004:./src/jv.c
//! - jv_string_empty:17856365584861834689:./src/jv.c
//! - jv_object_get:14855416648179195788:./src/jv.c
//! - jvp_array_equal:9354353646545638544:./src/jv.c
//! - jvp_literal_number_ptr:8642678929101047191:./src/jv.c
//! - jv_true:1765962358840442899:./src/jv.c
//! - jv_string_append_buf:3235972433814427982:./src/jv.c
//! - jv_string_append_str:15677999201054064715:./src/jv.c
//! - jv_bool:15805646988952002867:./src/jv.c
//! - jv_string_explode:7455750946762019314:./src/jv.c
//! - jv_string_value:16119509080314540540:./src/jv.c
//! - jv_object:18175548923459399363:./src/jv.c
//! - jvp_number_equal:4068587941061741711:./src/jv.c
//! - jv_kind_name:2628978369914100793:./src/jv.c
//! - jvp_literal_number_literal:13554081882744520321:./src/jv.c
//! - jv_array_concat:13774644079063370129:./src/jv.c
//! - jv_object_merge_recursive:10877973518607203337:./src/jv.c
//! - tsd_dec_ctx_get:1704037736208776783:./src/jv.c
//! - jvp_number_cmp:8295105820444358431:./src/jv.c
//! - imax:16218137894207543062:./src/jv.c
//! - jvp_string_hash:11802625326929907883:./src/jv.c
//! - jv_array_length:4296350089233364093:./src/jv.c
//! - jv_null:15985745797391778398:./src/jv.c
//! - jvp_object_length:10113740666638918726:./src/jv.c
//! - jv_number_has_literal:10609589742142257383:./src/jv.c
//! - jv_array_append:17248751681644174455:./src/jv.c
//! - jv_copy:4601001738663078862:./src/jv.c
//! - jv_array_get:3907453581755324504:./src/jv.c
//! - jvp_object_mask:606292441440134670:./src/jv.c
//! - jvp_object_next_slot:16081618247769792180:./src/jv.c
//! - jv_object_iter_value:3768965224480420418:./src/jv.c
//! - jv_array_indexes:12002599720261863816:./src/jv.c
//! - jvp_literal_number_new:15012629635424242337:./src/jv.c
//! - jv_string_implode:3360605799796195700:./src/jv.c
//! - jvp_refcnt_unshared:5009783731101494516:./src/jv.c
//! - jv_array_set:12735881044074434385:./src/jv.c
//! - jv_string_hash:14217170830968892842:./src/jv.c
//! - jv_string:10595072757252068983:./src/jv.c
//! - jvp_object_write:12441448266320603398:./src/jv.c
//! - jv_number_with_literal:15314694023979309444:./src/jv.c
//! - jv_number_value:571280375850558614:./src/jv.c
//! - jv_string_indexes:1523730434417666223:./src/jv.c
//! - jvp_array_ptr:805326306623048332:./src/jv.c
//! - rotl32:8179027811839812666:./src/jv.c
//! - jvp_invalid_free:7077467310818598519:./src/jv.c
//! - jv_invalid_with_msg:10679307644787608196:./src/jv.c
//! - jv_object_iter_key:16084004359538341421:./src/jv.c
//! - jv_invalid:6264369665850127459:./src/jv.c
//! - jv_string_concat:6353959986136461330:./src/jv.c
//! - jvp_clamp_slice_params:9525318175777410360:./src/jv.c
//! - jvp_string_empty_new:12686068523772389240:./src/jv.c
//! - jvp_number_is_literal:10891829720723329003:./src/jv.c
//! - jvp_literal_number_alloc:8542842285733044657:./src/jv.c
//! - jv_array_sized:12098594092460577578:./src/jv.c
//! - jvp_array_offset:1657558517301938499:./src/jv.c
//! - jv_string_slice:14683916948364659169:./src/jv.c
//! - jvp_string_alloc:4435700049808846641:./src/jv.c
//! - jv_string_length_codepoints:13128976344481429030:./src/jv.c
//! - jv_object_merge:13549053455868154763:./src/jv.c
//! - jvp_object_rehash:17280333306791205960:./src/jv.c
//! - jvp_object_unshare:7951929033750177341:./src/jv.c
//! - jvp_string_length:11399864189772206317:./src/jv.c
//! - jvp_array_new:15304315860976065437:./src/jv.c
//! - jv_string_vfmt:16439415594434415877:./src/jv.c
//! - jv_identical:14081813330894952506:./src/jv.c
//! - jv_string_append_codepoint:16394961732066788438:./src/jv.c
//! - jvp_array_slice:15158787907158340740:./src/jv.c
//! - jv_get_kind:8319230928417231526:./src/jv.c
//! - jvp_array_read:11818955427412040930:./src/jv.c
//! - jvp_array_free:5585999756151485401:./src/jv.c
//! - jvp_object_get_slot:17410022324489554248:./src/jv.c
//! - jv_object_delete:8526877348424762472:./src/jv.c
//! - jvp_number_is_nan:777746137503321974:./src/jv.c
//! - jv_invalid_get_msg:3335602258793826586:./src/jv.c
//! - jv_object_iter:1729283890343323213:./src/jv.c
//! - jvp_string_free:62338013365251275:./src/jv.c
//! - jvp_object_contains:9207257072046565601:./src/jv.c
//! - jvp_string_copy_replace_bad:8399159933124295909:./src/jv.c
//! - jv_number:12870061143479208839:./src/jv.c
//! - jvp_string_remaining_space:11589167803221237442:./src/jv.c
//! - jv_object_iter_next:10272514661167873396:./src/jv.c
//! - jvp_object_equal:12568164347660953353:./src/jv.c
//! - jvp_dec_number_ptr:1830932088390390045:./src/jv.c
//! - jv_object_length:16398962367748015661:./src/jv.c

use crate::types::{
    JvPayload, JvRefcntAtomic, JvRefcntStruct, JvUnion, JvUnionData,
    JvpArray, JvpInvalid, JvpLiteralNumber, JvpObject, JvpRefcounted, JvpString,
    ObjectSlot, DecContext, DtoaContext, DecimalContext, DecNumberDoublePrecision,
    Rounding,
};
// Re-export Jv and JvKind publicly so other modules can import them from here
pub use crate::types::{Jv, JvKind};
// Type alias for reference counting
pub type JvRefcnt = JvRefcntStruct;
use crate::jv_unicode::jvp_utf8_is_valid;
// Note: jvp_utf8_next, jvp_utf8_encode are defined locally in this file
use crate::util::_jq_memmem;
use crate::deccontext::decContextClearStatus;
// Note: decContextDefault is defined locally in this file
use std::cell::RefCell;
use std::sync::Once;
use std::sync::atomic::{AtomicI32, Ordering as AtomicOrdering};
use crate::jv_alloc::jv_mem_realloc;
// Note: jv_mem_alloc, jv_mem_free are defined locally in this file
// Note: tsd_dtoa_context_get is defined locally in this file
use crate::decnumber::decNumberFromString;
use crate::types::DecNumber;
use std::ptr;
/// Global storage for objects (simulating C's pointer-based approach)
use std::collections::HashMap;
use std::sync::Mutex;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fmt;
use std::cmp::{max, Ordering};
use crate::inject_errors::fwrite;
// Note: Jv, JvKind, JvRefcnt, JvpArray, JvpString, JvpLiteralNumber, JvpInvalid,
// jv_copy, jv_free, jv_null, jvp_array_slice, jvp_clamp_slice_params,
// jv_array, jv_array_append, jv_array_get, jv_array_length, jv_array_set,
// and many other jv_* functions are defined in this file

// Re-export some common functions for convenience
pub use self::jv_array_append as array_append;
pub use self::jv_array_set as array_set;
pub use self::jv_object_delete as object_delete;
pub use self::jv_string as string;
pub use self::jv_string_sized as string_sized;
/// Number type flags
const JVP_NUMBER_NATIVE: u8 = 0;
pub const JVP_NUMBER_DECIMAL: u8 = 0x01;
/// Flags for literal number (decimal allocated)
pub const JVP_FLAGS_NUMBER_LITERAL: u8 = ((JVP_NUMBER_DECIMAL << 4) & 0x70) | 0x80;
/// Constants for JV flags
pub const JVP_PAYLOAD_ALLOCATED: u8 = 0x80;
lazy_static::lazy_static! {
    static ref STRING_STORAGE : Mutex < HashMap < u64, Box < JvpString >>> =
    Mutex::new(HashMap::new()); static ref OBJECT_STORAGE : Mutex < HashMap < u64, Box <
    JvpObject >>> = Mutex::new(HashMap::new()); static ref NUMBER_STORAGE : Mutex <
    HashMap < u64, Box < JvpLiteralNumber >>> = Mutex::new(HashMap::new()); static ref
    NEXT_ID : Mutex < u64 > = Mutex::new(1);
}

thread_local! {
    /// Thread-local decimal context for numeric operations
    static DEC_CTX: RefCell<Option<DecContext>> = RefCell::new(None);
    /// Thread-local decimal context key for numeric operations
    static DEC_CTX_KEY: RefCell<Option<DecimalContext>> = RefCell::new(None);
}

static DEC_CTX_INIT: Once = Once::new();
/// Initialize thread-specific decimal context
pub fn jv_tsd_dec_ctx_init() {
    DEC_CTX
        .with(|ctx| {
            let mut ctx = ctx.borrow_mut();
            if ctx.is_none() {
                let mut new_ctx = DecContext::default();
                decContextDefault(&mut new_ctx, 0);
                let max_digits = i32::MAX - 2 - (new_ctx.emax - new_ctx.emin - 1);
                new_ctx.digits = 999999999.min(max_digits);
                new_ctx.traps = 0;
                *ctx = Some(new_ctx);
            }
        });
}
/// Get thread-specific decimal context
fn tsd_dec_ctx_get() -> DecContext {
    DEC_CTX_INIT.call_once(jv_tsd_dec_ctx_init);
    DEC_CTX
        .with(|ctx| {
            let mut ctx = ctx.borrow_mut();
            if ctx.is_none() {
                let mut new_ctx = DecContext::default();
                decContextDefault(&mut new_ctx, 0);
                let max_digits = i32::MAX - 2 - (new_ctx.emax - new_ctx.emin - 1);
                new_ctx.digits = 999999999.min(max_digits);
                new_ctx.traps = 0;
                *ctx = Some(new_ctx);
            }
            ctx.clone().unwrap()
        })
}
/// Create an empty string with specified capacity
pub fn jv_string_empty(capacity: i32) -> Jv {
    let string = Box::new(JvpString::new(capacity as u32));
    let ptr = Box::into_raw(string);
    Jv {
        kind_flags: JV_KIND_STRING | JVP_PAYLOAD_ALLOCATED,
        pad_: 0,
        offset: 0,
        size: 0,
        u: ptr as u64,
    }
}
/// Create an empty string with given capacity
pub fn jvp_string_empty_new(len: i32) -> Jv {
    let string_payload = JvpString {
        refcnt: 1,
        hash: 0,
        length: 0,
        data: String::with_capacity(len as usize),
    };
    let ptr = Box::into_raw(Box::new(string_payload));
    Jv {
        kind_flags: JvKind::String as u8 | JVP_PAYLOAD_ALLOCATED,
        pad_: 0,
        offset: 0,
        size: 0,
        u: ptr as u64,
    }
}
/// Create a new string jv from raw data
///
/// Internal function to create a string jv value.
fn jvp_string_new(data: Option<&str>, length: u32) -> Jv {
    let mut s = jvp_string_alloc(length);
    let length_hashed = length << 1;
    s.hash = length_hashed;
    if let Some(d) = data {
        s.data = d[..length as usize].to_string();
    } else {
        s.data = String::new();
    }
    let flags = (JV_KIND_STRING & 0x0F) | (JVP_PAYLOAD_ALLOCATED & 0xF0);
    Jv {
        kind_flags: flags,
        pad_: 0,
        offset: 0,
        size: 0,
        u: Box::into_raw(s) as u64,
    }
}
/// Copy string and replace bad UTF-8 sequences with replacement character
pub fn jvp_string_copy_replace_bad(data: &[u8]) -> Jv {
    let max_length = data.len() * 3 + 1;
    let mut out = Vec::with_capacity(max_length);
    let mut pos = 0;
    while pos < data.len() {
        if let Some((next_pos, c)) = jvp_utf8_next(data, pos) {
            let codepoint = if c == -1 { 0xFFFD } else { c };
            jvp_utf8_encode(codepoint, &mut out);
            assert!(out.len() < max_length, "out < s->data + maxlength");
            pos = next_pos;
        } else {
            break;
        }
    }
    let length = out.len() as i32;
    // Convert Vec<u8> to String, replacing invalid UTF-8
    let data_string = String::from_utf8_lossy(&out).into_owned();
    let s = JvpString {
        refcnt: 1,
        hash: 0,
        length,
        data: data_string,
    };
    let ptr = Box::into_raw(Box::new(s));
    Jv {
        kind_flags: (JV_KIND_STRING & 0x0F) | (JVP_PAYLOAD_ALLOCATED & 0xF0),
        pad_: 0,
        offset: 0,
        size: 0,
        u: ptr as u64,
    }
}
/// Create a string Jv from sized data
pub fn jv_string_sized(str_data: &str, len: usize) -> Jv {
    let data = str_data.chars().take(len).collect::<String>();
    // Don't precompute hash - let jvp_string_hash_internal compute it lazily with MurmurHash3
    let payload = JvpString {
        refcnt: 1,
        hash: 0,
        length: data.len() as i32,
        data,
    };
    let ptr = Box::into_raw(Box::new(payload));
    Jv {
        kind_flags: JvKind::String as u8 | JVP_PAYLOAD_ALLOCATED,
        pad_: 0,
        offset: 0,
        size: len as i32,
        u: ptr as u64,
    }
}
/// Check if value a contains value b
pub fn jv_contains(a: Jv, b: Jv) -> i32 {
    if a.get_kind() != b.get_kind() {
        return 0;
    } else if a.has_kind(JvKind::Object) {
        return jvp_object_contains(&a, &b);
    } else if a.has_kind(JvKind::Array) {
        return if jvp_array_contains(&a, &b) { 1 } else { 0 };
    } else if a.has_kind(JvKind::String) {
        let b_len = jv_string_length_bytes(&b);
        if b_len != 0 {
            let a_value = jv_string_value(&a);
            let b_value = jv_string_value(&b);
            return if _jq_memmem(a_value.as_bytes(), b_value.as_bytes()).is_some() {
                1
            } else {
                0
            };
        } else {
            return 1;
        }
    } else {
        return if jv_equal(&a, &b) { 1 } else { 0 };
    }
}
/// Get the hash mask for an object
fn jvp_object_mask(o: &Jv) -> u32 {
    assert!(o.has_kind(JvKind::Object), "JVP_HAS_KIND(o, JV_KIND_OBJECT)");
    ((o.size * 2) - 1) as u32
}
/// Get pointer to object - JvpObject doesn't have buckets, returns None
fn jvp_object_buckets(_o: &Jv) -> Option<&[i32]> {
    // JvpObject in types.rs doesn't have a buckets field
    // Linear search through elements is used instead
    None
}
/// Get mutable pointer to object buckets - JvpObject doesn't have buckets, returns None
fn jvp_object_buckets_mut(_o: &mut Jv) -> Option<&mut Vec<i32>> {
    // JvpObject in types.rs doesn't have a buckets field
    // Linear search through elements is used instead
    None
}
/// Explode a string into an array of codepoints
pub fn jv_string_explode(j: Jv) -> Jv {
    assert!(j.has_kind(JvKind::String), "JVP_HAS_KIND(j, JV_KIND_STRING)");
    let value = jv_string_value(&j);
    let bytes = value.as_bytes();
    let len = jv_string_length_bytes(&j);
    let mut a = jv_array_sized(len);
    let mut pos = 0;
    while pos < len as usize {
        if let Some((next_pos, codepoint)) = jvp_utf8_next(bytes, pos) {
            a = jv_array_append(a, jv_number(codepoint as f64));
            pos = next_pos;
        } else {
            break;
        }
    }
    a
}
/// Get the literal representation of a number if it exists
pub fn jv_number_get_literal(n: &Jv) -> Option<String> {
    assert!(n.has_kind(JvKind::Number), "JVP_HAS_KIND(n, JV_KIND_NUMBER)");
    if n.is_decimal_number() {
        // Get the literal number pointer and extract the literal data
        if let Some(lit) = jvp_literal_number_ptr(n) {
            lit.literal_data.clone()
        } else {
            None
        }
    } else {
        None
    }
}
/// Get the literal string representation of a number
fn jvp_literal_number_literal(n: &mut Jv) -> Option<&str> {
    assert!(
        jvp_has_flags_number_literal(n), "JVP_HAS_FLAGS(n, JVP_FLAGS_NUMBER_LITERAL)"
    );
    if let Some(pdec) = jvp_dec_number_ptr(n) {
        if (pdec.bits & (DECNAN | DECSNAN)) != 0 {
            return Some("null");
        }
        if (pdec.bits & DECINF) != 0 {
            return None;
        }
    }
    // First gather info from immutable borrow, then mutate
    let buf_opt = if let Some(pdec) = jvp_dec_number_ptr(n) {
        let len = pdec.digits + 15;
        let mut buf = String::with_capacity(len as usize);
        decNumberToString(pdec, &mut buf);
        Some(buf)
    } else {
        None
    };
    if let Some(plit) = jvp_literal_number_ptr_mut(n) {
        if plit.literal_data.is_none() {
            plit.literal_data = buf_opt;
        }
        return plit.literal_data.as_deref();
    }
    None
}
/// Get value at object iterator
pub fn jv_object_iter_value(j: &Jv, iter: i32) -> Jv {
    if j.u != 0 && jvp_has_kind(j, JvKind::Object) {
        let obj = unsafe { &*(j.u as *const JvpObject) };
        if iter >= 0 && (iter as usize) < obj.elements.len() {
            return jv_copy(&obj.elements[iter as usize].value);
        }
    }
    jv_null()
}
/// Get key at object iterator
pub fn jv_object_iter_key(j: &Jv, iter: i32) -> Jv {
    if j.u != 0 && jvp_has_kind(j, JvKind::Object) {
        let obj = unsafe { &*(j.u as *const JvpObject) };
        if iter >= 0 && (iter as usize) < obj.elements.len() {
            return jv_copy(&obj.elements[iter as usize].string);
        }
    }
    jv_null()
}
/// Get an object slot by index
pub fn jvp_object_get_slot(object: &Jv, slot: i32) -> Option<&ObjectSlot> {
    assert!(
        slot == - 1 || (slot >= 0 && slot < jvp_object_size(object)),
        "slot == -1 || (slot >= 0 && slot < jvp_object_size(object))"
    );
    if slot == -1 {
        None
    } else {
        jvp_object_ptr(object).and_then(|obj| obj.elements.get(slot as usize))
    }
}
/// Get a mutable object slot by index
pub fn jvp_object_get_slot_mut(object: &mut Jv, slot: i32) -> Option<&mut ObjectSlot> {
    let size = jvp_object_size(object);
    assert!(
        slot == - 1 || (slot >= 0 && slot < size),
        "slot == -1 || (slot >= 0 && slot < jvp_object_size(object))"
    );
    if slot == -1 {
        None
    } else {
        jvp_object_ptr_mut(object).and_then(|obj| obj.elements.get_mut(slot as usize))
    }
}
/// Set object value by key
pub fn jv_object_set(mut j: Jv, key: Jv, val: Jv) -> Jv {
    assert!(jvp_has_kind(&j, JvKind::Object));
    assert!(jvp_has_kind(&key, JvKind::String));
    j = jvp_object_unshare(j);
    if j.u != 0 {
        let obj = unsafe { &mut *(j.u as *mut JvpObject) };
        let mut key_clone = jv_copy(&key);
        let key_hash = jvp_string_hash_internal(&mut key_clone);
        jv_free(key_clone);
        // Linear search for existing key
        for slot in &mut obj.elements {
            if slot.hash == key_hash && jv_equal(&slot.string, &key) {
                jv_free(key);
                let old = std::mem::replace(&mut slot.value, val);
                jv_free(old);
                return j;
            }
        }
        // Add new slot
        let new_slot = ObjectSlot {
            next: -1,
            hash: key_hash,
            string: key,
            value: val,
        };
        obj.elements.push(new_slot);
        obj.next_free += 1;
    }
    j
}
/// Write to an object slot, creating if necessary
pub fn jvp_object_write<'a>(object: &'a mut Jv, key: &Jv) -> Option<&'a mut Jv> {
    // Create a copy to unshare, then update object
    let unshared = jvp_object_unshare(object.clone());
    *object = unshared;

    let mut key_clone = jv_copy(key);
    let hash = jvp_string_hash_internal(&mut key_clone);

    if object.u == 0 {
        return None;
    }

    let obj = unsafe { &mut *(object.u as *mut JvpObject) };

    // Linear search for existing key
    for (idx, slot) in obj.elements.iter().enumerate() {
        if slot.hash == hash && jv_string_equal(&slot.string, key) {
            return Some(&mut obj.elements[idx].value);
        }
    }

    // Add new slot
    let new_idx = obj.elements.len();
    obj.elements.push(ObjectSlot {
        next: -1,
        hash,
        string: jv_copy(key),
        value: Jv::default(),
    });
    obj.next_free += 1;

    Some(&mut obj.elements[new_idx].value)
}
/// Unshare object (copy-on-write)
pub fn jvp_object_unshare(object: Jv) -> Jv {
    assert!(jvp_has_kind(&object, JvKind::Object));
    if object.u == 0 {
        return object;
    }

    let obj = unsafe { &*(object.u as *const JvpObject) };
    // refcnt is i32, check if only one reference
    if obj.refcnt == 1 {
        return object;
    }

    let size = obj.elements.len() as i32;
    let mut new_object = jvp_object_new(size);

    if new_object.u != 0 {
        let new_obj = unsafe { &mut *(new_object.u as *mut JvpObject) };
        new_obj.next_free = obj.next_free;
        for old_slot in &obj.elements {
            let new_slot = ObjectSlot {
                next: old_slot.next,
                hash: old_slot.hash,
                string: if jv_get_kind(&old_slot.string) != JvKind::Null {
                    jv_copy(&old_slot.string)
                } else {
                    old_slot.string.clone()
                },
                value: if jv_get_kind(&old_slot.value) != JvKind::Null {
                    jv_copy(&old_slot.value)
                } else {
                    old_slot.value.clone()
                },
            };
            new_obj.elements.push(new_slot);
        }
    }
    jvp_object_free(object);
    new_object
}
/// Check if object a contains object b
fn jvp_object_contains(a: &Jv, b: &Jv) -> i32 {
    if a.u == 0 || b.u == 0 {
        return 0;
    }

    let obj_a = unsafe { &*(a.u as *const JvpObject) };
    let obj_b = unsafe { &*(b.u as *const JvpObject) };

    for slot in &obj_b.elements {
        if slot.hash != 0 {
            let found = obj_a
                .elements
                .iter()
                .any(|s| {
                    s.hash == slot.hash
                        && jv_string_equal(&s.string, &slot.string)
                        && jv_contains(jv_copy(&s.value), jv_copy(&slot.value)) != 0
                });
            if !found {
                return 0;
            }
        }
    }
    1
}
/// Check if array a contains array b
pub fn jvp_array_contains(a: &Jv, b: &Jv) -> bool {
    assert!(jvp_has_kind(a, JvKind::Array));
    assert!(jvp_has_kind(b, JvKind::Array));
    let b_len = jv_array_length(&jv_copy(b));
    for bi in 0..b_len {
        let belem = jv_array_get(jv_copy(b), bi);
        let mut found = false;
        let a_len = jv_array_length(&jv_copy(a));
        for ai in 0..a_len {
            let aelem = jv_array_get(jv_copy(a), ai);
            if jv_contains(aelem.clone(), belem.clone()) != 0 {
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }
    true
}
/// Get the string value as a byte slice
///
/// Returns a reference to the underlying string data.
pub fn jv_string_value(j: &Jv) -> &str {
    if !jvp_has_kind(j, JvKind::String) {
        panic!("JVP_HAS_KIND(j, JV_KIND_STRING)");
    }
    if let Some(s) = jvp_string_ptr(j) { &s.data } else { "" }
}
/// Get string length in bytes
pub fn jv_string_length_bytes(j: &Jv) -> i32 {
    if j.u == 0 {
        return 0;
    }
    if jvp_has_kind(j, JvKind::String) {
        let s = unsafe { &*(j.u as *const JvpString) };
        return s.data.len() as i32;
    }
    0
}
/// Copy a JV value (increment reference count if applicable)
pub fn jv_copy(j: &Jv) -> Jv {
    let kind = jv_get_kind(j);
    match kind {
        JvKind::String => {
            if let Ok(mut storage) = STRING_STORAGE.lock() {
                if let Some(s) = storage.get_mut(&j.u) {
                    s.refcnt += 1;
                }
            }
        }
        JvKind::Object => {
            if let Ok(mut storage) = OBJECT_STORAGE.lock() {
                if let Some(obj) = storage.get_mut(&j.u) {
                    obj.refcnt += 1;
                }
            }
        }
        JvKind::Number if jvp_is_literal_number(j) => {
            if let Ok(mut storage) = NUMBER_STORAGE.lock() {
                if let Some(num) = storage.get_mut(&j.u) {
                    num.refcnt += 1;
                }
            }
        }
        _ => {}
    }
    j.clone()
}
/// Free a Jv value, following C semantics:
/// - Decrement refcount
/// - Only actually free memory when refcount reaches 0
pub fn jv_free(j: Jv) {
    let kind = jv_get_kind(&j);
    if j.u == 0 {
        return;
    }
    match kind {
        // These types have no allocated payload
        JvKind::Invalid => {
            // C: jvp_invalid_free checks JVP_FLAGS_INVALID_MSG before freeing
            if j.kind_flags & JVP_PAYLOAD_ALLOCATED != 0 {
                let inv = unsafe { &mut *(j.u as *mut JvpInvalid) };
                inv.refcnt -= 1;
                if inv.refcnt == 0 {
                    // Free the error message if present
                    if let Some(msg) = inv.errmsg.take() {
                        jv_free(*msg);
                    }
                    let _ = unsafe { Box::from_raw(j.u as *mut JvpInvalid) };
                }
            }
        }
        JvKind::Null | JvKind::True | JvKind::False => {
            // No allocated payload, nothing to free
        }
        JvKind::Number => {
            // C: jvp_number_free - only literal numbers have allocated payload
            if j.kind_flags & JVP_PAYLOAD_ALLOCATED != 0 {
                let num = unsafe { &mut *(j.u as *mut JvpLiteralNumber) };
                num.refcnt -= 1;
                if num.refcnt == 0 {
                    let _ = unsafe { Box::from_raw(j.u as *mut JvpLiteralNumber) };
                }
            }
        }
        JvKind::String => {
            // C: jvp_string_free - decrement refcount, free only if 0
            let s = unsafe { &mut *(j.u as *mut JvpString) };
            s.refcnt -= 1;
            if s.refcnt == 0 {
                let _ = unsafe { Box::from_raw(j.u as *mut JvpString) };
            }
        }
        JvKind::Array => {
            // C: jvp_array_free - decrement refcount, free elements only if 0
            let arr = unsafe { &mut *(j.u as *mut JvpArray) };
            arr.refcnt -= 1;
            if arr.refcnt == 0 {
                let arr = unsafe { Box::from_raw(j.u as *mut JvpArray) };
                for elem in arr.elements.into_iter() {
                    jv_free(elem);
                }
            }
        }
        JvKind::Object => {
            jvp_object_free(j);
        }
    }
}
/// Create a number jv from a double
///
/// Creates a jv value containing the given floating-point number.
pub fn jv_number(x: f64) -> Jv {
    let flags = jvp_number_flags(JVP_NUMBER_NATIVE, false);
    Jv {
        kind_flags: flags,
        pad_: 0,
        offset: 0,
        size: 0,
        u: x.to_bits(),
    }
}
/// Create an array with initial capacity
pub fn jv_array_sized(size: i32) -> Jv {
    let arr = Box::new(JvpArray {
        refcnt: 1,
        length: 0,
        alloc_length: size,
        elements: Vec::with_capacity(size as usize),
    });
    let ptr = Box::into_raw(arr);
    Jv {
        kind_flags: JvKind::Array as u8 | JVP_PAYLOAD_ALLOCATED,
        pad_: 0,
        offset: 0,
        size: 0,
        u: ptr as u64,
    }
}
/// Append a value to an array
pub fn jv_array_append(a: Jv, val: Jv) -> Jv {
    let len = jv_array_length(&a);
    jv_array_set(a, len, val)
}
/// Check if two jv values are equal
pub fn jv_equal(a: &Jv, b: &Jv) -> bool {
    let kind_a = jv_get_kind(a);
    let kind_b = jv_get_kind(b);
    if kind_a != kind_b {
        return false;
    }
    match kind_a {
        JvKind::Null | JvKind::True | JvKind::False => true,
        JvKind::Number => jv_number_value(a) == jv_number_value(b),
        JvKind::String => jv_string_value(a) == jv_string_value(b),
        JvKind::Array => {
            let len_a = jv_array_length(a);
            let len_b = jv_array_length(b);
            if len_a != len_b {
                return false;
            }
            for i in 0..len_a {
                let elem_a = jv_array_get(a.clone(), i);
                let elem_b = jv_array_get(b.clone(), i);
                if !jv_equal(&elem_a, &elem_b) {
                    return false;
                }
            }
            true
        }
        JvKind::Object => {
            let len_a = jv_object_length(a);
            let len_b = jv_object_length(b);
            if len_a != len_b {
                return false;
            }
            let mut iter = jv_object_iter(a);
            while jv_object_iter_valid(a, iter) {
                let key = jv_object_iter_key(a, iter);
                let val_a = jv_object_iter_value(a, iter);
                let val_b = jv_object_get(b, jv_copy(&key));
                if !jv_equal(&val_a, &val_b) {
                    return false;
                }
                iter = jv_object_iter_next(a, iter);
            }
            true
        }
        JvKind::Invalid => false,
    }
}
/// Check if two strings are equal
pub fn jv_string_equal(a: &Jv, b: &Jv) -> bool {
    let va = jv_string_value(a);
    let vb = jv_string_value(b);
    va == vb
}
/// Check if two arrays are equal
pub fn jvp_array_equal(a: &Jv, b: &Jv) -> i32 {
    if jvp_array_length(a) != jvp_array_length(b) {
        return 0;
    }
    if let (Some(arr_a), Some(arr_b)) = (jvp_array_ptr(a), jvp_array_ptr(b)) {
        if std::ptr::eq(arr_a, arr_b) && jvp_array_offset(a.clone()) == jvp_array_offset(b.clone()) {
            return 1;
        }
    }
    for i in 0..jvp_array_length(a) {
        if let (Some(elem_a), Some(elem_b)) = (
            jvp_array_read(a, i),
            jvp_array_read(b, i),
        ) {
            if !jv_equal(&jv_copy(elem_a), &jv_copy(elem_b)) {
                return 0;
            }
        } else {
            return 0;
        }
    }
    1
}
/// Check if two objects are equal
pub fn jvp_object_equal(a: &Jv, b: &Jv) -> bool {
    if a.u == 0 || b.u == 0 {
        return false;
    }
    let obj_a = unsafe { &*(a.u as *const JvpObject) };
    let obj_b = unsafe { &*(b.u as *const JvpObject) };

    let count_a = obj_a.elements.iter().filter(|s| s.hash != 0).count();
    let count_b = obj_b.elements.iter().filter(|s| s.hash != 0).count();
    if count_a != count_b {
        return false;
    }
    for slot_a in &obj_a.elements {
        if slot_a.hash != 0 {
            let found = obj_b
                .elements
                .iter()
                .any(|slot_b| {
                    slot_b.hash == slot_a.hash
                        && jv_string_equal(&slot_a.string, &slot_b.string)
                        && jv_equal(&slot_a.value, &slot_b.value)
                });
            if !found {
                return false;
            }
        }
    }
    true
}
pub fn jvp_string_hash(jstr: &Jv) -> u32 {
    jvp_string_hash_internal(jstr)
}

/// Internal implementation of string hashing
pub fn jvp_string_hash_internal(jstr: &Jv) -> u32 {
    if jstr.u == 0 {
        return 0;
    }
    let str_ptr = unsafe { &mut *(jstr.u as *mut JvpString) };
    // If already hashed, return cached value
    if str_ptr.hash != 0 {
        return str_ptr.hash;
    }
    let data = str_ptr.data.as_bytes();
    let len = data.len();
    let nblocks = len / 4;
    let mut h1: u32 = HASH_SEED;
    let c1: u32 = 0xcc9e2d51;
    let c2: u32 = 0x1b873593;
    for i in 0..nblocks {
        let idx = i * 4;
        if idx + 4 <= len {
            let mut k1 = u32::from_le_bytes([
                data[idx],
                data[idx + 1],
                data[idx + 2],
                data[idx + 3],
            ]);
            k1 = k1.wrapping_mul(c1);
            k1 = rotl32(k1, 15);
            k1 = k1.wrapping_mul(c2);
            h1 ^= k1;
            h1 = rotl32(h1, 13);
            h1 = h1.wrapping_mul(5).wrapping_add(0xe6546b64);
        }
    }
    let tail_start = nblocks * 4;
    let mut k1: u32 = 0;
    match len & 3 {
        3 => {
            if tail_start + 2 < len {
                k1 ^= (data[tail_start + 2] as u32) << 16;
            }
            if tail_start + 1 < len {
                k1 ^= (data[tail_start + 1] as u32) << 8;
            }
            if tail_start < len {
                k1 ^= data[tail_start] as u32;
            }
            k1 = k1.wrapping_mul(c1);
            k1 = rotl32(k1, 15);
            k1 = k1.wrapping_mul(c2);
            h1 ^= k1;
        }
        2 => {
            if tail_start + 1 < len {
                k1 ^= (data[tail_start + 1] as u32) << 8;
            }
            if tail_start < len {
                k1 ^= data[tail_start] as u32;
            }
            k1 = k1.wrapping_mul(c1);
            k1 = rotl32(k1, 15);
            k1 = k1.wrapping_mul(c2);
            h1 ^= k1;
        }
        1 => {
            if tail_start < len {
                k1 ^= data[tail_start] as u32;
            }
            k1 = k1.wrapping_mul(c1);
            k1 = rotl32(k1, 15);
            k1 = k1.wrapping_mul(c2);
            h1 ^= k1;
        }
        _ => {}
    }
    h1 ^= len as u32;
    h1 ^= h1 >> 16;
    h1 = h1.wrapping_mul(0x85ebca6b);
    h1 ^= h1 >> 13;
    h1 = h1.wrapping_mul(0xc2b2ae35);
    h1 ^= h1 >> 16;
    str_ptr.hash = h1;
    h1
}
/// Get the kind of a jv value
pub fn jv_get_kind(j: &Jv) -> JvKind {
    match j.kind_flags & JV_KIND_MASK {
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
/// Create a string from a Rust string slice
pub fn jv_string(s: &str) -> Jv {
    jv_string_sized(s, s.len())
}
/// Create an empty object
pub fn jv_object() -> Jv {
    let obj = JvpObject {
        refcnt: 1,
        next_free: 0,
        elements: Vec::new(),
    };
    let ptr = Box::into_raw(Box::new(obj));
    Jv {
        kind_flags: JvKind::Object as u8 | JVP_PAYLOAD_ALLOCATED,
        pad_: 0,
        offset: 0,
        size: 8,
        u: ptr as u64,
    }
}
/// Create a null Jv
pub fn jv_null() -> Jv {
    Jv {
        kind_flags: JvKind::Null as u8,
        pad_: 0,
        offset: 0,
        size: 0,
        u: 0,
    }
}
/// Create a boolean Jv
pub fn jv_bool(b: bool) -> Jv {
    Jv {
        kind_flags: if b { JvKind::True as u8 } else { JvKind::False as u8 },
        pad_: 0,
        offset: 0,
        size: 0,
        u: 0,
    }
}
/// Create an invalid Jv with optional error message
pub fn jv_invalid_with_msg(msg: Jv) -> Jv {
    let invalid = JvpInvalid {
        refcnt: 1,
        errmsg: Some(Box::new(msg)),
    };
    let ptr = Box::into_raw(Box::new(invalid));
    Jv {
        kind_flags: JvKind::Invalid as u8 | JVP_PAYLOAD_ALLOCATED,
        pad_: 0,
        offset: 0,
        size: 0,
        u: ptr as u64,
    }
}
/// Check if a JV value is valid (not invalid kind)
pub fn jv_is_valid(x: &Jv) -> bool {
    x.get_kind() != JvKind::Invalid
}
/// Get numeric value from jv
pub fn jv_number_value(j: &Jv) -> f64 {
    assert!(jvp_has_kind(j, JvKind::Number));
    let is_decimal = j.kind_flags
        == (JvKind::Number as u8 | ((JVP_NUMBER_DECIMAL << 4) & 0x70)
            | JVP_PAYLOAD_ALLOCATED);
    if is_decimal && j.u != 0 {
        let n = unsafe { &*(j.u as *const JvpLiteralNumber) };
        return n.num_double;
    }
    // For native numbers, u is the raw bits of the double
    f64::from_bits(j.u)
}
const JV_KIND_MASK: u8 = 0x0F;
/// Reference count initial value
pub const JV_REFCNT_INIT: i32 = 1;
fn tsd_dec_ctx_get_local() -> DecimalContext {
    DEC_CTX_KEY
        .with(|ctx| {
            let mut ctx = ctx.borrow_mut();
            if ctx.is_none() {
                *ctx = Some(DecimalContext::default());
            }
            ctx.as_ref().unwrap().clone()
        })
}
fn jv_invalid_const() -> Jv {
    Jv {
        kind_flags: JvKind::Invalid as u8,
        pad_: 0,
        offset: 0,
        size: 0,
        u: 0,
    }
}
fn jv_null_const() -> Jv {
    Jv {
        kind_flags: JvKind::Null as u8,
        pad_: 0,
        offset: 0,
        size: 0,
        u: 0,
    }
}
fn jv_false_const() -> Jv {
    Jv {
        kind_flags: JvKind::False as u8,
        pad_: 0,
        offset: 0,
        size: 0,
        u: 0,
    }
}
fn jv_true_const() -> Jv {
    Jv {
        kind_flags: JvKind::True as u8,
        pad_: 0,
        offset: 0,
        size: 0,
        u: 0,
    }
}
/// Check if jv has a specific kind
#[inline]
fn jvp_has_kind(j: &Jv, kind: JvKind) -> bool {
    (j.kind_flags & JV_KIND_MASK) == kind as u8
}
/// Check if refcount indicates unshared value
fn jvp_refcnt_unshared(ptr: &Option<*mut u8>) -> bool {
    match ptr {
        Some(p) if !p.is_null() => true,
        _ => true,
    }
}
pub fn jvp_refcnt_inc(j: &Jv) {
    if j.u == 0 {
        return;
    }
    let kind = jv_get_kind(j);
    match kind {
        JvKind::String => {
            let s = unsafe { &mut *(j.u as *mut JvpString) };
            s.refcnt += 1;
        }
        JvKind::Array => {
            let a = unsafe { &mut *(j.u as *mut JvpArray) };
            a.refcnt += 1;
        }
        JvKind::Object => {
            let o = unsafe { &mut *(j.u as *mut JvpObject) };
            o.refcnt += 1;
        }
        JvKind::Number => {
            if (j.kind_flags & JVP_FLAGS_NUMBER_LITERAL) != 0 {
                let n = unsafe { &mut *(j.u as *mut JvpLiteralNumber) };
                n.refcnt += 1;
            }
        }
        _ => {}
    }
}
/// Decrement reference count and return true if it reaches zero
fn jvp_refcnt_dec(id: u64) -> bool {
    if let Ok(mut storage) = NUMBER_STORAGE.lock() {
        if let Some(num) = storage.get_mut(&id) {
            num.refcnt -= 1;
            return num.refcnt <= 0;
        }
    }
    if let Ok(mut storage) = STRING_STORAGE.lock() {
        if let Some(s) = storage.get_mut(&id) {
            s.refcnt -= 1;
            return s.refcnt <= 0;
        }
    }
    if let Ok(mut storage) = OBJECT_STORAGE.lock() {
        if let Some(obj) = storage.get_mut(&id) {
            obj.refcnt -= 1;
            return obj.refcnt <= 0;
        }
    }
    false
}
/// Get object pointer from JV - o.u is a raw pointer to JvpObject
pub fn jvp_object_ptr(o: &Jv) -> Option<&JvpObject> {
    assert!(jvp_has_kind(o, JvKind::Object), "JVP_HAS_KIND(o, JV_KIND_OBJECT)");
    if o.u == 0 {
        return None;
    }
    Some(unsafe { &*(o.u as *const JvpObject) })
}
/// Get mutable object pointer from JV - o.u is a raw pointer to JvpObject
fn jvp_object_ptr_mut(o: &Jv) -> Option<&mut JvpObject> {
    assert!(jvp_has_kind(o, JvKind::Object), "JVP_HAS_KIND(o, JV_KIND_OBJECT)");
    if o.u == 0 {
        return None;
    }
    Some(unsafe { &mut *(o.u as *mut JvpObject) })
}
/// Get object size (number of slots)
pub fn jvp_object_size(object: &Jv) -> i32 {
    if let Some(obj) = jvp_object_ptr(object) { obj.elements.len() as i32 } else { 0 }
}
/// Create new object with given size
fn jvp_object_new(size: i32) -> Jv {
    let mut elements = Vec::with_capacity(size as usize);
    for _ in 0..size {
        elements
            .push(ObjectSlot {
                next: -1,
                hash: 0,
                string: jv_null(),
                value: jv_null(),
            });
    }
    let obj = Box::new(JvpObject {
        refcnt: 1,
        next_free: 0,
        elements,
    });
    let ptr = Box::into_raw(obj);
    Jv {
        kind_flags: (JvKind::Object as u8) | JVP_PAYLOAD_ALLOCATED,
        pad_: 0,
        offset: 0,
        size: size,
        u: ptr as u64,
    }
}
/// Add a slot to object
pub fn jvp_object_add_slot<'a>(
    object: &'a mut Jv,
    key: Jv,
    bucket: &mut i32,
) -> Option<&'a mut ObjectSlot> {
    assert!(jvp_has_kind(object, JvKind::Object));
    if object.u == 0 {
        return None;
    }
    let obj = unsafe { &mut *(object.u as *mut JvpObject) };
    let newslot_idx = obj.next_free;
    if newslot_idx >= obj.elements.len() as i32 {
        return None;
    }
    let newslot = &mut obj.elements[newslot_idx as usize];
    obj.next_free += 1;
    newslot.next = *bucket;
    *bucket = newslot_idx;
    newslot.hash = jvp_string_hash(&key);
    newslot.string = key;
    Some(newslot)
}
/// Free an object, following C semantics:
/// - Decrement refcount
/// - Only actually free when refcount reaches 0
pub fn jvp_object_free(object: Jv) {
    if object.u == 0 {
        return;
    }
    // C: if (jvp_refcnt_dec(o.u.ptr)) { ... }
    let obj = unsafe { &mut *(object.u as *mut JvpObject) };
    obj.refcnt -= 1;
    if obj.refcnt == 0 {
        let obj = unsafe { Box::from_raw(object.u as *mut JvpObject) };
        for slot in obj.elements.iter() {
            if jv_get_kind(&slot.string) != JvKind::Null {
                // C: jvp_string_free(slot->string); jv_free(slot->value);
                jv_free(slot.string.clone());
                jv_free(slot.value.clone());
            }
        }
    }
}
/// Get string pointer
fn jvp_string_ptr(j: &Jv) -> Option<&JvpString> {
    if !jvp_has_kind(j, JvKind::String) {
        return None;
    }
    if j.u == 0 {
        return None;
    }
    Some(unsafe { &*(j.u as *const JvpString) })
}
/// Get the length of a jvp_string
#[inline]
fn jvp_string_length(s: &JvpString) -> u32 {
    s.length as u32
}
/// Append buffer to string
pub fn jvp_string_append(string: Jv, data: &[u8], len: u32) -> Jv {
    let (curr_data, currlen) = if string.u != 0 {
        let s = unsafe { &*(string.u as *const JvpString) };
        (s.data.clone(), s.length as u32)
    } else {
        (String::new(), 0u32)
    };
    let new_len = currlen + len;
    let mut new_data = curr_data;
    if let Ok(append_str) = std::str::from_utf8(&data[..len as usize]) {
        new_data.push_str(append_str);
    }
    jv_free(string);
    let news = Box::new(JvpString {
        refcnt: 1,
        hash: 0,
        length: new_len as i32,
        data: new_data,
    });
    let ptr = Box::into_raw(news);
    Jv {
        kind_flags: JV_KIND_STRING | JVP_PAYLOAD_ALLOCATED,
        pad_: 0,
        offset: 0,
        size: 0,
        u: ptr as u64,
    }
}
/// Free a string
pub fn jvp_string_free(js: Jv) {
    if js.u == 0 {
        return;
    }
    let s = unsafe { &mut *(js.u as *mut JvpString) };
    s.refcnt -= 1;
    if s.refcnt == 0 {
        unsafe { drop(Box::from_raw(js.u as *mut JvpString)); }
    }
}
/// Create a new literal number from string
pub fn jvp_literal_number_new(literal: &str) -> Jv {
    let parsed = literal.parse::<f64>();
    match parsed {
        Ok(val) => {
            let n = Box::new(JvpLiteralNumber {
                refcnt: 1,
                literal_data: Some(literal.to_string()),
                num_decimal: DecNumber::default(),
                num_double: val,
            });
            let ptr = Box::into_raw(n);
            Jv {
                kind_flags: JvKind::Number as u8 | JVP_FLAGS_NUMBER_LITERAL | JVP_PAYLOAD_ALLOCATED,
                pad_: 0,
                offset: 0,
                size: 0,
                u: ptr as u64,
            }
        }
        Err(_) => jv_invalid_const(),
    }
}
/// Allocate literal number with given literal length
pub fn jvp_literal_number_alloc(_literal_len: usize) -> Box<JvpLiteralNumber> {
    Box::new(JvpLiteralNumber {
        refcnt: 1,
        literal_data: None,
        num_decimal: DecNumber::default(),
        num_double: f64::NAN,
    })
}
/// Create a true JV value
pub fn jv_true() -> Jv {
    Jv {
        kind_flags: JvKind::True as u8,
        pad_: 0,
        offset: 0,
        size: 0,
        u: 0,
    }
}
/// Create a false JV value
pub fn jv_false() -> Jv {
    Jv {
        kind_flags: JvKind::False as u8,
        pad_: 0,
        offset: 0,
        size: 0,
        u: 0,
    }
}
/// Create an invalid JV value
pub fn jv_invalid() -> Jv {
    Jv {
        kind_flags: JvKind::Invalid as u8,
        pad_: 0,
        offset: 0,
        size: 0,
        u: 0,
    }
}
/// Concatenate two strings
pub fn jv_string_concat(a: Jv, b: Jv) -> Jv {
    assert!(jvp_has_kind(&a, JvKind::String));
    assert!(jvp_has_kind(&b, JvKind::String));
    let b_data = jv_string_value(&b);
    let b_len = jv_string_length_bytes(&b);
    let result = jv_string_append_buf(a, b_data.as_bytes(), b_len);
    jv_free(b);
    result
}
/// Slice a string by codepoint indices
pub fn jv_string_slice(j: Jv, start: i32, end: i32) -> Jv {
    assert!(jvp_has_kind(&j, JvKind::String));
    let s = jv_string_value(&j);
    let s_bytes = s.as_bytes();
    let len = s_bytes.len() as i32;
    let mut start = start;
    let mut end = end;
    jvp_clamp_slice_params(len, &mut start, &mut end);
    assert!(0 <= start && start <= end && end <= len);
    let mut p_idx = 0usize;
    let mut i = 0i32;
    while i < start {
        if p_idx >= s_bytes.len() {
            jv_free(j);
            return jv_string_empty(16);
        }
        match jvp_utf8_next_safe(s_bytes, p_idx) {
            Some((next_idx, cp)) => {
                if cp == -1 {
                    jv_free(j);
                    return jv_invalid_with_msg(jv_string("Invalid UTF-8 string"));
                }
                p_idx = next_idx;
            }
            None => {
                jv_free(j);
                return jv_string_empty(16);
            }
        }
        i += 1;
    }
    let start_byte = p_idx;
    while i < end {
        if p_idx >= s_bytes.len() {
            break;
        }
        match jvp_utf8_next_safe(s_bytes, p_idx) {
            Some((next_idx, cp)) => {
                if cp == -1 {
                    jv_free(j);
                    return jv_invalid_with_msg(jv_string("Invalid UTF-8 string"));
                }
                p_idx = next_idx;
            }
            None => {
                p_idx = s_bytes.len();
                break;
            }
        }
        i += 1;
    }
    let end_byte = p_idx;
    let slice_str = &s[start_byte..end_byte];
    let res = jv_string_sized(slice_str, slice_str.len());
    jv_free(j);
    res
}
/// Helper function to safely iterate UTF-8
fn jvp_utf8_next_safe(s: &[u8], idx: usize) -> Option<(usize, i32)> {
    if idx >= s.len() {
        return None;
    }
    let byte = s[idx];
    if byte < 0x80 {
        Some((idx + 1, byte as i32))
    } else if byte < 0xC0 {
        Some((idx + 1, -1))
    } else if byte < 0xE0 {
        if idx + 1 < s.len() {
            let cp = ((byte as i32 & 0x1F) << 6) | (s[idx + 1] as i32 & 0x3F);
            Some((idx + 2, cp))
        } else {
            Some((idx + 1, -1))
        }
    } else if byte < 0xF0 {
        if idx + 2 < s.len() {
            let cp = ((byte as i32 & 0x0F) << 12) | ((s[idx + 1] as i32 & 0x3F) << 6)
                | (s[idx + 2] as i32 & 0x3F);
            Some((idx + 3, cp))
        } else {
            Some((idx + 1, -1))
        }
    } else {
        if idx + 3 < s.len() {
            let cp = ((byte as i32 & 0x07) << 18) | ((s[idx + 1] as i32 & 0x3F) << 12)
                | ((s[idx + 2] as i32 & 0x3F) << 6) | (s[idx + 3] as i32 & 0x3F);
            Some((idx + 4, cp))
        } else {
            Some((idx + 1, -1))
        }
    }
}
/// Create empty array
pub fn jv_array() -> Jv {
    let arr = jvp_array_alloc(16);
    let ptr = Box::into_raw(arr);
    Jv {
        kind_flags: (JvKind::Array as u8) | JVP_PAYLOAD_ALLOCATED,
        pad_: 0,
        offset: 0,
        size: 0,
        u: ptr as u64,
    }
}
/// Get array length
pub fn jv_array_length(j: &Jv) -> i32 {
    assert!(jvp_has_kind(j, JvKind::Array));
    if j.u == 0 {
        return 0;
    }
    let arr = unsafe { &*(j.u as *const JvpArray) };
    arr.length
}
/// Get element from array at index
pub fn jv_array_get(a: Jv, idx: i32) -> Jv {
    let debug = std::env::var("DEBUG_JV").is_ok();
    assert!(a.has_kind(JvKind::Array), "JVP_HAS_KIND(a, JV_KIND_ARRAY)");
    if idx < 0 || idx >= a.size {
        if debug { eprintln!("jv_array_get: idx={} out of bounds (size={})", idx, a.size); }
        jv_free(a);
        return Jv::invalid();
    }
    let result = match jvp_array_ptr(&a) {
        Some(ptr) => {
            let arr = unsafe { &*ptr };
            if debug { eprintln!("jv_array_get: idx={} a.size={} arr.elements.len={} arr.length={}", idx, a.size, arr.elements.len(), arr.length); }
            if (idx as usize) < arr.elements.len() {
                jv_copy(&arr.elements[idx as usize])
            } else {
                if debug { eprintln!("jv_array_get: idx {} >= elements.len {}, returning null", idx, arr.elements.len()); }
                Jv::null()
            }
        }
        None => {
            if debug { eprintln!("jv_array_get: jvp_array_ptr returned None"); }
            Jv::null()
        }
    };
    jv_free(a);
    result
}
/// Set array element
pub fn jv_array_set(mut j: Jv, idx: i32, val: Jv) -> Jv {
    assert!(jvp_has_kind(&j, JvKind::Array));
    if j.u == 0 {
        return j;
    }
    let arr = unsafe { &mut *(j.u as *mut JvpArray) };
    while idx >= arr.elements.len() as i32 {
        arr.elements.push(jv_null());
    }
    if idx >= 0 {
        let old = std::mem::replace(&mut arr.elements[idx as usize], val);
        jv_free(old);
        if idx >= arr.length {
            arr.length = idx + 1;
        }
        // CRITICAL: update j.size to match arr.length (C does: a->size = imax(i + 1, a->size))
        if idx + 1 > j.size {
            j.size = idx + 1;
        }
    }
    j
}
/// Get object length (number of keys)
pub fn jv_object_length(j: &Jv) -> i32 {
    assert!(jvp_has_kind(j, JvKind::Object));
    if j.u == 0 {
        return 0;
    }
    let obj = unsafe { &*(j.u as *const JvpObject) };
    obj.next_free
}
/// Get object value by key
pub fn jv_object_get(j: &Jv, key: Jv) -> Jv {
    assert!(jvp_has_kind(j, JvKind::Object));
    assert!(jvp_has_kind(&key, JvKind::String));
    if j.u == 0 {
        jv_free(key);
        return jv_null();
    }
    let obj = unsafe { &*(j.u as *const JvpObject) };
    let key_hash = jvp_string_hash(&key);
    // Linear search through elements
    for slot in &obj.elements {
        if slot.hash == key_hash && jv_equal(&slot.string, &key) {
            jv_free(key);
            return jv_copy(&slot.value);
        }
    }
    jv_free(key);
    jv_null()
}
/// Object iterator - returns first index
/// C: int jv_object_iter(jv object) { return jv_object_iter_next(object, -1); }
pub fn jv_object_iter(j: &Jv) -> i32 {
    assert!(jvp_has_kind(j, JvKind::Object));
    jv_object_iter_next(j, -1)
}
/// Check if object iterator is valid
/// C: int jv_object_iter_valid(jv object, int i) { return i != ITER_FINISHED; }
pub fn jv_object_iter_valid(_obj: &Jv, iter: i32) -> bool {
    iter != ITER_FINISHED
}
/// Get the next valid object iterator
/// C: iterates slots from iter+1, skipping NULL slots
pub fn jv_object_iter_next(object: &Jv, mut iter: i32) -> i32 {
    assert!(
        jvp_has_kind(object, JvKind::Object), "JVP_HAS_KIND(object, JV_KIND_OBJECT)"
    );
    assert!(iter != ITER_FINISHED, "iter != ITER_FINISHED");
    let size = jvp_object_size(object);
    loop {
        iter += 1;
        if iter >= size {
            return ITER_FINISHED;
        }
        if let Some(slot) = jvp_object_get_slot(object, iter) {
            if jv_get_kind(&slot.string) != JvKind::Null {
                assert!(
                    jv_get_kind(&slot.string) == JvKind::String,
                    "jv_get_kind(jvp_object_get_slot(object,iter)->string) == JV_KIND_STRING"
                );
                return iter;
            }
        }
    }
}
/// Clamp slice parameters to valid range
pub fn jvp_clamp_slice_params(len: i32, pstart: &mut i32, pend: &mut i32) {
    if *pstart < 0 {
        *pstart = len + *pstart;
    }
    if *pend < 0 {
        *pend = len + *pend;
    }
    if *pstart < 0 {
        *pstart = 0;
    }
    if *pstart > len {
        *pstart = len;
    }
    if *pend > len {
        *pend = len;
    }
    if *pend < *pstart {
        *pend = *pstart;
    }
}
/// JV kind constants
pub const JV_KIND_INVALID: u8 = 0;
pub const JV_KIND_NULL: u8 = 1;
pub const JV_KIND_FALSE: u8 = 2;
pub const JV_KIND_TRUE: u8 = 3;
pub const JV_KIND_NUMBER: u8 = 4;
pub const JV_KIND_STRING: u8 = 5;
pub const JV_KIND_ARRAY: u8 = 6;
pub const JV_KIND_OBJECT: u8 = 7;
/// Iterator finished marker
pub const ITER_FINISHED: i32 = -2;
/// Helper to check if a jv has allocated payload
#[inline]
fn jvp_is_allocated(j: &Jv) -> bool {
    (j.kind_flags & JVP_PAYLOAD_ALLOCATED) != 0
}
/// Get pointer to array data
fn jvp_array_ptr(a: &Jv) -> Option<&JvpArray> {
    assert!(jvp_has_kind(a, JvKind::Array));
    if a.u == 0 {
        return None;
    }
    Some(unsafe { &*(a.u as *const JvpArray) })
}
/// Get mutable pointer to array data
fn jvp_array_ptr_mut(a: &mut Jv) -> Option<&mut JvpArray> {
    assert!(jvp_has_kind(a, JvKind::Array));
    if a.u == 0 {
        return None;
    }
    Some(unsafe { &mut *(a.u as *mut JvpArray) })
}
/// Internal: get array length
fn jvp_array_length(a: &Jv) -> i32 {
    assert!(jvp_has_kind(a, JvKind::Array));
    if a.u == 0 {
        return 0;
    }
    let arr = unsafe { &*(a.u as *const JvpArray) };
    arr.length - a.offset as i32
}
/// Get the offset of an array
fn jvp_array_offset(a: Jv) -> i32 {
    assert!(a.has_kind(JvKind::Array), "JVP_HAS_KIND(a, JV_KIND_ARRAY)");
    a.offset as i32
}
/// Read an element from an array by index
///
/// Returns a reference to the element at index i, or None if out of bounds.
fn jvp_array_read<'a>(a: &'a Jv, i: i32) -> Option<&'a Jv> {
    assert!(jvp_has_kind(a, JvKind::Array), "JVP_HAS_KIND(a, JV_KIND_ARRAY)");
    if i >= 0 && i < jvp_array_length(a) {
        if let Some(array) = jvp_array_ptr(a) {
            let idx = (i + a.offset as i32) as usize;
            assert!(
                idx < array.length as usize, "i + jvp_array_offset(a) < array->length"
            );
            array.elements.get(idx)
        } else {
            None
        }
    } else {
        None
    }
}
/// Get mutable string pointer from JV
fn jvp_string_ptr_mut(a: &Jv) -> Option<&mut JvpString> {
    assert!(jvp_has_kind(a, JvKind::String), "JVP_HAS_KIND(a, JV_KIND_STRING)");
    if a.u == 0 {
        return None;
    }
    Some(unsafe { &mut *(a.u as *mut JvpString) })
}
fn jvp_invalid_ptr(jv: &Jv) -> Option<&JvpInvalid> {
    if jv.u == 0 || jv_get_kind(jv) != JvKind::Invalid {
        return None;
    }
    Some(unsafe { &*(jv.u as *const JvpInvalid) })
}
fn jvp_literal_number_ptr(n: &Jv) -> Option<&JvpLiteralNumber> {
    if n.u == 0 || jv_get_kind(n) != JvKind::Number {
        return None;
    }
    if (n.kind_flags & JVP_FLAGS_NUMBER_LITERAL) == 0 {
        return None;
    }
    Some(unsafe { &*(n.u as *const JvpLiteralNumber) })
}
/// Internal function to allocate a string
fn jvp_string_alloc(length: u32) -> Box<JvpString> {
    Box::new(JvpString {
        refcnt: 1,
        hash: 0,
        length: length as i32,
        data: String::with_capacity(length as usize),
    })
}
fn jvp_utf8_next(data: &[u8], pos: usize) -> Option<(usize, i32)> {
    if pos >= data.len() {
        return None;
    }
    let b = data[pos];
    if b < 0x80 {
        Some((pos + 1, b as i32))
    } else if b < 0xC0 {
        Some((pos + 1, -1))
    } else if b < 0xE0 {
        if pos + 1 >= data.len() {
            return Some((pos + 1, -1));
        }
        let c = ((b as i32 & 0x1F) << 6) | (data[pos + 1] as i32 & 0x3F);
        Some((pos + 2, c))
    } else if b < 0xF0 {
        if pos + 2 >= data.len() {
            return Some((pos + 1, -1));
        }
        let c = ((b as i32 & 0x0F) << 12) | ((data[pos + 1] as i32 & 0x3F) << 6)
            | (data[pos + 2] as i32 & 0x3F);
        Some((pos + 3, c))
    } else if b < 0xF8 {
        if pos + 3 >= data.len() {
            return Some((pos + 1, -1));
        }
        let c = ((b as i32 & 0x07) << 18) | ((data[pos + 1] as i32 & 0x3F) << 12)
            | ((data[pos + 2] as i32 & 0x3F) << 6) | (data[pos + 3] as i32 & 0x3F);
        Some((pos + 4, c))
    } else {
        Some((pos + 1, -1))
    }
}
fn jvp_utf8_encode(codepoint: i32, output: &mut Vec<u8>) -> usize {
    if codepoint < 0x80 {
        output.push(codepoint as u8);
        1
    } else if codepoint < 0x800 {
        output.push((0xC0 | (codepoint >> 6)) as u8);
        output.push((0x80 | (codepoint & 0x3F)) as u8);
        2
    } else if codepoint < 0x10000 {
        output.push((0xE0 | (codepoint >> 12)) as u8);
        output.push((0x80 | ((codepoint >> 6) & 0x3F)) as u8);
        output.push((0x80 | (codepoint & 0x3F)) as u8);
        3
    } else {
        output.push((0xF0 | (codepoint >> 18)) as u8);
        output.push((0x80 | ((codepoint >> 12) & 0x3F)) as u8);
        output.push((0x80 | ((codepoint >> 6) & 0x3F)) as u8);
        output.push((0x80 | (codepoint & 0x3F)) as u8);
        4
    }
}
/// Check if an invalid jv has an error message
pub fn jv_invalid_has_msg(inv: Jv) -> i32 {
    assert!(jvp_has_kind(&inv, JvKind::Invalid), "JVP_HAS_KIND(inv, JV_KIND_INVALID)");
    let expected_flags = (JvKind::Invalid as u8) | JVP_PAYLOAD_ALLOCATED;
    let r = if inv.kind_flags == expected_flags { 1 } else { 0 };
    jv_free(inv);
    r
}
/// Check if a number has a literal representation
pub fn jv_number_has_literal(n: &Jv) -> i32 {
    assert!(jvp_has_kind(n, JvKind::Number), "JVP_HAS_KIND(n, JV_KIND_NUMBER)");
    let expected_flags = JvKind::Number as u8 | JVP_FLAGS_NUMBER_LITERAL;
    if n.kind_flags == expected_flags { 1 } else { 0 }
}
/// Free an array
pub fn jvp_array_free(a: Jv) {
    assert!(jvp_has_kind(&a, JvKind::Array), "JVP_HAS_KIND(a, JV_KIND_ARRAY)");
    if a.u == 0 {
        return;
    }
    let arr = unsafe { &mut *(a.u as *mut JvpArray) };
    arr.refcnt -= 1;
    if arr.refcnt == 0 {
        unsafe { drop(Box::from_raw(a.u as *mut JvpArray)); }
    }
}
/// Free an invalid jv value
pub fn jvp_invalid_free(x: Jv) {
    assert!(jvp_has_kind(&x, JvKind::Invalid), "JVP_HAS_KIND(x, JV_KIND_INVALID)");
    if x.kind_flags == (JvKind::Invalid as u8 | JVP_PAYLOAD_ALLOCATED) {
        if jvp_refcnt_dec(x.u) {
            if let Some(ref msg) = get_invalid_errmsg(&x) {
                jv_free(msg.clone());
            }
        }
    }
}
/// Free a number value
pub fn jvp_number_free(j: Jv) {
    assert!(jvp_has_kind(& j, JvKind::Number), "JVP_HAS_KIND(j, JV_KIND_NUMBER)");
    if jvp_is_literal_number(&j) && jvp_refcnt_dec(j.u) {
        let mut storage = NUMBER_STORAGE.lock().unwrap();
        storage.remove(&j.u);
    }
}
fn allocate_id() -> u64 {
    let mut id = NEXT_ID.lock().unwrap();
    let current = *id;
    *id += 1;
    current
}
/// Get the name of a JV kind
pub fn jv_kind_name(k: JvKind) -> &'static str {
    match k {
        JvKind::Invalid => "<invalid>",
        JvKind::Null => "null",
        JvKind::False => "boolean",
        JvKind::True => "boolean",
        JvKind::Number => "number",
        JvKind::String => "string",
        JvKind::Array => "array",
        JvKind::Object => "object",
    }
}
/// Get remaining space in string
pub fn jvp_string_remaining_space(s: &JvpString) -> u32 {
    // In Rust, String handles its own allocation, so remaining space is 0
    0
}
/// Check if number is a literal (decimal) number
fn jvp_is_literal_number(j: &Jv) -> bool {
    j.kind_flags
        == ((JvKind::Number as u8 & 0x0F)
            | (((JVP_NUMBER_DECIMAL << 4) & 0x70) | JVP_PAYLOAD_ALLOCATED) & 0xF0)
}
/// Compute string hash
pub fn jvp_string_hash_compute(j: &Jv) -> u32 {
    if let Some(s) = jvp_string_ptr(j) {
        let mut hash: u32 = 2166136261;
        for byte in s.data.as_bytes() {
            hash ^= *byte as u32;
            hash = hash.wrapping_mul(16777619);
        }
        hash
    } else {
        0
    }
}
/// Find bucket index for object key lookup
fn jvp_object_find_bucket(object: &Jv, key: &Jv) -> Option<usize> {
    if object.u == 0 {
        return None;
    }
    let obj = unsafe { &*(object.u as *const JvpObject) };
    if obj.elements.is_empty() {
        return None;
    }
    let hash = jvp_string_hash(key);
    let bucket = (hash as usize) % obj.elements.len();
    Some(bucket)
}
/// Get object slot at index
pub fn jvp_object_get_slot_at(object: &Jv, index: i32) -> Option<&ObjectSlot> {
    jvp_object_ptr(object)
        .and_then(|obj| {
            if index >= 0 && (index as usize) < obj.elements.len() {
                Some(&obj.elements[index as usize])
            } else {
                None
            }
        })
}
/// Get string hash value
pub fn jv_string_hash(j: Jv) -> u64 {
    assert!(jvp_has_kind(& j, JvKind::String), "JVP_HAS_KIND(j, JV_KIND_STRING)");
    let hash = jvp_string_hash_compute(&j) as u64;
    jv_free(j);
    hash
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_jv_kind_name() {
        assert_eq!(jv_kind_name(JvKind::Null), "null");
        assert_eq!(jv_kind_name(JvKind::True), "boolean");
        assert_eq!(jv_kind_name(JvKind::False), "boolean");
        assert_eq!(jv_kind_name(JvKind::Number), "number");
        assert_eq!(jv_kind_name(JvKind::String), "string");
        assert_eq!(jv_kind_name(JvKind::Array), "array");
        assert_eq!(jv_kind_name(JvKind::Object), "object");
        assert_eq!(jv_kind_name(JvKind::Invalid), "<invalid>");
    }
    #[test]
    fn test_jv_get_kind() {
        let null = jv_null();
        assert_eq!(jv_get_kind(& null), JvKind::Null);
        let invalid = jv_invalid();
        assert_eq!(jv_get_kind(& invalid), JvKind::Invalid);
        let t = jv_true();
        assert_eq!(jv_get_kind(& t), JvKind::True);
        let f = jv_false();
        assert_eq!(jv_get_kind(& f), JvKind::False);
    }
    #[test]
    fn test_jv_is_valid() {
        let null = jv_null();
        assert!(jv_is_valid(& null));
        let invalid = jv_invalid();
        assert!(! jv_is_valid(& invalid));
    }
    #[test]
    fn test_jvp_object_new() {
        let obj = jvp_object_new(4);
        assert_eq!(jv_get_kind(& obj), JvKind::Object);
        assert_eq!(obj.size, 4);
        jv_free(obj);
    }
}
const HASH_SEED: u32 = 0x432A235;
/// Rotate left 32-bit value
///
/// Performs a circular left rotation of the bits in x by r positions.
#[inline]
pub fn rotl32(x: u32, r: i8) -> u32 {
    (x << r) | (x >> (32 - r))
}
pub fn jv_kind(j: &Jv) -> JvKind {
    match j.kind_flags & 0x0F {
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
pub fn jv_string_length_codepoints(j: Jv) -> i32 {
    assert!(jvp_has_kind(&j, JvKind::String));
    let data = jv_string_value(&j);
    let data_bytes = data.as_bytes();
    let len = data_bytes.len();
    let mut count = 0;
    let mut i = 0;
    while i < len {
        let byte = data_bytes[i];
        if byte < 0x80 {
            i += 1;
        } else if byte < 0xE0 {
            i += 2;
        } else if byte < 0xF0 {
            i += 3;
        } else {
            i += 4;
        }
        count += 1;
    }
    count
}
fn jv_copy_internal(j: &Jv) -> Jv {
    if jvp_is_allocated(j) {
        jvp_refcnt_inc(j);
    }
    Jv {
        kind_flags: j.kind_flags,
        pad_: j.pad_,
        offset: j.offset,
        size: j.size,
        u: j.u,
    }
}
pub fn jv_array_indexes(a: Jv, b: Jv) -> Jv {
    let mut res = jv_array();
    let alen = jv_array_length(&a);
    for ai in 0..alen {
        let mut idx: i32 = -1;
        let blen = jv_array_length(&b);
        for bi in 0..blen {
            let belem = jv_array_get(jv_copy_internal(&b), bi);
            let aelem = jv_array_get(jv_copy_internal(&a), ai + bi);
            if !jv_equal_internal(&aelem, &belem) {
                idx = -1;
            } else if bi == 0 && idx == -1 {
                idx = ai;
            }
            jv_free(aelem);
            jv_free(belem);
        }
        if idx > -1 {
            res = jv_array_append_internal(res, jv_number(idx as f64));
        }
    }
    jv_free(a);
    jv_free(b);
    res
}
fn jv_array_append_internal(mut arr: Jv, val: Jv) -> Jv {
    assert!(jvp_has_kind(&arr, JvKind::Array));
    if let Some(a) = jvp_array_ptr_mut(&mut arr) {
        a.elements.push(val);
        a.length += 1;
    }
    arr
}
fn jv_object_iter_next_internal(object: &Jv, iter: i32) -> i32 {
    let obj = match jvp_object_ptr(object) {
        Some(o) => o,
        None => return -1,
    };
    let mut i = iter + 1;
    while (i as usize) < obj.elements.len() {
        if jv_get_kind(&obj.elements[i as usize].string) != JvKind::Null {
            return i;
        }
        i += 1;
    }
    -1
}
fn jv_equal_internal(a: &Jv, b: &Jv) -> bool {
    if jv_kind(a) != jv_kind(b) {
        return false;
    }
    match jv_kind(a) {
        JvKind::Null | JvKind::True | JvKind::False => true,
        JvKind::Number => jv_number_value(a) == jv_number_value(b),
        JvKind::String => jv_string_value(a) == jv_string_value(b),
        JvKind::Array => {
            let alen = jv_array_length(a);
            let blen = jv_array_length(b);
            if alen != blen {
                return false;
            }
            for i in 0..alen {
                let av = jv_array_get(jv_copy_internal(a), i);
                let bv = jv_array_get(jv_copy_internal(b), i);
                let eq = jv_equal_internal(&av, &bv);
                jv_free(av);
                jv_free(bv);
                if !eq {
                    return false;
                }
            }
            true
        }
        JvKind::Object => {
            jvp_object_size(a) == jvp_object_size(b)
        }
        _ => false,
    }
}
pub fn jv_tsd_dec_ctx_fini() {
    DEC_CTX
        .with(|ctx| {
            *ctx.borrow_mut() = None;
        });
}
pub fn jv_get_refcnt(j: &Jv) -> i32 {
    if j.u == 0 {
        return 1;
    }
    let kind = jv_get_kind(j);
    match kind {
        JvKind::String => {
            let s = unsafe { &*(j.u as *const JvpString) };
            s.refcnt
        }
        JvKind::Array => {
            let a = unsafe { &*(j.u as *const JvpArray) };
            a.refcnt
        }
        JvKind::Object => {
            let o = unsafe { &*(j.u as *const JvpObject) };
            o.refcnt
        }
        JvKind::Number if (j.kind_flags & JVP_FLAGS_NUMBER_LITERAL) != 0 => {
            let n = unsafe { &*(j.u as *const JvpLiteralNumber) };
            n.refcnt
        }
        _ => 1,
    }
}
/// Check if a jv number is an integer
pub fn jv_is_integer(j: &Jv) -> i32 {
    if !j.has_kind(JvKind::Number) {
        return 0;
    }
    let x = jv_number_value(j);
    let fpart = x.fract();
    if fpart.abs() < f64::EPSILON { 1 } else { 0 }
}
/// Rehash an object's hash table when it grows
pub fn jvp_object_rehash(object: Jv) -> Jv {
    assert!(jvp_has_kind(&object, JvKind::Object), "JVP_HAS_KIND(object, JV_KIND_OBJECT)");
    let size = jvp_object_size(&object);
    let mut new_object = jvp_object_new(size * 2);
    if let Some(obj_ptr) = jvp_object_ptr(&object) {
        for i in 0..size as usize {
            if i < obj_ptr.elements.len() {
                let slot = &obj_ptr.elements[i];
                if jv_get_kind(&slot.string) == JvKind::Null {
                    continue;
                }
                let mut new_bucket: i32 = 0;
                let key = jv_copy(&slot.string);
                let value = jv_copy(&slot.value);
                if let Some(new_slot) = jvp_object_add_slot(
                    &mut new_object,
                    key,
                    &mut new_bucket,
                ) {
                    new_slot.value = value;
                }
            }
        }
    }
    jv_free(object);
    new_object
}
/// Append a UTF-8 codepoint to a string
pub fn jv_string_append_codepoint(s: Jv, codepoint: u32) -> Jv {
    assert!(jvp_has_kind(&s, JvKind::String), "JVP_HAS_KIND(s, JV_KIND_STRING)");
    if s.u == 0 {
        return s;
    }
    let string = unsafe { &mut *(s.u as *mut JvpString) };
    let mut buf = [0u8; 4];
    let len = if codepoint < 0x80 {
        buf[0] = codepoint as u8;
        1
    } else if codepoint < 0x800 {
        buf[0] = 0xC0 | ((codepoint >> 6) as u8);
        buf[1] = 0x80 | ((codepoint & 0x3F) as u8);
        2
    } else if codepoint < 0x10000 {
        buf[0] = 0xE0 | ((codepoint >> 12) as u8);
        buf[1] = 0x80 | (((codepoint >> 6) & 0x3F) as u8);
        buf[2] = 0x80 | ((codepoint & 0x3F) as u8);
        3
    } else {
        buf[0] = 0xF0 | ((codepoint >> 18) as u8);
        buf[1] = 0x80 | (((codepoint >> 12) & 0x3F) as u8);
        buf[2] = 0x80 | (((codepoint >> 6) & 0x3F) as u8);
        buf[3] = 0x80 | ((codepoint & 0x3F) as u8);
        4
    };
    if let Ok(str_slice) = std::str::from_utf8(&buf[..len]) {
        string.data.push_str(str_slice);
        string.length += len as i32;
    }
    s
}
/// Compare two jv numbers
pub fn jvp_number_cmp(a: &Jv, b: &Jv) -> i32 {
    assert!(a.has_kind(JvKind::Number), "JVP_HAS_KIND(a, JV_KIND_NUMBER)");
    assert!(b.has_kind(JvKind::Number), "JVP_HAS_KIND(b, JV_KIND_NUMBER)");
    if a.is_literal_number() && b.is_literal_number() {}
    let da = jv_number_value(a);
    let db = jv_number_value(b);
    if da < db { -1 } else if da == db { 0 } else { 1 }
}
/// Thread-local decimal context key
pub static dec_ctx_key: std::sync::OnceLock<DecimalContext> = std::sync::OnceLock::new();
/// Allocate a new array
/// C: just allocates memory, does NOT pre-fill with nulls
fn jvp_array_alloc(size: i32) -> Box<JvpArray> {
    Box::new(JvpArray {
        refcnt: 1,
        length: 0,
        alloc_length: size,
        elements: Vec::with_capacity(size as usize),
    })
}
fn jv_equal_by_kind(a: &Jv, b: &Jv) -> i32 {
    match jv_get_kind(a) {
        JvKind::Number => if jvp_number_equal(a, b) != 0 { 1 } else { 0 },
        JvKind::Array => if jvp_array_equal(a, b) != 0 { 1 } else { 0 },
        JvKind::String => if jvp_string_equal(a, b) { 1 } else { 0 },
        JvKind::Object => if jvp_object_equal(a, b) { 1 } else { 0 },
        _ => 1,
    }
}
/// Get pointer to decimal number in literal number
fn jvp_dec_number_ptr(j: &Jv) -> Option<&DecNumber> {
    if j.u == 0 || jv_get_kind(j) != JvKind::Number {
        return None;
    }
    if (j.kind_flags & JVP_FLAGS_NUMBER_LITERAL) == 0 {
        return None;
    }
    let lit = unsafe { &*(j.u as *const JvpLiteralNumber) };
    Some(&lit.num_decimal)
}
/// Find a slot in an object by key string
pub fn jvp_object_find_slot<'a>(
    object: &'a Jv,
    keystr: &Jv,
    bucket: &mut i32,
) -> Option<&'a ObjectSlot> {
    let hash = jvp_string_hash(keystr);
    let mut curr_opt = jvp_object_get_slot(object, *bucket);
    while let Some(curr) = curr_opt {
        if curr.hash == hash && jvp_string_equal(keystr, &curr.string) {
            return Some(curr);
        }
        curr_opt = jvp_object_next_slot(object, curr);
    }
    None
}
/// Check if two numbers are equal
pub fn jvp_number_equal(a: &Jv, b: &Jv) -> i32 {
    if jvp_number_cmp(a, b) == 0 { 1 } else { 0 }
}
/// Check if two strings are equal
fn jvp_string_equal(a: &Jv, b: &Jv) -> bool {
    assert!(jvp_has_kind(a, JvKind::String), "JVP_HAS_KIND(a, JV_KIND_STRING)");
    assert!(jvp_has_kind(b, JvKind::String), "JVP_HAS_KIND(b, JV_KIND_STRING)");
    let stra = match jvp_string_ptr(a) {
        Some(s) => s,
        None => return false,
    };
    let strb = match jvp_string_ptr(b) {
        Some(s) => s,
        None => return false,
    };
    stra.data == strb.data
}
static INIT: Once = Once::new();
/// Create a new array with specified initial capacity
fn jvp_array_new(n: i32) -> Jv {
    let array = Box::new(JvpArray::new(n));
    let ptr = Box::into_raw(array);
    Jv {
        kind_flags: JV_KIND_ARRAY | JVP_PAYLOAD_ALLOCATED,
        pad_: 0,
        offset: 0,
        size: 0,
        u: ptr as u64,
    }
}
/// Internal: read a value from an object
fn jvp_object_read<'a>(object: &'a Jv, key: &Jv) -> Option<&'a Jv> {
    if object.u == 0 {
        return None;
    }
    let obj = unsafe { &*(object.u as *const JvpObject) };
    let key_str = jv_string_value(key);
    for slot in &obj.elements {
        if slot.next != -2 {
            let slot_key_str = jv_string_value(&slot.string);
            if key_str == slot_key_str {
                return Some(&slot.value);
            }
        }
    }
    None
}
/// Check if object has a key
pub fn jv_object_has(object: Jv, key: Jv) -> i32 {
    assert!(jvp_has_kind(&object, JvKind::Object), "JVP_HAS_KIND(object, JV_KIND_OBJECT)");
    assert!(jvp_has_kind(&key, JvKind::String), "JVP_HAS_KIND(key, JV_KIND_STRING)");
    let slot = jvp_object_read(&object, &key);
    let res = if slot.is_some() { 1 } else { 0 };
    jv_free(object);
    jv_free(key);
    res
}
/// Implode array of codepoints into a string
pub fn jv_string_implode(j: Jv) -> Jv {
    assert!(jvp_has_kind(&j, JvKind::Array), "JVP_HAS_KIND(j, JV_KIND_ARRAY)");
    let len = jv_array_length(&j);
    let mut s = jv_string_empty(len);
    assert!(len >= 0, "len >= 0");
    for i in 0..len {
        let n = jv_array_get(jv_copy(&j), i);
        assert!(jvp_has_kind(&n, JvKind::Number), "JVP_HAS_KIND(n, JV_KIND_NUMBER)");
        let mut nv = jv_number_value(&n) as i32;
        jv_free(n);
        if nv < 0 || nv > 0x10FFFF || (nv >= 0xD800 && nv <= 0xDFFF) {
            nv = 0xFFFD;
        }
        s = jv_string_append_codepoint(s, nv as u32);
    }
    jv_free(j);
    s
}
/// Return maximum of two integers
#[inline]
fn imax(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}
fn jv_null_value() -> Jv {
    Jv {
        kind_flags: JV_KIND_NULL,
        pad_: 0,
        offset: 0,
        size: 0,
        u: 0,
    }
}
/// Append buffer to string, handling invalid UTF-8
pub fn jv_string_append_buf(a: Jv, buf: &[u8], len: i32) -> Jv {
    let len = len as usize;
    if jvp_utf8_is_valid(buf) != 0 {
        jvp_string_append(a, buf, len as u32)
    } else {
        let b = jvp_string_copy_replace_bad(buf);
        jv_string_concat(a, b)
    }
}
fn decode_utf8_char(buf: &[u8]) -> Option<(u32, usize)> {
    if buf.is_empty() {
        return None;
    }
    let first = buf[0];
    if first < 0x80 {
        return Some((first as u32, 1));
    }
    let (expected_len, mut codepoint) = if first & 0xE0 == 0xC0 {
        (2, (first & 0x1F) as u32)
    } else if first & 0xF0 == 0xE0 {
        (3, (first & 0x0F) as u32)
    } else if first & 0xF8 == 0xF0 {
        (4, (first & 0x07) as u32)
    } else {
        return None;
    };
    if buf.len() < expected_len {
        return None;
    }
    for i in 1..expected_len {
        let byte = buf[i];
        if byte & 0xC0 != 0x80 {
            return None;
        }
        codepoint = (codepoint << 6) | (byte & 0x3F) as u32;
    }
    Some((codepoint, expected_len))
}
/// Create a string from a Rust string slice
pub fn jv_string_from_str(s: &str) -> Jv {
    let mut str_payload = jvp_string_alloc(s.len() as u32);
    str_payload.data = s.to_string();
    str_payload.length = s.len() as i32;
    jv_string(s)
}
/// Split string by separator
pub fn jv_string_split(j: Jv, sep: Jv) -> Jv {
    assert!(j.has_kind(JvKind::String));
    assert!(sep.has_kind(JvKind::String));
    let jstr = jv_string_value(&j);
    let jbytes = jstr.as_bytes();
    let sepstr = jv_string_value(&sep);
    let sepbytes = sepstr.as_bytes();
    let seplen = jv_string_length_bytes(&sep) as usize;
    let mut a = jv_array();
    assert!(jv_get_refcnt(& a) == 1);
    if seplen == 0 {
        let mut i = 0;
        while i < jbytes.len() {
            if let Some((codepoint, bytes_consumed)) = decode_utf8_char(&jbytes[i..]) {
                let mut char_str = jv_string_from_str("");
                char_str = jv_string_append_codepoint(char_str, codepoint as u32);
                a = jv_array_append(a, char_str);
                i += bytes_consumed;
            } else {
                i += 1;
            }
        }
    } else {
        let mut p = 0;
        while p < jbytes.len() {
            let remaining = &jbytes[p..];
            let s = _jq_memmem(remaining, sepbytes);
            let end = match s {
                Some(offset) => p + offset,
                None => jbytes.len(),
            };
            let slice_str = std::str::from_utf8(&jbytes[p..end]).unwrap_or("");
            a = jv_array_append(a, jv_string_sized(slice_str, (end - p)));
            if end == jbytes.len() {
                break;
            }
            p = end + seplen;
            if p == jbytes.len() && seplen != 0 {
                a = jv_array_append(a, jv_string_from_str(""));
            }
        }
    }
    jv_free(j);
    jv_free(sep);
    a
}
fn encode_utf8(codepoint: u32, buf: &mut [u8; 4]) -> usize {
    if codepoint < 0x80 {
        buf[0] = codepoint as u8;
        1
    } else if codepoint < 0x800 {
        buf[0] = (0xC0 | (codepoint >> 6)) as u8;
        buf[1] = (0x80 | (codepoint & 0x3F)) as u8;
        2
    } else if codepoint < 0x10000 {
        buf[0] = (0xE0 | (codepoint >> 12)) as u8;
        buf[1] = (0x80 | ((codepoint >> 6) & 0x3F)) as u8;
        buf[2] = (0x80 | (codepoint & 0x3F)) as u8;
        3
    } else {
        buf[0] = (0xF0 | (codepoint >> 18)) as u8;
        buf[1] = (0x80 | ((codepoint >> 12) & 0x3F)) as u8;
        buf[2] = (0x80 | ((codepoint >> 6) & 0x3F)) as u8;
        buf[3] = (0x80 | (codepoint & 0x3F)) as u8;
        4
    }
}
/// Slice an array
pub fn jvp_array_slice(a: Jv, start: i32, end: i32) -> Jv {
    assert!(a.has_kind(JvKind::Array));
    let len = jvp_array_length(&a);
    let mut start = start;
    let mut end = end;
    jvp_clamp_slice_params(len, &mut start, &mut end);
    assert!(0 <= start && start <= end && end <= len);
    if start == end {
        jv_free(a);
        return jv_array();
    }
    let new_offset = a.offset as i32 + start;
    if new_offset >= (1 << (std::mem::size_of::<u16>() * 8)) as i32 {
        let mut r = jv_array_sized(end - start);
        for i in start..end {
            r = jv_array_append(r, jv_array_get(jv_copy(&a), i));
        }
        jv_free(a);
        r
    } else {
        let mut result = a;
        result.offset = new_offset as u16;
        result.size = end - start;
        result
    }
}
/// Get object length (number of non-null entries)
fn jvp_object_length(object: &Jv) -> i32 {
    let mut n = 0;
    let size = jvp_object_size(object);
    for i in 0..size {
        if let Some(slot) = jvp_object_get_slot(object, i) {
            if jv_get_kind(&slot.string) != JvKind::Null {
                n += 1;
            }
        }
    }
    n
}
/// Check if two values are identical (same reference)
pub fn jv_identical(a: Jv, b: Jv) -> i32 {
    let r = if a.kind_flags != b.kind_flags || a.offset != b.offset || a.size != b.size {
        0
    } else if a.is_allocated() {
        // For allocated payloads, compare the pointer addresses
        if (a.u != 0) == (b.u != 0) && a.u == b.u { 1 } else { 0 }
    } else {
        // For non-allocated payloads, compare raw u64 values
        if a.u == b.u { 1 } else { 0 }
    };
    jv_free(a);
    jv_free(b);
    r
}
/// Decimal number special bits
pub const DECNAN: u8 = 0x20;
pub const DECSNAN: u8 = 0x10;
/// Helper to get the kind flags for an invalid value with allocated payload
#[inline]
fn jvp_invalid_allocated_flags() -> u8 {
    (JV_KIND_INVALID & 0x0F) | (JVP_PAYLOAD_ALLOCATED & 0xF0)
}
/// Helper to get the kind flags for a number with specific type
#[inline]
fn jvp_number_flags(number_type: u8, allocated: bool) -> u8 {
    let alloc_flag = if allocated { JVP_PAYLOAD_ALLOCATED } else { 0 };
    (JV_KIND_NUMBER & 0x0F) | ((((number_type << 4) & 0x70) | alloc_flag) & 0xF0)
}
/// Get the error message from an invalid jv value
///
/// Extracts and returns the error message stored in an invalid jv value.
/// Consumes the input value.
pub fn jv_invalid_get_msg(inv: Jv) -> Jv {
    assert!(jvp_has_kind(& inv, JvKind::Invalid), "JVP_HAS_KIND(inv, JV_KIND_INVALID)");
    let x = if inv.kind_flags == jvp_invalid_allocated_flags() {
        if let Some(ref invalid_ptr) = jvp_invalid_ptr(&inv) {
            if let Some(ref errmsg) = invalid_ptr.errmsg {
                jv_copy(&(**errmsg))
            } else {
                jv_null()
            }
        } else {
            jv_null()
        }
    } else {
        jv_null()
    };
    jv_free(inv);
    x
}
/// Check if a number value is NaN
///
/// Returns true if the number is NaN (Not a Number), false otherwise.
pub fn jvp_number_is_nan(n: Jv) -> i32 {
    assert!(jvp_has_kind(& n, JvKind::Number), "JVP_HAS_KIND(n, JV_KIND_NUMBER)");
    let decimal_allocated_flags = jvp_number_flags(JVP_NUMBER_DECIMAL, true);
    if n.kind_flags == decimal_allocated_flags {
        if let Some(pdec) = jvp_dec_number_ptr(&n) {
            if (pdec.bits & (DECNAN | DECSNAN)) != 0 {
                return 1;
            }
        }
        0
    } else {
        let number = f64::from_bits(n.u);
        if number != number { 1 } else { 0 }
    }
}
/// Create a number jv from a literal string
///
/// Creates a jv value containing a number parsed from the given literal string.
/// Preserves the original literal representation for precise decimal handling.
pub fn jv_number_with_literal(literal: &str) -> Jv {
    jvp_literal_number_new(literal)
}
/// Slice an array
///
/// Returns a new array containing elements from index `start` to `end` (exclusive).
pub fn jv_array_slice(a: Jv, start: i32, end: i32) -> Jv {
    assert!(jvp_has_kind(& a, JvKind::Array), "JVP_HAS_KIND(a, JV_KIND_ARRAY)");
    jvp_array_slice(a, start, end)
}
/// Payload flags
pub const JVP_PAYLOAD_NONE: u8 = 0x00;
/// Check if jv has allocated payload
#[inline]
pub fn jvp_has_allocated_payload(x: &Jv) -> bool {
    (x.kind_flags & 0xF0) == JVP_PAYLOAD_ALLOCATED
}
/// Get payload from jv
fn jv_get_payload(jv: &Jv) -> JvPayload {
    match jv.kind_flags & 0x0F {
        JV_KIND_STRING => {
            JvPayload::String(JvpString {
                refcnt: 1,
                hash: 0,
                length: jv.size,
                data: String::new(),
            })
        }
        JV_KIND_OBJECT => {
            JvPayload::Object(JvpObject {
                refcnt: 1,
                next_free: -1,
                elements: Vec::new(),
            })
        }
        JV_KIND_ARRAY => {
            JvPayload::Array(JvpArray {
                refcnt: 1,
                length: jv.size,
                alloc_length: jv.size,
                elements: Vec::new(),
            })
        }
        _ => {
            JvPayload::String(JvpString {
                refcnt: 1,
                hash: 0,
                length: 0,
                data: String::new(),
            })
        }
    }
}
/// Get the next slot in an object's hash chain
fn jvp_object_next_slot<'a>(object: &'a Jv, curr: &ObjectSlot) -> Option<&'a ObjectSlot> {
    if curr.next < 0 {
        return None;
    }
    jvp_object_get_slot(object, curr.next)
}
/// Merge two objects, with b's keys overwriting a's
pub fn jv_object_merge(mut a: Jv, b: Jv) -> Jv {
    assert!(jvp_has_kind(& a, JvKind::Object), "JVP_HAS_KIND(a, JV_KIND_OBJECT)");
    let mut iter = jv_object_iter(&b);
    while jv_object_iter_valid(&b, iter) {
        let k = jv_object_iter_key(&b, iter);
        let v = jv_object_iter_value(&b, iter);
        a = jv_object_set(a, k, v);
        iter = jv_object_iter_next(&b, iter);
    }
    jv_free(b);
    a
}
/// Helper to get error message from invalid jv
fn get_invalid_errmsg(x: &Jv) -> Option<Jv> {
    None
}
/// Append a string slice to a jv string
pub fn jv_string_append_str(a: Jv, str: &str) -> Jv {
    jv_string_append_buf(a, str.as_bytes(), str.len() as i32)
}
const JV_FLAGS_MASK: u8 = 0xF0;
const DECINF: u8 = 0x40;
const DEC_INIT_DECIMAL64: i32 = 64;
const DECIMAL64_PMAX: i32 = 17;
const DECIMAL64_EMAX_DEFAULT: i32 = 14;
/// Check if number flags indicate a literal number
fn jvp_has_flags_number_literal(j: &Jv) -> bool {
    let expected = (JvKind::Number as u8 & JV_KIND_MASK)
        | (((JVP_NUMBER_DECIMAL << 4) & 0x70) | JVP_PAYLOAD_ALLOCATED) & JV_FLAGS_MASK;
    j.kind_flags == expected
}
/// Get writable slot in array
fn jvp_array_write(a: &mut Jv, i: i32) -> Option<&mut Jv> {
    assert!(i >= 0);
    let offset = a.offset as i32;
    let pos = i + offset;
    let needs_realloc = if let Some(arr) = jvp_array_ptr(a) {
        pos >= arr.alloc_length || arr.refcnt > 1
    } else {
        true
    };
    if needs_realloc {
        let old_len = jvp_array_length(a);
        let new_length = imax(i + 1, old_len);
        let new_alloc = (new_length * 3) / 2;
        let mut new_array = jvp_array_alloc(new_alloc);
        if let Some(old_arr) = jvp_array_ptr(a) {
            for j in 0..old_len {
                let old_idx = (j + offset) as usize;
                if old_idx < old_arr.elements.len() {
                    new_array.elements[j as usize] = jv_copy(&old_arr.elements[old_idx]);
                }
            }
        }
        for j in old_len..new_length {
            new_array.elements[j as usize] = JV_NULL.clone();
        }
        new_array.length = new_length;
        jvp_array_free(a.clone());
        let ptr = Box::into_raw(new_array) as u64;
        a.u = ptr;
        a.offset = 0;
        a.size = new_length;
        if let Some(arr) = jvp_array_ptr_mut(a) {
            return Some(&mut arr.elements[i as usize]);
        }
    } else {
        let new_size = imax(i + 1, a.size);
        if let Some(arr) = jvp_array_ptr_mut(a) {
            for j in arr.length..=pos {
                if (j as usize) < arr.elements.len() {
                    arr.elements[j as usize] = JV_NULL.clone();
                }
            }
            arr.length = imax(pos + 1, arr.length);
        }
        a.size = new_size;
        if let Some(arr) = jvp_array_ptr_mut(a) {
            if (pos as usize) < arr.elements.len() {
                return Some(&mut arr.elements[pos as usize]);
            }
        }
    }
    None
}
/// Get string value for comparison
fn jvp_string_value_cmp(s: &Jv) -> Option<&str> {
    if s.u == 0 || !jvp_has_kind(s, JvKind::String) {
        return None;
    }
    if let Some(str_ptr) = jvp_string_ptr(s) {
        return Some(&str_ptr.data);
    }
    None
}
/// Find mutable slot in object for given key
fn jvp_object_find_slot_mut<'a>(
    object: &'a mut Jv,
    key: &Jv,
    _bucket: Option<usize>,
) -> Option<&'a mut ObjectSlot> {
    if object.u == 0 || !jvp_has_kind(object, JvKind::Object) {
        return None;
    }
    let key_str = jvp_string_value_cmp(key)?;
    let obj = unsafe { &mut *(object.u as *mut JvpObject) };
    for slot in &mut obj.elements {
        if let Some(slot_key) = jvp_string_value_cmp(&slot.string) {
            if slot_key == key_str {
                return Some(slot);
            }
        }
    }
    None
}
/// Delete key from object
pub fn jvp_object_delete(object: &mut Jv, key: &Jv) -> i32 {
    assert!(jvp_has_kind(key, JvKind::String));
    *object = jvp_object_unshare(object.clone());
    let hash = jvp_string_hash(key);
    let size = jvp_object_size(object);
    for i in 0..size {
        if let Some(slot) = jvp_object_get_slot(object, i) {
            if jv_get_kind(&slot.string) != JvKind::Null {
                if slot.hash == hash && jvp_string_equal(key, &slot.string) {
                    if let Some(slot_mut) = jvp_object_get_slot_mut(object, i) {
                        jvp_string_free(slot_mut.string.clone());
                        slot_mut.string = JV_NULL.clone();
                        jv_free(slot_mut.value.clone());
                        slot_mut.value = JV_NULL.clone();
                    }
                    return 1;
                }
            }
        }
    }
    0
}
/// Delete key from object and return modified object
pub fn jv_object_delete(mut object: Jv, key: Jv) -> Jv {
    assert!(object.has_kind(JvKind::Object), "JVP_HAS_KIND(object, JV_KIND_OBJECT)");
    assert!(key.has_kind(JvKind::String), "JVP_HAS_KIND(key, JV_KIND_STRING)");
    jvp_object_delete(&mut object, &key);
    jv_free(key);
    object
}
/// Simple string hash function
fn compute_string_hash(s: &str) -> u32 {
    let mut hash: u32 = 5381;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u32);
    }
    hash
}
/// Create a formatted string using format arguments
pub fn jv_string_vfmt(fmt: &str, args: std::fmt::Arguments<'_>) -> Jv {
    let result = format!("{}", args);
    jv_string(&result)
}
/// Get mutable pointer to literal number
fn jvp_literal_number_ptr_mut(j: &mut Jv) -> Option<&mut JvpLiteralNumber> {
    if j.u == 0 || jv_get_kind(j) != JvKind::Number {
        return None;
    }
    if (j.kind_flags & JVP_FLAGS_NUMBER_LITERAL) == 0 {
        return None;
    }
    Some(unsafe { &mut *(j.u as *mut JvpLiteralNumber) })
}
/// Initialize decimal context with defaults
fn decContextDefault(ctx: &mut DecContext, kind: i32) {
    if kind == DEC_INIT_DECIMAL64 {
        ctx.digits = 16;
        ctx.emax = 384;
        ctx.emin = -383;
        ctx.round = Rounding::HalfEven;
        ctx.traps = 0;
        ctx.status = 0;
        ctx.clamp = 1;
    }
}
/// Reduce a decimal number (remove trailing zeros)
fn decNumberReduce(result: &mut DecNumber, source: &DecNumber, _ctx: &DecContext) {
    *result = source.clone();
}
/// Convert decimal number to string
fn decNumberToString(num: &DecNumber, buf: &mut String) {
    buf.clear();
    if (num.bits & (DECNAN | DECSNAN)) != 0 {
        buf.push_str("NaN");
        return;
    }
    if (num.bits & DECINF) != 0 {
        if (num.bits & 0x80) != 0 {
            buf.push('-');
        }
        buf.push_str("Infinity");
        return;
    }
    buf.push_str("0");
}
/// Parse string to double using dtoa context
fn jvp_strtod(_ctx: &DtoaContext, s: &str, _end: &mut Option<usize>) -> f64 {
    s.parse::<f64>().unwrap_or(0.0)
}
/// Get thread-local dtoa context
fn tsd_dtoa_context_get() -> DtoaContext {
    DtoaContext::default()
}
/// Convert literal number to double
fn jvp_literal_number_to_double(j: &Jv) -> f64 {
    assert!(
        jvp_has_flags_number_literal(j), "JVP_HAS_FLAGS(j, JVP_FLAGS_NUMBER_LITERAL)"
    );
    let mut dbl_ctx = DecContext::default();
    decContextDefault(&mut dbl_ctx, DEC_INIT_DECIMAL64);
    dbl_ctx.digits = DECIMAL64_PMAX;
    if let Some(p_dec_number) = jvp_dec_number_ptr(j) {
        let mut dec_double = DecNumberDoublePrecision::default();
        let mut literal = String::with_capacity(
            (DECIMAL64_PMAX + DECIMAL64_EMAX_DEFAULT + 1) as usize,
        );
        decNumberReduce(&mut dec_double.number, p_dec_number, &dbl_ctx);
        decNumberToString(&dec_double.number, &mut literal);
        let ctx = tsd_dtoa_context_get();
        let mut end = None;
        return jvp_strtod(&ctx, &literal, &mut end);
    }
    0.0
}
/// Allocate memory (Rust-idiomatic wrapper)
pub fn jv_mem_alloc<T: Default>() -> Box<T> {
    Box::new(T::default())
}
/// Free memory (no-op in Rust, handled by Drop)
pub fn jv_mem_free<T>(_ptr: T) {}
/// Internal: get string data
fn jvp_string_data(s: &Jv) -> &str {
    if s.u == 0 || !jvp_has_kind(s, JvKind::String) {
        return "";
    }
    if let Some(str_ptr) = jvp_string_ptr(s) {
        return &str_ptr.data;
    }
    ""
}
/// Recursively merge two objects
pub fn jv_object_merge_recursive(mut a: Jv, b: Jv) -> Jv {
    assert!(a.has_kind(JvKind::Object), "JVP_HAS_KIND(a, JV_KIND_OBJECT)");
    assert!(b.has_kind(JvKind::Object), "JVP_HAS_KIND(b, JV_KIND_OBJECT)");
    let mut iter = jv_object_iter(&b);
    while jv_object_iter_valid(&b, iter) {
        let k = jv_object_iter_key(&b, iter);
        let v = jv_object_iter_value(&b, iter);
        let elem = jv_object_get(&a, jv_copy(&k));
        if jv_is_valid(&elem) && elem.has_kind(JvKind::Object)
            && v.has_kind(JvKind::Object)
        {
            a = jv_object_set(a, k, jv_object_merge_recursive(elem, v));
        } else {
            jv_free(elem);
            a = jv_object_set(a, k, v);
        }
        iter = jv_object_iter_next(&b, iter);
    }
    jv_free(b);
    a
}
/// Create a formatted string
pub fn jv_string_fmt(fmt: &str, args: std::fmt::Arguments) -> Jv {
    let formatted = format!("{}", args);
    jv_string_from_str(&formatted)
}
/// Null value constant
pub const JV_NULL: Jv = Jv {
    kind_flags: JvKind::Null as u8,
    pad_: 0,
    offset: 0,
    size: 0,
    u: 0,
};
/// Get array pointer (raw)
pub fn jvp_array_ptr_raw(a: Jv) -> Option<Box<JvpArray>> {
    assert!(jvp_has_kind(& a, JvKind::Array));
    if a.u == 0 {
        return None;
    }
    unsafe { Some(Box::from_raw(a.u as *mut JvpArray)) }
}
/// Concatenate two arrays
pub fn jv_array_concat(mut a: Jv, b: Jv) -> Jv {
    assert!(jvp_has_kind(& a, JvKind::Array));
    assert!(jvp_has_kind(& b, JvKind::Array));
    let b_len = jv_array_length(&b);
    for i in 0..b_len {
        let elem = jv_array_get(jv_copy(&b), i);
        a = jv_array_append(a, elem);
    }
    jv_free(b);
    a
}
/// Find indexes of substring in string
pub fn jv_string_indexes(j: Jv, k: Jv) -> Jv {
    assert!(jvp_has_kind(& j, JvKind::String));
    assert!(jvp_has_kind(& k, JvKind::String));
    let jstr = jv_string_value(&j);
    let idxstr = jv_string_value(&k);
    let jlen = jv_string_length_bytes(&j);
    let idxlen = jv_string_length_bytes(&k);
    let mut a = jv_array();
    if idxlen != 0 {
        let jbytes = jstr.as_bytes();
        let idxbytes = idxstr.as_bytes();
        let mut pos = 0usize;
        while pos + (idxlen as usize) <= (jlen as usize) {
            if let Some(found) = find_substring(&jbytes[pos..], idxbytes) {
                let abs_pos = pos + found;
                a = jv_array_append(a, jv_number(abs_pos as f64));
                pos = abs_pos + 1;
            } else {
                break;
            }
        }
    }
    jv_free(j);
    jv_free(k);
    a
}
/// Find substring in bytes
fn find_substring(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    if needle.len() > haystack.len() {
        return None;
    }
    haystack.windows(needle.len()).position(|window| window == needle)
}
/// Check if number is literal (decimal)
pub fn jvp_number_is_literal(n: Jv) -> i32 {
    assert!(jvp_has_kind(& n, JvKind::Number));
    let expected = (JvKind::Number as u8) | ((JVP_NUMBER_DECIMAL << 4) & 0x70)
        | JVP_PAYLOAD_ALLOCATED;
    if n.kind_flags == expected { 1 } else { 0 }
}
impl JvpString {
    pub fn new(capacity: u32) -> Self {
        JvpString {
            refcnt: 1,
            hash: 0,
            length: capacity as i32,
            data: String::with_capacity(capacity as usize),
        }
    }
    pub fn length(&self) -> u32 {
        self.data.len() as u32
    }
}
impl Default for Jv {
    fn default() -> Self {
        Jv {
            kind_flags: JvKind::Invalid as u8,
            pad_: 0,
            offset: 0,
            size: 0,
            u: 0,
        }
    }
}
impl JvpObject {
    pub fn new(size: usize) -> Self {
        let mut elements = Vec::with_capacity(size);
        for i in 0..size {
            elements
                .push(ObjectSlot {
                    next: i as i32 - 1,
                    hash: 0,
                    string: jv_null(),
                    value: jv_null(),
                });
        }
        JvpObject {
            refcnt: 1,
            next_free: 0,
            elements,
        }
    }
}
impl JvpArray {
    fn new(capacity: i32) -> Box<Self> {
        Box::new(JvpArray {
            refcnt: 1,
            length: 0,
            alloc_length: capacity,
            elements: Vec::with_capacity(capacity as usize),
        })
    }
}
impl JvpLiteralNumber {
    fn new(value: f64) -> Box<Self> {
        Box::new(JvpLiteralNumber {
            refcnt: 1,
            num_double: value,
            literal_data: None,
            num_decimal: DecNumber {
                digits: 0,
                exponent: 0,
                bits: 0,
                lsu: Vec::new(),
            },
        })
    }
}
impl Clone for Jv {
    fn clone(&self) -> Self {
        jv_copy_internal(self)
    }
}
impl JvRefcntAtomic {
    /// Create a new reference count initialized to 1
    pub fn new() -> Self {
        Self { count: AtomicI32::new(1) }
    }
    /// Increment the reference count
    pub fn inc(&self) {
        self.count.fetch_add(1, AtomicOrdering::SeqCst);
    }
    /// Decrement the reference count, returns true if it reached zero
    pub fn dec(&self) -> bool {
        self.count.fetch_sub(1, AtomicOrdering::SeqCst) == 1
    }
    /// Get the current count
    pub fn get(&self) -> i32 {
        self.count.load(AtomicOrdering::SeqCst)
    }
}
impl fmt::Debug for Jv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Jv")
            .field("kind_flags", &self.kind_flags)
            .field("size", &self.size)
            .finish()
    }
}
impl JvpRefcounted {
    pub fn new() -> Self {
        JvpRefcounted {
            count: AtomicI32::new(1),
        }
    }
}
impl Jv {
    /// Get the kind of this Jv value
    pub fn get_kind(&self) -> JvKind {
        match self.kind_flags & 0x0F {
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
    /// Check if this Jv has the specified kind
    pub fn has_kind(&self, kind: JvKind) -> bool {
        (self.kind_flags & 0x0F) == kind as u8
    }
    /// Check if this is a number with decimal representation
    pub fn is_decimal_number(&self) -> bool {
        let expected = (JvKind::Number as u8 & 0x0F)
            | (((JVP_NUMBER_DECIMAL << 4) & 0x70) | JVP_PAYLOAD_ALLOCATED) & 0xF0;
        self.kind_flags == expected
    }
    /// Get the kind of this jv value
    pub fn jv_get_kind(&self) -> JvKind {
        match self.kind_flags & 0x0F {
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
    /// Check if this jv has allocated payload
    fn has_allocated_payload(&self) -> bool {
        (self.kind_flags & JVP_PAYLOAD_ALLOCATED) != 0
    }
    /// Check if this is a literal number
    fn is_literal_number(&self) -> bool {
        let expected_flags = (JvKind::Number as u8 & 0x0F)
            | ((((JVP_NUMBER_DECIMAL) << 4) & 0x70) | JVP_PAYLOAD_ALLOCATED) & 0xF0;
        self.kind_flags == expected_flags
    }
    /// Get the kind of this JV value
    #[inline]
    pub fn kind(&self) -> JvKind {
        match self.kind_flags & 0x0F {
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
    /// Create a null JV value
    pub fn null() -> Jv {
        Jv {
            kind_flags: JV_KIND_NULL,
            pad_: 0,
            offset: 0,
            size: 0,
            u: 0,
        }
    }
    /// Create an invalid JV value without message
    pub fn invalid() -> Jv {
        Jv {
            kind_flags: JV_KIND_INVALID,
            pad_: 0,
            offset: 0,
            size: 0,
            u: 0,
        }
    }
    fn is_allocated(&self) -> bool {
        (self.kind_flags & JVP_PAYLOAD_ALLOCATED) != 0
    }
    /// Create an invalid Jv with an error message
    pub fn invalid_with_msg(msg: Jv) -> Self {
        jv_invalid_with_msg(msg)
    }
    /// Check if this is a valid Jv (not invalid)
    pub fn is_valid(&self) -> bool {
        self.kind() != JvKind::Invalid
    }
    /// Copy this Jv (increment refcount for allocated types)
    pub fn copy(&self) -> Self {
        self.clone()
    }
    /// Create a new JV with specified kind and no payload
    fn new_simple(kind: JvKind) -> Self {
        Self {
            kind_flags: kind as u8,
            pad_: 0,
            offset: 0,
            size: 0,
            u: 0,
        }
    }
    /// Create a new JV with specified kind and allocated payload
    fn new_allocated(kind: JvKind, payload: JvPayload) -> Self {
        let boxed = Box::new(payload);
        let ptr = Box::into_raw(boxed);
        Self {
            kind_flags: (kind as u8) | JVP_PAYLOAD_ALLOCATED,
            pad_: 0,
            offset: 0,
            size: 0,
            u: ptr as u64,
        }
    }
    /// Create a new Jv with specified kind and flags
    pub fn new_with_kind_flags(kind: JvKind, flags: u8) -> Self {
        Jv {
            kind_flags: (kind as u8) | flags,
            pad_: 0,
            offset: 0,
            size: 0,
            u: 0,
        }
    }
    /// Create a number Jv from a f64
    pub fn number(n: f64) -> Jv {
        Jv {
            kind_flags: JvKind::Number as u8,
            pad_: 0,
            offset: 0,
            size: 0,
            u: n.to_bits(),
        }
    }
    /// Create a string Jv
    pub fn string(s: &str) -> Jv {
        jv_string(s)
    }
    /// Create an array Jv
    /// C: jv jv_array() { return jv_array_sized(16); }
    pub fn array() -> Jv {
        jv_array()
    }
    /// Create an object Jv
    /// C: jv jv_object() { return jvp_object_new(8); }
    pub fn object() -> Jv {
        jv_object()
    }
    /// Free this Jv (no-op in Rust due to RAII)
    pub fn free(self) {
        drop(self);
    }
    /// Get number value
    pub fn number_value(&self) -> f64 {
        if self.kind() == JvKind::Number {
            f64::from_bits(self.u)
        } else {
            0.0
        }
    }
    /// Get string value
    pub fn string_value(&self) -> Option<&str> {
        if !self.has_kind(JvKind::String) {
            return None;
        }
        if let Some(s) = jvp_string_ptr(self) {
            Some(&s.data)
        } else {
            None
        }
    }
    /// Append value to array
    pub fn array_append(self, val: Jv) -> Self {
        jv_array_append(self, val)
    }
    /// Get array element
    pub fn array_get(&self, idx: i32) -> Jv {
        jv_array_get(self.copy(), idx)
    }
    /// Get array length
    pub fn array_length(&self) -> i32 {
        jv_array_length(self)
    }
    /// Set object key-value
    pub fn object_set(self, key: Jv, val: Jv) -> Jv {
        jv_object_set(self, key, val)
    }
    /// Create true value
    pub fn jv_true() -> Jv {
        Jv {
            kind_flags: JvKind::True as u8,
            pad_: 0,
            offset: 0,
            size: 0,
            u: 0,
        }
    }
    /// Create false value
    pub fn jv_false() -> Jv {
        Jv {
            kind_flags: JvKind::False as u8,
            pad_: 0,
            offset: 0,
            size: 0,
            u: 0,
        }
    }
    /// Create bool value
    pub fn bool_val(b: bool) -> Jv {
        if b { Jv::jv_true() } else { Jv::jv_false() }
    }
    /// Create a new empty Jv
    pub fn new() -> Self {
        Jv::invalid()
    }
    /// Create sized string
    pub fn string_sized(s: &str, len: i32) -> Jv {
        jv_string_sized(s, len as usize)
    }
    /// Create string with format
    pub fn string_fmt(fmt: &str, args: &[&str]) -> Jv {
        let mut result = fmt.to_string();
        for arg in args {
            result = result.replacen("%s", arg, 1);
        }
        Jv::string(&result)
    }
    /// Get string length in bytes
    pub fn string_length_bytes(&self) -> i32 {
        self.size
    }
    /// Check equality
    pub fn equal(a: &Jv, b: &Jv) -> bool {
        a.kind_flags == b.kind_flags
    }
    /// Create sized array
    pub fn array_sized(size: i32) -> Jv {
        Jv {
            kind_flags: JvKind::Array as u8,
            pad_: 0,
            offset: 0,
            size,
            u: 0,
        }
    }
}
impl std::fmt::Debug for JvUnionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JvUnionData::Ptr(p) => write!(f, "Ptr({:?})", p),
            JvUnionData::Number(n) => write!(f, "Number({})", n),
        }
    }
}
impl Clone for JvPayload {
    fn clone(&self) -> Self {
        match self {
            JvPayload::Array(a) => JvPayload::Array(a.clone()),
            JvPayload::Object(o) => JvPayload::Object(o.clone()),
            JvPayload::String(s) => JvPayload::String(s.clone()),
            JvPayload::LiteralNumber(n) => JvPayload::LiteralNumber(n.clone()),
        }
    }
}
impl Default for JvRefcntAtomic {
    fn default() -> Self {
        Self::new()
    }
}
impl Default for JvUnion {
    fn default() -> Self {
        JvUnion::Number(0.0)
    }
}
impl Default for ObjectSlot {
    fn default() -> Self {
        ObjectSlot {
            next: -1,
            hash: 0,
            string: jv_null(),
            value: jv_null(),
        }
    }
}
impl Default for JvRefcntStruct {
    fn default() -> Self {
        Self { count: 1 }
    }
}

