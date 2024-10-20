#[derive(Debug)]
pub struct PasswordStructure {
    pub long: usize,
    pub register_up: u8,
    pub register_low: u8,
    pub numbers: u8,
    pub special_char: u8,
}

pub fn vk() -> PasswordStructure {
    let long: usize = 8;
    let register_up: u8 = 1;
    let register_low: u8 = 1;
    let numbers: u8 = 1;
    let special_char: u8 = 1;
    PasswordStructure {
        long,
        register_up,
        register_low,
        numbers,
        special_char,
    }
}

pub fn ok() -> PasswordStructure {
    let long: usize = 12;
    let register_up: u8 = 1;
    let register_low: u8 = 1;
    let numbers: u8 = 1;
    let special_char: u8 = 1;
    PasswordStructure {
        long,
        register_up,
        register_low,
        numbers,
        special_char,
    }
}

pub fn default() -> PasswordStructure {
    let long: usize = 15;
    let register_up: u8 = 1;
    let register_low: u8 = 1;
    let numbers: u8 = 1;
    let special_char: u8 = 1;
    PasswordStructure {
        long,
        register_up,
        register_low,
        numbers,
        special_char,
    }
}
