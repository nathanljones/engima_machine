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
            name: name,
            rotor_position,
            ring_setting,
            wiring: String::new(),
            notch_position: 0,
            forward_wiring: Vec::<u32>::new(),
            backward_wiring: Vec::<u32>::new(),
        }
    }
    pub fn set_encodind(&mut self) {
        match self.name {
            RotorName::I => {
                self.wiring = "EKMFLGDQVZNTOWYHXUSPAIBRCJ".to_string();
                self.notch_position = 16;
            }
            RotorName::II => {
                self.wiring = "AJDKSIRUXBLHWTMCQGZNPYFVOE".to_string();
                self.notch_position = 4;
            }
            RotorName::III => {
                self.wiring = "BDFHJLCPRTXVZNYEIWGAKMUSQO".to_string();
                self.notch_position = 21;
            }
            RotorName::IV => {
                self.wiring = "ESOVPZJAYQUIRHXLNFTGKDCMWB".to_string();
                self.notch_position = 9;
            }
            RotorName::V => {
                self.wiring = "VZBRGITYUPSDNHLXAWMJQOFECK".to_string();
                self.notch_position = 25;
            }
            RotorName::VI => {
                self.wiring = "JPGVOUMFYQBENHZRDKASXLICTW".to_string();
                self.notch_position = 0;
            }
            RotorName::VII => {
                self.wiring = "NZJHGRCXMYSWBOUFAIVLPEKQDT".to_string();
                self.notch_position = 0;
            }
            RotorName::VIII => {
                self.wiring = "FKQHTLXOCBJSPDZRAMEWNIUYGV".to_string();
                self.notch_position = 0;
            }
            RotorName::Identity => {
                self.wiring = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
                self.notch_position = 0;
            }
        };
    }
    pub fn is_at_a_notch(&self) -> bool {
        match self.name {
            RotorName::VI => self.rotor_position == 12 || self.rotor_position == 25,
            RotorName::VII => self.rotor_position == 12 || self.rotor_position == 25,
            RotorName::VIII => self.rotor_position == 12 || self.rotor_position == 25,
            _ => self.notch_position == self.rotor_position,
        }
    }

    pub fn turnover(&mut self) {
        self.rotor_position = (self.rotor_position + 1) % 26;
    }

    pub fn position(&self) -> u32 {
        self.rotor_position
    }
    fn decode_wiring(&self, wiring: &str) -> Vec<u32> {
        wiring.chars().map(|x| x as u32 - 65).collect::<Vec<u32>>()
    }
    fn set_wiring(&mut self) {
        self.forward_wiring = self.decode_wiring(&self.wiring);
        self.backward_wiring = self.inverse_wiring();
    }

    fn inverse_wiring(&self) -> Vec<u32> {
        let mut ret_vec: Vec<u32> = vec![0; self.forward_wiring.len()];
        for (index, _val) in self.forward_wiring.iter().enumerate() {
            let forward = self.forward_wiring[index];
            ret_vec[forward as usize] = index as u32;
        }
        ret_vec
    }
    fn encipher(&self, k: u32, pos: u32, ring: u32, mapping: &[u32]) -> u32 {
        let shift = pos - ring;
        (mapping[((k + shift + 26) % 26) as usize] - shift + 26) % 26
    }

    pub fn forward(&self, c: u32) -> u32 {
        self.encipher(
            c,
            self.rotor_position,
            self.ring_setting,
            &self.forward_wiring,
        )
    }

    pub fn backward(&self, c: u32) -> u32 {
        self.encipher(
            c,
            self.rotor_position,
            self.ring_setting,
            &self.backward_wiring,
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let mut rotor = Rotor::new(RotorName::II, 3, 2);
        rotor.set_encodind();
        rotor.set_wiring();
    }
}
