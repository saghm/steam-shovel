[![crates.io](https://img.shields.io/crates/v/steam-shovel.svg)](https://crates.io/crates/steam-shovel)
# steam-shovel - Magic: The Gathering mulligan helper

`steam-shovel` calculates the odds of getting a certain number of lands in your hand given the
total number of lands in your deck (assuming a 60-card deck) and the size of your hand.

# Installation

Assuming you have Rust installed, simply run `cargo install steam-shovel`.

# Sample output

`$ steam-shovel --hand-size 7 --num-lands 24 -counts 3,4`

```
chance of 3 lands in a 7 card hand from a deck with 24 lands:  30.870%
chance of 4 lands in a 7 card hand from a deck with 24 lands:  19.645%
----------------------------------------------------------------------
chance of 3 or 4 lands in a 7 card hand from a deck with 24 lands:  50.515%
```

`$ steam-shovel -s 6 -l 20 -c all`

```
chance of 0 lands in a 6 card hand from a deck with 20 lands:   7.667%
chance of 1 lands in a 6 card hand from a deck with 20 lands:  26.287%
chance of 2 lands in a 6 card hand from a deck with 20 lands:  34.684%
chance of 3 lands in a 6 card hand from a deck with 20 lands:  22.498%
chance of 4 lands in a 6 card hand from a deck with 20 lands:   7.549%
chance of 5 lands in a 6 card hand from a deck with 20 lands:   1.239%
chance of 6 lands in a 6 card hand from a deck with 20 lands:   0.077%
----------------------------------------------------------------------
chance of 0, 1, 2, 3, 4, 5, or 6 lands in a 6 card hand from a deck with 20 lands: 100.000%
```

# Usage

```
steam-shovel 0.1.0
MTG mulligan helper

USAGE:
    steam-shovel [OPTIONS] --counts <counts> --num-lands <num_lands>

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information


OPTIONS:
    -c, --counts <counts>
            The number of lands in the hand to print the percentages for.

            May be either a comma-separated list of integers or the word 'all' (e.g. '1', '2,3,4', 'all').
    -s, --hand-size <hand_size>
            The size of the hand being drawn.

    -l, --num-lands <num_lands>
            The number of lands in the deck.
```
