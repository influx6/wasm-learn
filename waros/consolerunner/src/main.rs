extern crate botengine;
use botengine::{Combatant, Gameloop};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;
use std::time;

fn main() {    
    let gs = Arc::new(botengine::GameState::new());

    let b1 = botengine::Combatant::buffer_from_file(
        "./bots/dumbotrs.wasm");
    let bot1 = b1.unwrap();

    let b2 = botengine::Combatant::buffer_from_file(
        "./bots/rook.wasm");
    let bot2 = b2.unwrap();

    let rb = botengine::Combatant::buffer_from_file(
        "./bots/rabbit.wasm");
    let rabbit = rb.unwrap();

    let my_gs = gs.clone();
    let debug_gs = gs.clone();

    let (sender, receiver) = channel();
    thread::spawn(move || loop {
        match receiver.recv() {
            Ok(ge) => println!("{:?}", ge),
            Err(_) => {}
        }
    });

    let mut gl = Gameloop::new(my_gs, 100_000, 3, Some(sender));

    let _handle = Combatant::start("bot-1", bot1, gs.clone());
    let _handle2 = Combatant::start("rook", bot2, gs.clone());
    let _handle3 = Combatant::start("rabbit", rabbit, gs.clone());
    let game_result = gl.start();

    thread::sleep(time::Duration::from_secs(1));

    println!(
        "Game loop terminated: {:?}\nState: {:?}",
        game_result, debug_gs
    );
}
