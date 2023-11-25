/*use binrw::{binrw, BinReaderExt, BinWriterExt, NullString};
use binrw::io::{Cursor, Seek, SeekFrom};
use serde::{Serialize, Deserialize};


use super::{NuccBinaryParsed, NuccBinaryType};

#[binrw]
#[brw(little)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry { // param entry, looks like each entry is 0x24 bytes? sometimes 0x20?
    pub frame_min: u32,
    pub frame_max: u32,

    pub offset_min_x: f32,
    pub offset_min_y: f32,
    pub offset_min_z: f32,

    pub offset_max_x: f32,
    pub offset_max_y: f32,
    pub offset_max_z: f32,

    pub offset_type: u32
}

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Anmofs {

    #[brw(big)]
    pub size: u32,

    // need to implement fixed size string for serde and binrw
    #[br(count = 0x40)]
    #[serde(skip)]
    pub anm_name: Vec<u8>,
     // 0x40 bytes reserved for it?
     #[serde(skip)]
    pub characode: [u8; 0x10], // then characode, 0x10 bytes reserved?

    pub entry_count: u32, // amount of param entries

    //pub unk0: u32, // 0x0

    // count is located before the entries
    #[br(count = entry_count)]
    #[brw(pad_before = 0x4)]
    pub entries: Vec<Entry>
}

impl NuccBinaryParsed for Anmofs {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::Anmofs
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
}*/



