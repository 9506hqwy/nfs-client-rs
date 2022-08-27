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
                .validator(check_url)
                .required(true)
                .help("NFSv4 server's URL."),
        )
        .get_matches();

    let url = matches.value_of("url").unwrap();

    let mut client = Client::new(Url::parse(url).unwrap()).unwrap();

    let info = client.secinfo(Some(&Path::new(".")))?;
    dbg!(info);

    Ok(())
}

fn check_url(value: &str) -> Result<(), String> {
    let url = Url::parse(value).map_err(|_| format!("Invalid URL: {}", value))?;
    if url.scheme() == "nfs" {
        Ok(())
    } else {
        Err(format!("Invalid Schema: {}", url.scheme()))
    }
}
