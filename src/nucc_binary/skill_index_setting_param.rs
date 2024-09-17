use binrw::binrw;
use serde::{Serialize, Deserialize};

use super::{NuccBinaryParsed, NuccBinaryType};


// Format reversed by valant96
#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub characode_index: u32,

    pub first_jutsu_skl_index: i32,
    pub second_jutsu_skl_index: i32,

}

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct SkillIndexSettingParam {
    #[serde(skip)]
    #[bw(calc = 1001)]
    pub version: u32,

    pub entry_count: u32,

    #[serde(skip)]
    #[bw(calc = 0x8)]
    pub entry_ptr: u64,

    #[br(count = entry_count)]
    pub entries: Vec<Entry>
}

impl NuccBinaryParsed for SkillIndexSettingParam {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::SkillIndexSettingParam
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