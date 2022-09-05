use std::error::Error;
use std::fs;
use std::io::{BufRead, Write};

use crate::{args, caesar};
use crate::caesar::Caesar;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn with<R, W>(args: &[String], mut reader: R, mut writer: W) -> Result<(), Box<dyn Error>>
    where R: BufRead, W: Write {
    let args = args::parse(args)?;
    if args.version {
        writer.write_all(format!("{}{}\n", "v", VERSION).as_bytes())?;
        return Ok(());
    }
    let mut input = String::new();
    if args.input.is_empty() {
        reader.read_to_string(&mut input)?;
    } else {
        input = fs::read_to_string(args.input)?;
    }
    let mode: caesar::Mode = if args.decrypt {
        caesar::Mode::Decrypt
    } else {
        caesar::Mode::Encrypt
    };
    let result = Caesar::exec(input.as_str(), args.key, mode)?;
    if !args.output.is_empty() {
        fs::write(args.output, result)?;
    } else {
        writer.write_all(result.as_bytes())?;
    }
    Ok(())
}


#[cfg(test)]
mod test {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn it_uses_stdin_stdout() {
        let args = vec![
            "-k".to_string(),
            "1".to_string(),
        ];
        let input: &[u8] = b"Learning Rust";
        let mut output = Vec::new();

        with(args.as_slice(), input, &mut output).unwrap();

        let output = String::from_utf8(output).unwrap();
        assert_eq!("Mfbsojoh Svtu", output)
    }

    #[test]
    fn it_uses_input_output_files() {
        // Prepare files stuff
        let input_file_path = tmp_path();
        let contents: &[u8] = b"Learning Rust";
        fs::write(&input_file_path, contents).unwrap();
        let output_file_path = tmp_path();

        // No data in stdin/stdout
        let input: &[u8] = b"";
        let mut output = Vec::new();

        let args = vec![
            "-i".to_string(),
            input_file_path.to_owned(),
            "-o".to_string(),
            output_file_path.to_owned(),
            "-k".to_string(),
            "1".to_string(),
        ];

        with(args.as_slice(), input, &mut output).unwrap();

        let expected: &[u8] = b"Mfbsojoh Svtu";
        assert_eq!(expected, fs::read(&output_file_path).unwrap());

        fs::remove_file(input_file_path).unwrap();
        fs::remove_file(output_file_path).unwrap()
    }

    fn tmp_path() -> String {
        format!("{}{}{}", "/tmp/rust-test-", Uuid::new_v4(), ".txt")
    }

    #[test]
    fn it_shows_version() {
        let args = vec![
            "-v".to_string(),
        ];
        let input: &[u8] = b"";
        let mut output = Vec::new();

        with(args.as_slice(), input, &mut output).unwrap();

        let output = String::from_utf8(output).unwrap();
        assert_eq!(format!("{}{}\n", "v", VERSION), output)
    }
}
