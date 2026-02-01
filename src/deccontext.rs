//! Module: deccontext
//!
//! Contains 16 transpiled functions:
//! - decContextClearStatus:2384664571806209174:./src/decNumber/decContext.c
//! - decContextRestoreStatus:2680899897480666381:./src/decNumber/decContext.c
//! - decContextSetRounding:12375977709050330634:./src/decNumber/decContext.c
//! - decContextZeroStatus:7492028821010608630:./src/decNumber/decContext.c
//! - decContextTestStatus:743514140789972529:./src/decNumber/decContext.c
//! - decContextSaveStatus:10528310557600317909:./src/decNumber/decContext.c
//! - decContextStatusToString:17475428338472794979:./src/decNumber/decContext.c
//! - decContextTestEndian:17078744810587359964:./src/decNumber/decContext.c
//! - decContextSetStatusQuiet:7565612495958983085:./src/decNumber/decContext.c
//! - decContextGetRounding:2001709514451052864:./src/decNumber/decContext.c
//! - decContextDefault:13103048942867757793:./src/decNumber/decContext.c
//! - decContextSetStatusFromStringQuiet:345174547351277372:./src/decNumber/decContext.c
//! - decContextSetStatusFromString:16524599344814881779:./src/decNumber/decContext.c
//! - decContextTestSavedStatus:13842588371596485303:./src/decNumber/decContext.c
//! - decContextSetStatus:5182981892126069896:./src/decNumber/decContext.c
//! - decContextGetStatus:10120459268837777444:./src/decNumber/decContext.c
use std::ptr::NonNull;
use std::fmt;
use crate::types::Rounding;
use crate::types::{int32_t, uint8_t, uint32_t, DecContext};
/// Status flag constants
pub mod status {
    pub const CONVERSION_SYNTAX: u32 = 0x00000001;
    pub const DIVISION_BY_ZERO: u32 = 0x00000002;
    pub const DIVISION_IMPOSSIBLE: u32 = 0x00000004;
    pub const DIVISION_UNDEFINED: u32 = 0x00000008;
    pub const INSUFFICIENT_STORAGE: u32 = 0x00000010;
    pub const INEXACT: u32 = 0x00000020;
    pub const INVALID_CONTEXT: u32 = 0x00000040;
    pub const INVALID_OPERATION: u32 = 0x00000080;
    pub const OVERFLOW: u32 = 0x00000100;
    pub const CLAMPED: u32 = 0x00000200;
    pub const ROUNDED: u32 = 0x00000400;
    pub const SUBNORMAL: u32 = 0x00000800;
    pub const UNDERFLOW: u32 = 0x00001000;
}
/// Context kind constants for initialization
pub mod context_kind {
    pub const DEC_INIT_BASE: i32 = 0;
    pub const DEC_INIT_DECIMAL32: i32 = 32;
    pub const DEC_INIT_DECIMAL64: i32 = 64;
    pub const DEC_INIT_DECIMAL128: i32 = 128;
}
/// Get the status flags from a context
pub fn decContextGetStatus(context: &DecContext) -> u32 {
    context.status
}
/// Clear specific status flags in a context
pub fn decContextClearStatus(context: &mut DecContext, mask: u32) -> &mut DecContext {
    context.status &= !mask;
    context
}
/// Get the current rounding mode from a context
pub fn decContextGetRounding(context: &DecContext) -> Rounding {
    context.round
}
/// Set status from a string (quiet version - doesn't trigger traps)
/// Returns None if the string is not recognized
pub fn decContextSetStatusFromStringQuiet<'a>(
    context: &'a mut DecContext,
    string: &str,
) -> Option<&'a mut DecContext> {
    match string {
        "Conversion syntax" => {
            Some(decContextSetStatusQuiet(context, status::CONVERSION_SYNTAX))
        }
        "Division by zero" => {
            Some(decContextSetStatusQuiet(context, status::DIVISION_BY_ZERO))
        }
        "Division impossible" => {
            Some(decContextSetStatusQuiet(context, status::DIVISION_IMPOSSIBLE))
        }
        "Division undefined" => {
            Some(decContextSetStatusQuiet(context, status::DIVISION_UNDEFINED))
        }
        "Inexact" => Some(decContextSetStatusQuiet(context, status::INEXACT)),
        "Insufficient storage" => {
            Some(decContextSetStatusQuiet(context, status::INSUFFICIENT_STORAGE))
        }
        "Invalid context" => {
            Some(decContextSetStatusQuiet(context, status::INVALID_CONTEXT))
        }
        "Invalid operation" => {
            Some(decContextSetStatusQuiet(context, status::INVALID_OPERATION))
        }
        "Overflow" => Some(decContextSetStatusQuiet(context, status::OVERFLOW)),
        "Clamped" => Some(decContextSetStatusQuiet(context, status::CLAMPED)),
        "Rounded" => Some(decContextSetStatusQuiet(context, status::ROUNDED)),
        "Subnormal" => Some(decContextSetStatusQuiet(context, status::SUBNORMAL)),
        "Underflow" => Some(decContextSetStatusQuiet(context, status::UNDERFLOW)),
        "No status" => Some(context),
        _ => None,
    }
}
/// Set status quietly (without triggering traps)
pub fn decContextSetStatusQuiet(
    context: &mut DecContext,
    status: u32,
) -> &mut DecContext {
    context.status |= status;
    context
}
/// Set status and potentially raise a signal if traps are enabled
pub fn decContextSetStatus(context: &mut DecContext, status: u32) -> &mut DecContext {
    context.status |= status;
    if (status & context.traps) != 0 {
        #[cfg(feature = "raise_signals")]
        unsafe {
            libc::raise(libc::SIGFPE);
        }
    }
    context
}
/// Restore status from saved value with mask
pub fn decContextRestoreStatus(
    context: &mut DecContext,
    newstatus: u32,
    mask: u32,
) -> &mut DecContext {
    context.status = (context.status & !mask) | (newstatus & mask);
    context
}
/// Save status bits using a mask
pub fn decContextSaveStatus(context: &DecContext, mask: u32) -> u32 {
    context.decContextSaveStatus(mask)
}
/// Test saved status against a mask
pub fn decContextTestSavedStatus(oldstatus: u32, mask: u32) -> u32 {
    DecContext::decContextTestSavedStatus(oldstatus, mask)
}
/// Test status against a mask
pub fn decContextTestStatus(context: &mut DecContext, mask: u32) -> u32 {
    if (context.status & mask) != 0 { 1 } else { 0 }
}
/// Zero (clear) all status flags
pub fn decContextZeroStatus(context: &mut DecContext) -> &mut DecContext {
    context.status = 0;
    context
}
/// Set the rounding mode
pub fn decContextSetRounding(
    context: &mut DecContext,
    newround: Rounding,
) -> &mut DecContext {
    context.round = newround;
    context
}
/// Set status from a string description
pub fn decContextSetStatusFromString<'a>(
    context: &'a mut DecContext,
    string: &str,
) -> Option<&'a mut DecContext> {
    match string {
        "Conversion syntax" => Some(decContextSetStatus(context, DEC_CONVERSION_SYNTAX)),
        "Division by zero" => Some(decContextSetStatus(context, DEC_DIVISION_BY_ZERO)),
        "Division impossible" => {
            Some(decContextSetStatus(context, DEC_DIVISION_IMPOSSIBLE))
        }
        "Division undefined" => {
            Some(decContextSetStatus(context, DEC_DIVISION_UNDEFINED))
        }
        "Inexact" => Some(decContextSetStatus(context, DEC_INEXACT)),
        "Insufficient storage" => {
            Some(decContextSetStatus(context, DEC_INSUFFICIENT_STORAGE))
        }
        "Invalid context" => Some(decContextSetStatus(context, DEC_INVALID_CONTEXT)),
        "Invalid operation" => Some(decContextSetStatus(context, DEC_INVALID_OPERATION)),
        "Overflow" => Some(decContextSetStatus(context, DEC_OVERFLOW)),
        "Clamped" => Some(decContextSetStatus(context, DEC_CLAMPED)),
        "Rounded" => Some(decContextSetStatus(context, DEC_ROUNDED)),
        "Subnormal" => Some(decContextSetStatus(context, DEC_SUBNORMAL)),
        "Underflow" => Some(decContextSetStatus(context, DEC_UNDERFLOW)),
        "No status" => Some(context),
        _ => None,
    }
}
/// Convert status to string representation
pub fn decContextStatusToString(context: &DecContext) -> &'static str {
    let status = context.status;
    if status & status::CONVERSION_SYNTAX != 0 {
        "Conversion syntax"
    } else if status & status::DIVISION_BY_ZERO != 0 {
        "Division by zero"
    } else if status & status::DIVISION_IMPOSSIBLE != 0 {
        "Division impossible"
    } else if status & status::DIVISION_UNDEFINED != 0 {
        "Division undefined"
    } else if status & status::INSUFFICIENT_STORAGE != 0 {
        "Insufficient storage"
    } else if status & status::INEXACT != 0 {
        "Inexact"
    } else if status & status::INVALID_CONTEXT != 0 {
        "Invalid context"
    } else if status & status::INVALID_OPERATION != 0 {
        "Invalid operation"
    } else if status & status::OVERFLOW != 0 {
        "Overflow"
    } else if status & status::CLAMPED != 0 {
        "Clamped"
    } else if status & status::ROUNDED != 0 {
        "Rounded"
    } else if status & status::SUBNORMAL != 0 {
        "Subnormal"
    } else if status & status::UNDERFLOW != 0 {
        "Underflow"
    } else {
        "No status"
    }
}
/// Initialize context with default values for the specified kind
pub fn decContextDefault(context: &mut DecContext, kind: i32) -> &mut DecContext {
    context.digits = 9;
    context.emax = 999999999;
    context.emin = -999999999;
    context.round = Rounding::HalfUp;
    context.traps = DEC_DEFAULT_TRAPS;
    context.status = 0;
    context.clamp = 0;
    match kind {
        DEC_INIT_BASE => {}
        DEC_INIT_DECIMAL32 => {
            context.digits = 7;
            context.emax = 96;
            context.emin = -95;
            context.round = Rounding::HalfEven;
            context.traps = 0;
            context.clamp = 1;
        }
        DEC_INIT_DECIMAL64 => {
            context.digits = 16;
            context.emax = 384;
            context.emin = -383;
            context.round = Rounding::HalfEven;
            context.traps = 0;
            context.clamp = 1;
        }
        DEC_INIT_DECIMAL128 => {
            context.digits = 34;
            context.emax = 6144;
            context.emin = -6143;
            context.round = Rounding::HalfEven;
            context.traps = 0;
            context.clamp = 1;
        }
        _ => {
            decContextSetStatus(context, DEC_INVALID_OPERATION);
        }
    }
    context
}
/// Test system endianness
/// Returns 0 if endianness matches compile-time setting, non-zero otherwise
pub fn decContextTestEndian(quiet: u8) -> i32 {
    let test_value: u32 = 1;
    let bytes = test_value.to_ne_bytes();
    let is_little_endian = bytes[0] == 1;
    const DECLITEND: u8 = 1;
    let expected_little = DECLITEND == 1;
    if is_little_endian != expected_little {
        if quiet == 0 {
            let adj = if is_little_endian { "little" } else { "big" };
            println!(
                "Warning: DECLITEND is set to {}, but this computer appears to be {}-endian",
                DECLITEND, adj
            );
        }
        if is_little_endian { 1 - DECLITEND as i32 } else { 0 - DECLITEND as i32 }
    } else {
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default_context() {
        let mut ctx = DecContext::default();
        decContextDefault(&mut ctx, DEC_INIT_BASE);
        assert_eq!(ctx.digits, 9);
        assert_eq!(ctx.round, Rounding::HalfUp);
    }
    #[test]
    fn test_decimal64_context() {
        let mut ctx = DecContext::default();
        decContextDefault(&mut ctx, DEC_INIT_DECIMAL64);
        assert_eq!(ctx.digits, 16);
        assert_eq!(ctx.emax, 384);
        assert_eq!(ctx.emin, - 383);
        assert_eq!(ctx.round, Rounding::HalfEven);
        assert_eq!(ctx.clamp, 1);
    }
    #[test]
    fn test_status_operations() {
        let mut ctx = DecContext::default();
        ctx.traps = 0;
        decContextSetStatus(&mut ctx, DEC_OVERFLOW);
        assert_eq!(decContextTestStatus(& mut ctx, DEC_OVERFLOW), 1);
        assert_eq!(decContextStatusToString(& ctx), "Overflow");
        decContextZeroStatus(&mut ctx);
        assert_eq!(ctx.status, 0);
        assert_eq!(decContextStatusToString(& ctx), "No status");
    }
    #[test]
    fn test_status_from_string() {
        let mut ctx = DecContext::default();
        ctx.traps = 0;
        decContextSetStatusFromString(&mut ctx, "Division by zero");
        assert_eq!(decContextTestStatus(& mut ctx, DEC_DIVISION_BY_ZERO), 1);
    }
    #[test]
    fn test_rounding() {
        let mut ctx = DecContext::default();
        decContextSetRounding(&mut ctx, Rounding::Floor);
        assert_eq!(decContextGetRounding(& ctx), Rounding::Floor);
    }
}
/// Context kind constants
pub const DEC_INIT_BASE: i32 = 0;
pub const DEC_INIT_DECIMAL32: i32 = 32;
pub const DEC_INIT_DECIMAL64: i32 = 64;
pub const DEC_INIT_DECIMAL128: i32 = 128;
/// Status flag constants
pub const DEC_CONVERSION_SYNTAX: u32 = 0x00000001;
pub const DEC_DIVISION_BY_ZERO: u32 = 0x00000002;
pub const DEC_DIVISION_IMPOSSIBLE: u32 = 0x00000004;
pub const DEC_DIVISION_UNDEFINED: u32 = 0x00000008;
pub const DEC_INSUFFICIENT_STORAGE: u32 = 0x00000010;
pub const DEC_INEXACT: u32 = 0x00000020;
pub const DEC_INVALID_CONTEXT: u32 = 0x00000040;
pub const DEC_INVALID_OPERATION: u32 = 0x00000080;
pub const DEC_OVERFLOW: u32 = 0x00000100;
pub const DEC_CLAMPED: u32 = 0x00000200;
pub const DEC_ROUNDED: u32 = 0x00000400;
pub const DEC_SUBNORMAL: u32 = 0x00000800;
pub const DEC_UNDERFLOW: u32 = 0x00001000;
const DEC_CONDITION_CS: &str = "Conversion syntax";
const DEC_CONDITION_DZ: &str = "Division by zero";
const DEC_CONDITION_DI: &str = "Division impossible";
const DEC_CONDITION_DU: &str = "Division undefined";
const DEC_CONDITION_IE: &str = "Inexact";
const DEC_CONDITION_IS: &str = "Insufficient storage";
const DEC_CONDITION_IC: &str = "Invalid context";
const DEC_CONDITION_IO: &str = "Invalid operation";
const DEC_CONDITION_OV: &str = "Overflow";
const DEC_CONDITION_PA: &str = "Clamped";
const DEC_CONDITION_RO: &str = "Rounded";
const DEC_CONDITION_SU: &str = "Subnormal";
const DEC_CONDITION_UN: &str = "Underflow";
const DEC_CONDITION_ZE: &str = "";
const DEC_CONDITION_MU: &str = "Multiple status";
const DEC_CONDITION_LENGTH: usize = 21;
/// Default traps for base context
pub const DEC_DEFAULT_TRAPS: u32 = DEC_DIVISION_BY_ZERO | DEC_CONVERSION_SYNTAX
    | DEC_DIVISION_IMPOSSIBLE | DEC_DIVISION_UNDEFINED | DEC_INSUFFICIENT_STORAGE
    | DEC_INVALID_CONTEXT | DEC_INVALID_OPERATION | DEC_OVERFLOW | DEC_UNDERFLOW;
impl Default for DecContext {
    fn default() -> Self {
        DecContext {
            digits: 9,
            emax: 999999999,
            emin: -999999999,
            round: Rounding::HalfEven,
            traps: 0,
            status: 0,
            clamp: 0,
        }
    }
}
impl Default for Rounding {
    fn default() -> Self {
        Rounding::HalfEven
    }
}
impl TryFrom<u32> for Rounding {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Rounding::Ceiling),
            1 => Ok(Rounding::Up),
            2 => Ok(Rounding::HalfUp),
            3 => Ok(Rounding::HalfEven),
            4 => Ok(Rounding::HalfDown),
            5 => Ok(Rounding::Down),
            6 => Ok(Rounding::Floor),
            7 => Ok(Rounding::ZeroFiveUp),
            8 => Ok(Rounding::Max),
            _ => Err(()),
        }
    }
}
impl DecContext {
    /// Create a new DecContext with default settings
    pub fn new_default() -> Self {
        Self::default()
    }
    /// Get the current status flags
    pub fn get_status(&self) -> u32 {
        self.status
    }
    /// Clear specific status flags
    pub fn clear_status(&mut self, mask: u32) -> &mut Self {
        self.status &= !mask;
        self
    }
    /// Get the current rounding mode
    pub fn get_rounding(&self) -> Rounding {
        self.round
    }
    /// Set status from a string (quiet - doesn't trigger traps)
    pub fn set_status_from_string_quiet(&mut self, string: &str) -> Option<&mut Self> {
        match string {
            "Conversion syntax" => Some(self.decContextSetStatusQuiet(status::CONVERSION_SYNTAX)),
            "Division by zero" => Some(self.decContextSetStatusQuiet(status::DIVISION_BY_ZERO)),
            "Division impossible" => {
                Some(self.decContextSetStatusQuiet(status::DIVISION_IMPOSSIBLE))
            }
            "Division undefined" => {
                Some(self.decContextSetStatusQuiet(status::DIVISION_UNDEFINED))
            }
            "Inexact" => Some(self.decContextSetStatusQuiet(status::INEXACT)),
            "Insufficient storage" => {
                Some(self.decContextSetStatusQuiet(status::INSUFFICIENT_STORAGE))
            }
            "Invalid context" => Some(self.decContextSetStatusQuiet(status::INVALID_CONTEXT)),
            "Invalid operation" => Some(self.decContextSetStatusQuiet(status::INVALID_OPERATION)),
            "Overflow" => Some(self.decContextSetStatusQuiet(status::OVERFLOW)),
            "Clamped" => Some(self.decContextSetStatusQuiet(status::CLAMPED)),
            "Rounded" => Some(self.decContextSetStatusQuiet(status::ROUNDED)),
            "Subnormal" => Some(self.decContextSetStatusQuiet(status::SUBNORMAL)),
            "Underflow" => Some(self.decContextSetStatusQuiet(status::UNDERFLOW)),
            "No status" => Some(self),
            _ => None,
        }
    }
    /// Restore status bits from saved status using a mask
    pub fn decContextRestoreStatus(&mut self, newstatus: u32, mask: u32) -> &mut Self {
        self.status &= !mask;
        self.status |= mask & newstatus;
        self
    }
    /// Save status bits using a mask
    pub fn decContextSaveStatus(&self, mask: u32) -> u32 {
        self.status & mask
    }
    /// Get the current status flags
    pub fn decContextGetStatus(&self) -> u32 {
        self.status
    }
    /// Set status flags
    pub fn decContextSetStatus(&mut self, status: u32) -> &mut Self {
        self.status |= status;
        if (self.status & self.traps) != 0 {}
        self
    }
    /// Set status flags quietly (without checking traps)
    pub fn decContextSetStatusQuiet(&mut self, status: u32) -> &mut Self {
        self.status |= status;
        self
    }
    /// Clear status flags using a mask
    pub fn decContextClearStatus(&mut self, mask: u32) -> &mut Self {
        self.status &= !mask;
        self
    }
    /// Zero all status flags
    pub fn decContextZeroStatus(&mut self) -> &mut Self {
        self.status = 0;
        self
    }
    /// Test status flags against a mask
    pub fn decContextTestStatus(&self, mask: u32) -> u32 {
        self.status & mask
    }
    /// Test saved status against a mask
    pub fn decContextTestSavedStatus(oldstatus: u32, mask: u32) -> u32 {
        oldstatus & mask
    }
    /// Get the current rounding mode
    pub fn decContextGetRounding(&self) -> Rounding {
        self.round
    }
    /// Set the rounding mode
    pub fn decContextSetRounding(&mut self, newround: Rounding) -> &mut Self {
        self.round = newround;
        self
    }
    /// Convert status flags to a human-readable string
    pub fn decContextStatusToString(&self) -> &'static str {
        let status = self.status;
        if status.count_ones() > 1 {
            return DEC_CONDITION_MU;
        }
        match status {
            0 => DEC_CONDITION_ZE,
            DEC_CONVERSION_SYNTAX => DEC_CONDITION_CS,
            DEC_DIVISION_BY_ZERO => DEC_CONDITION_DZ,
            DEC_DIVISION_IMPOSSIBLE => DEC_CONDITION_DI,
            DEC_DIVISION_UNDEFINED => DEC_CONDITION_DU,
            DEC_INEXACT => DEC_CONDITION_IE,
            DEC_INSUFFICIENT_STORAGE => DEC_CONDITION_IS,
            DEC_INVALID_CONTEXT => DEC_CONDITION_IC,
            DEC_INVALID_OPERATION => DEC_CONDITION_IO,
            DEC_OVERFLOW => DEC_CONDITION_OV,
            DEC_CLAMPED => DEC_CONDITION_PA,
            DEC_ROUNDED => DEC_CONDITION_RO,
            DEC_SUBNORMAL => DEC_CONDITION_SU,
            DEC_UNDERFLOW => DEC_CONDITION_UN,
            _ => DEC_CONDITION_MU,
        }
    }
    /// Set status from a condition string
    pub fn decContextSetStatusFromString(&mut self, string: &str) -> &mut Self {
        if let Some(status) = Self::string_to_status(string) {
            self.decContextSetStatus(status);
        }
        self
    }
    /// Set status from a condition string quietly (without checking traps)
    pub fn decContextSetStatusFromStringQuiet(&mut self, string: &str) -> &mut Self {
        if let Some(status) = Self::string_to_status(string) {
            self.decContextSetStatusQuiet(status);
        }
        self
    }
    /// Convert a condition string to status flag
    fn string_to_status(string: &str) -> Option<u32> {
        match string {
            s if s == DEC_CONDITION_CS => Some(DEC_CONVERSION_SYNTAX),
            s if s == DEC_CONDITION_DZ => Some(DEC_DIVISION_BY_ZERO),
            s if s == DEC_CONDITION_DI => Some(DEC_DIVISION_IMPOSSIBLE),
            s if s == DEC_CONDITION_DU => Some(DEC_DIVISION_UNDEFINED),
            s if s == DEC_CONDITION_IE => Some(DEC_INEXACT),
            s if s == DEC_CONDITION_IS => Some(DEC_INSUFFICIENT_STORAGE),
            s if s == DEC_CONDITION_IC => Some(DEC_INVALID_CONTEXT),
            s if s == DEC_CONDITION_IO => Some(DEC_INVALID_OPERATION),
            s if s == DEC_CONDITION_OV => Some(DEC_OVERFLOW),
            s if s == DEC_CONDITION_PA => Some(DEC_CLAMPED),
            s if s == DEC_CONDITION_RO => Some(DEC_ROUNDED),
            s if s == DEC_CONDITION_SU => Some(DEC_SUBNORMAL),
            s if s == DEC_CONDITION_UN => Some(DEC_UNDERFLOW),
            s if s == DEC_CONDITION_ZE => Some(0),
            _ => None,
        }
    }
    /// Initialize context with default settings for a specific decimal format
    pub fn decContextDefault(&mut self, kind: i32) -> &mut Self {
        self.round = Rounding::HalfEven;
        self.traps = 0;
        self.status = 0;
        self.clamp = 0;
        match kind {
            DEC_INIT_BASE => {
                self.digits = 9;
                self.emax = 999999999;
                self.emin = -999999999;
            }
            DEC_INIT_DECIMAL32 => {
                self.digits = 7;
                self.emax = 96;
                self.emin = -95;
            }
            DEC_INIT_DECIMAL64 => {
                self.digits = 16;
                self.emax = 384;
                self.emin = -383;
            }
            DEC_INIT_DECIMAL128 => {
                self.digits = 34;
                self.emax = 6144;
                self.emin = -6143;
            }
            _ => {
                self.decContextSetStatus(DEC_INVALID_CONTEXT);
            }
        }
        self
    }
    /// Test the endianness of the system
    /// Returns 0 if the system endianness matches the compiled assumption
    /// Returns 1 if there's a mismatch
    /// If quiet is 0, prints a message on mismatch
    pub fn decContextTestEndian(quiet: u8) -> i32 {
        let test_value: u32 = 0x01020304;
        let bytes = test_value.to_ne_bytes();
        let is_little_endian = bytes[0] == 0x04;
        if cfg!(target_endian = "little") == is_little_endian {
            0
        } else {
            if quiet == 0 {
                eprintln!("decContextTestEndian: endianness mismatch detected");
            }
            1
        }
    }
}
