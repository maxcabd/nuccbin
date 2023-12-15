use binrw::binrw;
use serde::{Serialize, Deserialize};

use super::{NuccBinaryParsed, NuccBinaryType};

// Format reversed by Portable Productions (https://www.youtube.com/@PortableProductions)

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub characode_index: u32,

    pub unk1: u32,
    pub unk2: u32,

    pub support_type: u32,

    pub unk3: i32,
    pub unk4: i32,

}

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct SupportActionParam {
    #[serde(skip)]
    pub size: u32,

    #[serde(skip)]
    #[bw(calc = 1001)]
    pub version: u32,

    pub entry_count: u16,

    #[serde(skip)]
    pub unk0: u16,

    #[serde(skip)]
    #[bw(calc = 0x8)]
    pub entry_ptr: u64,

    #[br(count = entry_count)]
    pub entries: Vec<Entry>
}

impl NuccBinaryParsed for SupportActionParam {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::SupportActionParam
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