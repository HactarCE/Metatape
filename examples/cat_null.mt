// For each byte ...
[
    // Leave a flag up and to the left. We'll reset this flag if we see a 1
    // anywhere in the input byte, so it will only remain set if the byte was
    // all zeros.
    ex>e
    // Leave a marker eight cells to the right.
    ex<<<<<<<<
    // Input bits until you reach that marker, and then set the marker to null.
    [eexix>(n|])
    // Go back to the first bit that was inputted.
    [<(])
    // For each bit ...
    [>(
        // Output it and reset the flag if the bit is 1.
        eo(xx<n>e|x)
    ])
// Loop again if the flag was reset (i.e. the last byte was null).
xn<(|])
