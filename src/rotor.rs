use crate::common::{ASCII_OFFSET, NO_LETTERS_IN_ALPHABET};
// 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25
// A B C D E F G H I J K  L  M  N  O  P  Q  R  S  T  U  V  W  X  Y  Z
#[derive(Copy, Clone)]
pub enum RotorName {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
    Identity,
}
impl RotorName {
    fn wiring(self) -> String {
        match self {
            RotorName::I => String::from("EKMFLGDQVZNTOWYHXUSPAIBRCJ"),
            RotorName::II => String::from("AJDKSIRUXBLHWTMCQGZNPYFVOE"),
            RotorName::III => String::from("BDFHJLCPRTXVZNYEIWGAKMUSQO"),
            RotorName::IV => String::from("ESOVPZJAYQUIRHXLNFTGKDCMWB"),
            RotorName::V => String::from("VZBRGITYUPSDNHLXAWMJQOFECK"),
            RotorName::VI => String::from("JPGVOUMFYQBENHZRDKASXLICTW"),
            RotorName::VII => String::from("NZJHGRCXMYSWBOUFAIVLPEKQDT"),
            RotorName::VIII => String::from("FKQHTLXOCBJSPDZRAMEWNIUYGV"),
            RotorName::Identity => String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
        }
    }
    fn notch_position(self) -> u32 {
        match self {
            RotorName::I => 16,
            RotorName::II => 4,
            RotorName::III => 21,
            RotorName::IV => 9,
            RotorName::V => 25,
            RotorName::VI | RotorName::VII | RotorName::VIII | RotorName::Identity => 0,
        }
    }
}
#[derive(PartialEq, Debug)]
pub enum NotchStatus {
    AtANotch,
    NotAtANotch,
}

pub struct Rotor {
    name: RotorName,
    wiring: String,
    forward_wiring: Vec<u32>,
    backward_wiring: Vec<u32>,
    rotor_position: u32,
    notch_position: u32,
    ring_setting: u32,
}
impl Rotor {
    pub fn new(name: RotorName, rotor_position: u32, ring_setting: u32) -> Self {
        Rotor {
            name,
            rotor_position,
            ring_setting,
            wiring: String::new(),
            notch_position: 0,
            forward_wiring: Vec::<u32>::new(),
            backward_wiring: Vec::<u32>::new(),
        }
    }
    pub fn set_encoding(&mut self) {
        self.wiring = self.name.wiring();
        self.notch_position = self.name.notch_position();
        self.set_wiring();
    }
    pub fn notch_status(&self) -> NotchStatus {
        match self.name {
            RotorName::VI | RotorName::VII | RotorName::VIII => {
                if self.rotor_position == 12 || self.rotor_position == 25 {
                    NotchStatus::AtANotch
                } else {
                    NotchStatus::NotAtANotch
                }
            }
            _ => {
                if self.notch_position == self.rotor_position {
                    NotchStatus::AtANotch
                } else {
                    NotchStatus::NotAtANotch
                }
            }
        }
    }

    pub fn turnover(&mut self) {
        self.rotor_position = (self.rotor_position + 1) % NO_LETTERS_IN_ALPHABET;
        self.forward_wiring.rotate_right(1);
        self.backward_wiring.rotate_right(1);
    }

    pub fn position(&self) -> u32 {
        self.rotor_position
    }
    fn decode_wiring(wiring: &str) -> Vec<u32> {
        wiring
            .chars()
            .map(|x| x as u32 - ASCII_OFFSET)
            .collect::<Vec<u32>>()
    }
    fn set_wiring(&mut self) {
        self.forward_wiring = Rotor::decode_wiring(&self.wiring);
        self.backward_wiring = self.inverse_wiring();

        self.forward_wiring = self.rewrire_rotors(&self.forward_wiring);
        self.backward_wiring = self.rewrire_rotors(&self.backward_wiring);
    }

    fn rewrire_rotors(&self, wiring: &Vec<u32>) -> Vec<u32> {
        let mut return_wiring = wiring.clone();
        let a_dot_pos = return_wiring.iter().position(|&x| x == 0).unwrap();
        return_wiring = return_wiring
            .iter()
            .map(|&x| (x + self.ring_setting) % 26)
            .collect();
        let new_dot_pos = (a_dot_pos + self.ring_setting as usize) % 26;
        let ring_letter_pos = return_wiring
            .iter()
            .position(|&x| x == self.ring_setting)
            .unwrap();
        if new_dot_pos > ring_letter_pos {
            return_wiring.rotate_right(new_dot_pos - ring_letter_pos)
        } else {
            return_wiring.rotate_left(ring_letter_pos - new_dot_pos)
        };
        return_wiring.rotate_left(self.rotor_position as usize);
        return_wiring
    }

    fn inverse_wiring(&self) -> Vec<u32> {
        let mut ret_vec: Vec<u32> = vec![0; self.forward_wiring.len()];
        for (index, _val) in self.forward_wiring.iter().enumerate() {
            let forward = self.forward_wiring[index];
            ret_vec[forward as usize] = index as u32;
        }
        ret_vec
    }

    fn encipher(character: u32, pos: i32, ring: i32, mapping: &[u32]) -> u32 {
        let shift: i32 = pos - ring;
        let calc = ((character as i32 + shift + NO_LETTERS_IN_ALPHABET as i32)
            % NO_LETTERS_IN_ALPHABET as i32) as usize;
        let calc2: i32 = mapping[calc] as i32 - shift + NO_LETTERS_IN_ALPHABET as i32;
        (calc2 % NO_LETTERS_IN_ALPHABET as i32) as u32
    }

    pub fn forward(&self, character: u32) -> u32 {
        self.forward_wiring[character as usize]
    }

    pub fn backward(&self, character: u32) -> u32 {
        self.backward_wiring[character as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_at_a_notch() {
        let mut rotor = Rotor::new(RotorName::II, 1, 2);
        rotor.set_encoding();
        assert_eq!(rotor.notch_status(), NotchStatus::NotAtANotch);
        rotor.turnover();
        rotor.turnover();
        rotor.turnover();
        assert_eq!(rotor.notch_status(), NotchStatus::AtANotch);
    }
    #[test]
    fn is_at_a_notch_double_step() {
        let mut rotor = Rotor::new(RotorName::VI, 10, 2);
        rotor.set_encoding();
        assert_eq!(rotor.notch_status(), NotchStatus::NotAtANotch);
        rotor.turnover();
        rotor.turnover();
        assert_eq!(rotor.notch_status(), NotchStatus::AtANotch);

        let mut rotor2 = Rotor::new(RotorName::VI, 20, 2);
        rotor2.set_encoding();
        assert_eq!(rotor2.notch_status(), NotchStatus::NotAtANotch);
        rotor2.turnover();
        rotor2.turnover();
        rotor2.turnover();
        rotor2.turnover();
        rotor2.turnover();
        assert_eq!(rotor2.notch_status(), NotchStatus::AtANotch);
    }
    #[test]
    fn encode_forward() {
        let mut rotor = Rotor::new(RotorName::II, 1, 2);
        rotor.set_encoding();
        assert_eq!(rotor.forward(1), 2);
        assert_eq!(rotor.forward(9), 25);
        assert_eq!(rotor.forward(21), 17);
    }
    #[test]
    fn encode_backward() {
        let mut rotor = Rotor::new(RotorName::II, 1, 2);
        rotor.set_encoding();
        assert_eq!(rotor.backward(1), 1);
        assert_eq!(rotor.backward(9), 6);
        assert_eq!(rotor.backward(21), 8);
    }
    #[test]
    fn ring_settings() {
        let mut rotor = Rotor::new(RotorName::I, 1, 2);
        rotor.set_encoding();
        assert_eq!(rotor.forward_wiring[1], 4);
    }
}
