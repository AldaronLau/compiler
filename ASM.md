# Yeet Assembly
The safe assembly language!

This assembly language works on any processor.  This means we don't know how
many registers we have (1-64).  So we assume 256, and then if needed swap out
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
6: dst1 / args
6: dst0
6: src1
6: src0
# 32 bits optional
32: immediate
```

### 
```
syscall(long=0,opcode=0)
========================
nop(param=0,args=0)
quit_success(param=1,args=0) # Quit on success
quit_failure(param=1,args=1) # Quit on failure
log_info(param=2,arg=0,src0=length,src1=address) # Print text to stdout
log_error(param=2,arg=1,src0=length,src1=address) # Print text to stderr

branch(opcode=1)
================

operator_logic(opcode=4)
========================
and(param=0001)
ior(param=0111)
xor(param=0110)
(etc. - just store a truth table)

operator_int(opcode=5)
======================
div(param=b0001)
mod(param=b0010)
divmod(param=b=0011)
add(param=0100)
sub(param=0101)
negadd(param=0110)
negsub(param=0111)
mul(param=1000)
negmul(param=1001)
lshift(param=1100)
rshift(param=1101)
lshift_wrap(param=1110)
rshift_wrap(param=1111)

operator_float(opcode=6) - same as operator_int
===============================================
div(param=b0001)
mod(param=b0010)
divmod(param=b=0011)
add(param=0100)
sub(param=0101)
negadd(param=0110)
negsub(param=0111)
mul(param=1000)
negmul(param=1001)
lshift(param=1100)
rshift(param=1101)
lshift_wrap(param=1110)
rshift_wrap(param=1111)

operator_assign(opcode=7)
=========================
set(param=0)
not(param=1)
```
