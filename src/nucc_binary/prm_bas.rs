use binrw::binrw;
use serde::{Serialize, Deserialize};

use super::{NuccBinaryParsed, NuccBinaryType};

const STR_LEN: usize = 0x8;

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[br(map = |x: Vec<u8>| String::from_utf8_lossy(&x).trim_end_matches('\u{0}').to_string(), count = STR_LEN)] // Need to trim the null bytes
    #[bw(map = |x: &String| (x.clone() + String::from('\u{0}').repeat(STR_LEN - x.len()).as_str()).into_bytes())]
    pub characode: String,

    pub modelcodes: [VecString; 0x10],

    #[brw(pad_before = 0x18)]
    pub awamodelcodes: [VecString; 0x10],

    #[br(count = 0x1C0)]
    pub extra: Vec<u8>

}

#[binrw]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VecString {
    #[br(map = |x: Vec<u8>| String::from_utf8_lossy(&x).trim_end_matches('\u{0}').to_string(), count = STR_LEN)]
    #[bw(map = |x: &String| (x.clone() + String::from('\u{0}').repeat(STR_LEN - x.len()).as_str()).into_bytes())]
    pub code: String
}


#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct PrmBas {
    #[serde(skip)]
    pub size: u32,


    pub entry: Entry
}

impl NuccBinaryParsed for PrmBas {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::PrmBas
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

