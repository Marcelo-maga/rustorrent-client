use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read};
use std::net::UdpSocket;

#[derive(Serialize, Deserialize, Debug)]
struct TorrentFile {
    announce: String,
}

fn main() -> io::Result<()> {
    let mut file = File::open("puppy.torrent")?;

    let mut torrent = Vec::new();
    file.read_to_end(&mut torrent)?;

    let decoded: TorrentFile =
        serde_bencode::from_bytes(&torrent).expect("Erro ao decodificar Bencode");

    let torrent_url = decoded.announce;

       prinln!("{}", torrent_url);

    Ok(())
}
