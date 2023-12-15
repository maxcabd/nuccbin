use binrw::binrw;
use serde::{Serialize, Deserialize};

use super::{NuccBinaryParsed, NuccBinaryType};


// Format reversed by Kuroha Saenoki (https://www.youtube.com/@KurohaSaenoki)
#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub combo_begin_type: i32,
    pub command: i32, // Type of move (neutral = 0x1, up = 0x2 , down = 0x3, throw = 0x4, etc)
    pub delay: i32, // Delay before next move


}

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct ComboPrm {
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

impl NuccBinaryParsed for ComboPrm {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::ComboPrm
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