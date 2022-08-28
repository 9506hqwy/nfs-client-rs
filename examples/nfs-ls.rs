use chrono::{DateTime, NaiveDateTime, Utc};
use clap::{crate_version, Arg, Command};
use nfs_client::binding;
use nfs_client::error::Error;
use nfs_client::Client;
use serde_xdr::XdrIndexer;
use std::iter::repeat;
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

    let handle = client.lookup(Some(&Path::new(".")))?;

    let attrs = vec![
        binding::FATTR4_TYPE,
        binding::FATTR4_SIZE,
        binding::FATTR4_MODE,
        binding::FATTR4_NUMLINKS,
        binding::FATTR4_OWNER,
        binding::FATTR4_OWNER_GROUP,
        binding::FATTR4_TIME_MODIFY,
    ];
    let mut entries = client.read_dir(handle, attrs)?;

    entries.sort_by_key(|e| e.name.clone());

    let owner_width = owner_max_length(&entries);
    let group_width = group_max_length(&entries);
    let size_width = size_max_length(&entries);
    for entry in entries {
        let ty = to_i32(entry.attrs.get(&binding::FATTR4_TYPE).unwrap());
        print!(
            "{}",
            if ty == binding::NfsFtype4::NF4DIR.index() {
                "d"
            } else {
                "-"
            }
        );

        let mode = to_u32(entry.attrs.get(&binding::FATTR4_MODE).unwrap());
        print!("{}", if (mode & 0x0000_0100) > 0 { "r" } else { "-" });
        print!("{}", if (mode & 0x0000_0080) > 0 { "w" } else { "-" });
        print!("{}", if (mode & 0x0000_0040) > 0 { "x" } else { "-" });
        print!("{}", if (mode & 0x0000_0020) > 0 { "r" } else { "-" });
        print!("{}", if (mode & 0x0000_0010) > 0 { "w" } else { "-" });
        print!("{}", if (mode & 0x0000_0008) > 0 { "x" } else { "-" });
        print!("{}", if (mode & 0x0000_0004) > 0 { "r" } else { "-" });
        print!("{}", if (mode & 0x0000_0002) > 0 { "w" } else { "-" });
        print!("{}", if (mode & 0x0000_0001) > 0 { "x" } else { "-" });
        print!(". ");

        let num_links = to_u32(entry.attrs.get(&binding::FATTR4_NUMLINKS).unwrap());
        print!("{} ", num_links);

        let owner = to_string(entry.attrs.get(&binding::FATTR4_OWNER).unwrap());
        let owner_padding = owner_width - owner.len();
        print!("{}{} ", owner, padding(' ', owner_padding));

        let group = to_string(entry.attrs.get(&binding::FATTR4_OWNER_GROUP).unwrap());
        let group_padding = group_width - group.len();
        print!("{}{} ", group, padding(' ', group_padding));

        let size = to_u64(entry.attrs.get(&binding::FATTR4_SIZE).unwrap());
        let size_padding = size_width - size.to_string().len();
        print!("{}{} ", padding(' ', size_padding), size);

        let date = to_datetime(entry.attrs.get(&binding::FATTR4_TIME_MODIFY).unwrap());
        let date = DateTime::<Utc>::from_utc(date, Utc);
        print!("{} ", date.to_rfc2822());

        println!("{}", entry.name);
    }

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

fn owner_max_length(entries: &[nfs_client::Entry]) -> usize {
    entries
        .iter()
        .map(|entry| to_string(entry.attrs.get(&binding::FATTR4_OWNER).unwrap()))
        .map(|owner| owner.len())
        .max()
        .unwrap_or_default()
}

fn group_max_length(entries: &[nfs_client::Entry]) -> usize {
    entries
        .iter()
        .map(|entry| to_string(entry.attrs.get(&binding::FATTR4_OWNER_GROUP).unwrap()))
        .map(|group| group.len())
        .max()
        .unwrap_or_default()
}

fn size_max_length(entries: &[nfs_client::Entry]) -> usize {
    entries
        .iter()
        .map(|entry| to_u64(entry.attrs.get(&binding::FATTR4_SIZE).unwrap()))
        .map(|size| size.to_string().len())
        .max()
        .unwrap_or_default()
}

pub fn padding(ch: char, len: usize) -> String {
    repeat(ch).take(len).collect::<String>()
}

fn to_datetime(value: &[u8]) -> NaiveDateTime {
    let secs = i64::from_be_bytes(value[0..8].try_into().unwrap());
    let nsecs = u32::from_be_bytes(value[8..12].try_into().unwrap());
    NaiveDateTime::from_timestamp_opt(secs, nsecs).unwrap()
}

fn to_string(value: &[u8]) -> String {
    String::from_utf8(value.iter().take_while(|&c| c != &0).map(|c| *c).collect()).unwrap()
}

fn to_i32(value: &[u8]) -> i32 {
    i32::from_be_bytes(value[0..4].try_into().unwrap())
}

fn to_u32(value: &[u8]) -> u32 {
    u32::from_be_bytes(value[0..4].try_into().unwrap())
}

fn to_u64(value: &[u8]) -> u64 {
    u64::from_be_bytes(value[0..8].try_into().unwrap())
}
