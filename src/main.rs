use rand::Rng;


#[derive(Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors
}

enum GameResult {
    Won,
    Lose,
    Tie
}

impl Hand {
    const ROCK: &'static str = 
r"
  ╭────╮  
 ╭┘╭╯  └╮ 
 ╰──────╯ 
   ROCK   ";
    const PAPER: &'static str = 
r"
  ┌───╖  
  │   ║  
  ╘═══╝  
  PAPER  ";
    const SCISSORS: &'static str = 
r"
   ╲ ╱    
    ╳     
   O O    
 SCISSORS ";


    fn from_str(input: &str) -> Result<Self, String> {
        let input = input.trim().to_lowercase();
        match &*input {
            "1" | "r" => Ok(Hand::Rock),
            "2" | "p" => Ok(Hand::Paper),
            "3" | "s" => Ok(Hand::Scissors),
            err => Err(format!("Error: \"{err}\" is not a valid input"))
        }
    }
    fn won_against<'a>(&'a self, other: &'a Hand) -> (GameResult, &Hand) {
        match (self, other) {
            (Hand::Rock    , Hand::Scissors) |
            (Hand::Paper   , Hand::Rock    ) |
            (Hand::Scissors, Hand::Paper   ) => (GameResult::Won, &self),
            (Hand::Rock    , Hand::Paper   ) |
            (Hand::Paper   , Hand::Scissors) |
            (Hand::Scissors, Hand::Rock    ) => (GameResult::Lose, &other),
            _ => (GameResult::Tie, &self)
        }
    }
    fn to_string(&self) -> &'static str {
        match self {
            Hand::Rock => Hand::ROCK,
            Hand::Paper => Hand::PAPER,
            Hand::Scissors => Hand::SCISSORS,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Hand::Rock => "ROCK",
            Hand::Paper => "PAPER",
            Hand::Scissors => "SCISSORS",
        }
    }
}

fn main() {
    println!("Hello, world!");
    println!("\n ROCK-PAPER-SCISSORS\n");
    let title = concat_blocks(Hand::ROCK, Hand::PAPER);
    let title = concat_blocks(&title, Hand::SCISSORS);
    println!("{}\n", title);

    loop {
        let hand = get_player_hand();
        let other = get_comp_hand();
        println!("{}", concat_blocks(hand.to_string(), other.to_string()));
        let result = match hand.won_against(&other) {
            (GameResult::Won, _) => format!("{} WON!! :)", hand.name()),
            (GameResult::Lose, _) => format!("{} LOSE. :(", hand.name()),
            (GameResult::Tie, _) => format!("TIE..."),
        };
        println!("{result}\n");
    }
}

fn concat_blocks(block_1: &str, block_2: &str) -> String {
    let mut output = String::new();
    let line_1 = block_1.lines();
    let line_2 = block_2.lines();
    for line in line_1.zip(line_2) {
        output += &*(line.0.to_owned() + line.1 + "\n");
    }
    output
}

fn get_player_hand() -> Hand {
    let mut input = String::new();
    loop {
        println!("Pick: 1-ROCK, 2-PAPER, 3-SCISSORS");
        std::io::stdin().read_line(&mut input)
            .expect("Error: failed to read input");
        match Hand::from_str(&input) {
            Ok(hand) => return hand,
            Err(msg) => {
                println!("{msg}");
                input = String::new();
                continue;
            },
        }
    }
}

fn get_comp_hand() -> Hand {
    let mut rng = rand::thread_rng();
    let rand_num: f64 = rng.gen();
    if rand_num < 0.33 {
        Hand::Rock
    } else if rand_num < 0.67 {
        Hand::Paper
    } else {
        Hand::Scissors
    }
}