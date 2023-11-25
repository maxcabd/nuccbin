use binrw::{binrw, BinReaderExt, BinWriterExt, NullString};
use binrw::io::{Cursor, Seek, SeekFrom};
use serde::{Serialize, Deserialize};

use super::{NuccBinaryParsed, NuccBinaryType};


const HEADER_SIZE: usize = 0x14; // Size of NUCC Binary headers

// Format reversed by EliteAce170 (https://www.youtube.com/c/EliteAce)

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub characode_index: u32,
    pub unk1: i32,

    #[serde(skip)]
    pub bone_name_pointer: u64,


    pub animation_start_frame: i32,
    pub animation_end_frame: i32,
    pub unk2: i32,
    pub unk3: i32,

    #[serde(skip)]
    pub effect_name_pointer: u64,

    #[serde(skip)]
    pub anm1_name_pointer: u64,

    #[serde(skip)]
    pub anm2_name_pointer: u64,

    #[serde(skip)]
    pub anm3_name_pointer: u64,

    #[serde(skip)]
    pub anm4_name_pointer: u64,

    #[serde(skip)]
    pub anm5_name_pointer: u64,

    pub unk4: f32,
    pub spawn_location: f32,

    pub common_sound_id: i32,


    pub unk5: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,

    #[brw(pad_after = 4)]
    pub unk10: u32,


    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub bone_name: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub effect_name: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub anm1_name: String,


    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub anm2_name: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub anm3_name: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub anm4_name: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub anm5_name: String,
}

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerDoubleEffectParam {
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

impl NuccBinaryParsed for PlayerDoubleEffectParam {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::PlayerDoubleEffectParam
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


impl From<&[u8]> for PlayerDoubleEffectParam {
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
            entry.bone_name = read_string_from_pointer(&mut reader, entry.bone_name_pointer, current_offset + 0x8);
            entry.effect_name = read_string_from_pointer(&mut reader, entry.effect_name_pointer, current_offset + 0x20);
            entry.anm1_name = read_string_from_pointer(&mut reader, entry.anm1_name_pointer, current_offset + 0x28);
            entry.anm2_name = read_string_from_pointer(&mut reader, entry.anm2_name_pointer, current_offset + 0x30);
            entry.anm3_name = read_string_from_pointer(&mut reader, entry.anm3_name_pointer, current_offset + 0x38);
            entry.anm4_name = read_string_from_pointer(&mut reader, entry.anm4_name_pointer, current_offset + 0x40);
            entry.anm5_name = read_string_from_pointer(&mut reader, entry.anm5_name_pointer, current_offset + 0x48);
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


impl From<PlayerDoubleEffectParam> for Vec<u8> {
    fn from(mut player_double_effect_param: PlayerDoubleEffectParam) -> Self {
        // Consumes the deserialized version and returns the bytes
        let mut writer = Cursor::new(Vec::new());

        player_double_effect_param.entry_count = player_double_effect_param.entries.len() as u16; // Update entry count

        writer.write_be(&player_double_effect_param.size).unwrap();
        writer.write_le(&1000u32).unwrap(); // Write the version

        writer.write_le(&player_double_effect_param.entry_count).unwrap();
        writer.write_le(&player_double_effect_param.unk0).unwrap();

        writer.write_le(&8u64).unwrap(); // Write the pointer to the entries

        writer.write_le(&player_double_effect_param.entries).unwrap();

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
        for (current_offset, entry) in player_double_effect_param.entries
            .iter_mut()
            .enumerate()
            .map(|(i, e)| (((0x78 * i + HEADER_SIZE) as u64, e)))
        {
            write_pointer_to_string(&mut writer, &entry.bone_name, current_offset as u64, 0x8);
            write_pointer_to_string(&mut writer, &entry.effect_name, current_offset as u64, 0x20);
            write_pointer_to_string(&mut writer, &entry.anm1_name, current_offset as u64, 0x28);
            write_pointer_to_string(&mut writer, &entry.anm2_name, current_offset as u64, 0x30);
            write_pointer_to_string(&mut writer, &entry.anm3_name, current_offset as u64, 0x38);
            write_pointer_to_string(&mut writer, &entry.anm4_name, current_offset as u64, 0x40);
            write_pointer_to_string(&mut writer, &entry.anm5_name, current_offset as u64, 0x48);
        }

        writer.set_position(0);
        writer.write_be::<u32>(&((writer.get_ref().len() - 4) as u32)).unwrap();
        
        writer.into_inner()

    }
}

