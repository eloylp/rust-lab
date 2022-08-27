use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

const HELP: &str = "🏛 Caesar Cipher 🏛

WARNING: Users are encouraged to use modern cryptography instead of this tool.
This was only done for academic purposes with ❤ 🦀

Only -s argument is mandatory. If no other argument is provided stdin/stdout and
encryption mode are assumed.

Arguments:

-h     Shows this menu
-s     The shift, or key of the cipher (mandatory).
-o     Write results to specified file.
-i     Specify path to input file.
-e     Encryption mode. (default)
-d     Decryption mode.

Here's a full example command:

$ caesar -s 10 -i input.txt -o output.txt -e
";


#[derive(Debug)]
pub struct Args {
    pub help: bool,
    pub shift: i32,
    pub output: String,
    pub input: String,
    pub encrypt: bool,
    pub decrypt: bool,
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Received arguments are:
        -h {}
        -s {}
        -o {}
        -i {}
        -e {}
        -d {}
        ", self.help, self.shift, self.output, self.input, self.encrypt, self.decrypt)
    }
}

#[derive(Debug, PartialEq)]
pub struct ArgsError;

impl Display for ArgsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", HELP)
    }
}

impl Error for ArgsError {
    fn description(&self) -> &str {
        HELP
    }
}


type Result<T> = std::result::Result<T, ArgsError>;

pub(super) fn parse(args: &[String]) -> Result<Args> {
    if args.is_empty() {
        return Err(ArgsError);
    }
    let mut parsed_args = Args {
        help: false,
        shift: 0,
        input: "".to_string(),
        output: "".to_string(),
        encrypt: false,
        decrypt: false,
    };

    for (i, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "-h" => return Err(ArgsError),
            "-s" => {
                let arg_val = i + 1;
                if args.get(arg_val) == None {
                    return Err(ArgsError);
                }
                match args[arg_val].parse() {
                    Ok(val) => parsed_args.shift = val,
                    Err(_error) => return Err(ArgsError)
                }
            }
            "-i" => {
                let arg_val = i + 1;
                if args.get(arg_val) == None {
                    return Err(ArgsError);
                }
                parsed_args.input = args[arg_val].to_string()
            }
            "-o" => {
                let arg_val = i + 1;
                if args.get(arg_val) == None {
                    return Err(ArgsError);
                }
                parsed_args.output = args[arg_val].to_string()
            }
            "-e" => {
                parsed_args.encrypt = true
            }
            "-d" => {
                parsed_args.decrypt = true
            }
            _ => {}
        }
    }
    if parsed_args.encrypt && parsed_args.decrypt {
        return Err(ArgsError);
    }
    return Ok(parsed_args);
}


#[cfg(test)]
mod test {
    use crate::cli::args::{ArgsError, parse};

    #[test]
    fn it_parses_args_for_encryption() {
        let args = vec![
            "-s".to_string(),
            "10".to_string(),
            "-i".to_string(),
            "/home/user/in.txt".to_string(),
            "-o".to_string(),
            "/home/user/out.txt".to_string(),
            "-e".to_string(),
        ];

        let result = parse(&args).unwrap();

        assert_eq!(10, result.shift);
        assert_eq!("/home/user/in.txt", result.input);
        assert_eq!("/home/user/out.txt", result.output);
        assert_eq!(true, result.encrypt);
    }

    #[test]
    fn it_parses_args_for_decryption() {
        let args = vec![
            "-s".to_string(),
            "10".to_string(),
            "-d".to_string(),
        ];
        let result = parse(&args).unwrap();

        assert_eq!(10, result.shift);
        assert_eq!(true, result.decrypt);
    }

    #[test]
    fn it_exits_if_help_present() {
        let args = vec![
            "-h".to_string(),
            "-s".to_string(),
            "10".to_string(),
        ];
        let result = parse(&args).unwrap_err();
        assert_eq!(ArgsError, result);
    }

    #[test]
    fn it_returns_error_when_missing_shift_param_value() {
        let args = vec![
            "-s".to_string(),
        ];
        let result = parse(&args).unwrap_err();
        assert_eq!(ArgsError, result)
    }

    #[test]
    fn it_returns_error_when_missing_input_param_value() {
        let args = vec![
            "-i".to_string(),
        ];
        let result = parse(&args).unwrap_err();
        assert_eq!(ArgsError, result)
    }

    #[test]
    fn it_returns_error_when_missing_output_param_value() {
        let args = vec![
            "-o".to_string(),
        ];
        let result = parse(&args).unwrap_err();
        assert_eq!(ArgsError, result)
    }

    #[test]
    fn it_returns_error_when_no_args() {
        let args = vec![];
        let res = parse(&args).unwrap_err();
        assert_eq!(ArgsError, res)
    }

    #[test]
    fn it_returns_error_when_cannot_parse_arg_val() {
        let args = vec![
            "-s".to_string(),
            "aaa".to_string(),
        ];
        let res = parse(&args).unwrap_err();
        assert_eq!(ArgsError, res)
    }

    #[test]
    fn it_returns_error_when_encrypt_and_decrypt_are_activated() {
        let args = vec![
            "-e".to_string(),
            "-d".to_string(),
        ];
        let res = parse(&args).unwrap_err();
        assert_eq!(ArgsError, res)
    }
}

