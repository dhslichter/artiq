use std::vec::Vec;
use std::string::String;
use std::btree_map::BTreeMap;
use std::mem;
use proto::io;

#[derive(Debug)]
struct Entry {
    data: Vec<u8>
}

#[derive(Debug)]
pub struct Manager {
    entries: BTreeMap<String, Entry>,
    recording: Vec<u8>
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            entries: BTreeMap::new(),
            recording: Vec::new()
        }
    }

    pub fn record_start(&mut self) {
        self.recording.clear();
    }

    pub fn record_append(&mut self, timestamp: u64, channel: u32,
                         address: u32, data: &[u32]) {
        // See gateware/rtio/dma.py.
        let length = /*length*/1 + /*channel*/3 + /*timestamp*/8 + /*address*/2 +
                     /*data*/data.len() * 4;
        let writer = &mut self.recording;
        io::write_u8(writer, length as u8).unwrap();
        io::write_u8(writer, (channel >> 24) as u8).unwrap();
        io::write_u8(writer, (channel >> 16) as u8).unwrap();
        io::write_u8(writer, (channel >>  8) as u8).unwrap();
        io::write_u64(writer, timestamp).unwrap();
        io::write_u16(writer, address as u16).unwrap();
        for &word in data {
            io::write_u32(writer, word).unwrap();
        }
    }

    pub fn record_stop(&mut self, name: &str) {
        let mut recorded = Vec::new();
        mem::swap(&mut self.recording, &mut recorded);
        recorded.shrink_to_fit();

        info!("recorded DMA data: {:?}", recorded);

        self.entries.insert(String::from(name), Entry {
            data: recorded
        });
    }
}