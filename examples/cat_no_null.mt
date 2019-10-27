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
