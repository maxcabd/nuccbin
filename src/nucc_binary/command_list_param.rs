use binrw::{binrw, BinReaderExt, BinWriterExt, NullString};
use binrw::io::{Cursor, Seek, SeekFrom};
use serde::{Serialize, Deserialize};

use super::{NuccBinaryParsed, NuccBinaryType};

const HEADER_SIZE: usize = 0x14; // Size of NUCC Binary headers

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[serde(skip)]
    pub command_link_ptr: u64,
    #[brw(pad_after = 4)]
    pub command_order: u32,
    
    #[serde(skip)]
    pub char_name_ptr: u64,
    #[serde(skip)]
    pub characode_ptr: u64,

    #[brw(pad_after = 4)]
    pub unk1: u32, // some flag

    #[serde(skip)]
    pub command_name_ptr: u64,

    #[serde(skip)]
    pub btn_input_ptr: u64,
    #[serde(skip)]
    pub condition1_ptr: u64,

    #[serde(skip)]
    pub condition2_ptr: u64,
    pub unk2: u32,
    pub unk3: u32,

    pub command_type: u32, // Usually 2 for a playable character's command list
    pub unk5: u32, // 0
    pub unk6: i32,
    pub unk7: i32,

    #[serde(skip)]
    pub additional_command_link_ptr: u64,
   
    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub command_link: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub char_name: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub characode: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub command_name: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub btn_input: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub condition1: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub condition2: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub additional_command_link: String,
}

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct CommandListParam {
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

impl NuccBinaryParsed for CommandListParam {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::CommandListParam
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

impl From<&[u8]> for CommandListParam {
    fn from(data: &[u8]) -> Self {
        let mut reader = Cursor::new(data);
        
        let size = reader.read_be::<u32>().unwrap();
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
            .map(|(i, e)| (((0x68 * i + HEADER_SIZE) as u64, e))) 
        {
            entry.command_link = read_string_from_ptr(&mut reader, entry.command_link_ptr, current_offset);
            entry.char_name = read_string_from_ptr(&mut reader, entry.char_name_ptr, current_offset + 0x10);
            entry.characode = read_string_from_ptr(&mut reader, entry.characode_ptr, current_offset + 0x18);
            entry.command_name = read_string_from_ptr(&mut reader, entry.command_name_ptr, current_offset + 0x28);
            entry.btn_input = read_string_from_ptr(&mut reader, entry.btn_input_ptr, current_offset + 0x30);
            entry.condition1 = read_string_from_ptr(&mut reader, entry.condition1_ptr, current_offset + 0x38);
            entry.condition2 = read_string_from_ptr(&mut reader, entry.condition2_ptr, current_offset + 0x40);
            entry.additional_command_link = read_string_from_ptr(&mut reader, entry.additional_command_link_ptr, current_offset + 0x60);
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

impl From<CommandListParam> for Vec<u8> {
    fn from(mut command_list_param: CommandListParam) -> Self {
        // Consumes the deserialized version and returns the bytes
        let mut writer = Cursor::new(Vec::new());

        command_list_param.entry_count = command_list_param.entries.len() as u32; // Update entry count

        writer.write_be(&command_list_param.size).unwrap();
        writer.write_le(&1001u32).unwrap(); // Write the version
        writer.write_le(&command_list_param.entry_count).unwrap();
        writer.write_le(&8u64).unwrap(); // Write the entry ptr offset (always 8)

        writer.write_le(&command_list_param.entries).unwrap();

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
        for (current_offset, entry) in command_list_param.entries
            .iter_mut()
            .enumerate()
            .map(|(i, e)| (((0x68 * i + HEADER_SIZE) as u64, e)))
        {
            write_ptr_to_string(&mut writer, &entry.command_link, current_offset as u64, 0x0);
            write_ptr_to_string(&mut writer, &entry.char_name, current_offset as u64, 0x10);
            write_ptr_to_string(&mut writer, &entry.characode, current_offset as u64, 0x18);
            write_ptr_to_string(&mut writer, &entry.command_name, current_offset as u64, 0x28);
            write_ptr_to_string(&mut writer, &entry.btn_input, current_offset as u64, 0x30);
            write_ptr_to_string(&mut writer, &entry.condition1, current_offset as u64, 0x38);
            write_ptr_to_string(&mut writer, &entry.condition2, current_offset as u64, 0x40);
            write_ptr_to_string(&mut writer, &entry.additional_command_link, current_offset as u64, 0x60);

           
        }

        // Go to the start of buffer and write the size
        writer.set_position(0);
        writer.write_be::<u32>(&((writer.get_ref().len() - 4) as u32)).unwrap();
 
        writer.into_inner()
    }
}




