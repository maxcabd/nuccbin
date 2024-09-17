use binrw::binrw;
use serde::{Serialize, Deserialize};

use super::{NuccBinaryParsed, NuccBinaryType};

const STR_LEN: usize = 0x20;


#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[br(map = |x: Vec<u8>| String::from_utf8_lossy(&x).trim_end_matches('\u{0}').to_string(), count = STR_LEN)] // Need to trim the null bytes
    #[bw(map = |x: &String| (x.clone() + String::from('\u{0}').repeat(STR_LEN - x.len()).as_str()).into_bytes())]
    pub sound_name: String,

    #[brw(pad_before = 0x2)]
    pub volume: f32,
    pub unk1: i16,
    pub unk2: i16,

    pub unk3: i16,
    pub pitch: i16,

    pub unk4: f32,
    pub timing: f32,

    pub index: i16,
    pub unk6: i16,
    pub unk7: i16,

    pub unk8: f32,

}

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Snd {
    #[bw(calc = self.entries.len() as u16)]
    pub entry_count: u16,

    #[br(count = entry_count)]
    pub entries: Vec<Entry>
}


impl NuccBinaryParsed for Snd {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::Snd
    }

    fn extension(&self) -> String {
        String::from(".json")
    }

    fn serialize(&self) -> Vec<u8> {
        serde_json::to_string_pretty(self).unwrap().into()
    }

    fn deserialize(data: &[u8]) -> Self
        where
            Self: Sized,

        {   
            serde_json::from_slice(data).unwrap()
        }
}