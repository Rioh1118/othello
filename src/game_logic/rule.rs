use crate::game_logic::board::{Board, Position};
use crate::game_logic::player::Player;
use crate::error::OthelloError;
use crate::game_logic::game::Game;

pub trait Rule {
    fn valid_moves(board: &Board, player: Player) -> Vec<Position>;
    fn apply_move(board: &Board, player: Player, position: Position)-> Result<(),OthelloError>;
    fn determine_winner(board: &Board) -> Option<Player>;

    fn is_valid_move(board: &Board, player: Player, position: Position) -> bool {
        let valid_moves = Self::valid_moves(board, player);
        valid_moves.contains(&position)
    }
}

impl Rule for Game {
    fn valid_moves(board: &Board, player: Player) -> Vec<Position> {
        //
        let mut valid_moves: Vec<Position>;
        let empty_positions = board.empty_cells();

        valid_moves = empty_positions.into_iter().filter(|&pos| is_adjacent_opponent(board, pos, player)).collect();
        valid_moves = valid_moves.into_iter().filter(|&pos| can_flip(board, pos, player)).collect();
        valid_moves
    }
    fn apply_move(board: &Board,player: Player ,position: Position) -> Result<(),OthelloError>{
        board.put_piece(position, Some(player))?;
        let search_dir = vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right, Direction::UpLeft, Direction::UpRight, Direction::DownLeft, Direction::DownRight];

        for dir in search_dir {
            search_and_flip(board, position, dir, player)?;
        }
        Ok(())
    }

    fn determine_winner(board: &Board) -> Option<Player> {
        let (black, white) = board.count();
        if black > white {
            Some(Player::Black)
        } else if black < white {
            Some(Player::White)
        } else {
            None
        }

    }
}

fn can_flip(board: &Board, position: Position, player: Player) -> bool {
    let search_dir = vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right, Direction::UpLeft, Direction::UpRight, Direction::DownLeft, Direction::DownRight];
    for dir in search_dir {
        if can_flip_by_dir(board, position, player, dir) {
            return true;
        }
    }
    false
}
fn can_flip_by_dir(board: &Board, position: Position ,player: Player,dir: Direction) -> bool {
    match dir{
        Direction::Up => {
            let mut flag = false;
            let mut next_position = position.up();
            while let Some(pos) = next_position {
                match board.get_piece(pos) {
                    Some(p) => {
                        if p == player {
                            break;
                        }
                        flag = true;
                        next_position = pos.up();
                    },
                    None => {break;}
                }
            }
            flag
        },
        Direction::Down => {
            let mut flag = false;
            let mut next_position = position.down();
            while let Some(pos) = next_position {
                match board.get_piece(pos) {
                    Some(p) => {
                        if p == player {
                            break;
                        }
                        flag = true;
                        next_position = pos.down();
                    },
                    None => {break;}
                }
            }
            flag
        },
        Direction::Left => {
            let mut flag = false;
            let mut next_position = position.left();
            while let Some(pos) = next_position {
                match board.get_piece(pos) {
                    Some(p) => {
                        if p == player {
                            break;
                        }
                        flag = true;
                        next_position = pos.left();
                    },
                    None => {break;}
                }
            }
            flag
        },
        Direction::Right => {
            let mut flag = false;
            let mut next_position = position.right();
            while let Some(pos) = next_position {
                match board.get_piece(pos) {
                    Some(p) => {
                        if p == player {
                            break;
                        }
                        flag = true;
                        next_position = pos.right();
                    },
                    None => {break;}
                }
            }
            flag
        },
        Direction::UpLeft => {
            let mut flag = false;
            let mut next_position = position.up_left();
            while let Some(pos) = next_position {
                match board.get_piece(pos) {
                    Some(p) => {
                        if p == player {
                            break;
                        }
                        flag = true;
                        next_position = pos.up_left();
                    },
                    None => {break;}
                }
            }
            flag
        },
        Direction::UpRight => {
            let mut flag = false;
            let mut next_position = position.up_right();
            while let Some(pos) = next_position {
                match board.get_piece(pos) {
                    Some(p) => {
                        if p == player {
                            break;
                        }
                        flag = true;
                        next_position = pos.up_right();
                    },
                    None => {break;}
                }
            }
            flag
        },
        Direction::DownLeft => {
            let mut flag = false;
            let mut next_position = position.down_left();
            while let Some(pos) = next_position {
                match board.get_piece(pos) {
                    Some(p) => {
                        if p == player {
                            break;
                        }
                        flag = true;
                        next_position = pos.down_left();
                    },
                    None => {break;}
                }
            }
            flag
        },
        Direction::DownRight => {
            let mut flag = false;
            let mut next_position = position.down_right();
            while let Some(pos) = next_position {
                match board.get_piece(pos) {
                    Some(p) => {
                        if p == player {
                            break;
                        }
                        flag = true;
                        next_position = pos.down_right();
                    },
                    None => {break;}
                }
            }
            flag
        },
    }
}
fn is_adjacent_opponent(board: &Board, position: Position, player: Player) -> bool {
    let up_flag = position.up().map_or(false,|pos| board.get_piece(pos) == Some(player.opponent()));
    let down_flag = position.down().map_or(false,|pos| board.get_piece(pos) == Some(player.opponent()));
    let left_flag = position.left().map_or(false,|pos| board.get_piece(pos) == Some(player.opponent()));
    let right_flag = position.right().map_or(false,|pos| board.get_piece(pos) == Some(player.opponent()));
    let up_left_flag = position.up_left().map_or(false,|pos| board.get_piece(pos) == Some(player.opponent()));
    let up_right_flag = position.up_right().map_or(false,|pos| board.get_piece(pos) == Some(player.opponent()));
    let down_left_flag = position.down_left().map_or(false,|pos| board.get_piece(pos) == Some(player.opponent()));
    let down_right_flag = position.down_right().map_or(false,|pos| board.get_piece(pos) == Some(player.opponent()));

    up_flag || down_flag || left_flag || right_flag || up_left_flag || up_right_flag || down_left_flag || down_right_flag
}
fn search_and_flip(board: &Board, start_position: Position, direction: Direction, player: Player) -> Result<(),OthelloError>{
    let mut positions_to_flip: Vec<Position> = Vec::new();

    match direction {
        Direction::Up => {
            let mut next_position = start_position.up();
            while let Some(pos) = next_position {
                match board.get_piece(pos) {
                    Some(piece) => {
                        if piece != player {
                            positions_to_flip.push(pos);
                            next_position = pos.up();
                        } else {
                            for pos in &positions_to_flip {
                                board.put_piece(*pos, Some(player))?;
                            }
                        }
                    },
                    None => break,
                }
            }
        },
        Direction::Down => {
            let mut next_position = start_position.down();
            while let Some(pos) = next_position {
                match board.get_piece(pos) {
                    Some(piece) => {
                        if piece != player {
                            positions_to_flip.push(pos);
                            next_position = pos.down();
                        } else {
                            for pos in &positions_to_flip {
                                board.put_piece(*pos, Some(player))?;
                            }
                        }
                    },
                    None => break,
                }
            }
        },
        Direction::Left => {
            let mut next_position = start_position.left();
            while let Some(pos) = next_position {
                match board.get_piece(pos) {
                    Some(piece) => {
                        if piece != player {
                            positions_to_flip.push(pos);
                            next_position = pos.left();
                        } else {
                            for pos in &positions_to_flip {
                                board.put_piece(*pos, Some(player))?;
                            }
                        }
                    },
                    None => break,
                }
            }
        },
        Direction::Right => {
            let mut next_position = start_position.right();
            while let Some(pos) = next_position {
                match board.get_piece(pos) {
                    Some(piece) => {
                        if piece != player {
                            positions_to_flip.push(pos);
                            next_position = pos.right();
                        } else {
                            for pos in &positions_to_flip {
                                board.put_piece(*pos, Some(player))?;
                            }
                        }
                    },
                    None => break,
                }
            }
        },
        Direction::UpLeft => {
            let mut next_position = start_position.up_left();
            while let Some(pos) = next_position {
                match board.get_piece(pos) {
                    Some(piece) => {
                        if piece != player {
                            positions_to_flip.push(pos);
                            next_position = pos.up_left();
                        } else {
                            for pos in &positions_to_flip {
                                board.put_piece(*pos, Some(player))?;
                            }
                        }
                    },
                    None => break,
                }
            }
        },
        Direction::UpRight => {
            let mut next_position = start_position.up_right();
            while let Some(pos) = next_position {
                match board.get_piece(pos) {
                    Some(piece) => {
                        if piece != player {
                            positions_to_flip.push(pos);
                            next_position = pos.up_right();
                        } else {
                            for pos in &positions_to_flip {
                                board.put_piece(*pos, Some(player))?;
                            }
                        }
                    },
                    None => break,
                }
            }
        },
        Direction::DownLeft => {
            let mut next_position = start_position.down_left();
            while let Some(pos) = next_position {
                match board.get_piece(pos) {
                    Some(piece) => {
                        if piece != player {
                            positions_to_flip.push(pos);
                            next_position = pos.down_left();
                        } else {
                            for pos in &positions_to_flip {
                                board.put_piece(*pos, Some(player))?;
                            }
                        }
                    },
                    None => break,
                }
            }
        },
        Direction::DownRight => {
            let mut next_position = start_position.down_right();
            while let Some(pos) = next_position {
                match board.get_piece(pos) {
                    Some(piece) => {
                        if piece != player {
                            positions_to_flip.push(pos);
                            next_position = pos.down_right();
                        } else {
                            for pos in &positions_to_flip {
                                board.put_piece(*pos, Some(player))?;
                            }
                        }
                    },
                    None => break,
                }
            }
        },

    }
    Ok(())
}



enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}