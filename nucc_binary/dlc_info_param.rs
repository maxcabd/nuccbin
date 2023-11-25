use binrw::{binrw, BinReaderExt, BinWriterExt, NullString};
use binrw::io::{Cursor, Seek, SeekFrom};
use serde::{Serialize, Deserialize};

use super::{NuccBinaryParsed, NuccBinaryType};


const HEADER_SIZE: usize = 0x14; // Size of NUCC Binary headers

// Format was reversed by https://github.com/al-hydra
#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[serde(skip)]
    pub steam_app_id_pointer: u64,

    #[serde(skip)]
    pub ps5_content_id_pointer: u64,

    #[serde(skip)]
    pub ps4_content_id_pointer: u64,


    #[serde(skip)]
    pub nintendo_content_id_pointer: u64,


    #[serde(skip)]
    pub xbox_content_id_pointer: u64,

    #[brw(pad_before = 4)]
    pub unk1: u32,

    pub kind: u32, // Type of DLC (0 = Character, 1 = Costume, 2 = BGM, 3 = Season Pass, 4 = Accesories, 5 = Subsitution Item)
    pub unk2: u32,

    #[serde(skip)]
    pub cpk_path_pointer: u64,


    pub unk3: u32,
    pub unk4: u32,

    #[serde(skip)]
    pub dummy2_pointer: u64,

    #[serde(skip)]
    pub dummy3_pointer: u64,

    #[serde(skip)]
    pub dummy4_pointer: u64,


    pub unk5: u32,
    pub unk6: u32,


    #[serde(skip)]
    pub dummy5_pointer: u64,

    #[serde(skip)]
    pub dummy6_pointer: u64,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub steam_app_id: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub ps5_content_id: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub ps4_content_id: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub nintendo_content_id: String,


    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub xbox_content_id: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub cpk_path: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub dummy2: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub dummy3: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub dummy4: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub dummy5: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub dummy6: String,
}


#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct DlcInfoParam {
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

impl NuccBinaryParsed for DlcInfoParam {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::DlcInfoParam
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

impl From<&[u8]> for DlcInfoParam {
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
        .map(|(i, e)| (((0x78 * i + HEADER_SIZE) as u64, e))) 
        {
            entry.steam_app_id = read_string_from_pointer(&mut reader, entry.steam_app_id_pointer, current_offset);
            entry.ps5_content_id = read_string_from_pointer(&mut reader, entry.ps5_content_id_pointer, current_offset + 0x8);
            entry.ps4_content_id = read_string_from_pointer(&mut reader, entry.ps4_content_id_pointer, current_offset + 0x10);
            entry.nintendo_content_id = read_string_from_pointer(&mut reader, entry.nintendo_content_id_pointer, current_offset + 0x18);
            entry.xbox_content_id = read_string_from_pointer(&mut reader, entry.xbox_content_id_pointer, current_offset + 0x20);
            entry.cpk_path = read_string_from_pointer(&mut reader, entry.cpk_path_pointer, current_offset + 0x38);
            entry.dummy2 = read_string_from_pointer(&mut reader, entry.dummy2_pointer, current_offset + 0x48);
            entry.dummy3 = read_string_from_pointer(&mut reader, entry.dummy3_pointer, current_offset + 0x50);
            entry.dummy4 = read_string_from_pointer(&mut reader, entry.dummy4_pointer, current_offset + 0x58);
            entry.dummy5 = read_string_from_pointer(&mut reader, entry.dummy5_pointer, current_offset + 0x68);
            entry.dummy6 = read_string_from_pointer(&mut reader, entry.dummy5_pointer, current_offset + 0x70);

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

impl From<DlcInfoParam> for Vec<u8> {
    fn from(mut dlc_info_param: DlcInfoParam) -> Self {
        // Consumes the deserialized version and returns the bytes
        let mut writer = Cursor::new(Vec::new());

        dlc_info_param.entry_count = dlc_info_param.entries.len() as u16; // Update entry count

        writer.write_be(&dlc_info_param.size).unwrap();
        writer.write_le(&1000u32).unwrap(); // Write the version

        writer.write_le(&dlc_info_param.entry_count).unwrap();
        writer.write_le(&dlc_info_param.unk0).unwrap();

        writer.write_le(&8u64).unwrap(); // Write the pointer to the entries

        writer.write_le(&dlc_info_param.entries).unwrap();

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
        for (current_offset, entry) in dlc_info_param.entries
            .iter_mut()
            .enumerate()
            .map(|(i, e)| (((0x78 * i + HEADER_SIZE) as u64, e)))
        {
            write_pointer_to_string(&mut writer, &entry.steam_app_id, current_offset as u64, 0x0);
            write_pointer_to_string(&mut writer, &entry.ps5_content_id, current_offset as u64, 0x8);
            write_pointer_to_string(&mut writer, &entry.ps4_content_id, current_offset as u64, 0x10);
            write_pointer_to_string(&mut writer, &entry.nintendo_content_id, current_offset as u64, 0x18);
            write_pointer_to_string(&mut writer, &entry.xbox_content_id, current_offset as u64, 0x20);
            write_pointer_to_string(&mut writer, &entry.cpk_path, current_offset as u64, 0x38);
            write_pointer_to_string(&mut writer, &entry.dummy2, current_offset as u64, 0x48);
            write_pointer_to_string(&mut writer, &entry.dummy3, current_offset as u64, 0x50);
            write_pointer_to_string(&mut writer, &entry.dummy4, current_offset as u64, 0x58);
            write_pointer_to_string(&mut writer, &entry.dummy5, current_offset as u64, 0x68);
            write_pointer_to_string(&mut writer, &entry.dummy6, current_offset as u64, 0x70);
        }

        // Go to the start of buffer and write the size
        writer.set_position(0);
        writer.write_be::<u32>(&((writer.get_ref().len() - 4) as u32)).unwrap();

        writer.into_inner()
    }
}