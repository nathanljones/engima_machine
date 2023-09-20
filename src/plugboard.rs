use crate::common::{ASCII_OFFSET, NO_LETTERS_IN_ALPHABET};
pub struct Plugboard {
    wiring: Vec<u32>,
    connections: String,
}

impl Plugboard {
    pub fn new(connections: String) -> Self {
        Plugboard {
            wiring: Vec::new(),
            connections,
        }
    }
    pub fn forward(&self, c: u32) -> u32 {
        self.wiring[c as usize]
    }

    fn identity_plugboard() -> Vec<u32> {
        let mut mapping: Vec<u32> = vec![0; NO_LETTERS_IN_ALPHABET as usize];
        for i in 0..mapping.len() {
            mapping[i] = i as u32;
        }
        mapping
    }
    pub fn set_plugboard(&mut self) {
        self.wiring = self.decode_plugboard();
    }
    fn decode_plugboard(&self) -> Vec<u32> {
        if self.connections.is_empty() {
            return Plugboard::identity_plugboard();
        }
        let pairings: Vec<&str> = self.connections.split(' ').collect();

        let mut mapping: Vec<u32> = Plugboard::identity_plugboard();

        for pair in pairings {
            if pair.len() != 2 {
                return mapping;
            }
            let c1: u32 = pair.chars().next().unwrap() as u32 - ASCII_OFFSET;
            let c2: u32 = pair.chars().nth(1).unwrap() as u32 - ASCII_OFFSET;

            mapping[c1 as usize] = c2;
            mapping[c2 as usize] = c1;
        }
        mapping
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let plug_settings = String::from("AZ BF");
        let mut plugboard = Plugboard::new(plug_settings);
        plugboard.set_plugboard();
        assert_eq!(plugboard.forward(0), 25);
        assert_eq!(plugboard.forward(25), 0);
        assert_eq!(plugboard.forward(1), 5);
        assert_eq!(plugboard.forward(5), 1);
    }
}
