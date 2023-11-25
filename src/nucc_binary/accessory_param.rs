use binrw::{binrw, BinReaderExt, BinWriterExt, NullString};
use binrw::io::{Cursor, Seek, SeekFrom};
use serde::{Serialize, Deserialize};

use super::{NuccBinaryParsed, NuccBinaryType};


const HEADER_SIZE: usize = 0x14; // Size of NUCC Binary headers

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[serde(skip)]
    pub accessory_name_id_pointer: u64,

    #[serde(skip)]
    pub acc_entry_id_pointer: u64,

    #[serde(skip)]
    pub acc_file_pointer: u64,

    #[serde(skip)]
    pub acc_code_pointer: u64,


    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,

    pub unk5: u32,
    pub unk6: u32, // 0x1
    pub unk7: u32,
    pub unk8: u32,

    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,

    pub unk13: u32,
    pub unk14: u32,
    pub unk15: u32,
    pub unk16: u32,

    pub unk17: u32,
    pub unk18: u32,
    pub unk19: u32,
    pub unk20: u32,

    pub unk21: u32,
    pub unk22: u32,


    #[serde(skip)]
    pub acc_unk_pointer: u64,


    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub accessory_name_id: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub acc_entry_id: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub acc_file: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub acc_code: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub acc_unk: String,

}

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct AccessoryParam {
    #[serde(skip)]
    pub size: u32,

    #[serde(skip)]
    pub version: u32,

    pub entry_count: u16,

    #[serde(skip)]
    pub unk0: u16,

    #[serde(skip)]
    pub entry_pointer: u64,

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
        
        let size = reader.read_be::<u32>().unwrap();
        let version = reader.read_le::<u32>().unwrap();

        let entry_count = reader.read_le::<u16>().unwrap();
        let unk0 = reader.read_le::<u16>().unwrap();

        let entry_pointer = reader.read_le::<u64>().unwrap();

        let mut entries = Vec::new();
        entries.reserve_exact(entry_count as usize); // Make sure we reserve enough space to avoid reallocations

        for _ in 0..entry_count as usize {
            let entry = reader.read_le::<Entry>().unwrap();
            entries.push(entry);
        }

        fn read_string_from_pointer(reader: &mut Cursor<&[u8]>, pointer: u64, curent_offset: u64) -> String {
            if pointer != 0 {
                reader.seek(SeekFrom::Start(curent_offset as u64)).unwrap();
                reader.seek(SeekFrom::Current(pointer as i64)).unwrap();
                reader.read_be::<NullString>().unwrap().to_string()
            } else {
                String::from("")
            }
        }

        for (current_offset, entry) in entries
        .iter_mut()
        .enumerate()
        .map(|(i, e)| (((0x80 * i + HEADER_SIZE) as u64, e))) 
        {
            entry.accessory_name_id = read_string_from_pointer(&mut reader, entry.accessory_name_id_pointer, current_offset);
            entry.acc_entry_id = read_string_from_pointer(&mut reader, entry.acc_entry_id_pointer, current_offset + 0x8);
            entry.acc_file = read_string_from_pointer(&mut reader, entry.acc_file_pointer, current_offset + 0x10);
            entry.acc_code = read_string_from_pointer(&mut reader, entry.acc_code_pointer, current_offset + 0x18);
            entry.acc_unk = read_string_from_pointer(&mut reader, entry.acc_unk_pointer, current_offset + 0x78);

        }

        Self {
            size,
            version,
            entry_count,
            unk0,
            entry_pointer,
            entries
        }

    }
}

impl From<AccessoryParam> for Vec<u8> {
    fn from(mut accessory_param: AccessoryParam) -> Self {
        // Consumes the deserialized version and returns the bytes
        let mut writer = Cursor::new(Vec::new());

        accessory_param.entry_count = accessory_param.entries.len() as u16; // Update entry count

        writer.write_be(&accessory_param.size).unwrap();
        writer.write_le(&1000u32).unwrap(); // Write the version

        writer.write_le(&accessory_param.entry_count).unwrap();
        writer.write_le(&accessory_param.unk0).unwrap();

        writer.write_le(&8u64).unwrap(); // Write the pointer to the entries

        writer.write_le(&accessory_param.entries).unwrap();

        fn write_pointer_to_string(
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
            .map(|(i, e)| (((0x80 * i + HEADER_SIZE) as u64, e)))
        {
            write_pointer_to_string(&mut writer, &entry.accessory_name_id, current_offset as u64, 0x0);
            write_pointer_to_string(&mut writer, &entry.acc_entry_id, current_offset as u64, 0x8);
            write_pointer_to_string(&mut writer, &entry.acc_file, current_offset as u64, 0x10);
            write_pointer_to_string(&mut writer, &entry.acc_code, current_offset as u64, 0x18);
            write_pointer_to_string(&mut writer, &entry.acc_unk, current_offset as u64, 0x78);
        }

        writer.set_position(0);
        writer.write_be::<u32>(&((writer.get_ref().len() - 4) as u32)).unwrap();
        
        writer.into_inner()

    }
}






    