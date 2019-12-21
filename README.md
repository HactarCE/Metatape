# Metatape

Metatape is an esoteric programming language with just two data types: null and tape.

* [Introduction](#introduction)
* [Comments and whitespace](#comments-and-whitespace)
* [Mechanics](#mechanics)
    - [Basic instructions](#basic)
    - [Conditions](#conditions)
    - [Loops](#loops)
    - [Blocks](#blocks)
    - [Forking](#forking)
    - [Subroutines](#subroutines)
* [Usage](#usage)
* [Examples](#examples)
    - [Hello world](#hello-world)
    - [Cat](#cat)
    - [99 Bottles](#99-bottles)
    - [Bitwise Cyclic Tag](#bitwise-cyclic-tag)
* [Golf encoding](#golf-encoding)
* [Implementation](#implementation)

See also: [Metatape](https://esolangs.org/wiki/Metatape) on the [Esoteric programming languages wiki](https://esolangs.org/wiki/Main_Page)

## Introduction

Metatape is somewhat like a Turing machine; a single "pointer" moves left and right along an infinite (i.e. dynamically-allocated) "tape." This tape is initially filled with null values, but if the pointer enters a null cell (a spot on the tape), a new tape is created within that cell. The new tape is also initialized to null. The pointer can then exit this tape, returning to the original.

* A tape a dynamically allocated bidirectional array of cells (it can grow indefinitely)
* A cell either contains another tape, or null
* All cells in a tape have the same "parent" cell in another tape
* When the pointer enters a non-null cell, it will always point to the cell on that tape which it has most recently pointed to
* If the pointer enters a null cell, that cell is initialized to an empty tape
* If the pointer exits the "root" tape, then the "root" tape is placed inside a new tape (which the pointer now points to)

There's also a stack, but it's only used for conditions (mostly for convenience).

Basic Metatape is purely imperative. Supermetatape adds support for comments and subroutines.

## Comments and whitespace

Line comments begin with `//` and end with a line break. Block comments begin with `/*` and end with `*/`. Comments and whitespace are ignored anywhere that instructions are allowed. See [**Subroutines**](#subroutines) for whitespace handling in subroutine names.

## Mechanics

### Basic instructions

All instructions in Basic Metatape are a single character long.  Instructions are case-insensitive.

| Char | Mnemonic | Description                                                                        |
|:-----|:---------|:-----------------------------------------------------------------------------------|
| `.`  | No-op    | No operation                                                                       |
| `<`  | Left     | Move left along tape                                                               |
| `>`  | Right    | Move right along tape                                                              |
| `n`  | Null     | Set the current cell to null                                                       |
| `e`  | Enter    | Enter the current cell                                                             |
| `x`  | Exit     | Exit the current cell                                                              |
| `(`  | If       | Do `^?`                                                                            |
| `\|` | Else     | Skip forward to the matching `\|` or `)`                                           |
| `)`  | End If   | No operation                                                                       |
| `[`  | Loop     | No operation                                                                       |
| `]`  | End Loop | Skip backward to the matching `[`                                                  |
| `?`  | Random   | Generate a random bit; if that bit is `0`, do `n`                                  |
| `i`  | Input    | Read a single bit from the input buffer; if that bit is `0`, do `n`                |
| `o`  | Output   | If the current cell is null, append `0` to the output buffer; otherwise append `1` |
| `h`  | Halt     | Halt the program (breakpoint)                                                      |

"Matching" is defined as such:

* `(` pairs with `)`
* `[` pairs with `]`

When eight bits have been accumulated in the output buffer, it is flushed to `stdout` and the output buffer is reset.

### Conditions

The instructions `(`, `|`, and `)` can be used to construct the if-else statements found in many programming languages. Let `A`, `B`, and `C` be any sequence of zero or more instructions.

| Metatape | Pseudocode                            |
|:---------|:--------------------------------------|
| `(A)`    | `if (current cell) { A }`             |
| `(A\|B)` | `if (current cell) { A } else { B }`  |
| `(\|B)`  | `if (current cell is null) { B }`     |

Other constructions, such as `(A|B|C)`, are also permitted, but are not particularly useful. (`(A|B|C)` is equivalent to `(AC|B)`.)

### Loops

The `]` instruction unconditionally jumps backward to the matching `[`. This can be used with conditions to create more familiar looping behavior.

| Metatape      | Pseudocode                          |
|:--------------|:------------------------------------|
| `[A]`         | `forever do { A }`                  |
| `[A(])`       | `do { A } while (current cell)`     |
| `([A(]))`     | `while (current cell) do { A }`     |
| `[A(\|])`     | `do { A } while (not current cell)` |
| `(\|[A(\|]))` | `while (not current cell) do { A }` |

etc.

### Blocks

Code may be surrounded by `{` and `}` to form a block. Conditions and loops may not cross the boundary between blocks. For some instructions, such as [`f`](#forking), blocks may be used to collapse a sequence of characters into a single one.

### Forking

`f` is a special instruction, "Fork." It must be followed either by a single instruction (`f.`) or an instruction block (`f{...}`).

When the runtime encounters a fork instruction, it first saves the current state of the entire tape structure (call this "state A"), and then executes the instructions within the fork (call the result of this "state B"). Once that is done, it copies the contents of the current cell of state B into the the current cell of state A and then restores state A.

The main use of the fork instruction is copying data (e.g. `f<` to copy the contents of the cell on the left into the current one), but it can also be used to sandbox subroutines. E.g. `f!{messy subroutine}` prevents the messy subroutine from wrecking any data outside the current cell.

### Subroutines

Subroutines may be defined anywhere in the file that is not within another subroutine or code block.

| Metatape              | Description                         |
|:----------------------|:------------------------------------|
| `@name{instructions}` | Define the subroutine named `name`  |
| `!{name}`             | Execute the subroutine named `name` |
| `!a`                  | Execute the subroutine named `a`    |

Any consecutive string of whitespace is collapsed to a single space in a subroutine name; thus `my   sub` and `my␊sub` (`␊` symbolizes a newline) are equivalent to `my sub`, but not `mysub`. Additionally, leading and trailing whitespace is removed (e.g. ` my sub ` is equivalent to `my sub`). The empty string is allowed as a subroutine name (defined with `@{ ... }` and executed with `!{}`).

The following symbols are not allowed in subroutine names: `{}/`.

When calling subroutines whose name is only a single character, the `{}` braces may be omitted: `!{a}` is equivalent to `!a`.

## Usage

### Executable

Unfortunately I'm having issues with cross-compilation, so only Linux executables are available for now. You can down the [latest release here](https://github.com/HactarCE/Metatape/releases/latest), and call it from the terminal.

### Compile from source

1. [Install `cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. Clone this repository: `git clone https://github.com/HactarCE/Metatape.git && cd Metatape`
3. Run one of the examples: `cargo run -- examples/hello.mt`

## Examples

### Hello world

[hello.mt](examples/hello.mt)

```c
ex>
// The tape head is now pointing to a null cell, with a non-null cell to the
// left.
!H !e !l !l !o !_ !w !o !r !l !d !!

// Each of these functions moves left for a 0 bit and right for a 1 bit to
// output the ASCII value for the given character.
@ H { o<o>oo<o>ooo }
@ e { o<oo>oo<o>o<o> }
@ l { o<oo>o<oo>oo }
@ o { o<oo>o<oooo> }
@ _ { oo<o>ooooo }
@ w { o<ooo>o<ooo> }
@ r { o<ooo>oo<o>o }
@ d { o<oo>oo<o>oo }
@ ! { oo<o>oooo<o> }
```

### Cat

A "cat" program simply outputs whatever is given as input, like the Unix command `cat`. Here are three "cat" programs written in Metatape, each with different behavior on EOF.

#### Infinite cat

This cat prints null characters `0x00` forever after the input ends.

[`examples/cat_simple.mt`](examples/cat_simple.mt)

```
// Loop forever, inputting a bit and outputting the same bit.
[exio]
```

#### Null-terminated cat

This cat prints a null character `0x00` and then exits at the end of the input or at the first null.

[`examples/cat_null.mt`](examples/cat_null.mt)

```c
// For each byte ...
[
    // Leave a flag up and to the left. We'll reset this flag if we see a 1
    // anywhere in the input byte, so it will only remain set if the byte was
    // all zeros.
    ex>e
    // Leave a marker eight cells to the right.
    ex<<<<<<<<
    [eexix>(n|])
    // Input bits until you reach that marker, and then set the marker to null.
    // Go back to the first bit that was inputted.
    [<(])
    // For each bit ...
    [>(
        // Output it and reset the flag if the bit is 1.
        eo(xx<n>e|x)
    ])
// Loop again if the flag was reset (i.e. the last byte was null).
xn<(|])
```

Minified: `[ex>eex<<<<<<<<[eexix>(n|])[<(])[>(eo(xx<n>e|x)])xn<(|])` (56 chars; 28 bytes using golf encoding)

#### Unterminated cat

This cat exits at the end of the input or the first null but does _not_ print a null character. This is the behavior of Unix `cat`.

[`examples/cat_no_null.mt`](examples/cat_no_null.mt)

```c
// For each byte ...
[
    // Leave a flag up and to the left. We'll reset this flag if we see a 1
    // anywhere in the input byte, so it will only remain set if the byte was
    // all zeros.
    ex>e
    // Leave a marker eight cells to the right.
    ex<<<<<<<<
    // For each bit (eight times) ...
    [
        // Input a bit.
        eexi
        // Reset the flag if the bit is 1.
        (xx<n>e|x)
    // Repeat if we haven't reached the marker.
    >(n|])
    // Check the flag. If it is still set, then the current byte is null, so
    // exit the program (break out of the loop).
    x<(|>e
    // Return to the start of the byte.
    [<(])
    // Output each bit.
    [>(eox])
// Return to the start of the program (and close the condition on line 19).
xn<])
```

Minified: `[ex>eex<<<<<<<<[eexi(xx<n>e|x)>(n|])x<(|>e[<(])[>(eox])xn<])` (60 chars; 30 bytes using golf encoding)

### 99 Bottles

[The archetypal esoteric programming language challenge](https://esolangs.org/wiki/99_bottles_of_beer): printing the lyrics to "99 Bottles of Beer"

Here is a mostly-minified version of the program; the full program, including comments, can be found in [`examples/99_bottles.mt`](examples/99_bottles.mt).

```c
!{=9}>!{=9}>[
    !{print bottle count}!{" bottles of beer on the wall"}!{newline}
    !{print bottle count}!{" bottles of beer"}!{newline}
    !{"Take one down, pass it around"}!{newline}
    >(n<|<!{dec bottles}>f{ << !{=1?} ( < !{=0?} ) }<
    !{print bottle count}!{" bottles of beer on the wall"}!{newline}
    !{newline}
])

!N!o!{" bottles of beer on the wall"}!{newline}
!{newline}
!N!o!{" bottles of beer on the wall"}!{newline}
!N!o!{" bottles of beer"}!{newline}
!G!o!_!t!o!_!t!h!e!_!s!t!o!r!e!,!_!b!u!y!_!s!o!m!e!_!m!o!r!e!{newline}
!9!9!{" bottles of beer on the wall"}!{newline}

@ dec bottles { f{<!{=0?}}(<<!{dec}>!{=9}>|<!{dec}>)n }

@ print bottle count { f{<<!{=0?}}(n|<<!{printdigit}>>)<!{printdigit}> }

@ " bottles of beer" {
    !_!b!o!t!t!l!e >(<|<!s) !_!o!f!_!b!e!e!r
}
@ " bottles of beer on the wall" {
    !{" bottles of beer"}!_!o!n!_!t!h!e!_!w!a!l!l
}
@ "Take one down, pass it around" {
    !T!a!k!e!_ >(<!i!t|<!o!n!e) !_!d!o!w!n!,!_!p!a!s!s!_!i!t!_!a!r!o!u!n!d
}

@printdigit { e>oo<oo<<<(eox|o)>(eox|o)>(eox|o)>(eox|o)x }
@ dec { e>f{<x!{=0?}}(n<|<[(e(x|exx<]))enx!{_ trim leading zeros})x }
@ _ trim leading zeros { [<(])[>(e(x|xn])[>(])<|ex) }
@ =0? { f{ee(|x<(|nx|n)|n)} }
@ =1? { f{ee(x<(|nx|n)|n)} }
@ =9 { eeexx>ex>ex>eexxx }

@ , { oo<o>o<oo>oo }
@ 9 { oo<ooo>oo<o> }
@ newline { oooo<o>o<o>o }
@ _ { oo<o>ooooo }
@ a { o<oo>oooo<o> }
@ b { o<oo>ooo<o>o }
@ c { o<oo>ooo<oo> }
@ d { o<oo>oo<o>oo }
@ e { o<oo>oo<o>o<o> }
@ f { o<oo>oo<oo>o }
@ G { o<o>ooo<ooo> }
@ h { o<oo>o<o>ooo }
@ i { o<oo>o<o>oo<o> }
@ k { o<oo>o<o>o<oo> }
@ l { o<oo>o<oo>oo }
@ m { o<oo>o<oo>o<o> }
@ N { o<o>oo<ooo>o }
@ n { o<oo>o<ooo>o }
@ o { o<oo>o<oooo> }
@ p { o<ooo>oooo }
@ r { o<ooo>oo<o>o }
@ s { o<ooo>oo<oo> }
@ T { o<o>o<o>o<o>oo }
@ t { o<ooo>o<o>oo }
@ u { o<ooo>o<o>o<o> }
@ w { o<ooo>o<ooo> }
@ x { o<oooo>ooo }
@ y { o<oooo>oo<o> }
```

### Bitwise Cyclic Tag

[Bitwise Cyclic Tag](https://esolangs.org/wiki/Bitwise_Cyclic_Tag) is one of the simplest [Turing tarpits](https://esolangs.org/wiki/Turing_tarpit), making it an easy target for emulation to prove Turing-completeness. The following program (unminified and commented in [`examples/bct.mt`](examples/bct.mt)) emulates Bitwise Cyclic Tag (which is itself Turing-complete) using only the instructions `nexio<>()[]`, proving that this 11-instruction subset of Metatape (and by extension Metatape as a whole) is Turing-complete:

```c
// Usage: Input program as ASCII '0's and '1's, then a single space,
// and then input the initial data-string as ASCII '0's and '1's. The
// program may not be empty.
ex<<ex>[e[iiiexi>iiiexi<(x<e>exx>e>(x<eeexxx>e)])x>(>])<n<e[<(])>x<<e[
x>>e(x<<e[<(])[>(x>>e(x<<ee(xx>ex<e>(x>n<e)x>(n<e[<(])>x>)<eexx>>ee(x[
>(])exx<<ee(xx>>eeexxx<<ee)xx>>e[<(])>e)x<exx<<ee)xx>>en>([x>oo<oo>ooo
<eeox>(])x>oooo<o>o<o>o<e[<(])>x<<e)]))])
```

(253 chars; 127 bytes using golf encoding)

## Golf encoding

Metatape is not primarily intended to be used for [code golf](https://en.wikipedia.org/wiki/Code_golf), but in case anyone is interested in using it for code golf, here is a canonical encoding for compressing Metatape. This encoding was invented on 2019-11-02, so it can be used competetively on any challenges posted after that date.

| Instruction | Hexadecimal sequence | Byte count |
|:------------|:---------------------|:-----------|
| `.`         | `0`                  | 0.5        |
| `+`         | `1`                  | 0.5        |
| `(`         | `2`                  | 0.5        |
| `)`         | `3`                  | 0.5        |
| `<`         | `4`                  | 0.5        |
| `>`         | `5`                  | 0.5        |
| `e`         | `6`                  | 0.5        |
| `x`         | `7`                  | 0.5        |
| `i`         | `8`                  | 0.5        |
| `o`         | `9`                  | 0.5        |
| `[`         | `a`                  | 0.5        |
| `]`         | `b`                  | 0.5        |
| `{`         | `c`                  | 0.5        |
| `}`         | `d`                  | 0.5        |
| `n`         | `e`                  | 0.5        |
| `?`         | `f0`                 | 1.0        |
| `h`         | `f1`                 | 1.0        |
| `@_{`       | `f2__`               | 2.0        |
| `!_`        | `f3__`               | 2.0        |
| `f`         | `f4`                 | 1.0        |
| `@`         | `fa`                 | 1.0        |
| `!{`        | `fb`                 | 1.0        |

The bytes `f2` and `f3` must be followed by a single ASCII character, which is the name of the subroutine. Only 256 unique subroutines may be defined or called using `f2` and `f3`; beyond that, `fa` and `fb` must be used, which more directly correspond to ASCII Metatape.

## Implementation

This interpreter is written in Rust and represents the internal data structure using a sort of 2D [zipper](https://en.wikipedia.org/wiki/Zipper_(data_structure)) of linked lists. There are three structs, defined in [`src/metatape/tape.rs`](src/metatape/tape.rs):

```rust
pub struct Head {
    parent: Option<Arc<Tape>>, // extends up
    child: Option<Arc<Tape>>,  // extends down
    left: Option<Arc<Cell>>,   // extends left
    right: Option<Arc<Cell>>,  // extends right
}

struct Tape {
    next: Option<Arc<Tape>>,  // extends up/down
    left: Option<Arc<Cell>>,  // extends left
    right: Option<Arc<Cell>>, // extends right
}

struct Cell {
    child: Option<Arc<Tape>>, // extends down
    next: Option<Arc<Cell>>,  // extends left/right
}
```

[`Option`](https://doc.rust-lang.org/std/option/) allows the value to be `None` (so that the structure as a whole can be finite without loops) and [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) allows multiple references to the same object. `Head` is the root of the data structure. Extending "left" and "right" are other cells in the same tape. Extending "down" are the contents of this cell, the contents of this cell's contents, etc. And extending "up" are the tape containing this cell, the tape containing the tape containing this cell, etc.

When traversing the tape, the current `Head` is discarded and a new one is constructed using the cell or tape in the appropriate direction, appending the current `Head`'s values in the opposite direction. For example, here is the function to move left along the tape (corresponding to the `<` instuction):

```rust
pub fn move_left(&self) -> Head {
    let left = self.left.clone().unwrap_or_default();
    Head {
        parent: self.parent.clone(),
        child: left.child.clone(),
        left: left.next.clone(),
        right: if let (None, None) = (&self.right, &self.child) {
            None
        } else {
            Some(Arc::new(Cell {
                child: self.child.clone(),
                next: self.right.clone(),
            }))
        },
    }
}
```

The various calls to `.clone()` just increment the `Arc`'s reference count, allowing another immutable reference to the same data. The `if let (None, None) = (&self.right, &self.child) { None }` condition is an optimization to discard empty cells.
