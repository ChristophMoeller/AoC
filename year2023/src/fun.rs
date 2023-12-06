use crate::utils::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match *self {
            Cell::Dead => '.',
            Cell::Alive => 'x',
        };
        write!(f, "{}", c)
    }
}

pub struct GameOfLife {
    board: Grid<Cell>,
}

impl std::fmt::Display for GameOfLife {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board)
    }
}

impl GameOfLife {
    pub fn new(board: &str) -> Self {
        let mut board = Grid::parse(board, |c| match c {
            '.' | ' ' => Cell::Dead,
            'x' => Cell::Alive,
            _ => unimplemented!(),
        });
        board.set_wrapping(true);
        Self { board }
    }

    pub fn step(&mut self) {
        let mut result = Vec::with_capacity(self.board.height() as usize);

        for y in 0..self.board.height() {
            let mut row = Vec::with_capacity(self.board.width() as usize);
            for x in 0..self.board.width() {
                let count = self
                    .board
                    .neighbors8(x, y)
                    .filter(|(_, &x)| x == Cell::Alive)
                    .count();

                let cell = match count {
                    ..=1 => Cell::Dead,
                    2 => self.board[(x, y)],
                    3 => Cell::Alive,
                    4.. => Cell::Dead,
                    _ => unreachable!(),
                };
                row.push(cell);
            }
            result.push(row)
        }

        let mut board: Grid<Cell> = result.into();

        board.set_wrapping(true);
        self.board = board;
    }
}
