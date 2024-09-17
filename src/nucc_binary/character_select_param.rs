use binrw::{binrw, BinReaderExt, BinWriterExt, NullString};
use binrw::io::{Cursor, Seek, SeekFrom};
use serde::{Serialize, Deserialize};


use super::{NuccBinaryParsed, NuccBinaryType};

use super::HEADER_SIZE;

#[binrw]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Entry {
    #[serde(skip)]
    pub searchcode_ptr: u64,

    pub page_index: u32,
    pub slot_index: u32,

    #[brw(pad_after = 4)]
    pub costume_slot_index: u32,

    #[serde(skip)]
    pub char_name_ptr: u64,

    #[brw(pad_after = 4)]
    pub duel_player_param_model_index: u32,

    #[serde(skip)]
    pub costume_name_ptr: u64,

    #[serde(skip)]
    pub accessory_ptr: u64,

    #[serde(skip)]
    pub crsel_ptr: u64,
    
    pub render_settings: RenderSettings,

    #[serde(skip)]  
    pub dictionary_link_ptr: u64,

    #[brw(pad_after = 4)]
    pub index: i32, // not sure maybe an index?


    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub searchcode: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub char_name: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub costume_name: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub accessory: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub crsel: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub dictionary_link: String,
}

#[allow(non_snake_case)]
#[binrw]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RenderSettings {
    pub ofsX1P: f32,
    pub ofsY1P: f32,
    pub ofsZ1P: f32,

    pub ofsX2P: f32,
    pub ofsY2P: f32,
    pub ofsZ2P: f32,

    pub selOfsX1P: f32,
    pub selOfsY1P: f32,
    pub selOfsZ1P: f32,

    pub selOfsX2P: f32,
    pub selOfsY2P: f32,
    pub selOfsZ2P: f32,

    pub vsOfsX1P: f32,
    pub vsOfsY1P: f32,
    pub vsOfsZ1P: f32,

    pub vsOfsX2P: f32,
    pub vsOfsY2P: f32,
    pub vsOfsZ2P: f32,

    pub rot1P: f32,
    pub rot2P: f32,

    pub selRot1P: f32,
    pub selRot2P: f32,

    pub vsRot1P: f32,
    pub vsRot2P: f32,

    pub lightX1P: f32,
    pub lightY1P: f32,
    pub lightZ1P: f32,

    pub lightX2P: f32,
    pub lightY2P: f32,
    pub lightZ2P: f32,

    pub selLightX1P: f32,
    pub selLightY1P: f32,
    pub selLightZ1P: f32,

    pub selLightX2P: f32,
    pub selLightY2P: f32,
    pub selLightZ2P: f32,

    pub vsLightX1P: f32,
    pub vsLightY1P: f32,
    pub vsLightZ1P: f32,

    pub vsLightX2P: f32,
    pub vsLightY2P: f32,
    pub vsLightZ2P: f32,

    pub unk_x3: f32,
    pub unk_y3: f32,
    pub unk_z3: f32,

    pub unk_x4: f32,
    pub unk_y4: f32,
    pub unk_z4: f32,

    pub unk_x5: f32,
    pub unk_y5: f32,
    pub unk_z5: f32,

    pub unk_x6: f32,
    pub unk_y6: f32,
    pub unk_z6: f32,

    pub unk_x7: f32,
    pub unk_y7: f32,
    pub unk_z7: f32,

    pub unk_x8: f32,
    pub unk_y8: f32,
    pub unk_z8: f32,

}

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterSelectParam {
    #[serde(skip)]
    pub version: u32,

    pub entry_count: u32,

    #[serde(skip)]
    pub entry_ptr: u64,

    #[br(count = entry_count)]
    pub entries: Vec<Entry>
}

impl NuccBinaryParsed for CharacterSelectParam {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::CharacterSelectParam
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


impl From<&[u8]> for CharacterSelectParam {
    fn from(data: &[u8]) -> Self {
        let mut reader = Cursor::new(data);
        
        let version = reader.read_le::<u32>().unwrap();

        let entry_count = reader.read_le::<u32>().unwrap();

        let entry_ptr = reader.read_le::<u64>().unwrap();

        let mut entries = Vec::new();
        entries.reserve_exact(entry_count as usize); // Make sure we have enough space to avoid reallocations

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
        .map(|(i, e)| (((0x140 * i + HEADER_SIZE) as u64, e))) 
        {
            entry.searchcode = read_string_from_ptr(&mut reader, entry.searchcode_ptr, current_offset);
            entry.char_name = read_string_from_ptr(&mut reader, entry.char_name_ptr, current_offset + 0x18);
            entry.costume_name = read_string_from_ptr(&mut reader, entry.costume_name_ptr, current_offset + 0x28);
            entry.accessory = read_string_from_ptr(&mut reader, entry.accessory_ptr, current_offset + 0x30);
            entry.crsel = read_string_from_ptr(&mut reader, entry.crsel_ptr, current_offset + 0x38);
            entry.dictionary_link = read_string_from_ptr(&mut reader, entry.dictionary_link_ptr, current_offset + 0x130);
        }

        Self {
            version,
            entry_count,
            entry_ptr,
            entries,
        }

    }
}

impl From<CharacterSelectParam> for Vec<u8> {
    fn from(mut character_select_param: CharacterSelectParam) -> Self {
        let mut writer = Cursor::new(Vec::new());

        character_select_param.entry_count = character_select_param.entries.len() as u32; // Update entry count

        writer.write_le(&1001u32).unwrap(); // Write the version

        writer.write_le(&character_select_param.entry_count).unwrap();

        writer.write_le(&8u64).unwrap(); // Write the ptr to the entries

        writer.write_le(&character_select_param.entries).unwrap();

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
        for (current_offset, entry) in character_select_param.entries
            .iter_mut()
            .enumerate()
            .map(|(i, e)| (((0x140 * i + HEADER_SIZE) as u64, e)))
        {
            write_ptr_to_string(&mut writer, &entry.searchcode, current_offset, 0x0);
            write_ptr_to_string(&mut writer, &entry.char_name, current_offset, 0x18);
            write_ptr_to_string(&mut writer, &entry.costume_name, current_offset, 0x28);
            write_ptr_to_string(&mut writer, &entry.accessory, current_offset, 0x30);
            write_ptr_to_string(&mut writer, &entry.crsel, current_offset, 0x38);
            write_ptr_to_string(&mut writer, &entry.dictionary_link, current_offset, 0x130);
        }

  

        writer.into_inner()
    }
}
