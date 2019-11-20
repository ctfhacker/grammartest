//! JSON Generator based on JSON ANTLR4
//! https://github.com/antlr/grammars-v4/blob/master/json/JSON.g4
//!
//! Tries to be as 1:1 to the ANTLR4 as possible
//!
//! Second iteration (Not used anymore):
//!
//! Structs for each left hand object.
//! New `Generate` trait that each left hand object implements
//! Rng and depth passed in as arguments to help stop recursion
use crate::{Generate, Rng, MAX_REPEAT, MAX_STEPS};

#[derive(Debug)]
pub struct Exp;
impl Generate for Exp {
    fn generate(rng: &mut Rng, recursion: &mut u64) -> String {
        *recursion += 1;
        //print!("in Exp\n");
        // : [Ee] [+\-]? INT
        // [Ee]
        let mut res = String::new();
        match rng.next() % 2 {
            0 => res.push('e'),
            1 => res.push('E'),
            _ => unreachable!(),
        }

        // [+\-]?
        if (rng.next() % 2) == 1 {
            match rng.next() % 2 {
                0 => res.push('+'),
                1 => res.push('-'),
                _ => unreachable!(),
            }
        }

        // INT
        res.push_str(&Int::generate(rng, recursion));

        res
    }
}

#[derive(Debug)]
pub struct Int;
impl Generate for Int {
    fn generate(rng: &mut Rng, recursion: &mut u64) -> String {
        *recursion += 1;
        //print!("in Int\n");
        // '0' | [1-9] [0-9]*
        let mut res = String::new();
        match rng.next() % 2 {
            // '0'
            0 => res.push('0'),
            1 => {
                // [1-9] [0-9]*
                res.push(('1' as u8 + (rng.next() % 9) as u8) as char);
                for _ in 0..(rng.next() % MAX_REPEAT) {
                    res.push(('0' as u8 + (rng.next() % 10) as u8) as char)
                }
            }
            _ => unreachable!(),
        }

        res
    }
}

#[derive(Debug)]
pub struct Number;
impl Generate for Number {
    fn generate(rng: &mut Rng, recursion: &mut u64) -> String {
        *recursion += 1;
        //print!("in Num\n");
        // '-'? INT ('.' [0-9] +)? EXP?
        let mut res = String::new();

        // '-'?
        if rng.next() % 2 == 1 {
            res.push('-');
        }

        // INT
        res.push_str(&Int::generate(rng, recursion));
        // ('.' [0-9]+)?
        if rng.next() % 2 == 1 {
            res.push('.');
            res.push(('0' as u8 + (rng.next() % 10) as u8) as char);
            for _ in 0..(rng.next() % MAX_REPEAT) {
                res.push(('0' as u8 + (rng.next() % 10) as u8) as char);
            }
        }

        // EXP?
        if rng.next() % 2 == 1 {
            res.push_str(&Exp::generate(rng, recursion));
        }

        res
    }
}

#[derive(Debug)]
pub struct Hex;
impl Generate for Hex {
    fn generate(rng: &mut Rng, recursion: &mut u64) -> String {
        *recursion += 1;
        ////print!("in Hex\n");
        // : [0-9a-fA-F]
        let values = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'A',
            'B', 'C', 'D', 'E', 'F',
        ];
        values[rng.next() as usize % values.len()].to_string()
    }
}

#[derive(Debug)]
pub struct Unicode;
impl Generate for Unicode {
    fn generate(rng: &mut Rng, recursion: &mut u64) -> String {
        *recursion += 1;
        //print!("in Uni\n");
        let mut value = "u".to_string();
        value.push_str(&Hex::generate(rng, recursion));
        value.push_str(&Hex::generate(rng, recursion));
        value.push_str(&Hex::generate(rng, recursion));
        value.push_str(&Hex::generate(rng, recursion));
        value
    }
}

#[derive(Debug)]
pub struct SafeCodePoint;
impl Generate for SafeCodePoint {
    fn generate(rng: &mut Rng, recursion: &mut u64) -> String {
        *recursion += 1;
        //print!("in safecode\n");
        let mut num;
        let mut value;
        loop {
            // Try all possible unicode values
            num = (rng.next() % 0x10ffff) as u32;

            // Ignore \u0000..\u001f and " and \
            if num < 0x20 || num == ('"' as u8 as u32) || num == ('\\' as u8 as u32) {
                continue;
            }

            // Attempt to make the character from the random number
            value = core::char::from_u32(num);

            // Only leave the loop if we found a valid number
            if value.is_some() {
                break;
            }
        }
        value.unwrap().to_string()
    }
}

#[derive(Debug)]
pub struct Escape;
impl Generate for Escape {
    fn generate(rng: &mut Rng, recursion: &mut u64) -> String {
        *recursion += 1;
        //print!("in escapoe\n");
        // '\\' (["\\/bfnrt] | UNICODE)
        let mut value = String::new();

        // '\\'
        value.push('\\');
        match rng.next() % 2 {
            // | UNICODE)
            0 => value.push_str(&Unicode::generate(rng, recursion)),
            1 => {
                // (["\\/bfnrt]
                let values = ['"', '\\', '/', 'b', 'f', 'n', 'r', 't'];
                value.push(values[rng.next() as usize % values.len()]);
            }
            _ => unreachable!(),
        }

        value
    }
}

#[derive(Debug)]
pub struct JsonString;
impl Generate for JsonString {
    fn generate(rng: &mut Rng, recursion: &mut u64) -> String {
        *recursion += 1;
        //print!("in jsonstring\n");
        let mut value = String::new();
        value.push('"');
        if *recursion <= MAX_STEPS {
            for _ in 0..(rng.next() % MAX_REPEAT) {
                match rng.next() % 2 {
                    0 => value.push_str(&Escape::generate(rng, recursion)),
                    1 => value.push_str(&SafeCodePoint::generate(rng, recursion)),
                    _ => unreachable!(),
                }
            }
        }
        value.push('"');
        value
    }
}

#[derive(Debug)]
pub struct JsonValue;
impl Generate for JsonValue {
    fn generate(rng: &mut Rng, recursion: &mut u64) -> String {
        *recursion += 1;
        //print!("in jsonvalue\n");
        if *recursion >= MAX_STEPS {
            return Number::generate(rng, recursion);
        }
        let value = match rng.next() % 100 {
            0..15 => JsonString::generate(rng, recursion),
            15..30 => Number::generate(rng, recursion),
            30..60 => JsonObject::generate(rng, recursion),
            60..98 => JsonArray::generate(rng, recursion),
            98 => "true".to_string(),
            99 => "false".to_string(),
            100 => "null".to_string(),
            _ => unreachable!(),
        };
        value
    }
}

#[derive(Debug)]
pub struct JsonPair;
impl Generate for JsonPair {
    fn generate(rng: &mut Rng, recursion: &mut u64) -> String {
        *recursion += 1;
        let mut value = String::new();
        value.push_str(&JsonString::generate(rng, recursion));
        value.push(':');
        value.push_str(&JsonValue::generate(rng, recursion));
        value
    }
}

#[derive(Debug)]
pub struct JsonArray;
impl Generate for JsonArray {
    fn generate(rng: &mut Rng, recursion: &mut u64) -> String {
        *recursion += 1;
        //print!("in jsonarray\n");
        let mut value = String::new();
        match rng.next() % 10 {
            0 => {
                value.push('[');
                value.push(']');
            }
            _ => {
                value.push('[');
                value.push_str(&JsonValue::generate(rng, recursion));
                if *recursion <= MAX_STEPS {
                    for _ in 0..(rng.next() % MAX_REPEAT) {
                        value.push(',');
                        value.push_str(&JsonValue::generate(rng, recursion));
                    }
                }
                value.push(']');
            }
        }
        value
    }
}

#[derive(Debug)]
pub struct JsonObject;
impl Generate for JsonObject {
    fn generate(rng: &mut Rng, recursion: &mut u64) -> String {
        *recursion += 1;
        //print!("in jsonoby\n");
        let mut value = String::new();
        match rng.next() % 10 {
            0 => {
                value.push('{');
                value.push('}');
            }
            _ => {
                value.push('{');
                value.push_str(&JsonPair::generate(rng, recursion));
                if *recursion <= MAX_STEPS {
                    for _ in 0..(rng.next() % MAX_REPEAT) {
                        value.push(',');
                        value.push_str(&JsonPair::generate(rng, recursion));
                    }
                }
                value.push('}');
            }
        }
        value
    }
}

#[derive(Debug)]
pub struct Json;
impl Generate for Json {
    fn generate(rng: &mut Rng, recursion: &mut u64) -> String {
        *recursion += 1;
        //print!("in json\n");
        JsonValue::generate(rng, recursion)
    }
}
