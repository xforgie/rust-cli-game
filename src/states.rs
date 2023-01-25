use std::{cell::{Ref, RefCell}};

use crate::game_state::{GameState, GlobalGameState, InputOption, InputOptionBuilder};

// Inital state when beginning the game (main menu)
#[derive(game_state_proc_macro::GameStateImpl)]
pub struct StartGameState {
    header: String,
    body: Option<String>,
    init: Option<fn(&mut GlobalGameState)>,
    input_options: Vec<InputOption>,
}

impl StartGameState {
    pub fn new() -> Self {
        Self {
            header: format!("Welcome to Rust RPG V{}!", env!("CARGO_PKG_VERSION")),
            body: None,
            init: None,
            input_options: vec![
                InputOptionBuilder::new()
                    .name("New")
                    .shortcut("N")
                    .connect(StartGameState::test_new)
                    .build(),
                InputOptionBuilder::new()
                    .name("Options")
                    .shortcut("O")
                    .connect(StartGameState::test_options)
                    .build(),
                InputOptionBuilder::new()
                    .name("Quit")
                    .shortcut("Q")
                    .connect(StartGameState::test_quit)
                    .build(),
            ],
        }
    }

    fn test_new(g_game_state: &mut GlobalGameState) {

        let player_name = GlobalGameState::get_from_input_prompt::<String>("Please enter your name: ");

        g_game_state.create_player(player_name);
        
        GlobalGameState::display_prompt(&format!("Welcome, {}...", g_game_state.get_player().get_name()));

        g_game_state.append_state(RefCell::new(MainGameState::new()));
    }

    fn test_options(g_game_state: &mut GlobalGameState) {
        g_game_state.append_state(RefCell::new(OptionsMenuGameState::new()));
    }

    fn test_quit(g_game_state: &mut GlobalGameState) {
        g_game_state.remove_state();
    }
}

// Option menu accessed from the main menu
#[derive(game_state_proc_macro::GameStateImpl)]
struct OptionsMenuGameState {
    starting_level: i32,
    header: String,
    body: Option<String>,
    init: Option<fn(&mut GlobalGameState)>,
    input_options: Vec<InputOption>,
}

impl OptionsMenuGameState {
    pub fn new() -> Self {
        Self {
            starting_level: 1,
            header: String::from("Option Menu"),
            body: None,
            init: None,
            input_options: vec![
                InputOptionBuilder::new()
                    .name("Change Starting Level")
                    .shortcut("C")
                    .connect(OptionsMenuGameState::change_starting_level)
                    .build(),
                InputOptionBuilder::new()
                    .name("Back")
                    .shortcut("B")
                    .connect(OptionsMenuGameState::back)
                    .build(),
            ],
        }
    }

    fn change_starting_level(g_game_state: &mut GlobalGameState) {
        let g_state = g_game_state.get_current_state_downcast::<RefCell<OptionsMenuGameState>>();

        g_state.borrow_mut().starting_level =
            GlobalGameState::get_from_input_prompt::<i32>("Enter starting level (integer): ");
    }

    fn back(g_game_state: &mut GlobalGameState) {
        g_game_state.remove_state();
    }
}

#[derive(game_state_proc_macro::GameStateImpl)]
struct MainGameState {
    header: String,
    body: Option<String>,
    init: Option<fn(&mut GlobalGameState)>,
    input_options: Vec<InputOption>,
}

impl MainGameState {
    pub fn new() -> Self {
        Self {
            header: String::from("N/A"),
            body: None,
            init: None,
            input_options: vec![
                InputOptionBuilder::new()
                    .name("Walk")
                    .shortcut("W")
                    .connect(MainGameState::walk)
                    .build(),
                InputOptionBuilder::new()
                    .name("Inventory")
                    .shortcut("I")
                    .connect(MainGameState::open_inventory)
                    .build(),
                InputOptionBuilder::new()
                    .name("Quit")
                    .shortcut("Q")
                    .connect(MainGameState::quit)
                    .build()
            ]
        }
    }

    fn walk(_g_game_state: &mut GlobalGameState) {
        GlobalGameState::display_prompt("Walking...");
    }

    fn open_inventory(g_game_state: &mut GlobalGameState) {
        g_game_state.append_state(RefCell::new(InventoryGameState::new()))
    }

    fn quit(g_game_state: &mut GlobalGameState) {
        g_game_state.remove_state()
    }
}

#[derive(game_state_proc_macro::GameStateImpl)]
struct InventoryGameState {
    header: String,
    body: Option<String>,
    init: Option<fn(&mut GlobalGameState)>,
    input_options: Vec<InputOption>,
}

impl InventoryGameState {
    pub fn new() -> Self {
        Self {
            header: String::from("Inventory"),
            body: None,
            init: Some(InventoryGameState::init),
            input_options: vec![
                InputOptionBuilder::new()
                    .name("Back")
                    .shortcut("B")
                    .connect(InventoryGameState::back)
                    .build()
            ]
        }
    }

    fn init(g_game_state: &mut GlobalGameState) {
        let inv = g_game_state.get_player().get_inventory();
        let body = String::new();

        let mut inventory_inputs: Vec<InputOption> = Vec::new();

        for (i, item )in inv.iter().enumerate() {
            // body.push_str(&format!("{}\n", item.get_name()));

            let closure = move |_: &mut GlobalGameState| {
                println!("The item description from: {}", "test")
            };

            let input_opt = InputOptionBuilder::new()
                .name(item.get_name())
                .shortcut(&i.to_string())
                .connect(closure)
                .build();

            inventory_inputs.insert(0, input_opt);
        }
        
        let g_state = g_game_state.get_current_state_downcast::<RefCell<InventoryGameState>>();

        inventory_inputs.append(&mut g_state.borrow_mut().input_options);
        g_state.borrow_mut().input_options = inventory_inputs;

        g_state.borrow_mut().body = Some(body);
    }

    fn back(g_game_state: &mut GlobalGameState) {
        g_game_state.remove_state()
    }
}