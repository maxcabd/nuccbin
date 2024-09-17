use binrw::binrw;
use serde::{Serialize, Deserialize};

use super::{NuccBinaryParsed, NuccBinaryType};

const STR_LEN: usize = 0x20;

// Format was reversed by TheLeonX (https://github.com/TheLeonX)
#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[br(map = |x: Vec<u8>| String::from_utf8_lossy(&x).trim_end_matches('\u{0}').to_string(), count = STR_LEN)] // Need to trim the null bytes
    #[bw(map = |x: &String| (x.clone() + String::from('\u{0}').repeat(STR_LEN - x.len()).as_str()).into_bytes())]
    pub sound_name: String,

    pub unk0: i16,
    pub volume: f32,

    #[brw(pad_after = 0x4)]
    pub pitch: i16,
    
    pub timing: i16,

    pub unk4: f32,

    pub unk5: f32,

    #[br(map = |x: Vec<u8>| String::from_utf8_lossy(&x).trim_end_matches('\u{0}').to_string(), count = STR_LEN)]
    #[bw(map = |x: &String| (x.clone() + String::from('\u{0}').repeat(STR_LEN - x.len()).as_str()).into_bytes())]
    pub anm_path: String,


    #[br(map = |x: Vec<u8>| String::from_utf8_lossy(&x).trim_end_matches('\u{0}').to_string(), count = STR_LEN)]
    #[bw(map = |x: &String| (x.clone() + String::from('\u{0}').repeat(STR_LEN - x.len()).as_str()).into_bytes())]
    pub anm_name: String,


    #[br(map = |x: Vec<u8>| String::from_utf8_lossy(&x).trim_end_matches('\u{0}').to_string(), count = STR_LEN)]
    #[bw(map = |x: &String| (x.clone() + String::from('\u{0}').repeat(STR_LEN - x.len()).as_str()).into_bytes())]
    pub bone: String,


    #[brw(pad_after = 0x4)]
    pub unk6: i32, 
    pub unk8: i32,

    pub unk9: i32,

    pub unk10: i32,

    #[br(map = |x: Vec<u8>| String::from_utf8_lossy(&x).trim_end_matches('\u{0}').to_string(), count = STR_LEN)]
    #[bw(map = |x: &String| (x.clone() + String::from('\u{0}').repeat(STR_LEN - x.len()).as_str()).into_bytes())]
    pub pl_anm: String
}

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Ev {
    #[bw(calc = self.entries.len() as u16)]
    pub entry_count: u16,

    #[br(count = entry_count)]
    pub entries: Vec<Entry>
}

impl NuccBinaryParsed for Ev {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::Ev
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