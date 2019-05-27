# Yeet Assembly
The safe assembly language!

This assembly language works on any processor.  This means we don't know how
many registers we have (1-64).  So we assume 64, and then if needed swap out
registers.  Each register is 32 bits.  We actually have 32 64-bit registers, 16
128-bit registers & 8 256-bit registers.  Yeet assumes a 32-bit processor (but
still works on 64-bit).

```yeet
#!yeet 0.0

# Global Data
data {
    # Immutable text type
    let text: text "Hello, world\n"
}

# Immutable code
code {
    # Label where the program starts (store a0 & a1 on the stack, so we can use)
    @home r0, r1 {
        # Log info text
        assign r1, text.length
        assign r2, text.address
        syscall 2, r1, r2
    }
}
```

## Registers
All registers are general purpose registers (r0-r64).

## Instructions
Instructions are 64 bits for a 64-bit processor.  OpCodes are 8 bits.

### Instruction Layout
```
# 32 bits mandatory
1: long
3: opcode
4: param
6: args
6: src1
6: src0
6: dst
# 32 bits optional
32: immediate
```

### 
```
syscall_interrupts(long=0,opcode=0)
===================================
nop(args=000000,param=0000)
quit_success(args=000001,param=0000) # Quit on success
quit_failure(args=000001,param=0001) # Quit on failure
log_info(arg=000010,param=0000,src0=length,src1=address) # Print text to stdout
log_error(arg=000010,param=0001,src0=length,src1=address) # Print text to stderr

branch_and_stack(opcode=1)
==========================
jump(param=0000)
call(param=0001)
return(param=0010)
match(param=0011)
stack_push(param=0100)
stack_pop(param=0101)
if_zero(param=1000)
if_nonzero(param=1001)
if_even(param=1010)
if_odd(param=1011)
if_equal(param=1100)
if_notequal(param=1101)

operator_logic(opcode=2)
========================
contradiction(param=0000)
and(param=0001)
(param=0011)
(param=0100)
(param=0101)
(param=0110)
ior(param=0111)
xor(param=0110)
nor(param=1000)
(param=1001)
(param=1010)
(param=1011)
(param=1100)
(param=1101)
(param=1110)
tautology(param=1111)
(etc. - just store a truth table)

loads_and_stores(opcode=3)
==========================
store_to32(param=0000)
store_to32_not(param=0001)
store_to64(param=0010)
store_to64_not(param=0011)
store_to32_shrink(param=0100)
store_to32_not_shrink(param=0101)
store_to64_shrink(param=0110)
store_to64_not_shrink(param=0111)
load_from32(param=1000)
load_from32_not(param=1001)
load_from64(param=1010)
load_from64_not(param=1011)
load_from32_extend(param=1100)
load_from32_not_extend(param=1101)
load_from64_extend(param=1110)
load_from64_not_extend(param=1111)

operator_assign(opcode=4)
=========================
set(param=0000)
not(param=0001)
set32(param=0010)
not32(param=0011)
set64(param=0100)
not64(param=0101)
set96(param=0110)
not96(param=0111)
count_ones(param=1000)
count_zeros(param=1001)
set_less_than_unsigned(param=1010)
set_more_than_or_equal_unsigned(param=1011)
set_less_than_signed(param=1100)
set_more_than_or_equal_singed(param=1101)
sign_extend_or_retract_int(param=1110)
sign_extend_or_retract_float(param=1111)

operator_float(opcode=5)
========================
arithmetic_shift(param=0000)
------------------
div(param=b0001)
mod(param=b0010)
divmod(param=b0011)
add(param=0100)
sub(param=0101)
mul(param=0110)
mulneg(param=0111)

round_down(param=1000)
round(param=b1001)
floor(param=b1010)
ceiling(param=b1011)
round_down_to_signed_int(param=1100)
round_to_signed_int(param=1101)
floor_to_signed_int(param=1110)
ceiling_to_signed_int(param=1111)

operator_wrapping_int(opcode=6)
===============================
logical_shift_wrapping(param=0000)
logical_reverse_bits(param=1000)
------------------
div.unsigned(param=b0001)
mod.unsigned(param=b0010)
divmod.unsigned(param=b0011)
add.unsigned(param=0100)
sub.unsigned(param=0101)
mul.unsigned(param=0110)
mulneg.unsigned(param=0111)
div.signed(param=b1001)
mod.signed(param=b1010)
divmod.signed(param=b1011)
add.signed(param=1100)
sub.signed(param=1101)
mul.signed(param=1110)
mulneg.signed(param=1111)

operator_saturating_int(opcode=7)
=================================
logical_shift_in_zeros(param=0000)
logical_shift_in_ones(param=1000)
------------------
div.unsigned(param=b0001)
mod.unsigned(param=b0010)
divmod.unsigned(param=b0011)
add.unsigned(param=0100)
sub.unsigned(param=0101)
mul.unsigned(param=0110)
mulneg.unsigned(param=0111)
div.signed(param=b1001)
mod.signed(param=b1010)
divmod.signed(param=b1011)
add.signed(param=1100)
sub.signed(param=1101)
mul.signed(param=1110)
mulneg.signed(param=1111)
```

For opcodes 3-7, you may have vector instructions.  This uses the `args` value.

```
Data Size(3 bits):
0:1 byte / 8-bit
1:2 byte / 16-bit
2:4 byte / 32-bit
3:8 byte / 64-bit
4:16 byte / 128-bit
5:32 byte / 256-bit (only for divmod packed hi+lo values & opcode 3)
6:64 byte / 512-bit (only opcode 3)
7:128 byte / 1024-bit (only opcode 3)

How Many(3 bits - only for opcodes 4-7):
0:1 register
1:2 registers
2:4 registers
3:8 registers
4:16 registers
5:32 registers
6:64 registers
7:128 registers
```
