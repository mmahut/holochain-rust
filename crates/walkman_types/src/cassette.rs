use crate::event::{WalkmanLogItem};
use hc::sim2h_client::Sim2hClient;
use regex::Regex;
use std::{io::BufRead};

#[derive(Debug)]
pub struct Cassette {
    events: Vec<WalkmanLogItem>,
}

impl Cassette {
    pub fn from_file(file: std::fs::File) -> Cassette {
        let buf = std::io::BufReader::new(file);
        Cassette {
            events: buf.lines()
                .map(|line| line.expect("IO error while parsing log for walkman"))
                .filter_map(parse_line).collect()
        }
    }
}

fn parse_line(line: String) -> Option<WalkmanLogItem> {
    let re_tag = Regex::new(r"<walkman>(.*?)</walkman>").unwrap();
    re_tag.captures(&line).and_then(|caps| {
        caps.get(1).and_then(|m| {
            serde_json::from_str(m.as_str()).expect("Couldn't parse serialized walkman log item")
        })
    })
}

// struct Sim2hCassettePlayer {
//     clients: HashMap<String, Sim2hClient>
// }

// impl Sim2hCassettePlayer {
//     pub fn playback(&mut self, cassette: Cassette) {
//         for event in cassette.events {
//             match event {
//                 Sim2hLogItem { timestamp: _, event } => {
//                     match event {
//                         Sim2hEvent::Connect(url) => match self.clients.entry(url) {

//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
