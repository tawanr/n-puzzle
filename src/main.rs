use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;

type BoardState = Vec<Vec<i32>>;

#[derive(Debug)]
struct Pos(usize, usize);

#[derive(Debug)]
struct Board {
    state: BoardState,
    misplaced: i32,
    iteration: i32,
    distance: i32,
    prev: Option<Pos>,
    current: Pos,
    history: Vec<BoardState>,
}

impl Board {
    fn calculate(&mut self, goal: &BoardState) {
        self.iteration += 1;
        self.misplaced = 0;
        let mut empty_x = 0;
        let mut empty_y = 0;
        for (r_idx, row) in goal.iter().enumerate() {
            for (c_idx, col) in row.iter().enumerate() {
                if *col != self.state[r_idx][c_idx] {
                    self.misplaced += 1;
                }
                if self.state[r_idx][c_idx] == 0 {
                    empty_x = c_idx;
                    empty_y = r_idx;
                }
            }
        }
        self.history.push(self.state.clone());
        self.distance = self.misplaced;
        self.current = Pos(empty_x, empty_y);
    }

    fn get_string(&self) -> String {
        let mut string = String::new();
        for row in &self.state {
            for col in row {
                string += &(*col).to_string()[..];
            }
        }
        return string;
    }

    fn explore(&self, explored: &Vec<String>) -> Vec<Board> {
        let x = self.current.0;
        let y = self.current.1;
        let mut iterations: Vec<Board> = Vec::new();
        if x > 0 && (self.prev.is_none() || x - 1 != self.prev.as_ref().unwrap().0) {
            let mut new_state = self.state.clone();
            let value = new_state[y][x - 1];
            new_state[y][x] = value;
            new_state[y][x - 1] = 0;
            let new_board = Board {
                state: new_state,
                prev: Some(Pos(x, y)),
                current: Pos(x - 1, y),
                history: self.history.clone(),
                ..*self
            };
            let string = new_board.get_string();
            if !explored.iter().any(|s| string.contains(s)) {
                iterations.push(new_board);
            }
        }
        if x < self.state.len() - 1
            && (self.prev.is_none() || (x + 1 != self.prev.as_ref().unwrap().0))
        {
            let mut new_state = self.state.clone();
            let value = new_state[y][x + 1];
            new_state[y][x] = value;
            new_state[y][x + 1] = 0;
            let new_board = Board {
                state: new_state,
                prev: Some(Pos(x, y)),
                current: Pos(x + 1, y),
                history: self.history.clone(),
                ..*self
            };
            let string = new_board.get_string();
            if !explored.iter().any(|s| string.contains(s)) {
                iterations.push(new_board);
            }
        }
        if y > 0 && (self.prev.is_none() || (y - 1 != self.prev.as_ref().unwrap().1)) {
            let mut new_state = self.state.clone();
            let value = new_state[y - 1][x];
            new_state[y][x] = value;
            new_state[y - 1][x] = 0;
            let new_board = Board {
                state: new_state,
                prev: Some(Pos(x, y)),
                current: Pos(x, y - 1),
                history: self.history.clone(),
                ..*self
            };
            let string = new_board.get_string();
            if !explored.iter().any(|s| string.contains(s)) {
                iterations.push(new_board);
            }
        }
        if y < self.state.len() - 1
            && (self.prev.is_none() || (y + 1 != self.prev.as_ref().unwrap().1))
        {
            let mut new_state = self.state.clone();
            let value = new_state[y + 1][x];
            new_state[y][x] = value;
            new_state[y + 1][x] = 0;
            let new_board = Board {
                state: new_state,
                prev: Some(Pos(x, y)),
                current: Pos(x, y + 1),
                history: self.history.clone(),
                ..*self
            };
            let string = new_board.get_string();
            if !explored.iter().any(|s| string.contains(s)) {
                iterations.push(new_board);
            }
        }
        iterations
    }

    fn show_history(&self) {
        for history in self.history.as_ref() as &Vec<BoardState> {
            for row in history {
                for col in row {
                    print!("{} ", *col);
                }
                print!("\n");
            }
            print!("\n");
        }
    }
}

// fn get_goal(size: usize) {
//     let limit = size * size;
//     let mut x_offset: usize = 0;
//     let mut y_offset: usize = 0;
//     let mut x_dir = 1;
//     let mut y_dir = 1;
//     let goal: BoardState = vec![vec![0; size]; size];
//     let mut count = 1;

//     while count < size * size {

//     }
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid argument(s).");
    }
    println!("Input file: {:?}", args[1]);
    let path = Path::new(&args[1]);
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("couldn't read file");
    let mut lines = content.lines();
    let size: usize = lines.next().unwrap().parse().unwrap();
    let mut explored: Vec<String> = Vec::new();

    let mut game_state: BoardState = BoardState::new();
    for (_, line) in lines.enumerate() {
        let mut row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        if row.len() != size {
            panic!("Invalid input format.");
        }
        game_state.push(row);
    }

    let mut board = Board {
        state: game_state,
        misplaced: 0,
        iteration: 0,
        distance: 0,
        prev: None,
        current: Pos(0, 0),
        history: Vec::new(),
    };

    let goal = vec![vec![1, 2, 3], vec![8, 0, 4], vec![7, 6, 5]];
    // let goal = vec![
    //     vec![1, 2, 3, 4],
    //     vec![12, 13, 14, 5],
    //     vec![11, 0, 15, 6],
    //     vec![10, 9, 8, 7],
    // ];
    board.calculate(&goal);
    let mut boards: Vec<Board> = Vec::new();
    boards.push(board);
    while boards.len() > 0 {
        let board = match boards.first() {
            Some(brd) => brd,
            _ => break,
        };
        println!("iteration: {} - {}", board.iteration, board.distance);
        if board.misplaced == 0 {
            println!("final iteration: {}", board.iteration);
            board.show_history();
            break;
        }
        // if board.iteration > 20 {
        //     println!("breaking");
        //     break;
        // }

        let mut iterations = board.explore(&explored);
        for iteration in &mut iterations {
            iteration.calculate(&goal);
            explored.push(iteration.get_string());
        }
        boards.remove(0);
        boards.extend(iterations);
        boards.sort_by(|a, b| a.distance.cmp(&b.distance));
        println!("explored: {}", explored.len());
        // break;
    }
    // println!("{:?}", explored);
    // for board in boards {
    //     println!("{:?}", board);
    // }
}
