use rand::Rng;

#[derive(Clone, Debug)]
pub struct Case {
    pub color: u8,
}

pub struct Node {
    pub color: u8,
    neighbors: Option<Vec<Box<Node>>>,
}

pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cases: Vec<Vec<Case>>,
}

impl Board {
    pub fn generate_graph(self) -> Vec<Node> {
        let mut out = Vec::new();
        for i in 0..self.height {
            for j in 0..self.width {
                // First case
                if i == 0 && j == 0 {
                    out.push(Node {
                        color: self.cases[i][j].color,
                        neighbors: None,
                    });
                }
                // Above neighbor
                else if false {
                }
                // Left neighbor
                else if true {
                }
            }
        }
        todo!()
    }
}

pub struct Game {
    pub board: Board,
    pub difficulty: u8,
    pub graph: Vec<Node>,
}

impl Game {
    pub fn new(width: usize, height: usize, difficulty: u8) -> Self {
        let mut cases = vec![vec![Case { color: 0u8 }; width]; height];

        for i in 0..height {
            for j in 0..width {
                cases[i][j] = Case {
                    color: rand::thread_rng().gen_range(0..difficulty),
                };
            }
        }

        let board = Board {
            width,
            height,
            cases,
        };

        Game {
            board,
            difficulty,
            graph: Vec::new(),
        }
    }
}
