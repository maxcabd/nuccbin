use binrw::{binrw, BinReaderExt, BinWriterExt, NullString};
use binrw::io::{Cursor, Seek, SeekFrom};
use serde::{Serialize, Deserialize};

use super::{NuccBinaryParsed, NuccBinaryType};


const HEADER_SIZE: usize = 0x14; // Size of NUCC Binary headers
const SPL_VICTIM_COUNT: usize = 50;

// Format reversed by EliteAce170 (https://www.youtube.com/c/EliteAce)
#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub story_mode_id: i32,
    pub pair_sp_skill_id: i32, // Team Ultimate Jutsu ID
    pub player_setting_id: i32,

    pub costume_slot_index: i32,

    #[serde(skip)]
    pub sp_skill_1_name_pointer: u64,


    #[serde(skip)]
    #[brw(pad_after = 4)]
    pub sp_skill_2_name_pointer: u64,

    #[br(count = SPL_VICTIM_COUNT)]
    pub spl_fin_victims: Vec<SplFinVictim>,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub sp_skill_1_name: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub sp_skill_2_name: String,


}

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct SplFinVictim {
    pub victim_player_setting_id: i32,

    #[serde(skip)]
    pub victim_name_pointer: u64,

    #[serde(skip)]
    #[brw(pad_after = 4)]
    pub victim_texture_name_pointer: u64,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub victim_name: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub victim_texture_name: String
}


#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct FinalSpSkillCutIn {
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

impl NuccBinaryParsed for FinalSpSkillCutIn {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::FinalSpSkillCutIn
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

impl From<&[u8]> for FinalSpSkillCutIn {
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
        .map(|(i, e)| (((0x4d8 * i + HEADER_SIZE) as u64, e))) 
        {
            entry.sp_skill_1_name = read_string_from_pointer(&mut reader, entry.sp_skill_1_name_pointer, current_offset + 0x10);
            entry.sp_skill_2_name = read_string_from_pointer(&mut reader, entry.sp_skill_2_name_pointer, current_offset + 0x18);

        
            let mut spl_fin_victims = Vec::new();
            spl_fin_victims.reserve_exact(SPL_VICTIM_COUNT as usize); // Make sure we have enough space to avoid reallocations

            for _ in 0..SPL_VICTIM_COUNT as usize {
                let victim = reader.read_le::<SplFinVictim>().unwrap();
                spl_fin_victims.push(victim);
            }

            for (current_offset, victim) in spl_fin_victims
            .iter_mut()
            .enumerate()
            .map(|(i, e)| (((0x4b4 * i + (0x24 + 0x14)) as u64, e))) 
            {
                victim.victim_name = read_string_from_pointer(&mut reader, victim.victim_name_pointer, current_offset + 0x4);
                victim.victim_texture_name = read_string_from_pointer(&mut reader, victim.victim_texture_name_pointer, current_offset + 0x8);
            }

            entry.spl_fin_victims = spl_fin_victims;
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

impl From<FinalSpSkillCutIn> for Vec<u8> {
    fn from(mut final_spl_cutin: FinalSpSkillCutIn) -> Self {
        let mut writer = Cursor::new(Vec::new());

        final_spl_cutin.entry_count = final_spl_cutin.entries.len() as u16; // Update entry count

        writer.write_be(&final_spl_cutin.size).unwrap();
        writer.write_le(&1001u32).unwrap(); // Write the version

        writer.write_le(&final_spl_cutin.entry_count).unwrap();
        writer.write_le(&final_spl_cutin.unk0).unwrap();

        writer.write_le(&8u64).unwrap(); // Write the pointer to the entries

        writer.write_le(&final_spl_cutin.entries).unwrap();

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

        for (current_offset, entry) in final_spl_cutin.entries
            .iter_mut()
            .enumerate()
            .map(|(i, e)| (((0x4d8 * i + HEADER_SIZE) as u64, e)))
        {
            write_pointer_to_string(&mut writer, &entry.sp_skill_1_name, current_offset as u64, 0x10);
            write_pointer_to_string(&mut writer, &entry.sp_skill_2_name, current_offset as u64, 0x18);
            


            for (current_offset, victim) in entry.spl_fin_victims
            .iter_mut()
            .enumerate()
            .map(|(i, e)| (((0x18 * i + (HEADER_SIZE + 0x28)) as u64, e))) 
            {
                write_pointer_to_string(&mut writer, &victim.victim_name, current_offset as u64, 0x0);
                write_pointer_to_string(&mut writer, &victim.victim_texture_name, current_offset as u64, 0x4);
            }
        }

        // Go to the start of buffer and write the size
        writer.set_position(0);
        writer.write_be::<u32>(&((writer.get_ref().len() - 4) as u32)).unwrap();

        writer.into_inner()

    }
}



