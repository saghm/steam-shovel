#[allow(dead_code)]
mod math;

use std::ops::Add;

use structopt::StructOpt;

use crate::math::Fraction;

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Opt {
    #[structopt(short = "s", long = "hand-size")]
    hand_size: Option<u8>,
    #[structopt(short = "l", long = "--lands")]
    num_lands: u8,
    #[structopt(raw(required = "true"))]
    targets: Vec<u8>,
}

fn format_land_counts(counts: &[u8]) -> String {
    // structopt/clap enforce that `counts` will not be empty.

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

    let hand_size = opt.hand_size.unwrap_or(7);

    let results = math::chance_of_splits(
        hand_size.into(),
        &opt.targets
            .iter()
            .cloned()
            .map(Into::into)
            .collect::<Vec<_>>(),
        opt.num_lands.into(),
    );

    macro_rules! print_chance {
        ($count:expr, $hand_size:expr, $num_lands:expr, $chance:expr) => {
            println!(
                "chance of {} lands in a {} card hand from a deck with {} lands: {:.3}%",
                $count, $hand_size, $num_lands, $chance
            )
        };
    }

    for (i, result) in results.iter().enumerate() {
        print_chance!(
            opt.targets[i],
            hand_size,
            opt.num_lands,
            result.eval() * 100.0
        );
    }

    println!("---------------------------------------------------------------------");

    print_chance!(
        format_land_counts(&opt.targets),
        hand_size,
        opt.num_lands,
        results.iter().map(Fraction::eval).fold(0.0, Add::add) * 100.0
    );
}
