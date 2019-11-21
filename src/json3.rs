//! JSON Generator based on JSON ANTLR4
//! https://github.com/antlr/grammars-v4/blob/master/json/JSON.g4
//!
//! Tries to be as 1:1 to the ANTLR4 as possible
//!
//! Third iteration:
//!
//! Enums instead of structs so that each Object can't be instantiated
//! and can only be called via generate.
use crate::{Generate, Rng, MAX_DEPTH, MAX_REPEAT};

/// Exponent object
///
/// ANTLR4:
/// fragment EXP
///    : [Ee] [+\-]? INT
///
/// Example generations:
///
/// e0, E8151, E+0, E-8, E-5558
#[derive(Debug)]
pub enum Exp {}
impl Generate for Exp {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
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
        res.push_str(&Int::generate(rng, depth));

        res
    }
}

/// Integer object
///
/// ANTLR4:
/// fragment INT
///    : '0' | [1-9] [0-9]*
///
/// Example generations:
///
/// 0, 4632, 4080, 7849, 33129, 759739
#[derive(Debug)]
pub enum Int {}
impl Generate for Int {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
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

/// Number object
///
/// ANTLR4:
/// NUMBER
///   : '-'? INT ('.' [0-9] +)? EXP?
///
/// Example generations:
///
/// 157.038063E0, -0.57248E+48003, 0.577413E78
/// -45506644e+0, -0.8094341

#[derive(Debug)]
pub enum Number {}
impl Generate for Number {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
        // '-'? INT ('.' [0-9] +)? EXP?
        let mut res = String::new();

        // '-'?
        if rng.next() % 2 == 1 {
            res.push('-');
        }

        // INT
        res.push_str(&Int::generate(rng, depth));
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
            res.push_str(&Exp::generate(rng, depth));
        }

        res
    }
}

/// Hex object
///
/// ANTLR4:
/// fragment HEX
///    : [0-9a-fA-F]
///
/// Example generations:
/// f, F, 1, E, c
#[derive(Debug)]
pub enum Hex {}
impl Generate for Hex {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
        // : [0-9a-fA-F]
        let values = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'A',
            'B', 'C', 'D', 'E', 'F',
        ];
        values[rng.next() as usize % values.len()].to_string()
    }
}

/// Unicode object
///
/// ANTLR4:
/// fragment UNICODE
///    : 'u' HEX HEX HEX HEX
///
/// Example generations:
/// u56fc, uB9ce, u24cE, ueEf7, u1189
#[derive(Debug)]
pub enum Unicode {}
impl Generate for Unicode {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
        let mut value = "u".to_string();
        value.push_str(&Hex::generate(rng, depth));
        value.push_str(&Hex::generate(rng, depth));
        value.push_str(&Hex::generate(rng, depth));
        value.push_str(&Hex::generate(rng, depth));
        value
    }
}

/// SafeCodePoint object
///
/// ANTLR4:
/// fragment SAFECODEPOINT
///    : ~ ["\\\u0000-\u001F]
///
/// Example generations:
/// 񙸸 񖢋 񸼱 󰜫 񠝨
#[derive(Debug)]
pub enum SafeCodePoint {}
impl Generate for SafeCodePoint {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
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

/// Escape object
///
/// ANTLR4:
/// fragment ESC
///    : '\\' (["\\/bfnrt] | UNICODE)
///
/// Example generations:
/// \f, \n, \u6470, \u936c, \n
#[derive(Debug)]
pub enum Escape {}
impl Generate for Escape {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
        // '\\' (["\\/bfnrt] | UNICODE)
        let mut value = String::new();

        // '\\'
        value.push('\\');
        match rng.next() % 2 {
            // | UNICODE)
            0 => value.push_str(&Unicode::generate(rng, depth)),
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

/// JsonString object
///
/// ANTLR4:
/// STRING
///   : '"' (ESC | SAFECODEPOINT)* '"'
///
/// Example generations:
/// "򀺂\u0d07", "\t\ueBCD𦏔񽫀򚠬\/\b", "\r\u1bC4񘿥񇧅񈨋򣬦\b", "򶯒\u99fd\u456e\u2bA2󦑞\n", "\t\bἣ򗎹\u33ee",
#[derive(Debug)]
pub enum JsonString {}
impl Generate for JsonString {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
        let mut value = String::new();
        value.push('"');
        if *depth <= MAX_DEPTH {
            for _ in 0..(rng.next() % MAX_REPEAT) {
                match rng.next() % 2 {
                    0 => value.push_str(&Escape::generate(rng, depth)),
                    1 => value.push_str(&SafeCodePoint::generate(rng, depth)),
                    _ => unreachable!(),
                }
            }
        }
        value.push('"');
        value
    }
}

/// JsonValue object
///
/// ANTLR4:
/// value
///    : STRING
///    | NUMBER
///    | obj
///    | array
///    | 'true'
///    | 'false'
///    | 'null'
///
/// Example generations:
/// [true,-0,{},-0.4,4234], 6.6e-36761067, {"\b\uACbA񍥨򵄋򐹮":-6682.7E0}, {"\b􌣴󰶭":-7E+0},
/// {"\r\n󧂭\u2Dbc\uEDf7\u3d06":-0.33849685}
#[derive(Debug)]
pub enum JsonValue {}

impl Generate for JsonValue {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
        if *depth >= MAX_DEPTH {
            return Number::generate(rng, depth);
        }
        let value = match rng.next() % 100 {
            0..15 => JsonString::generate(rng, depth),
            15..30 => Number::generate(rng, depth),
            30..60 => JsonObject::generate(rng, depth),
            60..98 => JsonArray::generate(rng, depth),
            98 => "true".to_string(),
            99 => "false".to_string(),
            100 => "null".to_string(),
            _ => unreachable!(),
        };
        value
    }
}

/// JsonPair object
///
/// ANTLR4:
/// pair
///    : STRING ':' value
///
/// Example generations:
/// "":{"\ubfbF󙃄":96021392}, "\t񐇺򊲨":[-39733], "":[-0.24791,0.75134,0.44177522,-0e97638860,-4]
/// "񭞷򁷝":-7E0, "񶐽":[{"":0.8}]
#[derive(Debug)]
pub enum JsonPair {}
impl Generate for JsonPair {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
        let mut value = String::new();
        value.push_str(&JsonString::generate(rng, depth));
        value.push(':');
        value.push_str(&JsonValue::generate(rng, depth));
        value
    }
}

/// JsonArray object
///
/// ANTLR4:
/// array
///    : '[' value (',' value)* ']'
///    | '[' ']'
///
/// Example generations:
/// [563851], ["\udCFB\ud735"], [[[{"":-71246008.4165}]]], [[[[-81330724]]]], [[]]
#[derive(Debug)]
pub enum JsonArray {}
impl Generate for JsonArray {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
        let mut value = String::new();
        match rng.next() % 10 {
            0 => {
                value.push('[');
                value.push(']');
            }
            _ => {
                value.push('[');
                value.push_str(&JsonValue::generate(rng, depth));
                if *depth <= MAX_DEPTH {
                    for _ in 0..(rng.next() % MAX_REPEAT) {
                        value.push(',');
                        value.push_str(&JsonValue::generate(rng, depth));
                    }
                }
                value.push(']');
            }
        }
        value
    }
}

/// JsonObject generation
///
/// ANTLR4:
/// obj
///   : '{' pair (',' pair)* '}'
///   | '{' '}'
///
/// Example generations:
/// {"":[[0]]}, {"":[{"":0}]}, {"򉺠\b":{"":304}}, {"򌗖򓳝򻚤\\񤧌\f":-0E0}, {"\r":0,"":0E6}
#[derive(Debug)]
pub enum JsonObject {}
impl Generate for JsonObject {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
        let mut value = String::new();
        match rng.next() % 10 {
            0 => {
                value.push('{');
                value.push('}');
            }
            _ => {
                value.push('{');
                value.push_str(&JsonPair::generate(rng, depth));
                if *depth <= MAX_DEPTH {
                    for _ in 0..(rng.next() % MAX_REPEAT) {
                        value.push(',');
                        value.push_str(&JsonPair::generate(rng, depth));
                    }
                }
                value.push('}');
            }
        }
        value
    }
}

/// Top level JSON object
///
/// ANTLR4:
/// json
///    : value
///
/// Example generations:
/// ["\uFad0"], [0.7209,-0.85435856,0.6e-7,92.930901,-0.94057E+0,-24E0,-8.0,5903.16e-0], 0e0
/// -4231186.63066e-6, true
#[derive(Debug)]
pub enum Json {}
impl Generate for Json {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
        JsonValue::generate(rng, depth)
    }
}
