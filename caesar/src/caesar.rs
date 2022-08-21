use std::collections::HashMap;

pub enum Mode {
    Encrypt,
    Decrypt,
}

pub fn caesar(input: &str, shift: i32, dir: Mode) -> String {
    let alphabet_pos = [
        'a', 'b', 'c', 'd', 'e',
        'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't',
        'u', 'v', 'w', 'x', 'y', 'z'
    ];

    let mut alphabet_dic = HashMap::new();
    for (index, letter) in alphabet_pos.iter().enumerate() {
        alphabet_dic.insert(*letter, index);
    }

    let mut result = String::new();

    for ic in input.chars() {
        let ic_lower: Vec<_> = ic.to_lowercase().collect();
        match alphabet_dic.get(&ic_lower.get(0).unwrap()) {
            Some(index) => {
                let calc_index: usize;
                match dir {
                    Mode::Encrypt => calc_index = calc_index_forward(index, shift),
                    Mode::Decrypt => calc_index = calc_index_backward(index, shift)
                }
                let matched_char = alphabet_pos.get(calc_index).unwrap().to_string();
                if ic.is_uppercase() {
                    result.push_str(matched_char.to_uppercase().as_str());
                    continue;
                }
                result.push_str(matched_char.as_str());
            }
            None => result.push_str(ic.to_string().as_str())
        }
    }
    return result;
}

fn calc_index_forward(letter_index: &usize, shift: i32) -> usize {
    let li = *letter_index as i32;
    let result = (li + shift) % 26;
    return result as usize;
}

fn calc_index_backward(letter_index: &usize, shift: i32) -> usize {
    let li = *letter_index as i32;
    let mut result = (li - shift) % 26;
    if result.is_negative() {
        result = 26 + result
    }
    return result as usize;
}


#[cfg(test)]
mod tests {
    use crate::caesar::{caesar, Mode};

    #[test]
    fn it_encrypts_basic_string() {
        let result = caesar("ABC", 1, Mode::Encrypt);
        assert_eq!(result, "BCD");
    }

    #[test]
    fn it_decrypts_basic_string() {
        let result = caesar("BCD", 1, Mode::Decrypt);
        assert_eq!(result, "ABC");
    }

    #[test]
    fn it_ignores_but_keeps_non_alphabet_characters() {
        let result = caesar("(ABC)D", 1, Mode::Encrypt);
        assert_eq!(result, "(BCD)E");
    }

    #[test]
    fn it_respects_spaces() {
        let result = caesar("A B C", 1, Mode::Encrypt);
        assert_eq!(result, "B C D");
    }

    #[test]
    fn it_respects_multiline() {
        let result = caesar("A \n B \n C", 1, Mode::Encrypt);
        assert_eq!(result, "B \n C \n D");
    }

    #[test]
    fn it_respects_capitalization() {
        let result = caesar("ABC", 1, Mode::Encrypt);
        assert_eq!(result, "BCD");

        let result = caesar("abc", 1, Mode::Encrypt);
        assert_eq!(result, "bcd");
    }

    #[test]
    fn it_ignores_but_keeps_utf8_chars() {
        let result = caesar("ЗaЗ", 1, Mode::Encrypt);
        assert_eq!(result, "ЗbЗ")
    }

    #[test]
    fn it_handles_last_alpha_pos_encrypt() {
        let result = caesar("ABC", 26, Mode::Encrypt);
        assert_eq!(result, "ABC");
    }

    #[test]
    fn it_handles_last_alpha_pos_decrypt() {
        let result = caesar("ABC", 26, Mode::Decrypt);
        assert_eq!(result, "ABC");
    }

    #[test]
    // Letters close to the end and displacement exceeds last alpha.
    fn it_handles_relative_upper_overflow() {
        let result = caesar("XY", 3, Mode::Encrypt);
        assert_eq!(result, "AB");
    }

    #[test]
    // Letters close to the end and displacement exceeds last alpha.
    fn it_handles_relative_lower_overflow() {
        let result = caesar("BC", 3, Mode::Decrypt);
        assert_eq!(result, "YZ");
    }

    #[test]
    // Two times the alphabet + 2 (forward).
    fn it_handles_upper_bound_overflow() {
        let result = caesar("ABC", 54, Mode::Encrypt);
        assert_eq!(result, "CDE");
    }

    #[test]
    // Two times the alphabet + 3 (backward).
    fn it_handles_lower_bound_overflow() {
        let result = caesar("ABC", 55, Mode::Decrypt);
        assert_eq!(result, "XYZ");
    }

    #[test]
    fn it_returns_same_on_no_shift() {
        let result = caesar("ABC", 0, Mode::Encrypt);
        assert_eq!(result, "ABC");
    }
}
