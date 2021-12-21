# Currently, all supported targets follow the assembly code syntax used by LLVM’s internal assembler which usually
# corresponds to that of the GNU assembler (GAS). On x86, the ". intel_syntax noprefix" mode of GAS is used by default.
# |
# |-> From offical documentation. Available when https://github.com/rust-lang/reference/pull/1105 gets merged.

# This files defines three "add(a,b)"-functions with three different calling conventions.

.section .text

# always produce 64-bit code
.code64

    # Adds two 64-bit numbers and returns the 64-bit result by using the
    # Microsoft calling convention used in the "PE" format on Windows
    # and UEFI.
    #
    # More info: https://docs.microsoft.com/en-us/cpp/build/x64-calling-convention?view=msvc-160
    #
    # First param:  RCX
    # Second param: RDX
    # Return:       RAX
    pe_abi__asm_add:
        mov     rax, rcx
        add     rax, rdx
        ret


    # See Rust code. Thin wrapper to demonstrate,
    # that UEFI indeed uses the calling convention used in the "PE" format.
    efi_abi__asm_add:
        call pe_abi__asm_add
        ret


    # See Rust code. Thin wrapper to demonstrate,
    # that Windows indeed uses the calling convention used in the "PE" format.
    win64_abi__asm_add:
        call pe_abi__asm_add
        ret

    # Adds two 64-bit numbers and returns the 64-bit result by using the
    # "System V ABI" calling convention present in Linux and MacOS (x86_64).
    #
    # More info: https://www.uclibc.org/docs/psABI-x86_64.pdf
    #
    # First param:  RDI
    # Second param: RSI
    # Return:       RAX
    system_v_abi__asm_add:
        mov     rax, rdi
        add     rax, rsi
        ret


