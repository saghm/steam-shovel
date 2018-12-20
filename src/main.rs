mod math;

use std::ops::Add;

use structopt::StructOpt;

use crate::math::Fraction;

const DEFAULT_HAND_SIZE: u8 = 7;

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ArgRequiredElseHelp"))]
struct Opt {
    /// The number of lands in the deck.
    #[structopt(long = "num-lands", short = "l")]
    num_lands: u8,
    /// The size of the hand being drawn.
    #[structopt(long = "hand-size", short = "s")]
    hand_size: Option<u8>,
    /// The number of lands in the hand to print the percentages for.
    ///
    /// May be either a comma-separated list of integers or the word 'all' (e.g. '1', '2,3,4',
    /// 'all').
    #[structopt(long = "counts", short = "c")]
    counts: String,
}

fn format_land_counts(counts: &[u8]) -> String {
    // `counts` will not be empty.

    if counts.len() == 1 {
        return format!("{}", counts[0]);
    }

    if counts.len() == 2 {
        return format!("{} or {}", counts[0], counts[1]);
    }

    format!(
        "{}or {}",
        &counts[..counts.len() - 1]
            .into_iter()
            .map(|c| format!("{}, ", c))
            .collect::<String>(),
        counts[counts.len() - 1]
    )
}

fn main() {
    let opt = Opt::from_args();

    let hand_size = opt.hand_size.unwrap_or(DEFAULT_HAND_SIZE);

    let counts: Vec<_> = if opt.counts == "all" {
        (0..=hand_size).into_iter().collect()
    } else {
        opt.counts
            .split(',')
            .map(|c| {
                u8::from_str_radix(c, 10).unwrap_or_else(|_| {
                    eprintln!(
                        "error: --count must be either 'all' or a comma-separated list of integers"
                    );
                    std::process::exit(1);
                })
            })
            .collect()
    };

    let results = math::chance_of_splits(
        hand_size.into(),
        &counts.iter().cloned().map(Into::into).collect::<Vec<_>>(),
        opt.num_lands.into(),
    );

    macro_rules! print_chance {
        ($count:expr, $hand_size:expr, $num_lands:expr, $chance:expr) => {
            println!(
                "chance of {} lands in a {} card hand from a deck with {} lands: {:>7.3}%",
                $count, $hand_size, $num_lands, $chance
            )
        };
    }

    for (i, result) in results.iter().enumerate() {
        print_chance!(counts[i], hand_size, opt.num_lands, result.eval() * 100.0);
    }

    println!("----------------------------------------------------------------------");

    print_chance!(
        format_land_counts(&counts),
        hand_size,
        opt.num_lands,
        results.iter().map(Fraction::eval).fold(0.0, Add::add) * 100.0
    );
}
