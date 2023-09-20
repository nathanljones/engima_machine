use crate::common::ASCII_OFFSET;

pub enum ReflectorType {
    B,
    C,
    Default,
}
impl ReflectorType {
    fn wiring(&self) -> String {
        match self {
            ReflectorType::B => "YRUHQSLDPXNGOKMIEBFZCWVJAT".to_string(),
            ReflectorType::C => "FVPJIAOYEDRZXWGCTKUQSBNMHL".to_string(),
            ReflectorType::Default => "ZYXWVUTSRQPONMLKJIHGFEDCBA".to_string(),
        }
    }
}
pub struct Reflector {
    reflector_type: ReflectorType,
    forward_wiring: Vec<u32>,
}
impl Reflector {
    pub fn new(reflector_type: ReflectorType) -> Self {
        Reflector {
            reflector_type: reflector_type,
            forward_wiring: vec![],
        }
    }
    pub fn set_wiring(&mut self) {
        self.forward_wiring = self.decode_wiring(&self.reflector_type.wiring());
    }
    fn decode_wiring(&self, wiring: &str) -> Vec<u32> {
        wiring
            .chars()
            .map(|x| x as u32 - ASCII_OFFSET)
            .collect::<Vec<u32>>()
    }
    pub fn forward(&self, c: usize) -> u32 {
        self.forward_wiring[c]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_forward_value_b() {
        let mut reflector = Reflector::new(ReflectorType::B);
        reflector.set_wiring();
        assert_eq!(reflector.forward(24), 0);
        assert_eq!(reflector.forward(19), 25);
    }
    #[test]
    fn get_forward_value_c() {
        let mut reflector = Reflector::new(ReflectorType::C);
        reflector.set_wiring();
        assert_eq!(reflector.forward(0), 5);
    }
}
