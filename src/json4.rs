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
    fn generate(rng: &mut Rng, depth: &mut u64, buf: &mut Vec<u8>) {
        *depth += 1;
        // : [Ee] [+\-]? INT
        // [Ee]
        match rng.next() % 2 {
            0 => buf.push('e' as u8),
            1 => buf.push('E' as u8),
            _ => unreachable!(),
        }

        // [+\-]?
        if (rng.next() % 2) == 1 {
            match rng.next() % 2 {
                0 => buf.push('+' as u8),
                1 => buf.push('-' as u8),
                _ => unreachable!(),
            }
        }

        // INT
        Int::generate(rng, depth, buf);
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
    fn generate(rng: &mut Rng, depth: &mut u64, buf: &mut Vec<u8>) {
        *depth += 1;
        // '0' | [1-9] [0-9]*
        match rng.next() % 2 {
            // '0'
            0 => buf.push('0' as u8),
            1 => {
                // [1-9] [0-9]*
                buf.push(('1' as u8 + (rng.next() % 9) as u8));
                for _ in 0..(rng.next() % MAX_REPEAT) {
                    buf.push(('0' as u8 + (rng.next() % 10) as u8));
                }
            }
            _ => unreachable!(),
        }
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
    fn generate(rng: &mut Rng, depth: &mut u64, buf: &mut Vec<u8>) {
        *depth += 1;
        // '-'? INT ('.' [0-9] +)? EXP?

        // '-'?
        if rng.next() % 2 == 1 {
            buf.push('-' as u8);
        }

        // INT
        Int::generate(rng, depth, buf);

        // ('.' [0-9]+)?
        if rng.next() % 2 == 1 {
            buf.push('.' as u8);
            buf.push(('0' as u8 + (rng.next() % 10) as u8));
            for _ in 0..(rng.next() % MAX_REPEAT) {
                buf.push(('0' as u8 + (rng.next() % 10) as u8));
            }
        }

        // EXP?
        if rng.next() % 2 == 1 {
            Exp::generate(rng, depth, buf);
        }
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
    fn generate(rng: &mut Rng, depth: &mut u64, buf: &mut Vec<u8>) {
        *depth += 1;
        // : [0-9a-fA-F]
        let values = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'A',
            'B', 'C', 'D', 'E', 'F',
        ];
        buf.push(values[rng.next() as usize % values.len()] as u8);
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
    fn generate(rng: &mut Rng, depth: &mut u64, buf: &mut Vec<u8>) {
        *depth += 1;
        buf.push('u' as u8);
        Hex::generate(rng, depth, buf);
        Hex::generate(rng, depth, buf);
        Hex::generate(rng, depth, buf);
        Hex::generate(rng, depth, buf);
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
    fn generate(rng: &mut Rng, depth: &mut u64, buf: &mut Vec<u8>) {
        *depth += 1;
        let mut num;
        loop {
            // Try all possible unicode values
            num = (rng.next() % 0x10ffff) as u32;

            // Ignore \u0000..\u001f and " and \
            if num < 0x20 || num == ('"' as u8 as u32) || num == ('\\' as u8 as u32) {
                continue;
            }

            break;
        }

        buf.push((num & 0xff) as u8);
        buf.push((num >> 16) as u8);
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
    fn generate(rng: &mut Rng, depth: &mut u64, buf: &mut Vec<u8>) {
        *depth += 1;
        // '\\' (["\\/bfnrt] | UNICODE)

        // '\\'
        buf.push('\\' as u8);
        match rng.next() % 2 {
            // | UNICODE)
            0 => Unicode::generate(rng, depth, buf),
            1 => {
                // (["\\/bfnrt]
                let values = [
                    '"' as u8, '\\' as u8, '/' as u8, 'b' as u8, 'f' as u8, 'n' as u8, 'r' as u8,
                    't' as u8,
                ];
                buf.push(values[rng.next() as usize % values.len()]);
            }
            _ => unreachable!(),
        }
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
    fn generate(rng: &mut Rng, depth: &mut u64, buf: &mut Vec<u8>) {
        *depth += 1;
        buf.push('"' as u8);
        if *depth <= MAX_DEPTH {
            for _ in 0..(rng.next() % MAX_REPEAT) {
                match rng.next() % 2 {
                    0 => Escape::generate(rng, depth, buf),
                    1 => SafeCodePoint::generate(rng, depth, buf),
                    _ => unreachable!(),
                }
            }
        }
        buf.push('"' as u8);
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
    fn generate(rng: &mut Rng, depth: &mut u64, buf: &mut Vec<u8>) {
        *depth += 1;
        if *depth >= MAX_DEPTH {
            return Number::generate(rng, depth, buf);
        }
        let value = match rng.next() % 100 {
            0..15 => JsonString::generate(rng, depth, buf),
            15..30 => Number::generate(rng, depth, buf),
            30..60 => JsonObject::generate(rng, depth, buf),
            60..98 => JsonArray::generate(rng, depth, buf),
            98 => buf.extend_from_slice("true".as_bytes()),
            99 => buf.extend_from_slice("false".as_bytes()),
            100 => buf.extend_from_slice("null".as_bytes()),
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
    fn generate(rng: &mut Rng, depth: &mut u64, buf: &mut Vec<u8>) {
        *depth += 1;
        JsonString::generate(rng, depth, buf);
        buf.push(':' as u8);
        JsonValue::generate(rng, depth, buf);
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
    fn generate(rng: &mut Rng, depth: &mut u64, buf: &mut Vec<u8>) {
        *depth += 1;
        match rng.next() % 10 {
            0 => {
                buf.push('[' as u8);
                buf.push(']' as u8);
            }
            _ => {
                buf.push('[' as u8);
                JsonValue::generate(rng, depth, buf);
                if *depth <= MAX_DEPTH {
                    for _ in 0..(rng.next() % MAX_REPEAT) {
                        buf.push(',' as u8);
                        JsonValue::generate(rng, depth, buf);
                    }
                }
                buf.push(']' as u8);
            }
        }
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
    fn generate(rng: &mut Rng, depth: &mut u64, buf: &mut Vec<u8>) {
        *depth += 1;
        match rng.next() % 10 {
            0 => {
                buf.push('{' as u8);
                buf.push('}' as u8);
            }
            _ => {
                buf.push('{' as u8);
                JsonPair::generate(rng, depth, buf);
                if *depth <= MAX_DEPTH {
                    for _ in 0..(rng.next() % MAX_REPEAT) {
                        buf.push(',' as u8);
                        JsonPair::generate(rng, depth, buf);
                    }
                }
                buf.push('}' as u8);
            }
        }
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
    fn generate(rng: &mut Rng, depth: &mut u64, buf: &mut Vec<u8>) {
        *depth += 1;
        JsonValue::generate(rng, depth, buf)
    }
}
