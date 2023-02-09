use std::fmt::Display;

use clap::{Arg, Command};

async fn hammer_with_args(args: Vec<String>) -> Result<Vec<Stats>, Error> {
    let matches = Command::new("hammer").arg(Arg::new("url").short('u').required(true).long("url"));

    let matchs = matches.get_matches_from(args);

    let url = matchs.get_one::<String>("url").unwrap();

    let response = reqwest::get(url).await.unwrap();

    let stats = vec![Stats {
        code: response.status().as_u16(),
    }];
    Ok(stats)
}

struct Stats {
    code: u16,
}

#[derive(Debug)]
enum Error {
    General(String),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::General(message) => f.write_str(message),
        }
    }
}

#[cfg(test)]
mod tests {

    use httpmock::MockServer;

    use super::*;

    #[tokio::test]
    async fn it_executes_a_request() {
        let server = MockServer::start_async().await;
        server
            .mock_async(|when, then| {
                when.path("/hammer");
                then.status(200);
            })
            .await;

        let mut args: Vec<String> = Vec::new();

        args.push("hammer".to_string());
        args.push("-u".to_string());
        let mut server_url = server.base_url();
        server_url.push_str("/hammer");
        args.push(server_url);

        match hammer_with_args(args).await {
            Ok(stats) => {
                assert_eq!(stats.first().unwrap().code, 200)
            }

            Err(err) => {
                panic!("error not expected: {}", err);
            }
        }
    }
}
