use crate::common::ASCII_OFFSET;
use crate::plugboard::Plugboard;
use crate::reflector::{Reflector, ReflectorType};
use crate::rotor::{NotchStatus, Rotor, RotorName};

pub struct Enigma {
    left_rotor: Rotor,
    middle_rotor: Rotor,
    right_rotor: Rotor,
    reflector: Reflector,
    plugboard: Plugboard,
}
impl Enigma {
    fn new(
        rotors: &[RotorName],
        reflector: ReflectorType,
        rotor_positions: &[u32],
        ring_settings: &[u32],
        plugboard_connections: String,
    ) -> Self {
        Enigma {
            left_rotor: Rotor::new(rotors[0], rotor_positions[0], ring_settings[0]),
            middle_rotor: Rotor::new(rotors[1], rotor_positions[1], ring_settings[1]),
            right_rotor: Rotor::new(rotors[2], rotor_positions[2], ring_settings[2]),
            reflector: Reflector::new(reflector),
            plugboard: Plugboard::new(plugboard_connections),
        }
    }
    pub fn wire_enigma(&mut self) {
        self.left_rotor.set_encoding();
        self.middle_rotor.set_encoding();
        self.right_rotor.set_encoding();

        self.reflector.set_wiring();

        self.plugboard.set_plugboard();
    }
    fn rotate(&mut self) {
        // If middle rotor notch - double-stepping
        if self.middle_rotor.notch_status() == NotchStatus::AtANotch {
            self.middle_rotor.turnover();
            self.left_rotor.turnover();
        }
        // If left-rotor notch
        else if self.right_rotor.notch_status() == NotchStatus::AtANotch {
            self.middle_rotor.turnover();
        }

        // Increment right-most rotor
        self.right_rotor.turnover();
    }

    fn encrypt(&mut self, character: u32) -> u32 {
        self.rotate();

        // Plugboard in
        let c0 = self.plugboard.forward(character);

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

    fn encrypt_char(&mut self, character: char) -> char {
        let ret_number = self.encrypt(Enigma::convert_char_to_number(character));
        Enigma::convert_number_to_char(ret_number)
    }
    fn convert_char_to_number(character: char) -> u32 {
        character as u32 - ASCII_OFFSET
    }
    fn convert_number_to_char(letter_number: u32) -> char {
        char::from_u32(letter_number + ASCII_OFFSET).unwrap()
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
    fn encrypt_test() {
        let mut enigma = Enigma::new(
            &[RotorName::I, RotorName::II, RotorName::III],
            ReflectorType::B,
            &[0, 0, 0],
            &[0, 0, 0],
            String::from(""),
        );
        let input = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZAAAAAAAAAAAAAAAAAAAAAAAAAABBBBBBBBBBBBBBBBBBBBBBBBBBABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let output = String::from("BJELRQZVJWARXSNBXORSTNCFMEYHCXTGYJFLINHNXSHIUNTHEORXOPLOVFEKAGADSPNPCMHRVZCYECDAZIHVYGPITMSRZKGGHLSRBLHL");
        enigma.wire_enigma();
        let cypher_text = enigma.encrypt_text(&input);
        assert_eq!(cypher_text, output);
    }
    #[test]
    fn encrypt_varied_rotors() {
        let mut enigma = Enigma::new(
            &[RotorName::VII, RotorName::V, RotorName::IV],
            ReflectorType::B,
            &[10, 5, 12],
            &[1, 2, 3],
            String::from(""),
        );
        let input = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZAAAAAAAAAAAAAAAAAAAAAAAAAABBBBBBBBBBBBBBBBBBBBBBBBBBABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let output = String::from("FOTYBPKLBZQSGZBOPUFYPFUSETWKNQQHVNHLKJZZZKHUBEJLGVUNIOYSDTEZJQHHAOYYZSENTGXNJCHEDFHQUCGCGJBURNSEDZSEPLQP");
        enigma.wire_enigma();
        let cypher_text = enigma.encrypt_text(&input);
        assert_eq!(cypher_text, output);
    }
    #[test]
    fn long_input() {
        let mut enigma = Enigma::new(
            &[RotorName::III, RotorName::VI, RotorName::VIII],
            ReflectorType::B,
            &[3, 5, 9],
            &[11, 13, 19],
            String::from(""),
        );
        let mut long_input: String = String::new();
        for _i in 0..500 {
            long_input.push('A')
        }
        let output = format!("{}{}{}{}{}",
        "YJKJMFQKPCUOCKTEZQVXYZJWJFROVJMWJVXRCQYFCUVBRELVHRWGPYGCHVLBVJEVTTYVMWKJFOZHLJEXYXRDBEVEHVXKQSBPYZN",
        "IQDCBGTDDWZQWLHIBQNTYPIEBMNINNGMUPPGLSZCBRJULOLNJSOEDLOBXXGEVTKCOTTLDZPHBUFKLWSFSRKOMXKZELBDJNRUDUCO",
        "TNCGLIKVKMHHCYDEKFNOECFBWRIEFQQUFXKKGNTSTVHVITVHDFKIJIHOGMDSQUFMZCGGFZMJUKGDNDSNSJKWKENIRQKSUUHJYMIG",
        "WWNMIESFRCVIBFSOUCLBYEEHMESHSGFDESQZJLTORNFBIFUWIFJTOPVMFQCFCFPYZOJFQRFQZTTTOECTDOOYTGVKEWPSZGHCTQRP",
        "GZQOVTTOIEGGHEFDOVSUQLLGNOOWGLCLOWSISUGSVIHWCMSIUUSBWQIGWEWRKQFQQRZHMQJNKQTJFDIJYHDFCWTHXUOOCVRCVYOHL");
        enigma.wire_enigma();
        let cypher_text = enigma.encrypt_text(&long_input);
        assert_eq!(cypher_text, output);
    }
    #[test]
    fn simple_4_plugs() {
        // Simple test - 4 plugs
        let mut enigma = Enigma::new(
            &[RotorName::I, RotorName::II, RotorName::III],
            ReflectorType::B,
            &[0, 0, 0],
            &[0, 0, 0],
            String::from("AC FG JY LW"),
        );
        enigma.wire_enigma();
        let input = String::from("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
        let output = enigma.encrypt_text(&input);
        let expected_output = String::from("QREBNMCYZELKQOJCGJVIVGLYEMUPCURPVPUMDIWXPPWROOQEGI");
        assert_eq!(expected_output, output);
    }
    #[test]
    fn simple_6_plugs() {
        // Simple test - 4 plugs
        let mut enigma = Enigma::new(
            &[RotorName::IV, RotorName::VI, RotorName::III],
            ReflectorType::B,
            &[0, 10, 6],
            &[0, 0, 0],
            String::from("BM DH RS KN GZ FQ"),
        );
        enigma.wire_enigma();
        let input = String::from("WRBHFRROSFHBCHVBENQFAGNYCGCRSTQYAJNROJAKVKXAHGUZHZVKWUTDGMBMSCYQSKABUGRVMIUOWAPKCMHYCRTSDEYTNJLVWNQY");
        let output = enigma.encrypt_text(&input);
        let expected_output = String::from("FYTIDQIBHDONUPAUVPNKILDHDJGCWFVMJUFNJSFYZTSPITBURMCJEEAMZAZIJMZAVFCTYTKYORHYDDSXHBLQWPJBMSSWIPSWLENZ");
        assert_eq!(expected_output, output);
    }

    #[test]
    fn simple_10_plugs() {
        // Simple test - 4 plugs
        let mut enigma = Enigma::new(
            &[RotorName::I, RotorName::II, RotorName::III],
            ReflectorType::B,
            &[0, 1, 20],
            &[5, 5, 4],
            String::from("AG HR YT KI FL WE NM SD OP QJ"),
        );
        enigma.wire_enigma();
        let input = String::from("RNXYAZUYTFNQFMBOLNYNYBUYPMWJUQSBYRHPOIRKQSIKBKEKEAJUNNVGUQDODVFQZHASHMQIHSQXICTSJNAUVZYIHVBBARPJADRH");
        let output = enigma.encrypt_text(&input);
        let expected_output = String::from("CFBJTPYXROYGGVTGBUTEBURBXNUZGGRALBNXIQHVBFWPLZQSCEZWTAWCKKPRSWOGNYXLCOTQAWDRRKBCADTKZGPWSTNYIJGLVIUQ");
        assert_eq!(expected_output, output);
    }
    #[test]
    fn decrypt_test() {
        let allrotors: Vec<RotorName> = vec![
            RotorName::I,
            RotorName::II,
            RotorName::III,
            RotorName::IV,
            RotorName::V,
            RotorName::VI,
            RotorName::VII,
            RotorName::VIII,
        ];
        let input: String = std::iter::repeat_with(|| fastrand::uppercase())
            .take(1000)
            .collect();
        let rotors_used: Vec<RotorName> = fastrand::choose_multiple(allrotors.into_iter(), 3);
        let rotor_positions = vec![
            fastrand::u32(0..25),
            fastrand::u32(0..25),
            fastrand::u32(0..25),
        ];
        let rotor_settings = vec![
            fastrand::u32(0..25),
            fastrand::u32(0..25),
            fastrand::u32(0..25),
        ];
        let mut enigma1 = Enigma::new(
            &rotors_used,
            ReflectorType::B,
            &rotor_positions,
            &rotor_settings,
            String::from(""),
        );
        enigma1.wire_enigma();
        let cypher_text = enigma1.encrypt_text(&input);
        let mut enigma2 = Enigma::new(
            &rotors_used,
            ReflectorType::B,
            &rotor_positions,
            &rotor_settings,
            String::from(""),
        );
        enigma2.wire_enigma();
        let plain_text = enigma2.encrypt_text(&cypher_text);
        assert_eq!(plain_text, input);
    }
}
