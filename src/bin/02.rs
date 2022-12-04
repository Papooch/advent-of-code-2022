use aoc2022::file_io::lines_in_file;
use RPS::*;

#[derive(Clone, Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn score_against(&self, other: &RPS) -> u32 {
        match self {
            Rock => match other {
                Rock => 3,
                Paper => 0,
                Scissors => 6,
            },
            Paper => match other {
                Rock => 6,
                Paper => 3,
                Scissors => 0,
            },
            Scissors => match other {
                Rock => 0,
                Paper => 6,
                Scissors => 3,
            },
        }
    }

    fn get_winning(&self) -> RPS {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn get_losing(&self) -> RPS {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn get_draw(&self) -> RPS {
        self.clone()
    }
}

fn create_rps(symbol: &str) -> RPS {
    match symbol {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => panic!("Invalid RPS"),
    }
}

fn create_opposing_rps(symbol: &str, rps: &RPS) -> RPS {
    match symbol {
        "X" => rps.get_losing(),
        "Y" => rps.get_draw(),
        "Z" => rps.get_winning(),
        _ => panic!("Invalid RPS"),
    }
}

fn main() {
    let mut total = 0;

    for line in lines_in_file("src/inputs/02.input") {
        let mut round_score = 0;
        let line = line.unwrap();
        let (first_symbol, second_symbol) = line.split_once(" ").unwrap();
        let first = create_rps(first_symbol);
        let second = create_opposing_rps(second_symbol, &first);
        round_score += match second {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };
        round_score += second.score_against(&first);
        println!("round: {:?} vs {:?} = {}", first, second, round_score);
        total += round_score;
    }
    println!("total: {}", total)
}
