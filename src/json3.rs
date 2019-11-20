use crate::{MAX_REPEAT, MAX_DEPTH, Rng, Generate};

#[derive(Debug)]
pub enum Exp {}
impl Generate for Exp {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
        //print!("in Exp\n");
        // : [Ee] [+\-]? INT
        // [Ee] 
        let mut res = String::new();
        match rng.next() % 2 {
            0 => res.push('e'),
            1 => res.push('E'),
            _ => unreachable!()
        }

        // [+\-]?
        if (rng.next() % 2) == 1 {
            match rng.next() % 2 {
                0 => res.push('+'),
                1 => res.push('-'),
                _ => unreachable!()
            }
        }

        // INT
        res.push_str(&Int::generate(rng, depth));

        res
    }
}

#[derive(Debug)]
pub enum Int {}
impl Generate for Int {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
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
            },
            _ => unreachable!()
        }

        res
    }
}

#[derive(Debug)]
pub enum Number {}
impl Generate for Number {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
        //print!("in Num\n");
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

#[derive(Debug)]
pub enum Hex {}
impl Generate for Hex {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
        ////print!("in Hex\n");
        // : [0-9a-fA-F]
        let values = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 
            'a', 'b', 'c', 'd', 'e', 'f', 'A', 'B', 'C', 'D', 'E', 'F']; 
        values[rng.next() as usize % values.len()].to_string()
    }
}

#[derive(Debug)]
pub enum Unicode {}
impl Generate for Unicode {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
        //print!("in Uni\n");
        let mut value = "u".to_string();
        value.push_str(&Hex::generate(rng, depth));
        value.push_str(&Hex::generate(rng, depth));
        value.push_str(&Hex::generate(rng, depth));
        value.push_str(&Hex::generate(rng, depth));
        value
    }
}

#[derive(Debug)]
pub enum SafeCodePoint {}
impl Generate for SafeCodePoint {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
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
pub enum Escape {}
impl Generate for Escape {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
        //print!("in escapoe\n");
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
            },
            _ => unreachable!()
        }

        value 
    }
}

#[derive(Debug)]
pub enum JsonString {}
impl Generate for JsonString {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
        //print!("in jsonstring\n");
        let mut value = String::new();
        value.push('"');
        if *depth <= MAX_DEPTH {
            for _ in 0..(rng.next() % MAX_REPEAT) {
                match rng.next() % 2 {
                    0 => value.push_str(&Escape::generate(rng, depth)),
                    1 => value.push_str(&SafeCodePoint::generate(rng, depth)),
                    _ => unreachable!()
                }
            }
        }
        value.push('"');
        value 
    }
}

#[derive(Debug)]
pub enum JsonValue {}
impl Generate for JsonValue {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
        //print!("in jsonvalue\n");
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
            _ => unreachable!()
        };
        value
    }
}

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

#[derive(Debug)]
pub enum JsonArray {}
impl Generate for JsonArray {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
        //print!("in jsonarray\n");
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

#[derive(Debug)]
pub enum JsonObject {}
impl Generate for JsonObject {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
        //print!("in jsonoby\n");
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

#[derive(Debug)]
pub enum Json {}
impl Generate for Json {
    fn generate(rng: &mut Rng, depth: &mut u64) -> String {
        *depth += 1;
        //print!("in json\n");
        JsonValue::generate(rng, depth)
    }
}
