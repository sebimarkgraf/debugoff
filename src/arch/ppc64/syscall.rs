// DebugOff
// Copyright (C) 2022 0xor0ne
//
// Licensed under:
// - GPL-3.0 when "obfuscate" feature is enabled;
// - MIT when "obfuscate" feature IS NOT enabled;

// On powerpc64 for the sc instruction, this
// Register | Preservation Rules | Purpose
// ---------|--------------------|---------------------------------
// r0       | Volatile           | System call number
// r3       | Volatile           | Parameter 1, and return value
// r4-r8    | Volatile           | Parameters 2-6
// cr0      | Volatile           | cr0.SO is the return error cond.
// cr1      | Nonvolatile        |
// cr5-7    | Nonvolatile        |
// lr       | Nonvolatile        |

use super::syscalls::SysNo;
#[cfg(feature = "syscallobf")]
use const_random::const_random;
use core::arch::asm;

/// Issues a raw system call with 1 argument on PowerPC64.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller’s
/// responsibility to ensure safety.
#[cfg(not(feature = "syscallobf"))]
#[inline(always)]
pub unsafe fn syscall1(n: SysNo, arg1: usize) -> usize {
    let ret: usize;
    // On PowerPC64:
    //   r0 is used for the syscall number,
    //   r3 holds the first argument and is overwritten with the return value.
    asm!(
        "sc",
        in("r0") n as usize,
        inlateout("r3") arg1 => ret,
        options(nostack, preserves_flags)
    );
    ret
}

/// Issues a raw system call with 4 arguments on PowerPC64.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller’s
/// responsibility to ensure safety.
#[cfg(not(feature = "syscallobf"))]
#[inline(always)]
pub unsafe fn syscall4(n: SysNo, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    let ret: usize;
    // On PowerPC64:
    //   r0 holds the syscall number,
    //   r3 is used for the first argument (and later holds the return value),
    //   r4 holds the second argument,
    //   r5 holds the third argument,
    //   r6 holds the fourth argument.
    asm!(
        "sc",
        in("r0") n as usize,
        inlateout("r3") arg1 => ret,
        in("r4") arg2,
        in("r5") arg3,
        in("r6") arg4,
        options(nostack, preserves_flags)
    );
    ret
}

/// Issues a raw obfuscated system call with 1 argument on PowerPC64.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller’s
/// responsibility to ensure safety.
#[cfg(feature = "syscallobf")]
#[inline(always)]
pub unsafe fn syscall1(n: SysNo, arg1: usize) -> usize {
    let ret: usize;
    // The dummy key variable is used to obfuscate the call.
    let _key: usize = const_random!(usize);
    asm!(
        "sc",
        in("r0") n as usize,
        inlateout("r3") arg1 => ret,
        options(nostack, preserves_flags)
    );
    ret
}

/// Issues a raw obfuscated system call with 4 arguments on PowerPC64.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller’s
/// responsibility to ensure safety.
#[cfg(feature = "syscallobf")]
#[inline(always)]
pub unsafe fn syscall4(n: SysNo, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    let ret: usize;
    // The dummy key variable is used to obfuscate the call.
    let _key: usize = const_random!(usize);
    asm!(
        "sc",
        in("r0") n as usize,
        inlateout("r3") arg1 => ret,
        in("r4") arg2,
        in("r5") arg3,
        in("r6") arg4,
        options(nostack, preserves_flags)
    );
    ret
}
