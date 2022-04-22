# Architecture

## Components

### Register

Registers are used to store 64-bit values to make them addressable in program instructions. Writing a value shorter than 64 bits in in a register resets the remaining bits to 0.

There are 256 registers in Epism.

A `r` marks a register operand, which is 8-bit long.
The register `r0` is always equal to `0`.

### Lock

A lock is a special register used to control the parallelism of a program. It does not contain a value but has a state, which is either locked or unlocked.

There are 64 locks in Epism.

A `l` marks a lock operand, which is 8-bit long.
Running an asynchronous operation with the lock `l0` makes it synchronous.

### Thread

An instruction thread is the Epism equivalent of a software thread, or execution unit. Conceptually, all threads run in parallel, although they share all registers, locks, calculators and memory.

There are 16 threads in Epism.

A `t` marks an thread operand, which is 8-bit long.
The thread `t0` is the only thread active at the beginning of a program.

### Constant

A constant is an operand which contains an inline value. A constant can be 8-bit long, 16-bit long, 32-bit long or 64-bit long.

A `c8`, `c16`, `c32` or `c64` marks a constant operand.

## Instructions

### Nop

#### Nop

Opcode: `0x00`

Format: `nop`

Size: 1

Description:
- Has no effect.

### Constants

#### Const8

Opcode: `0x01`

Format: `const8 <r:destination> <c8:constant>`

Size: 3

Description:
- Loads the 8-bit constant `constant` into the register `destination`.

#### Const16

Opcode: `0x02`

Format: `const16 <r:destination> <c16:constant>`

Size: 4

Description:
- Loads the 16-bit constant `constant` into the register `destination`.

#### Const32

Opcode: `0x03`

Format: `const32 <r:destination> <c32:constant>`

Size: 6

Description:
- Loads the 32-bit constant `constant` into the register `destination`.

#### Const64

Opcode: `0x04`

Format: `const64 <r:destination> <c64:constant>`

Size: 10

Description:
- Loads the 64-bit constant `constant` into the register `destination`.

### Memory

#### Load8

Opcode: `0x05`

Format: `load8 <r:source> <r:destination> <l:lock>`

Size: 4

Description:
- Copies an 8-bit value from memory at the address `source` to the register `destination`.
- This operation is asynchronous and unlocks `lock` once completed.

#### Load16

Opcode: `0x06`

Format: `load16 <r:source> <r:destination> <l:lock>`

Size: 4

Description:
- Copies a 16-bit value from memory at the address `source` to the register `destination`.
- This operation is asynchronous and unlocks `lock` once completed.

#### Load32

Opcode: `0x07`

Format: `load32 <r:source> <r:destination> <l:lock>`

Size: 4

Description:
- Copies a 32-bit value from memory at the address `source` to the register `destination`.
- This operation is asynchronous and unlocks `lock` once completed.

#### Load64

Opcode: `0x08`

Format: `load64 <r:source> <r:destination> <l:lock>`

Size: 4

Description:
- Copies a 64-bit value from memory at the address `source` to the register `destination`.
- This operation is asynchronous and unlocks `lock` once completed.

#### Store8

Opcode: `0x09`

Format: `store8 <r:source> <r:destination> <l:lock>`

Size: 4

Description:
- Copies an 8-bit value from the register `source` to memory at the address `destination`.
- This operation is asynchronous and unlocks `lock` once completed.

#### Store16

Opcode: `0x0A`

Format: `store16 <r:source> <r:destination> <l:lock>`

Size: 4

Description:
- Copies a 16-bit value from the register `source` to memory at the address `destination`.
- This operation is asynchronous and unlocks `lock` once completed.

#### Store32

Opcode: `0x0B`

Format: `store32 <r:source> <r:destination> <l:lock>`

Size: 4

Description:
- Copies an 32-bit value from the register `source` to memory at the address `destination`.
- This operation is asynchronous and unlocks `lock` once completed.

#### Store64

Opcode: `0x0C`

Format: `store64 <r:source> <r:destination> <l:lock>`

Size: 4

Description:
- Copies an 64-bit value from the register `source` to memory at the address `destination`.
- This operation is asynchronous and unlocks `lock` once completed.

### Calculus

#### Bitwise AND

Opcode: `0x0D`

Format: `and <r:a> <r:b> <r:result> <l:lock>`

Size: 5

Description:
- Performs a bitwise and of `a` and `b` and stores the result in `result`.
- This operation is asynchronous and unlocks `lock` once completed.

#### Bitwise OR

Opcode: `0x0E`

Format: `or <r:a> <r:b> <r:result> <l:lock>`

Size: 5

Description:
- Performs a bitwise or of `a` and `b` and stores the result in `result`.
- This operation is asynchronous and unlocks `lock` once completed.

#### Bitwise XOR

Opcode: `0x0F`

Format: `xor <r:a> <r:b> <r:result> <l:lock>`

Size: 5

Description:
- Performs a bitwise exclusive or of `a` and `b` and stores the result in `result`.
- This operation is asynchronous and unlocks `lock` once completed.

#### Logical shift left

Opcode: `0x10`

Format: `sll <r:a> <r:b> <r:result> <l:lock>`

Size: 5

Description:
- Performs a logical left shift of `a` by `b` bits and stores the result in `result`.
- This operation is asynchronous and unlocks `lock` once completed.

#### Logical shift right

Opcode: `0x11`

Format: `srl <r:a> <r:b> <r:result> <l:lock>`

Size: 5

Description:
- Performs a logical right shift of `a` by `b` bits and stores the result in `result`.
- This operation is asynchronous and unlocks `lock` once completed.

#### Integer add

Opcode: `0x12`

Format: `add <r:a> <r:b> <r:result> <l:lock>`

Size: 5

Description:
- Performs an integer addition of `a` and `b` and stores the result in the `result`.
- This operation is asynchronous and unlocks `lock` once completed.

#### Integer sub

Opcode: `0x13`

Format: `sub <r:a> <r:b> <r:result> <l:lock>`

Size: 5

Description:
- Performs an integer substraction of `a` by `b` and stores the result in the `result`.
- This operation is asynchronous and unlocks `lock` once completed.

#### Integer mul

Opcode: `0x14`

Format: `mul <r:a> <r:b> <r:result> <l:lock>`

Size: 5

Description:
- Performs an integer multiplication of `a` and `b` and stores the result in the `result`.
- This operation is asynchronous and unlocks `lock` once completed.

#### Integer div

Opcode: `0x15`

Format: `div <r:a> <r:b> <r:result> <l:lock>`

Size: 5

Description:
- Performs an integer division of `a` by `b` and stores the result in the `result`.
- This operation is asynchronous and unlocks `lock` once completed.

#### Integer rem

Opcode: `0x16`

Format: `div <r:a> <r:b> <r:result> <l:lock>`

Size: 5

Description:
- Performs an integer remainder of `a` by `b` and stores the result in the `result`.
- This operation is asynchronous and unlocks `lock` once completed.

### Jumps

#### Jump

Opcode: `0x17`

Format: `jump <r:address>`

Size: 2

Description:
- Jumps the current thread to the address `address`.

#### Jump if

Opcode: `0x18`

Format: `jumpif <r:address> <r:condition>`

Size: 3

Description:
- Jumps the current thread to the address `address` if `condition` is equal to zero.

### Locks

#### Wait

Opcode: `0x19`

Format: `wait <l:lock>`

Size: 2

Description:
- Stops the current thread execution until the `lock` is unlocked.

#### Lock

Opcode: `0x1A`

Format: `lock <l:lock>`

Size: 2

Description:
- Locks the lock `lock`.

#### Unlock

Opcode: `0x1B`

Format: `unlock <l:lock>`

Size: 2

Description:
- Unlocks the lock `lock`.

### Threads

#### Start

Opcode: `0x1C`

Format: `start <t:thread> <r:address>`

Size: 3

Description:
- Starts the thread `thread` at the address `address`.

### Stop

Opcode: `0x1D`

Format: `stop <t:thread>`

Size: 2

Description:
- Stops the execution of the thread `thread`.

#### End

Opcode: `0x1E`

Format: `End`

Size: 1

Description:
- Stops the execution of the current thread.

### Primitives

#### Scan

Opcode: `0x1F`

Format: `scan <r:register>`

Size: 2

Description:
- Asks the user to enter an integer and stores it in `register`.

#### Print

Opcode: `0x20`

Format: `print <r:register>`

Size: 2

Description:
- Prints the integer in `register`.

#### Exit

Opcode: `0x21`

Format: `exit`

Size: 1

Description:
- Exits the program, ending the execution of all the threads.

## Errors

- Deadlock: No thread can proceed in the program.
- Cursor out of bounds: Tried to read the program out of its bounds.
- Division by zero: Tried to divide by zero.
- Input read: Failed to read the user input.
- Input parse: Failed to parse the user input into an integer.
