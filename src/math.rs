use std::ops::Mul;

// Decks always will have 60 cards in them.
const DECK_SIZE: u128 = 60;

// `Fraction` is used to store division operation whose quotient can be lazily evaluated, which
// allows for use in subsequent mathematical operations without losing precision due to conversion
// to floating point values.
#[derive(Debug)]
pub struct Fraction {
    num: u128,
    denom: u128,
}

impl Fraction {
    fn new(num: u128, denom: u128) -> Self {
        Self { num, denom }
    }

    pub fn eval(&self) -> f64 {
        self.num as f64 / self.denom as f64
    }
}

// Evaluate the product of the numbers between `begin` and `end` (inclusive).
fn mult_range(begin: u128, end: u128) -> u128 {
    (begin..=end).into_iter().fold(1, Mul::mul)
}

// Evaluate the product of the range of numbers of length `down` starting with `begin`. For
// example, `mult_range_down_by(5, 3)` would be `5 * 4 * 3`, or 60.
//
// This is used to facilitate evaluating quotients of factorials. The above example is equivalent
// to evaluting `5! / 2!`.
fn mult_range_down_by(begin: u128, down: u128) -> u128 {
    if down == 0 {
        return 1;
    }

    mult_range(begin - down + 1, begin)
}

// Calculates the number of combinations of `k` elements from a set of size `n`. Mathematiclaly,
// this is expressed as `n! / (k! * (n - k)!)`, which can be simplifed by dividing both the
// numerator and denominator by `(n - k)!`, yielding the implementation below.
fn n_choose_k(n: u128, k: u128) -> u128 {
    mult_range_down_by(n, k) / mult_range(1, k)
}

// Calculate the chance of drawing exactly `hand_count_of_type` of a given type of card in hand of
// size `hand_size` given that there are `deck_count_of_type` of that type of card in the entire
// deck. We call this set of circumstances a "split", as the hand is split between cards of a
// certain type and cards that are not that type in a given ratio.
fn chance_of_split(
    hand_size: u128,
    hand_count_of_type: u128,
    deck_count_of_type: u128,
) -> Fraction {
    // Calculate the number of cards not of the given type in the expected hand.
    let hand_count_of_other = hand_size - hand_count_of_type;

    // Calculate the number of cards not of the given type in the entire deck.
    let deck_count_of_other = DECK_SIZE - deck_count_of_type;

    // The number of combinations of a certain number cards of a given type can be calculated by
    // multiplying the number of choices for each card selected. For example, if there are 20 cards
    // of the given type in the deck and we want to select 3 of them, there are 20 choices for the
    // first card, 19 for the second (since one card was removed), and 18 for the third (since two
    // cards were removed), so the number of combinations of 3 cards from those 20 is
    // `20 * 19 * 18`.
    let combinations_of_given_type = mult_range_down_by(deck_count_of_type, hand_count_of_type);
    let combinations_of_other_type = mult_range_down_by(deck_count_of_other, hand_count_of_other);

    // Similarly, the number of total hands we can draw can be calculated by the number of choices
    // to draw for each card in the hand. For example, if there are 60 cards in the deck and 6
    // cards in the hand, the number of hands that can be drawn is `60 * 59 * 58 * 57 * 56 * 55`.
    let total_number_of_possible_hands = mult_range_down_by(DECK_SIZE, hand_size);

    // The number of possible hands with the given split is the product of the number of
    // combinations of cards of the given type, the number of cards not of the given type, and the
    // number of ways to draw each of those combinations in a given hand. The last of those
    // operands can be calculated by counting the number of orderings of a given hand. For example,
    // using `T` to represent cards of the given type and `N` to represent cards not of the given
    // type, the possible ways to draw 3 cards of the type and two cards not of the type in a hand
    // of five cards are:
    //
    //    1. T, T, T, N, N
    //    2. T, T, N, T, N
    //    3. T, T, N, N, T
    //    4. T, N, T, T, N
    //    5. T, N, T, N, T
    //    6. T, N, N, T, T
    //    7. N, T, T, T, N
    //    8. N, T, T, N, T
    //    9. N, T, N, T, T
    //   10. N, N, T, T, T
    //
    // This is equivalent to choosing 3 arbitrary places in the list of 5 cards to contain T, which
    // means we can calculate it with `n_choose_k`.
    let number_of_possible_hands_with_split = n_choose_k(hand_size, hand_count_of_type)
        * combinations_of_given_type
        * combinations_of_other_type;

    // Putting that all together, the chance of drawing a given split is the number of possible
    // hands with that split divided by the total number of hands. Multiplying the
    Fraction::new(
        number_of_possible_hands_with_split,
        total_number_of_possible_hands,
    )
}

// Helper function to calculate the chance of a number of splits in bulk.
pub fn chance_of_splits(hand_size: u128, hand_counts: &[u128], deck_count: u128) -> Vec<Fraction> {
    hand_counts
        .into_iter()
        .map(|c| chance_of_split(hand_size, *c, deck_count))
        .collect()
}
