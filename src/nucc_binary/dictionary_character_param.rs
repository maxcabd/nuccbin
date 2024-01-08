use binrw::{binrw, BinReaderExt, BinWriterExt, NullString};
use binrw::io::{Cursor, Seek, SeekFrom};
use serde::{Serialize, Deserialize};

use super::{NuccBinaryParsed, NuccBinaryType};

const HEADER_SIZE: usize = 0x14; // Size of NUCC Binary headers

// Format reversed by Portable Productions (https://www.youtube.com/@PortableProductions)
#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[serde(skip)]
    #[brw(pad_before = 0x8)]
    dictionary_link_ptr: u64,

    pub page_entry_index: u32,
    pub page_number: u32,

    #[serde(skip)]
    #[brw(pad_after = 0x8)]
    pub lock_link_ptr: u64,

    #[serde(skip)]
    pub char_portrait_link_ptr: u64,

    #[serde(skip)]
    pub background_ptr: u64,

    #[serde(skip)]
    pub char_name_ptr: u64,

    #[serde(skip)]
    pub char_quote_ptr: u64,

    #[serde(skip)]
    pub ninja_reg_no_ptr: u64,

    #[serde(skip)]
    pub char_birthday_ptr: u64,

    #[serde(skip)]
    pub char_affiliation_ptr: u64,

    #[serde(skip)]
    pub char_height_ptr: u64,
    
    #[serde(skip)]
    pub char_weight_ptr: u64,

    #[serde(skip)]
    pub dictionary_desc_ptr: u64,

    pub justu1: i32,
    pub jutsu2: i32,
    #[brw(pad_after = 0x94)]
    pub jutsu3: i32,
    
    #[serde(skip)]
    pub additional_link1_ptr: u64,

    #[serde(skip)]
    pub additional_link2_ptr: u64,

    #[serde(skip)]
    #[brw(pad_after = 0x78)]
    pub additional_link3_ptr: u64,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub empty: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub lock_link: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub dictionary_link: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub char_portrait_link: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub background: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub char_name: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub char_quote: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub ninja_reg_no: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub char_birthday: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub char_affiliation: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub char_height: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub char_weight: String,


    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub dictionary_desc: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub additional_link1: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub additional_link2: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub additional_link3: String,
}

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct DictionaryCharacterParam {
    #[serde(skip)]
    pub size: u32,

    #[serde(skip)]
    pub version: u32,

    pub entry_count: u32,

    #[serde(skip)]
    pub entry_ptr: u64,

    #[br(count = entry_count)]
    pub entries: Vec<Entry>
}

impl NuccBinaryParsed for DictionaryCharacterParam {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::DictionaryCharacterParam
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

impl From<&[u8]> for DictionaryCharacterParam {
    fn from(data: &[u8]) -> Self {
        let mut reader = Cursor::new(data);
        
        let size = reader.read_be::<u32>().unwrap();
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
        .map(|(i, e)| (((0x1a8 * i + HEADER_SIZE) as u64, e))) 
        {
            entry.dictionary_link = read_string_from_ptr(&mut reader, entry.dictionary_link_ptr, current_offset + 0x8);
            entry.lock_link = read_string_from_ptr(&mut reader, entry.lock_link_ptr, current_offset + 0x18);
            entry.char_portrait_link = read_string_from_ptr(&mut reader, entry.char_portrait_link_ptr, current_offset + 0x28);
            entry.background = read_string_from_ptr(&mut reader, entry.background_ptr, current_offset + 0x30);
            entry.char_name = read_string_from_ptr(&mut reader, entry.char_name_ptr, current_offset + 0x38);
            entry.char_quote = read_string_from_ptr(&mut reader, entry.char_quote_ptr, current_offset + 0x40);
            entry.ninja_reg_no = read_string_from_ptr(&mut reader, entry.ninja_reg_no_ptr, current_offset + 0x48);
            entry.char_birthday = read_string_from_ptr(&mut reader, entry.char_birthday_ptr, current_offset + 0x50);
            entry.char_affiliation = read_string_from_ptr(&mut reader, entry.char_affiliation_ptr, current_offset + 0x58);
            entry.char_height = read_string_from_ptr(&mut reader, entry.char_height_ptr, current_offset + 0x60);
            entry.char_weight = read_string_from_ptr(&mut reader, entry.char_weight_ptr, current_offset + 0x68);
            entry.dictionary_desc = read_string_from_ptr(&mut reader, entry.dictionary_desc_ptr, current_offset + 0x70);

            entry.additional_link1 = read_string_from_ptr(&mut reader, entry.additional_link1_ptr, current_offset + 0x118);
            entry.additional_link2 = read_string_from_ptr(&mut reader, entry.additional_link2_ptr, current_offset + 0x120);
            entry.additional_link3 = read_string_from_ptr(&mut reader, entry.additional_link3_ptr, current_offset + 0x128);

            
        }

        Self {
            size,
            version,
            entry_count,
            entry_ptr,
            entries
        }

    }
}

impl From<DictionaryCharacterParam> for Vec<u8> {
    fn from(mut dictionary_character_param: DictionaryCharacterParam) -> Self {
        let mut writer = Cursor::new(Vec::new());

        dictionary_character_param.entry_count = dictionary_character_param.entries.len() as u32; // Update entry count

        writer.write_be(&dictionary_character_param.size).unwrap();
        writer.write_le(&1000u32).unwrap(); // Write the version

        writer.write_le(&dictionary_character_param.entry_count).unwrap();

        writer.write_le(&8u64).unwrap(); // Write the ptr to the entries

        writer.write_le(&dictionary_character_param.entries).unwrap();

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
        for (current_offset, entry) in dictionary_character_param.entries
            .iter_mut()
            .enumerate()
            .map(|(i, e)| (((0x1a8 * i + HEADER_SIZE) as u64, e)))
        {
            write_ptr_to_string(&mut writer, &entry.dictionary_link, current_offset as u64, 0x8);
            write_ptr_to_string(&mut writer, &entry.lock_link, current_offset as u64, 0x18);
            write_ptr_to_string(&mut writer, &entry.char_portrait_link, current_offset as u64, 0x28);
            write_ptr_to_string(&mut writer, &entry.background, current_offset as u64, 0x30);
            write_ptr_to_string(&mut writer, &entry.char_name, current_offset as u64, 0x38);
            write_ptr_to_string(&mut writer, &entry.char_quote, current_offset as u64, 0x40);
            write_ptr_to_string(&mut writer, &entry.ninja_reg_no, current_offset as u64, 0x48);
            write_ptr_to_string(&mut writer, &entry.char_birthday, current_offset as u64, 0x50);
            write_ptr_to_string(&mut writer, &entry.char_affiliation, current_offset as u64, 0x58);
            write_ptr_to_string(&mut writer, &entry.char_height, current_offset as u64, 0x60);
            write_ptr_to_string(&mut writer, &entry.char_weight, current_offset as u64, 0x68);
            write_ptr_to_string(&mut writer, &entry.dictionary_desc, current_offset as u64, 0x70);

            write_ptr_to_string(&mut writer, &entry.additional_link1, current_offset as u64, 0x118);
            write_ptr_to_string(&mut writer, &entry.additional_link2, current_offset as u64, 0x120);
            write_ptr_to_string(&mut writer, &entry.additional_link3, current_offset as u64, 0x128);
        }

        // Go to the start of buffer and write the size
        writer.set_position(0);
        writer.write_be::<u32>(&((writer.get_ref().len() - 4) as u32)).unwrap();

        writer.into_inner()

    }
}



    



    
