/// This program proves that Metatape is Turing-complete by emulating Bitwise
/// Cyclic Tag: https://esolangs.org/wiki/Bitwise_Cyclic_Tag

// Usage: Input program as ASCII '0's and '1's, then a single space, and then
// input the initial data-string as ASCII '0's and '1's. The program may not be
// empty.

// Top-level tape: program, [scratchpad A], data-string, scratchpad B

// Place a cell for the data-string and program, then go to scratchpad A.
ex<<ex>
// Start on scratchpad A, and input the program. Once that's done, move to
// scratchpad B and input the data-string using the same code.
[
    // Enter the scratchpad, and for each byte ...
    e[
        // Input eight bits, but we only care about bit 4 (0x10) and bit 0 (0x01).
        iiiexi> // bit 4 (0x10)
        iiiexi< // bit 0 (0x01)
        // Check bit 4 (0x10). If it is 0, this is a space character (0x20); if it
        // is 1, this is a digit (0x30 or 0x31).
        (
            // Bit 4 is 1, so this is a digit. Add a new bit to the program string.
            x<e>exx>e
            // Read the lowest bit of the input char to determine whether this is a
            // '1' or a '0', set the bit in the program string if it is a '1'.
            >(x<eeexxx>e)
    // Loop.
    ])x
    // Now we are back at the origin (pointing to scratchpad A or B) and we are
    // done inputting this bitstring. If there is something one cell to the
    // right (i.e. if we are at scratchpad A rather than scratchpad B), then ...
    >(
        // Move to scratchpad B and loop to input the data-string.
        >
])<
// Null scratchpad B.
n
// Move to the left end of the data-string and go to scratchpad A.
<e[<(])>x<

// We are now at scratchpad A and we are ready to begin execution.

// Enter the program string.
<e

// For each cycle through the program, while there is data ...
[x>>e(x<<e
    // Return to the beginning of the program string.
    [<(])
    // For each instruction ...
    [
        // Move to the next instruction.
        >
        // If there is an instruction and there is data ...
        (x>>e(x<<e
            // If that instruction is 1 ...
            e(
                // Find the next instruction.
                {
                    // Leave a flag in scratchpad A.
                    xx>ex
                    // Is there another instruction after this one?
                    <e>(
                        // If so, null the flag in scratchpad A.
                        x>n<e
                    )
                    // Is there not another instruction after this one? (Check
                    // the flag in scratchpad A.) If there isn't ...
                    x>(n<e
                        // Return to the beignning of the program string.
                        [<(])>
                    x>)<e
                    e
                }
                // Enter the data string.
                xx>>ee
                // If the next bit of the data string is 1 ...
                (
                    // Go to the end of the data string and add a new bit there.
                    x[>(])e
                    // Copy the next instruction bit into the new bit at the end
                    // of the data-string.
                    xx<<ee(xx>>eeexxx<<ee)
                    // Return to the beginning of the data-string.
                    xx>>e[<(])>e
                )
                // Make a new dummy bit at the beginning of the data string
                // so that when we try to remove the first bit, this dummy
                // bit is the only thing removed.
                x<e
                // Exit the data string.
                xx<<ee
            )x
            // Return to scratchpad A.
            x>
            // Enter the data-string.
            >e
            // Remove the first bit of the data-string.
            n>
            // If there is anything left of the data-string ...
            (
                // For each bit in the data-string ...
                [
                    // Output 0x30 for '0' or 0x31 for '1'.
                    x>oo<oo>ooo<eeox
                    >
                // Loop if there is another bit.
                (])
                // Output 0x0a (newline).
                x>oooo<o>o<o>o<e
                // Return to the beginning of the data-string.
                [<(])>
                // Return to the program string.
                x<<e
            )
            // TODO if there is nothing left, exit the program!
    // Loop.
    ]))
])
