use binrw::binrw;
use serde::{Serialize, Deserialize};

use super::{NuccBinaryParsed, NuccBinaryType};


// Format reversed by Xact (https://www.youtube.com/@Xact__)
#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub characode_index: u32,

    pub skll1: f32,
    pub skll2: f32,
    pub skll3: f32,
    pub skll4: f32,
    pub skll5: f32,
    pub skll6: f32,

    pub awa_skll1: f32, // Awakening skill
    pub awa_skll2: f32,



}

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct SupportSkillRecoverySpeedParam {
    #[serde(skip)]
    pub size: u32,

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

impl NuccBinaryParsed for SupportSkillRecoverySpeedParam {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::SupportSkillRecoverySpeedParam
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