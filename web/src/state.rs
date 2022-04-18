use crate::{
    components::game::Snippet,
    constant::Status,
    utils::calculate::{calculate_accuracy, calculate_progress, calculate_wpm},
};

use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq)]
pub struct Code {
    pub lines: usize,
    pub cursor: Option<char>,
    pub remaining: String,
    pub correct: String,
    pub wrong: String,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Stats {
    pub progress: u8,
    pub mistakes: u8,
    pub wpm: u8,
    pub accuracy: u8,
    pub time: usize,
    pub combos: u8,
}

#[derive(Default, Clone, PartialEq, Store)]
pub struct GameState {
    pub code: Code,
    pub stats: Stats,
    pub status: Status,
    pub language: String,
}

pub enum Action {
    NewSnippet(Snippet),
    KeyPress(char),
    BackSpace,
    Tick,
    Reset,
}

impl Reducer<GameState> for Action {
    fn apply(&self, state: &mut GameState) {
        match self {
            Action::NewSnippet(snippet) => {
                *state = GameState::reset();

                let mut chars = snippet.code.chars();

                state.code.cursor = chars.next();
                state.code.remaining = chars.as_str().to_string();
                state.code.lines = snippet.code.lines().count() - 1;

                state.language = snippet.language.clone();
            }

            Action::Tick => {
                state.stats.time += 1;
                state.stats.wpm = calculate_wpm(state.stats.time, &state.code.correct);
            }

            Action::BackSpace => {
                let mut code = &mut state.code;

                if !code.wrong.is_empty() {
                    if let Some(cursor) = code.cursor {
                        if cursor == '❚' {
                            code.remaining = format!("{}{}", ' ', code.remaining);
                        } else {
                            code.remaining = format!("{}{}", cursor, code.remaining);
                        }
                    }

                    code.cursor = code.wrong.pop();
                    if let Some(c) = code.cursor {
                        if c == '❚' {
                            code.cursor = Some(' ')
                        }
                    }

                    while code.cursor == Some('\t') {
                        if let Some(cursor) = code.cursor {
                            code.remaining = format!("{}{}", cursor, code.remaining);
                        }
                        code.cursor = code.wrong.pop();
                    }
                }
            }

            // TODO: clean this up
            Action::KeyPress(key) => {
                if state.status != Status::Passed {
                    state.status = Status::Playing
                };

                let mut code = &mut state.code;
                let mut stats = &mut state.stats;
                let mut chars = code.remaining.chars();

                if let Some(cursor) = code.cursor {
                    if code.wrong.is_empty() && cursor == *key {
                        code.correct.push(*key);

                        code.cursor = chars.next();

                        if code.remaining.is_empty() {
                            stats.accuracy =
                                calculate_accuracy(&code.correct, &code.remaining, stats.mistakes);
                            state.status = Status::Passed;
                        }

                        while code.cursor == Some('\t') {
                            code.correct.push('\t');
                            code.cursor = chars.next();
                        }
                    } else if code.wrong.len() < 10 {
                        stats.mistakes += 1;

                        if let Some(cursor) = code.cursor {
                            if cursor == ' ' {
                                code.wrong.push('❚');
                            } else {
                                code.wrong.push(cursor);
                            }
                        }

                        code.cursor = chars.next();

                        while code.cursor == Some('\t') {
                            code.wrong.push('\t');
                            code.cursor = chars.next();
                        }
                    }
                }

                code.remaining = chars.as_str().to_string();
                stats.progress = calculate_progress(&code.correct, &code.remaining);
            }

            Action::Reset => {
                *state = GameState::reset();
            }
        }
    }
}

impl GameState {
    pub fn reset() -> GameState {
        GameState::default()
    }
}
