use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct Caesar {}

impl Caesar {
    const A_UPPERCASE: u8 = b'A';
    const A_LOWERCASE: u8 = b'a';

    pub fn exec(input: &str, key: i32, mode: Mode) -> Result<String, KeyError> {
        if key.is_negative() {
            return Err(KeyError);
        }
        if key > 999_999 {
            return Err(KeyError);
        }

        let mut result = String::new();

        for ic in input.chars() {
            match ic.is_ascii_alphabetic() {
                false => result.push(ic),
                true => {
                    let mut k = key;
                    if mode == Mode::Decrypt {
                        k = -key
                    }
                    match ic.is_ascii_uppercase() {
                        true => {
                            result.push(Caesar::rotate_with_base_char(Caesar::A_UPPERCASE, ic, k))
                        }
                        false => {
                            result.push(Caesar::rotate_with_base_char(Caesar::A_LOWERCASE, ic, k))
                        }
                    }
                }
            }
        }
        Ok(result)
    }

    fn rotate_with_base_char(base_char: u8, ic: char, key: i32) -> char {
        let start_pos = ic as u8 - base_char;
        let rotated_char = base_char + Caesar::rotate(start_pos as i32, key);
        rotated_char as char
    }

    fn rotate(start_pos: i32, key: i32) -> u8 {
        let mut result = (start_pos + key) % 26;
        if result.is_negative() {
            result += 26
        }
        result as u8
    }
}

#[derive(PartialEq)]
pub enum Mode {
    Encrypt,
    Decrypt,
}

#[derive(Debug, PartialEq, Eq)]
pub struct KeyError;

const KEY_ERROR_MSG: &str = "the key parameter must be a positive number between 0 - 999999.";

impl Display for KeyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", KEY_ERROR_MSG)
    }
}

impl Error for KeyError {
    fn description(&self) -> &str {
        KEY_ERROR_MSG
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_encrypts_basic_string() {
        let result = Caesar::exec("ABC", 1, Mode::Encrypt).unwrap();
        assert_eq!("BCD", result);
    }

    #[test]
    fn it_decrypts_basic_string() {
        let result = Caesar::exec("BCD", 1, Mode::Decrypt).unwrap();
        assert_eq!("ABC", result);
    }

    #[test]
    fn it_ignores_but_keeps_non_alphabet_characters() {
        let result = Caesar::exec("(ABC)D", 1, Mode::Encrypt).unwrap();
        assert_eq!("(BCD)E", result);
    }

    #[test]
    fn it_respects_spaces() {
        let result = Caesar::exec("A B C", 1, Mode::Encrypt).unwrap();
        assert_eq!("B C D", result);
    }

    #[test]
    fn it_respects_multiline() {
        let result = Caesar::exec("A \n B \n C", 1, Mode::Encrypt).unwrap();
        assert_eq!("B \n C \n D", result);
    }

    #[test]
    fn it_respects_capitalization() {
        let result = Caesar::exec("ABC", 1, Mode::Encrypt).unwrap();
        assert_eq!("BCD", result);

        let result = Caesar::exec("abc", 1, Mode::Encrypt).unwrap();
        assert_eq!("bcd", result);
    }

    #[test]
    fn it_ignores_but_keeps_utf8_chars() {
        let result = Caesar::exec("ЗaЗ", 1, Mode::Encrypt).unwrap();
        assert_eq!("ЗbЗ", result)
    }

    #[test]
    fn it_handles_last_alpha_pos_encrypt() {
        let result = Caesar::exec("ABC", 26, Mode::Encrypt).unwrap();
        assert_eq!("ABC", result);
    }

    #[test]
    fn it_handles_last_alpha_pos_decrypt() {
        let result = Caesar::exec("ABC", 26, Mode::Decrypt).unwrap();
        assert_eq!("ABC", result);
    }

    #[test]
    // Letters close to the end and displacement exceeds last alpha.
    fn it_handles_relative_upper_overflow() {
        let result = Caesar::exec("XY", 3, Mode::Encrypt).unwrap();
        assert_eq!("AB", result);
    }

    #[test]
    // Letters close to the end and displacement exceeds last alpha.
    fn it_handles_relative_lower_overflow() {
        let result = Caesar::exec("BC", 3, Mode::Decrypt).unwrap();
        assert_eq!("YZ", result);
    }

    #[test]
    // Two times the alphabet + 2 (forward).
    fn it_handles_upper_bound_overflow() {
        let result = Caesar::exec("ABC", 54, Mode::Encrypt).unwrap();
        assert_eq!("CDE", result);
    }

    #[test]
    // Two times the alphabet + 3 (backward).
    fn it_handles_lower_bound_overflow() {
        let result = Caesar::exec("ABC", 55, Mode::Decrypt).unwrap();
        assert_eq!("XYZ", result);
    }

    #[test]
    fn it_returns_same_on_no_key() {
        let result = Caesar::exec("ABC", 0, Mode::Encrypt).unwrap();
        assert_eq!("ABC", result);
    }

    #[test]
    fn it_returns_error_on_negative_key() {
        let result = Caesar::exec("ABC", -1, Mode::Encrypt).unwrap_err();
        assert_eq!(KeyError, result);
    }

    #[test]
    fn errors_key_has_display() {
        let error = KeyError {};
        assert_eq!("the key parameter must be a positive number between 0 - 999999.", format!("{}", error));
    }

    #[test]
    fn it_returns_error_on_max_key_size() {
        let result = Caesar::exec("ABC", 1_000_000, Mode::Encrypt).unwrap_err();
        assert_eq!(KeyError, result);
    }

    #[test]
    fn it_deals_with_max_key_size() {
        let result = Caesar::exec("ABC", 999_999, Mode::Encrypt).unwrap();
        assert_eq!("NOP", result);
    }
}
