use crate::common::ASCII_OFFSET;
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
        rotors: Vec<RotorName>,
        reflector: ReflectorType,
        rotor_positions: Vec<u32>,
        ring_settings: Vec<u32>,
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
        self.left_rotor.set_wiring();

        self.middle_rotor.set_encoding();
        self.middle_rotor.set_wiring();

        self.right_rotor.set_encoding();
        self.right_rotor.set_wiring();

        self.reflector.set_wiring();

        self.plugboard.set_plugboard();
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
        let ret_number = (self.encrypt(c as u32 - ASCII_OFFSET));
        return char::from_u32(ret_number + ASCII_OFFSET).unwrap();
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
            vec![RotorName::I, RotorName::II, RotorName::III],
            ReflectorType::B,
            vec![0, 0, 0],
            vec![0, 0, 0],
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
            vec![RotorName::VII, RotorName::V, RotorName::IV],
            ReflectorType::B,
            vec![10, 5, 12],
            vec![1, 2, 3],
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
            vec![RotorName::III, RotorName::VI, RotorName::VIII],
            ReflectorType::B,
            vec![3, 5, 9],
            vec![11, 13, 19],
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
            vec![RotorName::I, RotorName::II, RotorName::III],
            ReflectorType::B,
            vec![0, 0, 0],
            vec![0, 0, 0],
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
            vec![RotorName::IV, RotorName::VI, RotorName::III],
            ReflectorType::B,
            vec![0, 10, 6],
            vec![0, 0, 0],
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
            vec![RotorName::I, RotorName::II, RotorName::III],
            ReflectorType::B,
            vec![0, 1, 20],
            vec![5, 5, 4],
            String::from("AG HR YT KI FL WE NM SD OP QJ"),
        );
        enigma.wire_enigma();
        let input = String::from("RNXYAZUYTFNQFMBOLNYNYBUYPMWJUQSBYRHPOIRKQSIKBKEKEAJUNNVGUQDODVFQZHASHMQIHSQXICTSJNAUVZYIHVBBARPJADRH");
        let output = enigma.encrypt_text(&input);
        let expected_output = String::from("CFBJTPYXROYGGVTGBUTEBURBXNUZGGRALBNXIQHVBFWPLZQSCEZWTAWCKKPRSWOGNYXLCOTQAWDRRKBCADTKZGPWSTNYIJGLVIUQ");
        assert_eq!(expected_output, output);
    }
}

/*

class EnigmaTest {



    @Test
    void decryptTest() {
        Random rand = new Random();
        String[] allRotors = new String[] {"I", "II", "III", "IV", "V", "VI", "VII", "VIII"};

        char[] input = new char[1000];
        for (int i = 0; i < 1000; i++) {
            input[i] = (char)(rand.nextInt(26) + 65);
        }

        for (int test = 0; test < 10; test++) {
            // Random initialisation
            String[] rotors = new String[] { allRotors[rand.nextInt(8)],
                    allRotors[rand.nextInt(8)],
                    allRotors[rand.nextInt(8)]};

            int[] startingPositions = new int[] {rand.nextInt(26),rand.nextInt(26),rand.nextInt(26)};
            int[] ringSettings = new int[] {rand.nextInt(26), rand.nextInt(26), rand.nextInt(26)};

            // Machine 1 - Encryption
            Enigma e = new Enigma(rotors, "B", startingPositions, ringSettings, "");
            char[] ciphertext = e.encrypt(input);

            // Machine 2 - Decryption
            Enigma e2 = new Enigma(rotors, "B", startingPositions, ringSettings, "");
            char[] plaintext = e2.encrypt(ciphertext);

            assertArrayEquals(input, plaintext);
        }

    }

}
*/
