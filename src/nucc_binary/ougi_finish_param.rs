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
    pub char_name_ptr: u64,

    #[serde(skip)]
    pub ougi_fin_link_ptr: u64,

    #[serde(skip)]
    pub unk_ptr: u64,

    #[serde(skip)]
    pub spl_fin_ptr: u32,
    pub index: u32,

    #[serde(skip)]
    pub spl_fin_path_ptr: u32,
    pub unk4: u32,

    #[serde(skip)]
    pub spl_fin_small_ptr: u64,

    #[serde(skip)]
    pub spl_fin_big_ptr: u64,

    pub price: u32,
    pub unlock_condition: u32,

    #[serde(skip)]
    pub search_code_ptr: u32,

    pub unk1: u32,

    #[serde(skip)]
    pub section_id_ptr: u32,

    pub unk2: u32,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub char_name: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub ougi_fin_link: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub unk: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub spl_fin: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub spl_fin_path: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub spl_fin_small: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub spl_fin_big: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub search_code: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub section_id: String

}

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct OugiFinishParam {
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

impl NuccBinaryParsed for OugiFinishParam {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::OugiFinishParam
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

impl From<&[u8]> for OugiFinishParam {
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
            // If the pointer is not 0 or -1, read the string from the pointer

  
            if ptr != 0 && ptr < 100 && ptr != 0xffffffff as u64 {
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
        .map(|(i, e)| (((0x50 * i + HEADER_SIZE) as u64, e))) 
        {
            entry.char_name = read_string_from_ptr(&mut reader, entry.char_name_ptr, current_offset as u64);
            entry.ougi_fin_link = read_string_from_ptr(&mut reader, entry.ougi_fin_link_ptr, current_offset + 0x8);
            entry.unk = read_string_from_ptr(&mut reader, entry.unk_ptr, current_offset + 0x10);
            entry.spl_fin = read_string_from_ptr(&mut reader, entry.spl_fin_ptr as u64, current_offset + 0x18);
            entry.spl_fin_path = read_string_from_ptr(&mut reader, entry.spl_fin_path_ptr as u64, current_offset + 0x20);
            entry.spl_fin_small = read_string_from_ptr(&mut reader, entry.spl_fin_small_ptr, current_offset + 0x28);
            entry.spl_fin_big = read_string_from_ptr(&mut reader, entry.spl_fin_big_ptr, current_offset + 0x30);
            entry.search_code = read_string_from_ptr(&mut reader, entry.search_code_ptr as u64, current_offset + 0x40);
            entry.section_id = read_string_from_ptr(&mut reader, entry.section_id_ptr as u64, current_offset + 0x48);
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

impl From<OugiFinishParam> for Vec<u8> {
    fn from(mut ougi_finish_param: OugiFinishParam) -> Self {
        let mut writer = Cursor::new(Vec::new());

        ougi_finish_param.entry_count = ougi_finish_param.entries.len() as u32; // Update entry count

        writer.write_be(&ougi_finish_param.size).unwrap();
        writer.write_le(&1000u32).unwrap(); // Write the version

        writer.write_le(&ougi_finish_param.entry_count).unwrap();

        writer.write_le(&8u64).unwrap(); // Write the ptr to the entries

        writer.write_le(&ougi_finish_param.entries).unwrap();

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
        for (current_offset, entry) in ougi_finish_param.entries
            .iter_mut()
            .enumerate()
            .map(|(i, e)| (((0x50 * i + HEADER_SIZE) as u64, e)))
        {
            write_ptr_to_string(&mut writer, &entry.char_name, current_offset as u64, 0x0);
            write_ptr_to_string(&mut writer, &entry.ougi_fin_link, current_offset as u64, 0x8);
            write_ptr_to_string(&mut writer, &entry.spl_fin, current_offset as u64, 0x18);
            write_ptr_to_string(&mut writer, &entry.spl_fin_path, current_offset as u64, 0x20);
            write_ptr_to_string(&mut writer, &entry.spl_fin_small, current_offset as u64, 0x28);
            write_ptr_to_string(&mut writer, &entry.spl_fin_big, current_offset as u64, 0x30);
            write_ptr_to_string(&mut writer, &entry.search_code, current_offset as u64, 0x40);
            write_ptr_to_string(&mut writer, &entry.section_id, current_offset as u64, 0x48);
        }

        // Update the indices in case they were changed
        for (i, entry) in ougi_finish_param.entries.iter_mut().enumerate() {
            entry.index = i as u32;
        }

        // Go to the start of buffer and write the size
        writer.set_position(0);
        writer.write_be::<u32>(&((writer.get_ref().len() - 4) as u32)).unwrap();

        writer.into_inner()

    }
}




