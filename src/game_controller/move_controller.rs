//use std::io::{stdin, stdout, Write};

#[derive(Debug, Clone)]
pub struct Move {
    pub to: super::diff_controller::StateDiff,
    pub by: String,
}
impl Move {
    pub fn print(&self) -> &str {
        self.by.as_str()
    }
}

#[derive(Debug, Clone)]
pub enum Minds {
    Human,
    Lollop,
    Ronnie,
    Ruth,
}
impl Minds {
    pub fn descern_mind(kind: &str) -> Minds {
        match kind {
            "ronnie" => return Minds::Ronnie,
            "lollop" => return Minds::Lollop,
            "blank" => return Minds::Lollop,
            "ruth" => return Minds::Ruth, // This will be unimplemented for a long time
            "human" => return Minds::Human,
            _ => return Minds::Human,
        }
    }

    pub fn choose(
        mind: &Minds,
        moves: &Vec<Move>,
        gs: &super::GameState,
        phase: Option<&str>,
    ) -> usize {
        match mind {
            //Minds::Human => moves[0],
            Minds::Lollop => {
                super::super::most_cards::lollop::chooser_switcher(moves, gs, phase.unwrap())
            }
            //Ronnie => moves[0],
            //Ruth => moves[0],
            _ => panic!("testing"),
        }
    }
}

// fn human_choose(moves: Vec<Move>) -> Move {
//     // for (i, a_move) in moves.iter().enumerate() {
//     //     println!("{}) {}", i, a_move.print())
//     // }

//     let mut s = String::new();
//     print!("Please enter your move choice: ");
//     let _ = stdout().flush();
//     stdin()
//         .read_line(&mut s)
//         .expect("Did not enter a correct string");

//     let i = s.parse::<usize>().unwrap_or(0);

//     moves[i].clone()
// }
