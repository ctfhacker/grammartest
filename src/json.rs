//! JSON Generator based on JSON ANTLR4
//! https://github.com/antlr/grammars-v4/blob/master/json/JSON.g4
//!
//! Tries to be as 1:1 to the ANTLR4 as possible
//!
//! First iteration (Not used anymore):
//!
//! Structs for each left hand object.
//! Global random number generator
//! Using Rust's Default trait to generate all strings

use crate::{MAX_REPEAT, MAX_STEPS};
use core::sync::atomic::{AtomicU64, Ordering};

fn rdrand() -> u64 {
    let res: u64;
    unsafe {
        asm!("rdrand $0" : "=r"(res));
    }
    res
}

lazy_static! {
    static ref SEED: AtomicU64 = AtomicU64::new(rdrand());
    static ref COUNT: AtomicU64 = AtomicU64::new(0);
}

/// Rng seeded with rdrand that is generated using xorshift
fn rand() -> usize {
    let mut val = SEED.load(Ordering::SeqCst);
    val ^= val << 13;
    val ^= val >> 17;
    val ^= val << 43;
    SEED.store(val, Ordering::SeqCst);
    val as usize
}

#[derive(Debug)]
pub struct Exp {
    pub value: String,
}
impl Default for Exp {
    fn default() -> Exp {
        let _recursion = COUNT.fetch_add(1, Ordering::SeqCst);
        // : [Ee] [+\-]? INT
        // [Ee]
        let mut res = String::with_capacity(512);
        match rand() % 2 {
            0 => res.push('e'),
            1 => res.push('E'),
            _ => unreachable!(),
        }

        // [+\-]?
        if (rand() % 2) == 1 {
            match rand() % 2 {
                0 => res.push('+'),
                1 => res.push('-'),
                _ => unreachable!(),
            }
        }

        // INT
        res.push_str(&Int::default().value);

        Exp { value: res }
    }
}
impl ToString for Exp {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug)]
pub struct Int {
    pub value: String,
}
impl Default for Int {
    fn default() -> Int {
        let _recursion = COUNT.fetch_add(1, Ordering::SeqCst);
        // '0' | [1-9] [0-9]*
        let mut res = String::with_capacity(512);
        match rand() % 2 {
            // '0'
            0 => res.push('0'),
            1 => {
                // [1-9] [0-9]*
                res.push(('1' as u8 + (rand() % 9) as u8) as char);
                for _i in 0..(rand() % MAX_REPEAT) {
                    res.push(('0' as u8 + (rand() % 10) as u8) as char)
                }
            }
            _ => unreachable!(),
        }

        Int { value: res }
    }
}

#[derive(Debug)]
pub struct Number {
    pub value: String,
}
impl Default for Number {
    fn default() -> Number {
        let _recursion = COUNT.fetch_add(1, Ordering::SeqCst);
        // '-'? INT ('.' [0-9] +)? EXP?
        let mut res = String::with_capacity(512);

        // '-'?
        if rand() % 2 == 1 {
            res.push('-');
        }

        // INT
        res.push_str(&Int::default().value);
        // ('.' [0-9]+)?
        if rand() % 2 == 1 {
            res.push('.');
            res.push(('0' as u8 + (rand() % 10) as u8) as char);
            for _ in 0..(rand() % MAX_REPEAT) {
                res.push(('0' as u8 + (rand() % 10) as u8) as char);
            }
        }

        // EXP?
        if rand() % 2 == 1 {
            res.push_str(&Exp::default().value);
        }

        Number { value: res }
    }
}

#[derive(Debug)]
pub struct Hex {
    pub value: String,
}
impl Default for Hex {
    fn default() -> Hex {
        let _recursion = COUNT.fetch_add(1, Ordering::SeqCst);
        // : [0-9a-fA-F]
        let values = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'A',
            'B', 'C', 'D', 'E', 'F',
        ];
        let value = values[rand() % values.len()].to_string();
        Hex { value }
    }
}

#[derive(Debug)]
pub struct Unicode {
    pub value: String,
}

impl Default for Unicode {
    fn default() -> Unicode {
        let _recursion = COUNT.fetch_add(1, Ordering::SeqCst);
        let mut value = "u".to_string();
        value.push_str(&Hex::default().value);
        value.push_str(&Hex::default().value);
        value.push_str(&Hex::default().value);
        value.push_str(&Hex::default().value);
        Unicode { value }
    }
}

#[derive(Debug)]
pub struct SafeCodePoint {
    pub value: String,
}
impl Default for SafeCodePoint {
    fn default() -> SafeCodePoint {
        let _recursion = COUNT.fetch_add(1, Ordering::SeqCst);
        let mut num;
        let mut value;
        loop {
            // Try all possible unicode values
            num = (rand() % 0x10ffff) as u32;

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
        SafeCodePoint {
            value: value.unwrap().to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Escape {
    pub value: String,
}
impl Default for Escape {
    fn default() -> Escape {
        let _recursion = COUNT.fetch_add(1, Ordering::SeqCst);
        // '\\' (["\\/bfnrt] | UNICODE)
        let mut value = String::with_capacity(512);

        // '\\'
        value.push('\\');
        match rand() % 2 {
            // | UNICODE)
            0 => value.push_str(&Unicode::default().value),
            1 => {
                // (["\\/bfnrt]
                let values = ['"', '\\', '/', 'b', 'f', 'n', 'r', 't'];
                value.push(values[rand() % values.len()]);
            }
            _ => unreachable!(),
        }

        Escape { value }
    }
}

#[derive(Debug)]
pub struct JsonString {
    pub value: String,
}
impl Default for JsonString {
    fn default() -> JsonString {
        let recursion = COUNT.fetch_add(1, Ordering::SeqCst);
        let mut value = String::with_capacity(512);
        value.push('"');
        if recursion <= MAX_STEPS {
            for _ in 0..(rand() % MAX_REPEAT) {
                match rand() % 2 {
                    0 => value.push_str(&Escape::default().value),
                    1 => value.push_str(&SafeCodePoint::default().value),
                    _ => unreachable!(),
                }
            }
        }
        value.push('"');
        JsonString { value }
    }
}

#[derive(Debug)]
pub struct JsonValue {
    pub value: String,
}
impl Default for JsonValue {
    fn default() -> JsonValue {
        let recursion = COUNT.fetch_add(1, Ordering::SeqCst);
        if recursion >= MAX_STEPS {
            return JsonValue {
                value: Number::default().value,
            };
        }
        let value = match rand() % 100 {
            0..15 => JsonString::default().value,
            15..30 => Number::default().value,
            30..60 => JsonObject::default().value,
            60..98 => JsonArray::default().value,
            98 => "true".to_string(),
            99 => "false".to_string(),
            100 => "null".to_string(),
            _ => unreachable!(),
        };
        JsonValue { value }
    }
}

#[derive(Debug)]
pub struct JsonPair {
    pub value: String,
}
impl Default for JsonPair {
    fn default() -> JsonPair {
        let _recursion = COUNT.fetch_add(1, Ordering::SeqCst);
        let mut value = String::with_capacity(512);
        value.push_str(&JsonString::default().value);
        value.push(':');
        value.push_str(&JsonValue::default().value);
        JsonPair { value }
    }
}

#[derive(Debug)]
pub struct JsonArray {
    pub value: String,
}
impl Default for JsonArray {
    fn default() -> JsonArray {
        let recursion = COUNT.fetch_add(1, Ordering::SeqCst);
        let mut value = String::with_capacity(512);
        match rand() % 10 {
            0 => {
                value.push('[');
                value.push(']');
            }
            _ => {
                value.push('[');
                value.push_str(&JsonValue::default().value);
                if recursion <= MAX_STEPS {
                    for _ in 0..(rand() % MAX_REPEAT) {
                        value.push(',');
                        value.push_str(&JsonValue::default().value);
                    }
                }
                value.push(']');
            }
        }
        JsonArray { value }
    }
}

#[derive(Debug)]
pub struct JsonObject {
    pub value: String,
}
impl Default for JsonObject {
    fn default() -> JsonObject {
        let recursion = COUNT.fetch_add(1, Ordering::SeqCst);
        let mut value = String::with_capacity(512);
        match rand() % 10 {
            0 => {
                value.push('{');
                value.push('}');
            }
            _ => {
                value.push('{');
                value.push_str(&JsonPair::default().value);
                if recursion <= MAX_STEPS {
                    for _ in 0..(rand() % MAX_REPEAT) {
                        value.push(',');
                        value.push_str(&JsonPair::default().value);
                    }
                }
                value.push('}');
            }
        }
        JsonObject { value }
    }
}

#[derive(Debug)]
pub struct Json {
    pub value: String,
}
impl Default for Json {
    fn default() -> Json {
        let _recursion = COUNT.fetch_add(1, Ordering::SeqCst);
        Json {
            value: JsonValue::default().value,
        }
    }
}
