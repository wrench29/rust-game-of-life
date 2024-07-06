# Game of Life

Implementation of the Game of Life in Rust, using Raylib as graphics library.

## How to run

The game is cross-platform and can be run on Windows, macOS and Linux.

`$ cargo run`

## Details

There is a 200x200 cells field, on which 4 different groups of life spawn (marked in different colors).
I also added the ability to change the speed of the population process, by default it's 1 generation per second, but
you can change it from 1 to 20 gen/s (in steps of 1) using the `Arrow UP` / `Arrow DOWN` keys.
