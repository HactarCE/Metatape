// There are four cells we care about in the top-level tape:
// - tens, ones, [scratchpad], last bottle?
// The scratchpad should always be null after any operation.


!{=9}>!{=9}>
[
    !{print bottle count}
    !{" bottles of beer on the wall"}
    !{newline}
    !{print bottle count}
    !{" bottles of beer"}
    !{newline}
    !{"Take one down, pass it around"}
    !{newline}
    // Break out of the loop if this is the last bottle.
    >(n<|<
    !{dec bottles}
    // Check whether this is the last bottle.
    >f{ << !{=1?} ( < !{=0?} ) }<
    !{print bottle count}
    !{" bottles of beer on the wall"}
    !{newline}
    !{newline}
])

!N !o !{" bottles of beer on the wall"}
!{newline}
!{newline}
!N !o !{" bottles of beer on the wall"}
!{newline}
!N !o !{" bottles of beer"}
!{newline}
!G !o !_ !t !o !_ !t !h !e !_ !s !t !o !r !e !, !_ !b !u !y !_ !s !o !m !e !_ !m !o !r !e
!{newline}
!9 !9 !{" bottles of beer on the wall"}
!{newline}



@ dec bottles {
    // Check whether the ones place is 0.
    f{ < !{=0?} }(
        // Decrement the tens place.
        <<  !{dec}
        // Reset the ones place to 9.
        >   !{=9}
        >
    |
        // Decrement the ones place.
        <   !{dec}
        >
    )n
}

@ print bottle count {
    // Check whether the tens place is not 0.
    f{ << !{=0?} }(n|
        // If it is not 0, print it.
        << !{print digit} >>
    )
    // Print the ones digit.
    < !{print digit} >
}

@ " bottles of beer" {
    // Print " bottle"
    !_ !b !o !t !t !l !e
    // Only print "s" if this is not the last bottle.
    >(<|< !s )
    // Print " of beer"
    !_ !o !f !_ !b !e !e !r
}

@ " bottles of beer on the wall" {
    !{" bottles of beer"}
    !_ !o !n !_ !t !h !e !_ !w !a !l !l
}

@ "Take one down, pass it around" {
    !T !a !k !e !_
    // If this is the last bottle ...
    >(<
        // ... then print "it"
        !i !t
    |<
        // ... otherwise print "one"
        !o !n !e
    )
    !_ !d !o !w !n !,
    !_ !p !a !s !s
    !_ !i !t
    !_ !a !r !o !u !n !d
}

@ print digit {
    // Ascii '0' is 0x30, '1' is 0x31, ..., '9' is 0x39. So just output 0011 as
    // the first four bits, and then the next four bits come straight from the
    // number.
    e
    // Output 0x39 = bin 0011.
    >oo<oo
    // Output the highest bit. (If the bit is null, do not enter.)
    <<<(eox|o)
    // Output the next bit. (If the bit is null, do not enter.)
    >(eox|o)
    // etc.
    >(eox|o)
    >(eox|o)
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
    x
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
    f{ // Fork
        e
        // Check that the lowest bit is 1 ...
        e(x
            // ... and that the next bit doesn't exist
            <(|
                nx // Return true.
            |n) // Else return false.
        |n) // Else return false.
    }
}


/// Set the current cell to an integer with the value 9 (bin 1001).
@ =9 {
    e
    eexx // highest bit
    >ex
    >ex
    >eexx // lowest bit
    x
}



// We need lowercase "gjqvz" and the only capital letters we need are "GNT".
// Each of these assumes `ex>`.
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
