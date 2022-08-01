#[macro_use]
extern crate lazy_static;

mod board;
mod game;

use board::{Coordinate, GamePiece, Move};
use game::GameEngine;
use mut_static::MutStatic;
use crate::game::MoveResult;

lazy_static! {
    pub static ref GAME_ENGINE: MutStatic<GameEngine> =  MutStatic::from(GameEngine::new());
}

// expose two methods: get_piece, move_piece and get_current_turn as webassembly functions.

extern "C" {
    fn notify_piecemoved(fromX: i32, fromY: i32, toX: i32, toY: i32);
    fn notify_piececrowned(x: i32, y: i32);
}

#[no_mangle]
pub extern "C" fn get_piece(x: i32, y:i32) -> i32 {
    let engine = GAME_ENGINE.read().unwrap();

    let piece = engine.get_piece(Coordinate(x as usize, y as usize));
    match piece {
        Ok(Some(p)) => p.into(),
        Ok(None) => -1,
        Err(_) => -1,
    }
}

#[no_mangle]
pub extern "C" fn get_current_turn() -> i32 {
    let engine = GAME_ENGINE.read().unwrap();
    GamePiece::new(engine.current_turn()).into()
}


#[no_mangle]
pub extern "C" fn move_piece(fx: i32, fy: i32, tx: i32, ty: i32) -> i32 {
    let mut engine = GAME_ENGINE.write().unwrap();
    let mv: Move = Move::new((fx as usize, fy as usize), (tx as usize, ty as usize));
    let res: Result<MoveResult, ()> = engine.move_piece(&mv);
    match res {
        Ok(mr ) => {
            unsafe {
                notify_piecemoved( fx,  fy, tx, ty);
            }
            if mr.crowned {
                unsafe {
                    notify_piececrowned(tx, ty);
                }
            }
            1
        }
        Err(_) => 0,
    }
}