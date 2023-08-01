use std::collections::HashMap;
use lazy_static::lazy_static;

pub const EMPTY: i32 = 0b0000000;
pub const ZERO: i32 = 0b1110111;
pub const ONE: i32 = 0b0010010;
pub const TWO: i32 = 0b1011101;
pub const THREE: i32 = 0b1011011;
pub const FOUR: i32 = 0b0111010;
pub const FIVE: i32 = 0b1101011;
pub const SIX: i32 = 0b1101111;
pub const SEVEN: i32 = 0b1010010;
pub const EIGHT: i32 = 0b1111111;
pub const NINE: i32 = 0b1111011;

lazy_static! {
    pub static ref MASK_TO_NUMBER: HashMap<i32, i32> = {
        let mut map = HashMap::new();
        map.insert(EMPTY, 0);
        map.insert(ZERO, 0);
        map.insert(ONE, 1);
        map.insert(TWO, 2);
        map.insert(THREE, 3);
        map.insert(FOUR, 4);
        map.insert(FIVE, 5);
        map.insert(SIX, 6);
        map.insert(SEVEN, 7);
        map.insert(EIGHT, 8);
        map.insert(NINE, 9);
        map
    };
}

pub const NUMBER_TO_MASK: [i32; 10] = [
    ZERO,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE
];

pub const PLUS: i32 = 0b1100;
pub const MINUS: i32 = 0b1000;
pub const TIMES: i32 = 0b0011;
pub const DIVIDED: i32 = 0b0010;

pub const INVALID_EXPRESSION: &str = "INVALID_EXPRESSION";

pub fn masks_to_string(masks: Vec<i32>) -> String {
    let a = {
        let mut a = 0;
        for i in 0..3 {
            a += 10_i32.pow(2-i) * MASK_TO_NUMBER[&masks[i as usize]];
        }
        a
    };

    let b = {
        let mut b = 0;
        for i in 0..3 {
            b += 10_i32.pow(2-i) * MASK_TO_NUMBER[&masks[(i + 3) as usize]];
        }
        b
    };

    let c = {
        let mut c = 0;
        for i in 0..3 {
            c += 10_i32.pow(2-i) * MASK_TO_NUMBER[&masks[(i + 6) as usize]];
        }
        c
    };

    let operation_mask = *masks.last().unwrap();

    let result = match operation_mask {
        PLUS => format!("{} + {} = {}", a, b, c),
        MINUS => format!("{} - {} = {}", a, b, c),
        TIMES => format!("{} * {} = {}", a, b, c),
        DIVIDED => format!("{} / {} = {}", a, b, c),
        _ => return INVALID_EXPRESSION.to_string(),
    };

    result
}

pub fn string_to_masks(text: String) -> Vec<i32> {
    let parts: Vec<&str> = text.split(|c| c == ' ').collect();
    let a: i32 = parts[0].trim().parse().unwrap();
    let op: char = parts[1].chars().next().unwrap();
    let b: i32 = parts[2].trim().parse().unwrap();
    let c: i32 = parts[4].trim().parse().unwrap();

    let mut masks: Vec<i32> = Vec::new();

    masks.push( if a < 100 { EMPTY } else { NUMBER_TO_MASK[(a/100) as usize] } );
    masks.push( if a < 10 { EMPTY } else { NUMBER_TO_MASK[((a/10) % 10) as usize] } );
    masks.push( NUMBER_TO_MASK[(a % 10) as usize] );

    masks.push( if b < 100 { EMPTY } else { NUMBER_TO_MASK[(b/100) as usize] } );
    masks.push( if b < 10 { EMPTY } else { NUMBER_TO_MASK[((b/10) % 10) as usize] } );
    masks.push( NUMBER_TO_MASK[(b % 10) as usize] );

    masks.push( if c < 100 { EMPTY } else { NUMBER_TO_MASK[(c/100) as usize] } );
    masks.push( if c < 10 { EMPTY } else { NUMBER_TO_MASK[((c/10) % 10) as usize] } );
    masks.push( NUMBER_TO_MASK[(c % 10) as usize] );

    masks.push(
        match op {
            '+' => PLUS,
            '-' => MINUS,
            '*' => TIMES,
            '/' => DIVIDED,
            _ => PLUS
        }
    );

    return masks;
}
