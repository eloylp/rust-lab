use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

const HELP: &str = "🏛 Caesar Cipher 🏛

WARNING: Users are encouraged to use modern cryptography instead of this tool.
This was made for academic purposes with ❤ 🦀

Only -k argument is mandatory. If no other argument is provided stdin/stdout and
encryption mode are assumed.

Arguments:

-h     Shows this menu.
-v     Shows the version.
-k     The key, or positive shift number of the cipher (mandatory). Max is a 6 digit number.
-o     Write results to specified file.
-i     Specify path to input file.
-e     Encryption mode. (default).
-d     Decryption mode.

Here's a full example command:

$ caesar -k 10 -i input.txt -o output.txt -e
";

pub(crate) fn parse(args: &[String]) -> Result<Args, ArgsError> {
    if args.is_empty() {
        return Err(ArgsError);
    }
    let mut parsed_args = Args {
        help: false,
        version: false,
        key: 0,
        input: "".to_string(),
        output: "".to_string(),
        encrypt: false,
        decrypt: false,
    };

    for (i, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "-h" => return Err(ArgsError),
            "-v" => {
                parsed_args.version = true;
                return Ok(parsed_args);
            }
            "-k" => {
                let arg_val = i + 1;
                if args.get(arg_val) == None {
                    return Err(ArgsError);
                }
                match args[arg_val].parse() {
                    Ok(val) => parsed_args.key = val,
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
    Ok(parsed_args)
}

#[derive(Debug)]
pub struct Args {
    pub version: bool,
    pub help: bool,
    pub key: i32,
    pub output: String,
    pub input: String,
    pub encrypt: bool,
    pub decrypt: bool,
}

impl Display for Args {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Received arguments are:
        -h {}
        -k {}
        -o {}
        -i {}
        -e {}
        -d {}
        ", self.help, self.key, self.output, self.input, self.encrypt, self.decrypt)
    }
}

#[derive(Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_args_for_encryption() {
        let args = vec![
            "-k".to_string(),
            "10".to_string(),
            "-i".to_string(),
            "/home/user/in.txt".to_string(),
            "-o".to_string(),
            "/home/user/out.txt".to_string(),
            "-e".to_string(),
        ];

        let result = parse(&args).unwrap();

        assert_eq!(10, result.key);
        assert_eq!("/home/user/in.txt", result.input);
        assert_eq!("/home/user/out.txt", result.output);
        assert_eq!(true, result.encrypt);
    }

    #[test]
    fn it_parses_args_for_decryption() {
        let args = vec![
            "-k".to_string(),
            "10".to_string(),
            "-d".to_string(),
        ];
        let result = parse(&args).unwrap();

        assert_eq!(10, result.key);
        assert_eq!(true, result.decrypt);
    }

    #[test]
    fn it_exits_if_help_present() {
        let args = vec![
            "-h".to_string(),
            "-k".to_string(),
            "10".to_string(),
        ];
        let result = parse(&args).unwrap_err();
        assert_eq!(ArgsError, result);
    }

    #[test]
    fn it_exits_if_version_present() {
        let args = vec![
            "-v".to_string(),
            "-k".to_string(),
            "10".to_string(),
        ];
        let result = parse(&args).unwrap();
        assert_eq!(true, result.version);
        assert_eq!(0, result.key);
    }

    #[test]
    fn it_returns_error_when_missing_key_param_value() {
        let args = vec![
            "-k".to_string(),
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
            "-k".to_string(),
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

