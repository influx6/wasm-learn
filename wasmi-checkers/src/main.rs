extern crate wasmi;

mod checkersgame;
mod imports;
mod runtime;

use checkersgame::CheckersGame;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut game = CheckersGame::new("./checkers.wat.wasm");
    game.init()?;

    let board_display = game.get_board_contents()?;
    println!("game board at start:\n{}\n", board_display);

    println!(
        "At game start, current turn is : {:?}",
        game.get_turn_owner()?
    );
    game.move_piece(&(0, 5), &(0, 4))?;
    println!(
        "After first move, current turn is : {:?}",
        game.get_turn_owner()?
    );

    let board_display = game.get_board_contents()?;
    println!("game board after 1 move:\n{}\n", board_display);

    /*
        So remember the key points:
        1. Implement a module ImportResolver implementing the wasmi-> ModuleImportResolver.resolve_func (returns a signature and index via FuncRef to the wasm module)
        2. Implement a runtime that takes a index which it uses to execute the target function with the args list it receives (it invokes these via implementation of wasmi::Externals.invoke_index)
        3. Instantiate your wasm module (assgining the moduleimport resolver) and on every function invocation pass in our runtime.
     */

    Ok(())
}
