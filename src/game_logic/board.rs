/// 8x8マスのオセロ盤を表す構造体
use std::cell::RefCell;
use crate::error;
use crate::game_logic::player::Player;

pub type Cell = Option<Player>;


pub struct Board {
    board: Vec<Vec<RefCell<Cell>>>
}

impl Board {
    pub fn new() -> Self {
        let board: Vec<Vec<RefCell<Cell>>> =
            (0..8).map(|_| (0..8).map(|_| RefCell::new(None)).collect()).collect();
        Board { board }
    }

    pub fn count(&self) -> (u8,u8) {
        // (Black, White)
        let mut black: u8 = 0;
        let mut white: u8 = 0;

        for x in 0..8 {
            for y in 0..8 {
                let borrow_cell = self.board[x][y].borrow();
                match *borrow_cell {
                    Some(Player::Black) => black += 1,
                    Some(Player::White) => white += 1,
                    None => (),
                }
            }
        }
        (black,white)
    }
    pub fn put_piece(&self, position: Position, cell: Cell) -> Result<(), error::OthelloError> {
        let mut borrow_cell = self.board[position.0 as usize][position.1 as usize].borrow_mut();
        *borrow_cell = cell;
        Ok(())
    }

    pub fn empty_cells(&self) -> Vec<Position> {
        let mut empty_cells: Vec<Position> = Vec::new();

        for x in 0..8 {
            for y in 0..8 {
                let borrow_cell = self.board[x][y].borrow();
                if *borrow_cell == None {
                    empty_cells.push(Position(x as u8, y as u8))
                }
            }
        }
        empty_cells
    }
    pub fn get_piece(&self, position: Position) -> Cell {
        let borrow_cell = self.board[position.0 as usize][position.1 as usize].borrow();
        *borrow_cell
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Position(u8,u8);

impl Position {
    pub fn new(x: u8, y: u8) -> Result<Self, error::OthelloError> {
        if x > 7 || y > 7 {
            return Err(error::OthelloError::OutOfRange(x, y));
        }
        Ok(Self(x,y))
    }

    pub fn up(&self) -> Option<Self> {
        let up = Self::new(self.0,self.1-1);
        match up {
            Ok(position) => Some(position),
            Err(_) => None,
        }
    }
    pub fn down(&self) -> Option<Self> {
        let down = Self::new(self.0,self.1+1);
        match down {
            Ok(position) => Some(position),
            Err(_) => None,
        }
    }
    pub fn right(&self) -> Option<Self> {
        let right = Self::new(self.0+1,self.1);
        match right {
            Ok(position) => Some(position),
            Err(_) => None,
        }
    }
    pub fn left(&self) -> Option<Self> {
        let left = Self::new(self.0-1,self.1);
        match left {
            Ok(position) => Some(position),
            Err(_) => None,
        }
    }
    pub fn up_right(&self) -> Option<Self> {
        let up_right = Self::new(self.0+1,self.1-1);
        match up_right {
            Ok(position) => Some(position),
            Err(_) => None,
        }
    }
    pub fn up_left(&self) -> Option<Self> {
        let up_left = Self::new(self.0-1,self.1-1);
        match up_left {
            Ok(position) => Some(position),
            Err(_) => None,
        }
    }

    pub fn down_right(&self) -> Option<Self> {
        let down_right = Self::new(self.0+1,self.1+1);
        match down_right {
            Ok(position) => Some(position),
            Err(_) => None,
        }
    }
    pub fn down_left(&self) -> Option<Self> {
        let down_left = Self::new(self.0-1,self.1+1);
        match down_left {
            Ok(position) => Some(position),
            Err(_) => None,
        }
    }
}