const DECK_SIZE: u128 = 60;

fn mult_range(begin: u128, end: u128) -> u128 {
    (begin..=end).into_iter().fold(1, std::ops::Mul::mul)
}

fn mult_range_down_by(begin: u128, down: u128) -> u128 {
    if down == 0 {
        return 1;
    }

    mult_range(begin - down + 1, begin)
}

fn n_choose_k(n: u128, k: u128) -> u128 {
    mult_range(n - k + 1, n) / mult_range(1, k)
}

fn chance_of_split(
    hand_size: u128,
    hand_count_of_type: u128,
    deck_count_of_type: u128,
) -> (u128, u128) {
    let hand_count_of_other = hand_size - hand_count_of_type;
    let deck_count_of_other = DECK_SIZE - deck_count_of_type;

    let n1 = n_choose_k(hand_size, hand_count_of_type);
    let n2 = mult_range_down_by(deck_count_of_type, hand_count_of_type);
    let n3 = mult_range_down_by(deck_count_of_other, hand_count_of_other);
    let d = mult_range_down_by(DECK_SIZE, hand_size);

    (n1 * n2 * n3, d)
}

pub fn chance_of_splits(hand_size: u128, hand_counts: &[u128], deck_count: u128) -> f64 {
    hand_counts
        .into_iter()
        .map(|n| divide(chance_of_split(hand_size, *n, deck_count)))
        .fold(0.0, std::ops::Add::add)
}

fn divide((n, d): (u128, u128)) -> f64 {
    n as f64 / d as f64
}
