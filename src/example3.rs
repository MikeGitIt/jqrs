//! Module: example3
//!
//! Contains 1 transpiled functions:
//! - main:14488106908486361945:./src/decNumber/example3.c
use crate::types::*;
use crate::decnumber::{
    decNumberFromString, decNumberDivide, decNumberAdd, decNumberPower,
    decNumberMultiply, decNumberRescale, decNumberToString,
};
use crate::deccontext::{decContextDefault, decContextStatusToString};
use std::env;
const DEC_CONVERSION_SYNTAX: u32 = 0x00000001;
const DEC_DIVISION_BY_ZERO: u32 = 0x00000002;
const DEC_DIVISION_IMPOSSIBLE: u32 = 0x00000004;
const DEC_DIVISION_UNDEFINED: u32 = 0x00000008;
const DEC_INSUFFICIENT_STORAGE: u32 = 0x00000010;
const DEC_INEXACT: u32 = 0x00000020;
const DEC_INVALID_CONTEXT: u32 = 0x00000040;
const DEC_INVALID_OPERATION: u32 = 0x00000080;
const DEC_OVERFLOW: u32 = 0x00000200;
const DEC_UNDERFLOW: u32 = 0x00002000;
const DEC_ERRORS: u32 = DEC_DIVISION_BY_ZERO | DEC_CONVERSION_SYNTAX
    | DEC_DIVISION_IMPOSSIBLE | DEC_DIVISION_UNDEFINED | DEC_INSUFFICIENT_STORAGE
    | DEC_INVALID_CONTEXT | DEC_INVALID_OPERATION | DEC_OVERFLOW | DEC_UNDERFLOW;
const DEC_INIT_BASE: i32 = 0;
const DECNUMDIGITS: usize = 34;
/// Helper to create a new DecNumber with proper LSU allocation
fn new_dec_number() -> DecNumber {
    DecNumber {
        digits: 1,
        exponent: 0,
        bits: 0,
        lsu: vec![0u16; DECNUMDIGITS],
    }
}
/// Helper to create a string buffer for decNumber output
fn new_string_buffer() -> Vec<u8> {
    vec![0u8; 52]
}
pub fn main(argc: i32, argv: &[String]) -> i32 {
    let need = 3;
    if argc < need + 1 {
        println!("Please supply {} number(s).", need);
        return 1;
    }
    let mut one = new_dec_number();
    let mut mtwo = new_dec_number();
    let mut hundred = new_dec_number();
    let mut start = new_dec_number();
    let mut rate = new_dec_number();
    let mut years = new_dec_number();
    let mut total = new_dec_number();
    let mut set = DecContext {
        digits: 0,
        emax: 0,
        emin: 0,
        round: Rounding::HalfEven,
        traps: 0,
        status: 0,
        clamp: 0,
    };
    decContextDefault(&mut set, DEC_INIT_BASE);
    set.traps = 0;
    set.digits = 25;
    decNumberFromString(&mut one, "1", &mut set);
    decNumberFromString(&mut mtwo, "-2", &mut set);
    decNumberFromString(&mut hundred, "100", &mut set);
    decNumberFromString(&mut start, &argv[1], &mut set);
    decNumberFromString(&mut rate, &argv[2], &mut set);
    decNumberFromString(&mut years, &argv[3], &mut set);
    if (set.status & DEC_ERRORS) != 0 {
        let status_str = decContextStatusToString(&set);
        println!("An input argument word was invalid [{}]", status_str);
        return 1;
    }
    let rate_copy = rate.clone();
    decNumberDivide(&mut rate, &rate_copy, &hundred, &mut set);
    let rate_copy = rate.clone();
    decNumberAdd(&mut rate, &rate_copy, &one, &mut set);
    let rate_copy = rate.clone();
    decNumberPower(&mut rate, &rate_copy, &years, &set);
    decNumberMultiply(&mut total, &rate, &start, &mut set);
    let total_copy = total.clone();
    decNumberRescale(&mut total, &total_copy, &mtwo, &mut set);
    if (set.status & DEC_ERRORS) != 0 {
        set.status &= DEC_ERRORS;
        let status_str = decContextStatusToString(&set);
        println!("Result could not be calculated [{}]", status_str);
        return 1;
    }
    let result_str = decNumberToString(&total);
    println!(
        "{} at {}% for {} years => {}", & argv[1], & argv[2], & argv[3], result_str
    );
    0
}
/// Entry point wrapper that converts standard main signature
pub fn main_entry() {
    let args: Vec<String> = env::args().collect();
    let exit_code = main(args.len() as i32, &args);
    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}
