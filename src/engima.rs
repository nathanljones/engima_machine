use crate::plugboard::*;
use crate::reflector::*;
use crate::rotor::*;

pub struct Enigma {
    left_rotor: Rotor,
    middle_rotor: Rotor,
    right_rotor: Rotor,

    reflector: Reflector,

    plugboard: Plugboard,
}
impl Enigma {
    fn new(
        rotors: String,
        reflector: String,
        rotor_positions: Vec<u32>,
        ring_settings: Vec<u32>,
        plugboard_connections: String,
    ) -> Self {
        Enigma {
            left_rotor: Rotor::new(RotorName::II, 3, 2),
            middle_rotor: Rotor::new(RotorName::II, 3, 2),
            right_rotor: Rotor::new(RotorName::II, 3, 2),
            reflector: Reflector::new(ReflectorType::C),
            plugboard: Plugboard::new(String::from("AZ BF")),
        }
    }
    pub fn rotate(&mut self) {
        // If middle rotor notch - double-stepping
        if self.middle_rotor.is_at_a_notch() {
            self.middle_rotor.turnover();
            self.left_rotor.turnover();
        }
        // If left-rotor notch
        else if self.right_rotor.is_at_a_notch() {
            self.middle_rotor.turnover();
        }

        // Increment right-most rotor
        self.right_rotor.turnover();
    }

    pub fn encrypt(&mut self, c: u32) -> u32 {
        self.rotate();

        // Plugboard in
        let c0 = self.plugboard.forward(c);

        // Right to left
        let c1 = self.right_rotor.forward(c0);
        let c2 = self.middle_rotor.forward(c1);
        let c3 = self.left_rotor.forward(c2);

        // Reflector
        let c4 = self.reflector.forward(c3 as usize);

        // Left to right
        let c5 = self.left_rotor.backward(c4);
        let c6 = self.middle_rotor.backward(c5);
        let c7 = self.right_rotor.backward(c6);

        // Plugboard out
        self.plugboard.forward(c7)
    }

    pub fn encrypt_char(&mut self, c: char) -> char {
        char::from_u32(self.encrypt(c as u32) + 65).unwrap()
    }

    pub fn encrypt_text(&mut self, text: &str) -> String {
        let mut return_text = String::new();
        for character in text.chars() {
            return_text.push(self.encrypt_char(character));
        }
        return_text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let mut enigma = Enigma::new(
            String::from("123"),
            String::from("123"),
            vec![1, 2, 3],
            vec![1, 2, 3],
            String::from("123"),
        );
        enigma.encrypt_char('A');
    }
}
