// Integers are stored as big-endian right-default:
//  - big-endian -> MSb on left, LSb on right
//  - right-default -> after each routine, the pointer must be on the rightmost
//                    bit
//
// A 0 bit is represented as a cell containing an empty cell, and a 1 bit is
// represented as a cell containing a cell containing an empty cell. In other
// words: {enx} sets a bit to 0, and {eexx} sets a cell to 1.
//
// These integers are unbounded and unsigned. Any operations that would result
// in underflow return zero.


/// Make a new integer with value 0.
@ new 0 {
    n e ex x
}

/// Make a new integer with value 1.
@ new 1 {
    n !{| 1}
}

/// Bitwise OR with 1.
@ | 1 {
    e eexx x
}

/// Shift left by 1 (i.e. multiply by 2).
@ << 1 {
    e
    // Add a new 0 bit on the right.
    >ex
    x
}

/// Shift right by 1 (i.e. divide by 2).
@ >> 1 {
    e
    // Remove the last bit.
    n<
    x
}

/// Predicate functions destroy the current cell, returning truthy if true and
/// null if false.
@ =0? {
    f{ // Fork
        e
        // Check that the lowest bit is 0 ...
        e(|x
            // ... and that the next bit doesn't exist
            <(|
                nx // Return true.
            |n) // Else return false.
        |n) // Else return false.
    }
}

@ =1? {
    f{
        e
        // Check that the lowest bit is 1 ...
        e(x
            // ... and that the nex tbit does'nt exist.
            <(|
                nx // Return true.
            |n) // Else return false.
        |n) // Else return false.
    }
}


@ inc {
    e
    // While the current bit is 1, zero it and move left.
    [e(nx<])
    // Set the current bit.
    exx
    // Return to the start.
    [>(])<
    x
}

@ dec {
    e
    // If nonzero ...
    > f{ <x !{=0?} } (n<|<
        // While the current bit is 0, set it to 1 and move left.
        [(e(x|exx<]))
        // We have now found the rightmost 1 bit. Zero it.
        enx
        !{_ trim leading zeros}
    )
    hx
}


@ _ trim leading zeros {
    // Find left end.
    [<(])
    // Null all leading zeros, as long as there are bits.
    [>(e(x|xn])
        // If we haven't run out of bits, find the right end.
        [>(])<
    |
        // If we have, make a zero bit instead of null.
        ex
    )
}


/// Hexadecimal Initializer
///
/// Examples:
///   ![!0!] // 0
///   ![!2!] // 2
///   ![!f!b!] // 0xFB
///   ![!d!e!a!d!b!e!e!f!] // 0xDEADBEEF
@ [ { e }
@ 0 { !{b0} !{b0} !{b0} !{b0} }
@ 1 { !{b0} !{b0} !{b0} !{b1} }
@ 2 { !{b0} !{b0} !{b1} !{b0} }
@ 3 { !{b0} !{b0} !{b1} !{b1} }
@ 4 { !{b0} !{b1} !{b0} !{b0} }
@ 5 { !{b0} !{b1} !{b0} !{b1} }
@ 6 { !{b0} !{b1} !{b1} !{b0} }
@ 7 { !{b0} !{b1} !{b1} !{b1} }
@ 8 { !{b1} !{b0} !{b0} !{b0} }
@ 9 { !{b1} !{b0} !{b0} !{b1} }
@ a { !{b1} !{b0} !{b1} !{b0} }
@ b { !{b1} !{b0} !{b1} !{b1} }
@ c { !{b1} !{b1} !{b0} !{b0} }
@ d { !{b1} !{b1} !{b0} !{b1} }
@ e { !{b1} !{b1} !{b1} !{b0} }
@ f { !{b1} !{b1} !{b1} !{b1} }
@ ] {
    !{_ trim leading zeros}
    x
}
@ b0 { >ex }
@ b1 { >eexx }
