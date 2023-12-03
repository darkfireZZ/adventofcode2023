use std::{fs::File, io::Read, iter::Iterator};

fn main() -> Result<(), String> {
    let input_file_path = parse_args()?;
    let mut input_file = File::open(input_file_path).map_err(|err| err.to_string())?;

    let input = {
        let mut buffer = String::new();
        input_file.read_to_string(&mut buffer).map_err(|err| err.to_string())?;
        buffer
    };

    let sum: u32 = parse_games(&input).filter(|game| {
        game.sets.iter().all(|set| {
            (set.red <= 12) & (set.green <= 13) & (set.blue <= 14)
        })
    }).map(|game| game.id).sum();

    println!("sum of IDs of possible games: {}", sum);

    Ok(())
}

struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    fn empty() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

fn parse_games(input: &str) -> impl Iterator<Item = Game> + '_ {
    input.lines().map(|line| {
        let line = line.strip_prefix("Game ").expect("every line starts with this prefix");
        let (id, sets) = line.split_once(':').expect("every line contains a colon");

        let id = u32::from_str_radix(id, 10).expect("id is always a valid number");
        let sets = sets.split(';').map(|set| {
            let mut parsed_set = CubeSet::empty();
            for entry in set.split(',') {
                let entry = entry.trim();
                let (count, color) = entry.split_once(' ').expect("entries are always in the format \"count color\"");
                let count = u32::from_str_radix(count, 10).expect("count is always a valid number");
                match color {
                    "red" => parsed_set.red += count,
                    "green" => parsed_set.green += count,
                    "blue" => parsed_set.blue += count,
                    _ => panic!("Unknown color: {}", color),
                }
            }
            parsed_set
        }).collect();

        Game { id, sets }
    })
}

fn parse_args() -> Result<String, String> {
    let mut args = std::env::args();

    if args.len() == 2 {
        Ok(args.next_back().expect("there are 2 arguments"))
    } else {
        Err(format!("expected 2 arguments, got {}", args.len()))
    }
}
