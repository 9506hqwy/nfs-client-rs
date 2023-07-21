use clap::{crate_version, Arg, Command};
use nfs_client::error::Error;
use nfs_client::Client;
use std::path::Path;
use url::Url;

fn main() -> Result<(), Error> {
    let matches = Command::new("ls for NFSv4")
        .version(crate_version!())
        .arg(
            Arg::new("url")
                .value_parser(check_url)
                .required(true)
                .help("NFSv4 server's URL."),
        )
        .get_matches();

    let url = matches.get_one::<Url>("url").unwrap();

    let mut client = Client::new(url.clone()).unwrap();

    let file = client.read(Some(&Path::new(".")))?;

    print!("{}", String::from_utf8_lossy(&file.data));

    Ok(())
}

fn check_url(value: &str) -> Result<Url, String> {
    let url = Url::parse(value).map_err(|_| format!("Invalid URL: {}", value))?;
    if url.scheme() == "nfs" {
        Ok(url)
    } else {
        Err(format!("Invalid Schema: {}", url.scheme()))
    }
}
