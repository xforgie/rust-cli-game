use std::{
    any::Any,
    cell::{Ref, RefCell},
    io::Write,
    str::FromStr,
};

use crate::{states::StartGameState, player::Player};

pub trait AToAny: 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// Allow downcasting to a RefCell<T>.
// This is needed to be able to get a mutable state
// as well as downcast from a generic GameState.
impl<T: 'static> AToAny for RefCell<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// Base functions to run against the current state of the game in a main loop
pub trait GameState
where
    Self: AToAny,
{
    fn get_input_options(&self) -> Ref<Vec<InputOption>>;
    fn get_header(&self) -> String;
    fn get_body(&self) -> Option<String>;

    // optional function called on GameStates when they are added to the stack
    fn get_init_fn(&self) -> Option<fn(&mut GlobalGameState)>;

    fn display(&self) {
        match self.get_body() {
            Some(body) => println!("{}\n\n{}\n", self.get_header(), body),
            None => println!("{}\n", self.get_header())
        }

        self.display_input_options();
    }

    fn display_input_options(&self) {
        let input_options = self.get_input_options();

        for input_option in input_options.iter() {
            match &input_option.shortcut {
                Some(shortcut) => {
                    println!(" > ({}) {}", shortcut, input_option.name);
                }
                None => {
                    println!(" > {}", input_option.name);
                }
            };
        }

        println!();
    }

    // Returns a single match to the current choices (if any) given the user input
    fn parse_input(&self, input: &str) -> Option<fn(&mut GlobalGameState)> {
        let input_options = self.get_input_options();

        let mut opts: Vec<&InputOption> = input_options
            .iter()
            .filter(|input_opt| {
                input_opt.name.eq_ignore_ascii_case(input)
                    || match &input_opt.shortcut {
                        Some(shortcut) => shortcut.eq_ignore_ascii_case(input),
                        None => false,
                    }
            })
            .collect();

        opts.pop().map(|input_opt| input_opt.callback)
    }
}

// Defines a choice a user can make as well a function to call if it is chosen
pub struct InputOption {
    name: String,
    shortcut: Option<String>,
    callback: fn(&mut GlobalGameState),
}

pub struct InputOptionBuilder {
    name: Option<String>,
    shortcut: Option<String>,
    callback: Option<fn(&mut GlobalGameState)>,
}

impl InputOptionBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            shortcut: None,
            callback: None,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn shortcut(mut self, shortcut: &str) -> Self {
        self.shortcut = Some(shortcut.to_string());
        self
    }

    pub fn connect(mut self, f: fn(&mut GlobalGameState)) -> Self {
        self.callback = Some(f);
        self
    }

    pub fn build(self) -> InputOption {
        InputOption {
            name: self.name.unwrap_or(String::from("N/A")),
            shortcut: self.shortcut,
            callback: self.callback.unwrap_or(|_| {})
        }
    }
}


// Holds the main game info, all InputOption callbacks get a mutable reference to this
pub struct GlobalGameState {
    g_stack: Vec<Box<dyn GameState>>,
    player: Option<Player>
}

impl GlobalGameState {
    pub fn new() -> Self {
        Self {
            g_stack: vec![Box::new(RefCell::new(StartGameState::new()))],
            player: None
        }
    }

    pub fn create_player(&mut self, name: String) {
        self.player = Some(Player::new(name))
    }

    pub fn get_player(&self) -> &Player {
        match &self.player {
            Some(p) => p,
            None => panic!("Player does not exist"),
        }
    }

    pub fn append_state<GameStateType: GameState + 'static>(&mut self, new_state: GameStateType) {
        let callback = match new_state.get_init_fn() {
            Some(f) => f,
            None => |_: &mut GlobalGameState| {},
        };

        self.g_stack.push(Box::new(new_state));

        callback(self);
    }

    pub fn remove_state(&mut self) {
        self.g_stack.pop();
    }

    pub fn get_current_state_downcast<GameStateType: GameState + 'static>(
        &mut self,
    ) -> &GameStateType {
        let g_state = self
            .g_stack
            .last()
            .expect("Attempted to retrieve state when there was none")
            .as_any();

        let result = g_state.downcast_ref::<GameStateType>();

        match result {
            Some(gs) => gs,
            None => panic!("Could not downcast mutable reference to GameState"),
        }
    }

    pub fn get_current_state(&mut self) -> &dyn GameState {
        let g_state = self
            .g_stack
            .last()
            .expect("Attempted to retrieve state when there was none")
            .as_ref();

        g_state
    }

    pub fn is_states_empty(&self) -> bool {
        self.g_stack.len() == 0
    }

    pub fn get_from_input_prompt<T: FromStr>(prompt: &str) -> T {
        loop {
            let mut user_input = String::new();

            print!("{}", prompt);
            std::io::stdout().flush().expect("Could not flush output");

            std::io::stdin()
                .read_line(&mut user_input)
                .expect("Could not parse user input");

            match user_input.trim().parse::<T>() {
                Ok(input) => return input,
                Err(_) => {
                    println!("Invalid input");
                    continue;
                }
            };
        }
    }

    pub fn display_prompt(prompt: &str) {
        println!("{}", prompt)
    }

    // main game loop
    pub fn run_game(&mut self) {
        while !self.is_states_empty() {
            let callback: Option<fn(&mut GlobalGameState)>;

            {
                let g_state = self.get_current_state();

                g_state.display();

                let user_input = GlobalGameState::get_from_input_prompt::<String>("");

                callback = g_state.parse_input(user_input.trim());
            }

            match callback {
                Some(f) => f(self),
                None => println!("Invalid input"),
            }
        }
    }
}
