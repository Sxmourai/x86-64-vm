use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};

///! 3.4.1 General-Purpose Registers
/// The special uses of general-purpose registers by instructions are described in Chapter 5, “Instruction Set
/// Summary,” in this volume. See also: Chapter 3, Chapter 4, Chapter 5, and Chapter 6 of the Intel® 64 and IA-32
/// Architectures Software Developer’s Manual, Volumes 2A, 2B, 2C, & 2D. The following is a summary of special uses:
/// • EAX — Accumulator for operands and results data.
/// • EBX — Pointer to data in the DS segment.
/// • ECX — Counter for string and loop operations.
/// • EDX — I/O pointer.
/// • ESI — Pointer to data in the segment pointed to by the DS register; source pointer for string operations.
/// • EDI — Pointer to data (or destination) in the segment pointed to by the ES register; destination pointer for
/// string operations.
/// • ESP — Stack pointer (in the SS segment).
/// • EBP — Pointer to data on the stack (in the SS segment).
/// 
#[derive(IntoPrimitive, TryFromPrimitive)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u64)]
pub enum RegisterId {
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RBP,
    RSP,
    EIP,
}

pub enum GeneralPurpose {
}

pub enum Segment {
    CS,
    DS,
    SS,
    ES,
    FS,
    GS,
    EFLAGS,
    EIP,
}
