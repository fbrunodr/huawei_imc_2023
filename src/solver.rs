use crate::digits_helper::*;

pub const MOVE_TYPE: i32 = 0;
pub const ADD_TYPE: i32 = 1;
pub const REMOVE_TYPE: i32 = -1;

fn get_diffs_between_states(state_1: &Vec<i32>, state_2: &Vec<i32>) -> i32 {
    let mut diff: i32 = 0;

    for i in 0..state_1.len() {
        diff += (state_1[i] ^ state_2[i]).count_ones() as i32;
    }

    diff
}

pub fn solve(start_expression: String, moves: i32, problem_type: i32){
    let mut valid_states: Vec<Vec<i32>> = Vec::new();

    for a in 1..=999usize {
        for b in 1..=999usize {
            if a + b <= 999 {
                let c = a + b;
                let target: Vec<i32> = string_to_masks(
                    format!("{} + {} = {}", a, b, c)
                );
                valid_states.push( target );
            }
            if a >= b {
                let c = a - b;
                let target: Vec<i32> = string_to_masks(
                    format!("{} - {} = {}", a, b, c)
                );
                valid_states.push( target );
            }
            if a*b <= 999 {
                let c = a * b;
                let target: Vec<i32> = string_to_masks(
                    format!("{} * {} = {}", a, b, c)
                );
                valid_states.push( target );
            }
            if a % b == 0 {
                let c = a / b;
                let target: Vec<i32> = string_to_masks(
                    format!("{} / {} = {}", a, b, c)
                );
                valid_states.push( target );
            }
        }
    }

    let mut answers: Vec<String> = Vec::new();

    let source: Vec<i32> = string_to_masks(start_expression);
    let mut number_of_sticks_source = 0;
    for i in 0..source.len() {
        number_of_sticks_source += source[i].count_ones();
    }    

    match problem_type {
        MOVE_TYPE => {
            for state in valid_states {
                let mut number_of_sticks_state = 0;
                for i in 0..source.len() {
                    number_of_sticks_state += state[i].count_ones();
                }
                if number_of_sticks_source != number_of_sticks_state { continue; }

                let diff = get_diffs_between_states(&source, &state);

                if diff == 2 * moves {
                    answers.push(masks_to_string(state));
                }
            }
        }
        ADD_TYPE => {
            'states_iter: for state in valid_states {
                for i in 0..source.len() {
                    if state[i] & source[i] != source[i] { continue 'states_iter; }
                }

                let diff = get_diffs_between_states(&source, &state);

                if diff == moves {
                    answers.push(masks_to_string(state));
                }
            }
        }
        REMOVE_TYPE => {
            'states_iter: for state in valid_states {
                for i in 0..source.len() {
                    if state[i] & source[i] != state[i] { continue 'states_iter; }
                }

                let diff = get_diffs_between_states(&source, &state);

                if diff == moves {
                    answers.push(masks_to_string(state));
                }
            }
        }
        _ => ()
    };

    let mut answers_sorted: Vec<String> = answers.iter().cloned().collect();
    answers_sorted.sort();

    for answer in answers_sorted {
        println!("{}", answer);
    }
}
