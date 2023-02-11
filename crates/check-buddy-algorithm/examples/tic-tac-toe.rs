use rand::Rng;

use std::fmt::{Debug, Formatter};
use std::io::BufRead;
use std::process::exit;

const PLAYER_CHAR: char = 'x';
const OPPONENT_CHAR: char = 'o';

fn main() {
    let mut game = Game::new();
    game.player_turn = false;
    let stdin = std::io::stdin();
    loop {
        game.print_board();
        if !game.player_turn {
            let best_move = game.find_best_move();
            game.board[best_move.row][best_move.col] = OPPONENT_CHAR;
        } else {
            let mut buffer = String::new();
            stdin.lock().read_line(&mut buffer).expect("");
            let numbers = buffer
                .split(',')
                .map(|mut x| {
                    x = x.trim();
                    x.parse::<usize>()
                })
                .filter(|x| x.is_ok())
                .map(|x| x.unwrap())
                .collect::<Vec<_>>();
            if numbers.len() < 2 {
                println!("Incorrect input!");
                continue;
            }
            let (row, col) = (numbers[0], numbers[1]);
            game.board[row][col] = PLAYER_CHAR;
        }
        game.player_turn = !game.player_turn;
    }
}

#[derive(Clone)]
struct Move {
    row: usize,
    col: usize,
}

impl Debug for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.row, self.col)
    }
}

type Board = [[char; 3]; 3];

#[derive(Clone)]
struct Game {
    board: Board,
    player_turn: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: [['_'; 3]; 3],
            player_turn: true,
        }
    }
    fn print_board(&self) {
        for row in self.board {
            let mut line = String::new();
            for item in row {
                line.push_str(&*format!("{} |", item));
            }
            line.pop();
            println!("{}", line);
        }
    }
    fn is_moves_left(&self) -> bool {
        return self.board.iter().flat_map(|x| x).any(|x| *x == '_');
    }
    fn evaluate(&self) -> i32 {
        // rows
        for row in 0..self.board.len() {
            if self.board[row][0] == self.board[row][1] && self.board[row][1] == self.board[row][2]
            {
                if self.board[row][0] == PLAYER_CHAR {
                    return 10;
                } else if self.board[row][0] == OPPONENT_CHAR {
                    return -10;
                }
            }
        }

        // columns
        for col in 0..self.board.len() {
            if self.board[0][col] == self.board[1][col] && self.board[1][col] == self.board[2][col]
            {
                if self.board[0][col] == PLAYER_CHAR {
                    return 10;
                } else if self.board[0][col] == OPPONENT_CHAR {
                    return -10;
                }
            }
        }

        // cross patterns
        if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] {
            if self.board[0][0] == PLAYER_CHAR {
                return 10;
            } else if self.board[0][0] == OPPONENT_CHAR {
                return -10;
            }
        } else if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
            if self.board[0][2] == PLAYER_CHAR {
                return 10;
            } else if self.board[0][2] == OPPONENT_CHAR {
                return -10;
            }
        }

        0
    }
    fn minimax(&mut self, depth: usize, is_max: bool) -> i32 {
        let score = self.evaluate();

        if score == 10 || score == -10 {
            return score;
        }

        if !self.is_moves_left() {
            return 0;
        }

        return if is_max {
            let mut best = -1000;
            for i in 0..self.board.len() {
                for j in 0..self.board[i].len() {
                    if self.board[i][j] == '_' {
                        self.board[i][j] = PLAYER_CHAR;
                        best = std::cmp::max(best, self.minimax(depth + 1, !is_max));
                        self.board[i][j] = '_';
                    }
                }
            }
            best
        } else {
            let mut best = 1000;
            for i in 0..self.board.len() {
                for j in 0..self.board[i].len() {
                    if self.board[i][j] == '_' {
                        self.board[i][j] = OPPONENT_CHAR;
                        best = std::cmp::min(best, self.minimax(depth + 1, !is_max));
                        self.board[i][j] = '_';
                    }
                }
            }
            best
        };
    }
    fn find_best_move(&mut self) -> Move {
        let mut best_val = -1000;
        let mut best_moves = vec![];
        for i in 0..self.board.len() {
            let row = self.board[i];
            for j in 0..self.board[i].len() {
                let item = row[j];
                if item == '_' {
                    self.board[i][j] = PLAYER_CHAR;
                    let move_val = self.minimax(0, self.player_turn);
                    self.board[i][j] = '_';

                    if move_val == best_val {
                        best_moves.push(Move { row: i, col: j });
                    } else if move_val > best_val {
                        best_moves.clear();
                        best_moves.push(Move { row: i, col: j });
                        best_val = move_val;
                    }
                }
            }
        }

        if best_val < 0 {
            println!("Game ended, thanks for playing!");
            exit(0);
        }

        println!("best move value is {}", best_val);
        println!("possible moves are {:?}", best_moves);

        let mut rand = rand::thread_rng();
        let index = rand.gen_range(0..best_moves.len());
        return best_moves[index].clone();
    }
}
