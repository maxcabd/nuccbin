mod accessory_param;
mod anime_song_bgm_param;
mod anmoffset;
mod characode;
mod chara_pose_param;
mod character_select_param;
mod costume_param;
mod dds;
mod dlc_info_param;
mod effectprm;
mod ev;
mod lua;
mod message_info;
mod player_double_effect_param; 
mod player_setting_param;
mod player_icon;
mod png;
mod prm_load;
mod staff_roll_text_param;
mod support_action_param;  
mod xml;

use binrw::{BinReaderExt, BinWriterExt};
use binrw::io::Cursor;
use downcast_rs::{impl_downcast, Downcast};

use super::NuccBinaryType;

//--------------------//
pub use accessory_param::AccessoryParam;
pub use anime_song_bgm_param::AnimeSongBgmParam;
pub use characode::Characode;
pub use chara_pose_param::CharaPoseParam;
pub use character_select_param::CharacterSelectParam;
pub use costume_param::CostumeParam;
pub use dds::Dds;
pub use dlc_info_param::DlcInfoParam;
pub use ev::Ev;
pub use effectprm::EffectPrm;
pub use lua::Lua;
pub use message_info::MessageInfo;
pub use player_double_effect_param::PlayerDoubleEffectParam;
pub use player_setting_param::PlayerSettingParam;
pub use player_icon::PlayerIcon;
pub use png::Png;
pub use prm_load::PrmLoad;
pub use staff_roll_text_param::StaffRollTextParam;
pub use support_action_param::SupportActionParam;
pub use xml::Xml;


//--------------------//
pub trait NuccBinaryParsed: Downcast {
    fn binary_type(&self) -> NuccBinaryType;
    fn extension(&self) -> String;
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Self
    where 
        Self: Sized;
}

impl_downcast!(NuccBinaryParsed);

pub struct NuccBinaryParsedReader<'a> (pub NuccBinaryType, pub &'a [u8]);

impl From<NuccBinaryParsedReader<'_>> for Box<dyn NuccBinaryParsed> {
    fn from(reader: NuccBinaryParsedReader<'_>) -> Self {
        let NuccBinaryParsedReader(nucc_binary_type, data) = reader;

        match nucc_binary_type {
            NuccBinaryType::AccessoryParam => Box::new(AccessoryParam::from(&data[..])),
            NuccBinaryType::AnimeSongBgmParam => Box::new(AnimeSongBgmParam::from(&data[..])),
            
            NuccBinaryType::Characode => {
                let mut characode = Cursor::new(data);
                Box::new(characode.read_le::<Characode>().unwrap())
            }
            NuccBinaryType::CharaPoseParam => Box::new(CharaPoseParam::from(&data[..])),
            NuccBinaryType::CharacterSelectParam => Box::new(CharacterSelectParam::from(&data[..])),
            NuccBinaryType::CostumeParam => Box::new(CostumeParam::from(&data[..])),
            NuccBinaryType::Dds => Box::new(Dds::from(&data[..])),
            NuccBinaryType::DlcInfoParam => Box::new(DlcInfoParam::from(&data[..])),

            NuccBinaryType::EffectPrm => {
                let mut effect_prm = Cursor::new(data);
                Box::new(effect_prm.read_le::<EffectPrm>().unwrap())
            }

            NuccBinaryType::Ev => {
                let mut ev = Cursor::new(data);
                Box::new(ev.read_le::<Ev>().unwrap())
            }

            NuccBinaryType::Lua => Box::new(Lua::from(&data[..])),
            NuccBinaryType::MessageInfo => Box::new(MessageInfo::from(&data[..])),
            NuccBinaryType::PlayerDoubleEffectParam => Box::new(PlayerDoubleEffectParam::from(&data[..])),
            NuccBinaryType::PlayerSettingParam => Box::new(PlayerSettingParam::from(&data[..])),
            NuccBinaryType::PlayerIcon => Box::new(PlayerIcon::from(&data[..])),
            NuccBinaryType::Png => Box::new(Png::from(&data[..])),

            NuccBinaryType::PrmLoad => {
                let mut prm_load = Cursor::new(data);
                Box::new(prm_load.read_le::<PrmLoad>().unwrap())
            }

            NuccBinaryType::StaffRollTextParam => Box::new(StaffRollTextParam::from(&data[..])),
            NuccBinaryType::SupportActionParam => {
                let mut support_action_param = Cursor::new(data);
                Box::new(support_action_param.read_le::<SupportActionParam>().unwrap())
            }
            NuccBinaryType::Xml => Box::new(Xml::from(&data[..])),
        }
    }
}

pub struct NuccBinaryParsedWriter(pub Box<dyn NuccBinaryParsed>);

impl From<NuccBinaryParsedWriter> for Vec<u8> {
    fn from(writer: NuccBinaryParsedWriter) -> Self {
        let NuccBinaryParsedWriter(boxed) = writer;
        
        match boxed.binary_type() {
            NuccBinaryType::AccessoryParam => { (*boxed.downcast::<AccessoryParam>().ok().unwrap()).into() },
            NuccBinaryType::AnimeSongBgmParam => { (*boxed.downcast::<AnimeSongBgmParam>().ok().unwrap()).into() },

            NuccBinaryType::Characode => {
                let mut characode = Cursor::new(Vec::new());
                characode.write_le(&*boxed.downcast::<Characode>().ok().unwrap()).unwrap();
                
                characode.set_position(0);
                characode.write_be::<u32>(&((characode.get_ref().len() - 4) as u32)).unwrap();
                characode.into_inner()
            },

            NuccBinaryType::CharaPoseParam => { (*boxed.downcast::<CharaPoseParam>().ok().unwrap()).into() },
            NuccBinaryType::CharacterSelectParam => { (*boxed.downcast::<CharacterSelectParam>().ok().unwrap()).into() },
            NuccBinaryType::CostumeParam => { (*boxed.downcast::<CostumeParam>().ok().unwrap()).into() },
            NuccBinaryType::Dds => { (*boxed.downcast::<Dds>().ok().unwrap()).into() },
            NuccBinaryType::DlcInfoParam => { (*boxed.downcast::<DlcInfoParam>().ok().unwrap()).into() },

            NuccBinaryType::EffectPrm => {
                let mut effect_prm = Cursor::new(Vec::new());
                effect_prm.write_le(&*boxed.downcast::<EffectPrm>().ok().unwrap()).unwrap();
                
                effect_prm.set_position(0);
                effect_prm.write_be::<u32>(&((effect_prm.get_ref().len() - 4) as u32)).unwrap();
                effect_prm.into_inner()
            },

            NuccBinaryType::Ev => {
                let mut ev = Cursor::new(Vec::new());
                ev.write_le(&*boxed.downcast::<Ev>().ok().unwrap()).unwrap();
                
                ev.set_position(0);
                ev.write_be::<u32>(&((ev.get_ref().len() - 4) as u32)).unwrap();
                ev.into_inner()
            },

            NuccBinaryType::Lua => { (*boxed.downcast::<Lua>().ok().unwrap()).into() },
            NuccBinaryType::MessageInfo => { (*boxed.downcast::<MessageInfo>().ok().unwrap()).into() },
            NuccBinaryType::PlayerDoubleEffectParam => { (*boxed.downcast::<PlayerDoubleEffectParam>().ok().unwrap()).into() },
            NuccBinaryType::PlayerSettingParam => { (*boxed.downcast::<PlayerSettingParam>().ok().unwrap()).into() },
            NuccBinaryType::PlayerIcon => { (*boxed.downcast::<PlayerIcon>().ok().unwrap()).into() },
            NuccBinaryType::Png => { (*boxed.downcast::<Png>().ok().unwrap()).into() },

            NuccBinaryType::PrmLoad => {
                let mut prm_load = Cursor::new(Vec::new());
                prm_load.write_le(&*boxed.downcast::<PrmLoad>().ok().unwrap()).unwrap();
                
                prm_load.set_position(0);
                prm_load.write_be::<u32>(&((prm_load.get_ref().len() - 4) as u32)).unwrap();
                prm_load.into_inner()
            },

            NuccBinaryType::SupportActionParam => {
                let mut support_action_param = Cursor::new(Vec::new());
                support_action_param.write_le(&*boxed.downcast::<SupportActionParam>().ok().unwrap()).unwrap();
                
                support_action_param.set_position(0);
                support_action_param.write_be::<u32>(&((support_action_param.get_ref().len() - 4) as u32)).unwrap();
                support_action_param.into_inner()
            },

            NuccBinaryType::StaffRollTextParam => { (*boxed.downcast::<StaffRollTextParam>().ok().unwrap()).into() },

            NuccBinaryType::Xml => { (*boxed.downcast::<Xml>().ok().unwrap()).into() }
        }
    }
}

pub struct NuccBinaryParsedSerializer(pub Box<dyn NuccBinaryParsed>);

impl From<NuccBinaryParsedSerializer> for Vec<u8> {
    fn from(serializer: NuccBinaryParsedSerializer) -> Self {
        let NuccBinaryParsedSerializer(nucc_binary_parsed) = serializer;
        nucc_binary_parsed.serialize()
    }
}

#[derive(Debug)]
pub struct NuccBinaryParsedDeserializer(pub NuccBinaryType, pub Vec<u8>);

impl From<NuccBinaryParsedDeserializer> for Box<dyn NuccBinaryParsed> {
    fn from(deserializer: NuccBinaryParsedDeserializer) -> Self {
       let NuccBinaryParsedDeserializer(nucc_binary_type, data) = deserializer;

        match nucc_binary_type {
            NuccBinaryType::AccessoryParam => Box::new(AccessoryParam::deserialize(&data)),
            NuccBinaryType::AnimeSongBgmParam => Box::new(AnimeSongBgmParam::deserialize(&data)),
            NuccBinaryType::Characode => Box::new(Characode::deserialize(&data)),
            NuccBinaryType::CharaPoseParam => Box::new(CharaPoseParam::deserialize(&data)),
            NuccBinaryType::CharacterSelectParam => Box::new(CharacterSelectParam::deserialize(&data)),
            NuccBinaryType::CostumeParam => Box::new(CostumeParam::deserialize(&data)),
            NuccBinaryType::Dds => Box::new(Dds::deserialize(&data)),
            NuccBinaryType::DlcInfoParam => Box::new(DlcInfoParam::deserialize(&data)),
            NuccBinaryType::EffectPrm => Box::new(EffectPrm::deserialize(&data)),
            NuccBinaryType::Ev => Box::new(Ev::deserialize(&data)),
            NuccBinaryType::Lua => Box::new(Lua::deserialize(&data)),
            NuccBinaryType::MessageInfo => Box::new(MessageInfo::deserialize(&data)),
            NuccBinaryType::PlayerDoubleEffectParam => Box::new(PlayerDoubleEffectParam::deserialize(&data)),
            NuccBinaryType::PlayerSettingParam => Box::new(PlayerSettingParam::deserialize(&data)),
            NuccBinaryType::PlayerIcon => Box::new(PlayerIcon::deserialize(&data)),
            NuccBinaryType::Png => Box::new(Png::deserialize(&data)),
            NuccBinaryType::PrmLoad => Box::new(PrmLoad::deserialize(&data)),
            NuccBinaryType::StaffRollTextParam => Box::new(StaffRollTextParam::deserialize(&data)),
            NuccBinaryType::SupportActionParam => Box::new(SupportActionParam::deserialize(&data)),
            NuccBinaryType::Xml => Box::new(Xml::deserialize(&data))
        }
    }
}