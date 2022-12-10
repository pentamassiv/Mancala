#![allow(clippy::similar_names)]
#![allow(clippy::cast_sign_loss)]

const STARTING_MARBLES: i8 = 4;
const NO_OF_HOLES_OF_EACH_PLAYER: usize = 6;

// Calculated constants
const START_IDX_PLAYER_A: usize = 0;
const START_IDX_PLAYER_B: usize = NO_OF_HOLES_OF_EACH_PLAYER + 1;
const END_IDX_PLAYER_A: usize = MANCALA_IDX_PLAYER_A - 1; // Needed because pattern matching on exclusive ranges is experimental
const END_IDX_PLAYER_B: usize = MANCALA_IDX_PLAYER_B - 1; // Needed because pattern matching on exclusive ranges is experimental
const MANCALA_IDX_PLAYER_A: usize = NO_OF_HOLES_OF_EACH_PLAYER;
const MANCALA_IDX_PLAYER_B: usize = 2 * NO_OF_HOLES_OF_EACH_PLAYER + 1;

/// There can be two players
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Player {
    A,
    B,
}

/// A dip in a mancala board that can contain a number of marbles
#[derive(Debug, Copy, Clone)]
struct Hole {
    marbles: i8,
}

impl Hole {
    /// Adds x marbles to the hole
    fn add_x(&mut self, x: i8) {
        self.marbles += x;
    }
    /// Takes all marbles from the hole and returns their number
    fn take_all(&mut self) -> i8 {
        let marbles = self.marbles;
        self.marbles = 0;
        marbles
    }
    /// Returns the number of marbles in the hole
    fn count(self) -> i8 {
        self.marbles
    }
}

/// A mancala board with all its holes and mancalas to count the players points
#[derive(Debug, Copy, Clone)]
pub struct Board {
    holes: [Hole; NO_OF_HOLES_OF_EACH_PLAYER * 2 + 2],
}

impl Board {
    /// Create, initialize and return a new mancala board
    #[must_use]
    pub fn new() -> Self {
        let init_hole = Hole {
            marbles: STARTING_MARBLES,
        };
        let mut holes = [init_hole; NO_OF_HOLES_OF_EACH_PLAYER * 2 + 2];
        // Empty the mancalas of the players
        holes[NO_OF_HOLES_OF_EACH_PLAYER].take_all();
        holes[NO_OF_HOLES_OF_EACH_PLAYER * 2 + 1].take_all();
        Board { holes }
    }

    /// Take all marbles from the chosen hole and add them to the following holes and the player's mancala
    /// player: Player whos turn it is
    /// no: number of the selected hole. The numbering starts with 0 on the very left hole of the player whos turn it is
    fn choose_hole(&mut self, player: Player, no: usize) -> Player {
        let (own_mancala_idx, opponent_mancala_idx, opponent) = match player {
            Player::A => (MANCALA_IDX_PLAYER_A, MANCALA_IDX_PLAYER_B, Player::B),
            Player::B => (MANCALA_IDX_PLAYER_B, MANCALA_IDX_PLAYER_A, Player::A),
        };
        let marbles_to_spend = self.holes[no].take_all() as usize;
        if marbles_to_spend == 0 {
            println!("You picked an empty hole dumb dumb. Pick again!");
            return player;
        }
        let indices = (0..(2 * NO_OF_HOLES_OF_EACH_PLAYER + 2))
            .cycle()
            .skip(no + 1)
            .filter(|&v| v != opponent_mancala_idx)
            .take(marbles_to_spend);

        let mut last_idx = 0; // Needs to be initialized for the compiler, but it should always be overwritten if the hole contained at least one marble
        for i in indices {
            self.holes[i].add_x(1);
            last_idx = i;
        }
        // Check if the last marble landed in the players mancala
        // If it did, the player can go again
        if last_idx == own_mancala_idx {
            return player;
        }

        // If it did not land in the players mancala, check if the hole was empty. If it wasn't, chose that hole and continue
        if self.holes[last_idx].count() > 1 {
            self.choose_hole(player, last_idx)
        } else {
            opponent
        }
    }

    /// Test if the player chose one of their own holes. If they did, call the `choose_hole` function. Otherwise warn the player and let them chose again.
    pub fn player_choses_hole(&mut self, player: Player, no: usize) -> Player {
        let valid_no = match no {
            MANCALA_IDX_PLAYER_A | MANCALA_IDX_PLAYER_B => false, // Mancalas can not be chosen
            START_IDX_PLAYER_A..=END_IDX_PLAYER_A => player == Player::A, // Player A can chose any of their holes
            START_IDX_PLAYER_B..=END_IDX_PLAYER_B => player == Player::B, // Player B can chose any of their holes
            _ => false, // Any number outside the range can not be chosen
        };
        if valid_no {
            self.choose_hole(player, no)
        } else {
            println!("You can only chose one of your holes");
            player
        }
    }

    /// Test if one of the players already won
    /// Returns the winning player
    #[must_use]
    pub fn test_if_won(&self) -> Option<Player> {
        // See if one of the players already has the majority of marbles
        if self.holes[MANCALA_IDX_PLAYER_A].count() as usize
            > STARTING_MARBLES as usize * NO_OF_HOLES_OF_EACH_PLAYER
        {
            return Some(Player::A);
        }
        if self.holes[MANCALA_IDX_PLAYER_B].count() as usize
            > STARTING_MARBLES as usize * NO_OF_HOLES_OF_EACH_PLAYER
        {
            return Some(Player::B);
        }

        // Test if game is over
        let marbles_on_player_a_side = self.holes[START_IDX_PLAYER_A..=END_IDX_PLAYER_A]
            .iter()
            .fold(0, |acc, hole| acc + hole.count());
        let marbles_on_player_b_side = self.holes[START_IDX_PLAYER_B..=END_IDX_PLAYER_B]
            .iter()
            .fold(0, |acc, hole| acc + hole.count());
        if marbles_on_player_a_side == 0 || marbles_on_player_b_side == 0 {
            if marbles_on_player_a_side > marbles_on_player_b_side {
                return Some(Player::A);
            }
            return Some(Player::B);
        }
        None
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl std::fmt::Display for Board {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "|  ").unwrap();
        for idx in (START_IDX_PLAYER_B..MANCALA_IDX_PLAYER_B).rev() {
            write!(f, "|{idx:02}",).unwrap();
        }
        writeln!(f, "|  |").unwrap();

        write!(f, "|  |").unwrap();
        // START_IDX_PLAYER_B, MANCALA_IDX_PLAYER_A
        for idx in (START_IDX_PLAYER_B..MANCALA_IDX_PLAYER_B).rev() {
            write!(f, "{:02}|", self.holes[idx].count()).unwrap();
        }
        writeln!(f, "  |").unwrap();

        write!(f, "|{:02}|", self.holes[MANCALA_IDX_PLAYER_B].count()).unwrap();
        // START_IDX_PLAYER_B, MANCALA_IDX_PLAYER_A
        for _ in 0..NO_OF_HOLES_OF_EACH_PLAYER - 1 {
            write!(f, "---").unwrap();
        }
        writeln!(f, "--|{:02}|", self.holes[MANCALA_IDX_PLAYER_A].count()).unwrap();

        write!(f, "|  |").unwrap();
        // START_IDX_PLAYER_B, MANCALA_IDX_PLAYER_A
        for idx in START_IDX_PLAYER_A..MANCALA_IDX_PLAYER_A {
            write!(f, "{:02}|", self.holes[idx].count()).unwrap();
        }
        writeln!(f, "  |").unwrap();

        write!(f, "|  ").unwrap();
        for idx in START_IDX_PLAYER_A..MANCALA_IDX_PLAYER_A {
            write!(f, "|{idx:02}").unwrap();
        }
        writeln!(f, "|  |")
    }
}
