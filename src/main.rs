mod math;

fn main() {
    println!(
        "chance of getting 2-3 lands on a six card mulligan with 20 lands in the deck: {:.3}%",
        math::chance_of_splits(6, &[2, 3], 20) * 100.0
    );
    println!(
        "chance of getting 1-4 lands on a six card mulligan with 20 lands in the deck: {:.3}%",
        math::chance_of_splits(6, &[1, 2, 3, 4], 20) * 100.0
    );
}
