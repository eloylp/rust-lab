use std::env;

#[tokio::main]
async fn main() {
    let res = hammer::hammer_with_args(env::args().collect()).await;
    match res {
        Ok(stats) => println!("{:?}", stats),
        Err(err) => {
            println!("{}", err);
            std::process::exit(1)
        }
    }
}
