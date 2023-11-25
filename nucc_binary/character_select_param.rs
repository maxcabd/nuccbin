use binrw::{binrw, BinReaderExt, BinWriterExt, NullString};
use binrw::io::{Cursor, Seek, SeekFrom};
use serde::{Serialize, Deserialize};


use super::{NuccBinaryParsed, NuccBinaryType};

const HEADER_SIZE: usize = 0x14; // Size of NUCC Binary headers

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[serde(skip)]
    pub entrycode_pointer: u64,

    pub page_index: u32,
    pub slot_index: u32,
    pub costume_slot_index: u32,
    pub unk1: u32,

    #[serde(skip)]
    pub message_id_pointer: u64,


    pub unk2: u32,
    pub unk3: u32,


    #[serde(skip)]
    pub cha_a_id_pointer: u64,

    #[serde(skip)]
    pub accessory_pointer: u64,

    #[serde(skip)]
    pub charsel_pointer: u64,



    pub p1_no_select_x: f32,
    pub p1_no_select_y: f32,
    pub p1_no_select_z: f32,

    pub p2_no_select_x: f32,
    pub p2_no_select_y: f32,
    pub p2_no_select_z: f32,

    pub p1_select_x: f32,
    pub p1_select_y: f32,
    pub p1_select_z: f32,

    pub p2_select_x: f32,
    pub p2_select_y: f32,
    pub p2_select_z: f32,

    pub p1_vs_x: f32,
    pub p1_vs_y: f32,
    pub p1_vs_z: f32,

    pub p2_vs_x: f32,
    pub p2_vs_y: f32,
    pub p2_vs_z: f32,

    pub p1_rotation_no_select: f32,
    pub p2_rotation_no_select: f32,

    pub p1_rotation_select: f32,
    pub p2_rotation_select: f32,

    pub p1_rotation_vs: f32,
    pub p2_rotation_vs: f32,

    pub p1_lighting_x_no_select: f32,
    pub p1_lighting_y_no_select: f32,
    pub p1_lighting_z_no_select: f32,

    pub p2_lighting_x_no_select: f32,
    pub p2_lighting_y_no_select: f32,
    pub p2_lighting_z_no_select: f32,

    pub p1_lighting_x_select: f32,
    pub p1_lighting_y_select: f32,
    pub p1_lighting_z_select: f32,

    pub p2_lighting_x_select: f32,
    pub p2_lighting_y_select: f32,
    pub p2_lighting_z_select: f32,

    pub unk_x: f32,
    pub unk_y: f32,
    pub unk_z: f32,

    pub unk_x2: f32,
    pub unk_y2: f32,
    pub unk_z2: f32,

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

    #[serde(skip)]
    pub unk_pointer: u64,


    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub entrycode: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub message_id: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub cha_a_id: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub accesory_file: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub charsel_file: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub dictionary_file: String,


}

// 0x8 + 0x4 

// What is the size in bytes of the Entry struct?
// It is: 


#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterSelectParam {
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
        
        let size = reader.read_be::<u32>().unwrap();
        let version = reader.read_le::<u32>().unwrap();

        let entry_count = reader.read_le::<u16>().unwrap();
        let unk0 = reader.read_le::<u16>().unwrap();

        let entry_pointer = reader.read_le::<u64>().unwrap();

        let mut entries = Vec::new();
        entries.reserve_exact(entry_count as usize); // Make sure we have enough space to avoid reallocations

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
        .map(|(i, e)| (((0x138 * i + HEADER_SIZE) as u64, e))) 
        {
            entry.entrycode = read_string_from_pointer(&mut reader, entry.entrycode_pointer, current_offset);
            entry.message_id = read_string_from_pointer(&mut reader, entry.message_id_pointer, current_offset + 0x18);
            entry.cha_a_id = read_string_from_pointer(&mut reader, entry.cha_a_id_pointer, current_offset + 0x28);
            entry.accesory_file = read_string_from_pointer(&mut reader, entry.accessory_pointer, current_offset + 0x30);
            entry.charsel_file = read_string_from_pointer(&mut reader, entry.charsel_pointer, current_offset + 0x38);
            entry.dictionary_file = read_string_from_pointer(&mut reader, entry.unk_pointer, current_offset + 0x130);
        }

        Self {
            size,
            version,
            entry_count,
            unk0,
            entry_pointer,
            entries,
        }

    }
}

impl From<CharacterSelectParam> for Vec<u8> {
    fn from(mut character_select_param: CharacterSelectParam) -> Self {
        let mut writer = Cursor::new(Vec::new());

        character_select_param.entry_count = character_select_param.entries.len() as u16; // Update entry count

        writer.write_be(&character_select_param.size).unwrap();
        writer.write_le(&1001u32).unwrap(); // Write the version

        writer.write_le(&character_select_param.entry_count).unwrap();
        writer.write_le(&character_select_param.unk0).unwrap();

        writer.write_le(&8u64).unwrap(); // Write the pointer to the entries

        writer.write_le(&character_select_param.entries).unwrap();

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
        for (current_offset, entry) in character_select_param.entries
            .iter_mut()
            .enumerate()
            .map(|(i, e)| (((0x138 * i + HEADER_SIZE) as u64, e)))
        {
            write_pointer_to_string(&mut writer, &entry.entrycode, current_offset, 0x0);
            write_pointer_to_string(&mut writer, &entry.message_id, current_offset, 0x18);
            write_pointer_to_string(&mut writer, &entry.cha_a_id, current_offset, 0x28);
            write_pointer_to_string(&mut writer, &entry.accesory_file, current_offset, 0x30);
            write_pointer_to_string(&mut writer, &entry.charsel_file, current_offset, 0x38);
            write_pointer_to_string(&mut writer, &entry.dictionary_file, current_offset, 0x130);
        }

        // Go to the start of buffer and write the size
        writer.set_position(0);
        writer.write_be::<u32>(&((writer.get_ref().len() - 4) as u32)).unwrap();

        writer.into_inner()
    }
}
