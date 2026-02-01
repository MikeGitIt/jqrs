//! Module: exec_stack
//!
//! Contains 7 transpiled functions:
//! - stack_block:18366361408695529627:./src/exec_stack.h
//! - stack_reset:6809220776526140583:./src/exec_stack.h
//! - stack_pop_block:4095405605797875640:./src/exec_stack.h
//! - stack_init:9705703242587607101:./src/exec_stack.h
//! - stack_reallocate:8447911245971592601:./src/exec_stack.h
//! - align_round_up:9727113335816096859:./src/exec_stack.h
//! - stack_block_next:402661876393537026:./src/exec_stack.h
use crate::jv;
use crate::types::{Stack, StackPtr};
use std::mem;
/// Alignment constant for stack operations (typically 16 bytes for modern systems)
const ALIGNMENT: usize = 16;
/// Round up a size to the next alignment boundary
///
/// # Arguments
/// * `sz` - The size to round up
///
/// # Returns
/// The size rounded up to the next multiple of ALIGNMENT
#[inline]
pub fn align_round_up(sz: usize) -> usize {
    ((sz + (ALIGNMENT - 1)) / ALIGNMENT) * ALIGNMENT
}
/// Get a pointer to the data at a given stack position
///
/// # Arguments
/// * `s` - The stack
/// * `p` - The stack pointer (negative offset from end)
///
/// # Returns
/// A pointer to the data at the given position
#[inline]
pub fn stack_block<T>(s: &Stack, p: StackPtr) -> Option<&T> {
    if s.mem_end.is_null() || p >= 0 {
        return None;
    }
    // p is a negative offset from mem_end
    let ptr = unsafe { s.mem_end.offset(p as isize) as *const T };
    if ptr.is_null() {
        return None;
    }
    unsafe { ptr.as_ref() }
}
/// Get a mutable pointer to the data at a given stack position
///
/// # Arguments
/// * `s` - The stack
/// * `p` - The stack pointer (negative offset from end)
///
/// # Returns
/// A mutable pointer to the data at the given position
#[inline]
pub fn stack_block_mut<T>(s: &mut Stack, p: StackPtr) -> Option<&mut T> {
    if s.mem_end.is_null() || p >= 0 {
        return None;
    }
    // p is a negative offset from mem_end
    let ptr = unsafe { s.mem_end.offset(p as isize) as *mut T };
    if ptr.is_null() {
        return None;
    }
    unsafe { ptr.as_mut() }
}
/// Get the next block pointer stored at the beginning of a stack block
///
/// The stack stores a linked list of blocks, with each block containing
/// a pointer to the previous block at its start.
///
/// # Arguments
/// * `s` - The stack
/// * `p` - The stack pointer to the current block
///
/// # Returns
/// Reference to the next block pointer
pub fn stack_block_next(s: &Stack, p: StackPtr) -> Option<&StackPtr> {
    let next_ptr_pos = p - ALIGNMENT as i32;
    stack_block::<StackPtr>(s, next_ptr_pos)
}
/// Get a mutable reference to the next block pointer
pub fn stack_block_next_mut(s: &mut Stack, p: StackPtr) -> Option<&mut StackPtr> {
    let next_ptr_pos = p - ALIGNMENT as i32;
    stack_block_mut::<StackPtr>(s, next_ptr_pos)
}
/// Initialize a new empty stack
/// C: s->mem_end = 0; s->bound = ALIGNMENT; s->limit = 0;
pub fn stack_init(s: &mut Stack) {
    s.mem_end = std::ptr::null_mut();
    s.bound = ALIGNMENT as i32;
    s.limit = 0;
}
/// Resets the stack to its initial state, freeing any allocated memory.
///
/// # Panics
/// Panics if the stack is freed while not empty (limit != 0)
pub fn stack_reset(s: &mut Stack) {
    assert!(s.limit == 0, "stack freed while not empty");
    if !s.mem_end.is_null() {
        let offset = ((-s.bound) + ALIGNMENT as i32) as usize;
        let mem_start = unsafe { s.mem_end.sub(offset) };
        unsafe {
            let layout = std::alloc::Layout::from_size_align_unchecked(
                offset,
                ALIGNMENT,
            );
            std::alloc::dealloc(mem_start as *mut u8, layout);
        }
    }
    stack_init(s);
}
/// Reallocate the stack to accommodate more data
pub fn stack_reallocate(s: &mut Stack, sz: usize) {
    // C: int old_mem_length = -(s->bound) + ALIGNMENT;
    let old_mem_length = ((-s.bound) + ALIGNMENT as i32) as usize;
    let new_mem_length = align_round_up((old_mem_length + sz + 256) * 2);

    unsafe {
        let layout = std::alloc::Layout::from_size_align_unchecked(new_mem_length, ALIGNMENT);
        let new_mem = std::alloc::alloc_zeroed(layout);
        if new_mem.is_null() {
            std::alloc::handle_alloc_error(layout);
        }

        // Copy old data if it exists
        if !s.mem_end.is_null() && old_mem_length > ALIGNMENT {
            let old_mem_start = s.mem_end.sub(old_mem_length) as *const u8;
            let new_mem_start = new_mem.add(new_mem_length - old_mem_length);
            std::ptr::copy_nonoverlapping(old_mem_start, new_mem_start, old_mem_length);

            // Free old memory
            let old_layout = std::alloc::Layout::from_size_align_unchecked(old_mem_length, ALIGNMENT);
            std::alloc::dealloc(s.mem_end.sub(old_mem_length) as *mut u8, old_layout);
        }

        s.mem_end = new_mem.add(new_mem_length) as *mut i8;
        s.bound = -((new_mem_length - ALIGNMENT) as i32);
    }
}
/// Push a new block onto the stack
///
/// # Arguments
/// * `s` - The stack
/// * `p` - Current stack pointer
/// * `sz` - Size of the block to push
///
/// # Returns
/// New stack pointer pointing to the pushed block
pub fn stack_push_block(s: &mut Stack, p: StackPtr, sz: usize) -> StackPtr {
    let alloc_sz = align_round_up(sz) + ALIGNMENT;
    let new_limit = s.limit - alloc_sz as i32;
    if new_limit < s.bound {
        stack_reallocate(s, alloc_sz);
    }
    s.limit = new_limit;
    if let Some(next_ptr) = stack_block_next_mut(s, s.limit + alloc_sz as i32) {
        *next_ptr = p;
    }
    s.limit + alloc_sz as i32
}
/// Pop a block from the stack
///
/// # Arguments
/// * `s` - The stack
/// * `p` - Stack pointer to the block to pop
/// * `sz` - Size of the block
///
/// # Returns
/// Stack pointer to the previous block
pub fn stack_pop_block(s: &mut Stack, p: StackPtr, sz: usize) -> StackPtr {
    let next = stack_block_next(s, p).copied().unwrap_or(0);
    // C: if (p == s->limit) { s->limit += alloc_sz; }
    if p == s.limit {
        let alloc_sz = align_round_up(sz) + ALIGNMENT;
        s.limit += alloc_sz as i32;
    }
    next
}
/// Check if popping a block will free memory
/// C: return p == s->limit;
pub fn stack_pop_will_free(s: &Stack, p: StackPtr) -> i32 {
    if p == s.limit { 1 } else { 0 }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_align_round_up() {
        assert_eq!(align_round_up(0), 0);
        assert_eq!(align_round_up(1), ALIGNMENT);
        assert_eq!(align_round_up(ALIGNMENT), ALIGNMENT);
        assert_eq!(align_round_up(ALIGNMENT + 1), ALIGNMENT * 2);
    }
    #[test]
    fn test_stack_init() {
        let mut stack = Stack {
            mem_end: std::ptr::null_mut(),
            bound: 0,
            limit: 0,
        };
        stack_init(&mut stack);
        assert!(stack.mem_end.is_null());
        // Per C code: s->bound = ALIGNMENT
        assert_eq!(stack.bound, ALIGNMENT as i32);
        assert_eq!(stack.limit, 0);
    }
    #[test]
    fn test_stack_reset() {
        let mut stack = Stack {
            mem_end: std::ptr::null_mut(),
            bound: 0,
            limit: 0,
        };
        stack_reset(&mut stack);
        assert!(stack.mem_end.is_null());
        // Per C code: stack_reset calls stack_init which sets bound = ALIGNMENT
        assert_eq!(stack.bound, ALIGNMENT as i32);
        assert_eq!(stack.limit, 0);
    }
    #[test]
    fn test_stack_reallocate() {
        let mut stack = Stack {
            mem_end: std::ptr::null_mut(),
            bound: 0,
            limit: 0,
        };
        stack_reallocate(&mut stack, 100);
        assert!(!stack.mem_end.is_null());
        assert!(stack.bound < 0);
        // Clean up
        stack.limit = 0;
        stack_reset(&mut stack);
    }
}
/// Set the next block pointer
pub fn stack_block_next_set(s: &mut Stack, p: StackPtr, next: StackPtr) {
    if s.mem_end.is_null() || p <= 0 {
        return;
    }
    // p is a negative offset, so we need to compute the pointer to the next field
    let next_offset = p - mem::size_of::<StackPtr>() as i32;
    let ptr = unsafe { s.mem_end.offset(next_offset as isize) as *mut StackPtr };
    if !ptr.is_null() {
        unsafe { *ptr = next };
    }
}
/// Check if a jv value is valid (placeholder - actual implementation in jv module)
pub fn jv_is_valid(x: &jv::Jv) -> bool {
    x.kind_flags != 0
}
