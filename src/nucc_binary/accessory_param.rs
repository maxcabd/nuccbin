use binrw::{binrw, BinReaderExt, BinWriterExt, NullString};
use binrw::io::{Cursor, Seek, SeekFrom};
use serde::{Serialize, Deserialize};

use super::{NuccBinaryParsed, NuccBinaryType};

use super::HEADER_SIZE;

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[serde(skip)]
    pub accessory_ptr: u64,
    pub unk1: u32,
    pub unk2: u32,

    pub head: u32, // A value of 0x1 means it's a head accessory
    pub face: u32,
    pub unk5: u32,
    pub unk6: u32,

    pub back: u32,
    pub waist: u32,
    pub unk9: u32,
    pub arms: u32,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub accessory: String,
}

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct AccessoryParam {
    #[serde(skip)]
    pub version: u32,

    pub entry_count: u32,

    #[serde(skip)]
    pub entry_ptr: u64,

    #[br(count = entry_count)]
    pub entries: Vec<Entry>
}

impl NuccBinaryParsed for AccessoryParam {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::AccessoryParam
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


impl From<&[u8]> for AccessoryParam {
    fn from(data: &[u8]) -> Self {
        let mut reader = Cursor::new(data);
        
        let version = reader.read_le::<u32>().unwrap();

        let entry_count = reader.read_le::<u32>().unwrap();
        let entry_ptr = reader.read_le::<u64>().unwrap();

        let mut entries = Vec::new();
        entries.reserve_exact(entry_count as usize); // Make sure we reserve enough space to avoid reallocations

        for _ in 0..entry_count as usize {
            let entry = reader.read_le::<Entry>().unwrap();
            entries.push(entry);
        }

        fn read_string_from_ptr(reader: &mut Cursor<&[u8]>, ptr: u64, curent_offset: u64) -> String {
            if ptr != 0 {
                reader.seek(SeekFrom::Start(curent_offset as u64)).unwrap();
                reader.seek(SeekFrom::Current(ptr as i64)).unwrap();
                reader.read_be::<NullString>().unwrap().to_string()
            } else {
                String::from("")
            }
        }

        for (current_offset, entry) in entries
        .iter_mut()
        .enumerate()
        .map(|(i, e)| (((0x30 * i + HEADER_SIZE) as u64, e))) 
        {
            entry.accessory = read_string_from_ptr(&mut reader, entry.accessory_ptr, current_offset);
            
        }

        Self {
            version,
            entry_count,
            entry_ptr,
            entries
        }

    }
}

impl From<AccessoryParam> for Vec<u8> {
    fn from(mut accessory_param: AccessoryParam) -> Self {
        // Consumes the deserialized version and returns the bytes
        let mut writer = Cursor::new(Vec::new());

        accessory_param.entry_count = accessory_param.entries.len() as u32; // Update entry count

        writer.write_le(&1000u32).unwrap(); // Write the version
        writer.write_le(&accessory_param.entry_count).unwrap();
        writer.write_le(&8u64).unwrap(); // Write the ptr to the entries
        writer.write_le(&accessory_param.entries).unwrap();

        fn write_ptr_to_string(
            writer: &mut Cursor<Vec<u8>>,
            string: &String,
            current_offset: u64,
            adjustment: u64,
        ) {
            if !string.is_empty() {
               writer.seek(SeekFrom::End(0)).unwrap();
                let string_pos = writer.seek(SeekFrom::End(0)).unwrap();
                writer.write_be::<NullString>(&NullString::from(string.clone())).unwrap();

                // Align to 8 bytes
                let pos = writer.seek(SeekFrom::Current(0)).unwrap() - string_pos;
                if 8 - (pos % 8) != 8  {
                    writer.write_le::<Vec<u8>>(&vec![0; 8 - (pos % 8) as usize]).unwrap();
                }

                writer.seek(SeekFrom::Start((current_offset + adjustment) as u64)).unwrap();
                writer.write_le::<u64>(&(string_pos - current_offset - &adjustment)).unwrap();
                
            }
        }

        for (current_offset, entry) in accessory_param.entries
            .iter_mut()
            .enumerate()
            .map(|(i, e)| (((0x30 * i + HEADER_SIZE) as u64, e)))
        {
            write_ptr_to_string(&mut writer, &entry.accessory, current_offset as u64, 0x0);
        }
        
        writer.into_inner()

    }
}






    