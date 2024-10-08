mod accessories_param;
mod accessory_exception_param;
mod accessory_param;
mod anime_song_bgm_param;
mod anm_offset;
mod bodacc;
mod characode;
mod chara_pose_param;
mod character_select_param;
mod combo_prm;
mod command_list_param;
mod costume_break_param;
mod costume_param;
mod dds;
mod dictionary_character_param;
mod dlc_info_param;
mod effectprm;
mod ev;
mod ev_spl;
mod final_sp_skill_cutin;
mod lua;
pub mod message_info;
mod ougi_finish_param;
mod player_double_effect_param; 
mod player_setting_param;
mod player_icon;
mod png;
mod prm_bas;
mod prm_load;
mod prohibited_substring_param;
mod skill_index_setting_param;
mod snd;
mod staff_roll_text_param;
mod support_action_param;
mod support_skill_recovery_speed_param;
mod update_info_param;
mod xml;

use binrw::{BinReaderExt, BinWriterExt};
use binrw::io::Cursor;
use downcast_rs::{impl_downcast, Downcast};

use super::NuccBinaryType;

pub const HEADER_SIZE: usize = 0x10; // Size of NUCC Binary headers

//--------------------//
pub use accessories_param::AccessoriesParam;
pub use accessory_exception_param::AccessoryExceptionParam;
pub use accessory_param::AccessoryParam;
pub use anime_song_bgm_param::AnimeSongBgmParam;
pub use anm_offset::Anmofs;
pub use bodacc::BodAcc;
pub use characode::Characode;
pub use chara_pose_param::CharaPoseParam;
pub use character_select_param::CharacterSelectParam;
pub use combo_prm::ComboPrm;
pub use command_list_param::CommandListParam;
pub use costume_break_param::CostumeBreakParam;
pub use costume_param::CostumeParam;
pub use dds::Dds;
pub use dictionary_character_param::DictionaryCharacterParam;
pub use dlc_info_param::DlcInfoParam;
pub use ev::Ev;
pub use ev_spl::EvSpl;
pub use final_sp_skill_cutin::FinalSpSkillCutIn;
pub use effectprm::EffectPrm;
pub use lua::Lua;
pub use message_info::MessageInfo;
pub use ougi_finish_param::OugiFinishParam;
pub use player_double_effect_param::PlayerDoubleEffectParam;
pub use player_setting_param::PlayerSettingParam;
pub use player_icon::PlayerIcon;
pub use png::Png;
pub use prm_bas::PrmBas;
pub use prm_load::PrmLoad;
pub use prohibited_substring_param::ProhibitedSubstringParam;
pub use skill_index_setting_param::SkillIndexSettingParam;
pub use snd::Snd;
pub use staff_roll_text_param::StaffRollTextParam;
pub use support_action_param::SupportActionParam;
pub use support_skill_recovery_speed_param::SupportSkillRecoverySpeedParam;
pub use update_info_param::UpdateInfoParam;
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
            NuccBinaryType::AccessoriesParam => Box::new(AccessoriesParam::from(&data[..])),
            NuccBinaryType::AccessoryExceptionParam => Box::new(AccessoryExceptionParam::from(&data[..])),
            NuccBinaryType::AccessoryParam => Box::new(AccessoryParam::from(&data[..])),
            NuccBinaryType::AnimeSongBgmParam => Box::new(AnimeSongBgmParam::from(&data[..])),
            NuccBinaryType::Anmofs => {
                let mut anm_offset = Cursor::new(data);
                Box::new(anm_offset.read_le::<Anmofs>().unwrap())
            }

            NuccBinaryType::BodAcc => Box::new(BodAcc::from(&data[..])),
            
            NuccBinaryType::Characode => {
                let mut characode = Cursor::new(data);
                Box::new(characode.read_le::<Characode>().unwrap())
            }

            NuccBinaryType::CharaPoseParam => Box::new(CharaPoseParam::from(&data[..])),
            NuccBinaryType::CharacterSelectParam => Box::new(CharacterSelectParam::from(&data[..])),

            NuccBinaryType::ComboPrm => {
                let mut combo_prm = Cursor::new(data);
                Box::new(combo_prm.read_le::<ComboPrm>().unwrap())
            }

            NuccBinaryType::CommandListParam => Box::new(CommandListParam::from(&data[..])),
            NuccBinaryType::CostumeBreakParam => Box::new(CostumeBreakParam::from(&data[..])),
            NuccBinaryType::CostumeParam => Box::new(CostumeParam::from(&data[..])),
            NuccBinaryType::Dds => Box::new(Dds::from(&data[..])),
            NuccBinaryType::DictionaryCharacterParam => Box::new(DictionaryCharacterParam::from(&data[..])),
            NuccBinaryType::DlcInfoParam => Box::new(DlcInfoParam::from(&data[..])),

            NuccBinaryType::EffectPrm => {
                let mut effect_prm = Cursor::new(data);
                Box::new(effect_prm.read_le::<EffectPrm>().unwrap())
            }

            NuccBinaryType::Ev => {
                let mut ev = Cursor::new(data);
                Box::new(ev.read_le::<Ev>().unwrap())
            }

            NuccBinaryType::EvSpl => {
                let mut ev_spl = Cursor::new(data);
                Box::new(ev_spl.read_le::<EvSpl>().unwrap())
            }

            NuccBinaryType::FinalSpSkillCutIn => Box::new(FinalSpSkillCutIn::from(&data[..])),

            NuccBinaryType::Lua => Box::new(Lua::from(&data[..])),
            NuccBinaryType::MessageInfo => Box::new(MessageInfo::from(&data[..])),
            NuccBinaryType::OugiFinishParam => Box::new(OugiFinishParam::from(&data[..])),
            NuccBinaryType::PlayerDoubleEffectParam => Box::new(PlayerDoubleEffectParam::from(&data[..])),
            NuccBinaryType::PlayerSettingParam => Box::new(PlayerSettingParam::from(&data[..])),
            NuccBinaryType::PlayerIcon => Box::new(PlayerIcon::from(&data[..])),
            NuccBinaryType::Png => Box::new(Png::from(&data[..])),

            NuccBinaryType::PrmBas => {
                let mut prm_bas = Cursor::new(data);
                Box::new(prm_bas.read_le::<PrmBas>().unwrap())
            }

            NuccBinaryType::PrmLoad => {
                let mut prm_load = Cursor::new(data);
                Box::new(prm_load.read_le::<PrmLoad>().unwrap())
            }

            NuccBinaryType::ProhibitedSubstringParam => Box::new(ProhibitedSubstringParam::from(&data[..])),

            NuccBinaryType::SkillIndexSettingParam => {
                let mut skill_index_setting_param = Cursor::new(data);
                Box::new(skill_index_setting_param.read_le::<SkillIndexSettingParam>().unwrap())
            }

            NuccBinaryType::Snd => {
                let mut snd = Cursor::new(data);
                Box::new(snd.read_le::<Snd>().unwrap())
            }

            NuccBinaryType::StaffRollTextParam => Box::new(StaffRollTextParam::from(&data[..])),

            NuccBinaryType::SupportActionParam => {
                let mut support_action_param = Cursor::new(data);
                Box::new(support_action_param.read_le::<SupportActionParam>().unwrap())
            }

            NuccBinaryType::SupportSkillRecoverySpeedParam => {
                let mut support_skill_recovery_speed_param = Cursor::new(data);
                Box::new(support_skill_recovery_speed_param.read_le::<SupportSkillRecoverySpeedParam>().unwrap())
            }

            NuccBinaryType::UpdateInfoParam => Box::new(UpdateInfoParam::from(&data[..])),

            NuccBinaryType::Xml => Box::new(Xml::from(&data[..])),
        }
    }
}

pub struct NuccBinaryParsedWriter(pub Box<dyn NuccBinaryParsed>);

impl From<NuccBinaryParsedWriter> for Vec<u8> {
    fn from(writer: NuccBinaryParsedWriter) -> Self {
        let NuccBinaryParsedWriter(boxed) = writer;
        
        match boxed.binary_type() {
            NuccBinaryType::AccessoriesParam => { (*boxed.downcast::<AccessoriesParam>().ok().unwrap()).into() },
            NuccBinaryType::AccessoryExceptionParam => { (*boxed.downcast::<AccessoryExceptionParam>().ok().unwrap()).into() },
            NuccBinaryType::AccessoryParam => { (*boxed.downcast::<AccessoryParam>().ok().unwrap()).into() },
            NuccBinaryType::AnimeSongBgmParam => { (*boxed.downcast::<AnimeSongBgmParam>().ok().unwrap()).into() },

            NuccBinaryType::Anmofs => {
                let mut anm_offset = Cursor::new(Vec::new());
                anm_offset.write_le(&*boxed.downcast::<Anmofs>().ok().unwrap()).unwrap();
                anm_offset.into_inner()
            },

            NuccBinaryType::BodAcc => { (*boxed.downcast::<BodAcc>().ok().unwrap()).into() },
            NuccBinaryType::Characode => {
                let mut characode = Cursor::new(Vec::new());
                characode.write_le(&*boxed.downcast::<Characode>().ok().unwrap()).unwrap();
                characode.into_inner()
            },

            NuccBinaryType::CharaPoseParam => { (*boxed.downcast::<CharaPoseParam>().ok().unwrap()).into() },
            NuccBinaryType::CharacterSelectParam => { (*boxed.downcast::<CharacterSelectParam>().ok().unwrap()).into() },
            NuccBinaryType::ComboPrm => {
                let mut combo_prm = Cursor::new(Vec::new());
                combo_prm.write_le(&*boxed.downcast::<ComboPrm>().ok().unwrap()).unwrap();
                combo_prm.into_inner()
            },

            NuccBinaryType::CommandListParam => { (*boxed.downcast::<CommandListParam>().ok().unwrap()).into() },
            NuccBinaryType::CostumeBreakParam => { (*boxed.downcast::<CostumeBreakParam>().ok().unwrap()).into() },
            NuccBinaryType::CostumeParam => { (*boxed.downcast::<CostumeParam>().ok().unwrap()).into() },
            NuccBinaryType::Dds => { (*boxed.downcast::<Dds>().ok().unwrap()).into() },
            NuccBinaryType::DictionaryCharacterParam => { (*boxed.downcast::<DictionaryCharacterParam>().ok().unwrap()).into() },
            NuccBinaryType::DlcInfoParam => { (*boxed.downcast::<DlcInfoParam>().ok().unwrap()).into() },
            NuccBinaryType::EffectPrm => {
                let mut effect_prm = Cursor::new(Vec::new());
                effect_prm.write_le(&*boxed.downcast::<EffectPrm>().ok().unwrap()).unwrap();
                effect_prm.into_inner()
            },

            NuccBinaryType::Ev => {
                let mut ev = Cursor::new(Vec::new());
                ev.write_le(&*boxed.downcast::<Ev>().ok().unwrap()).unwrap();
                ev.into_inner()
            },

            NuccBinaryType::EvSpl => {
                let mut ev_spl = Cursor::new(Vec::new());
                ev_spl.write_le(&*boxed.downcast::<EvSpl>().ok().unwrap()).unwrap();
                ev_spl.into_inner()
            },

            NuccBinaryType::FinalSpSkillCutIn => { (*boxed.downcast::<FinalSpSkillCutIn>().ok().unwrap()).into() },
            NuccBinaryType::Lua => { (*boxed.downcast::<Lua>().ok().unwrap()).into() },
            NuccBinaryType::MessageInfo => { (*boxed.downcast::<MessageInfo>().ok().unwrap()).into() },
            NuccBinaryType::OugiFinishParam => { (*boxed.downcast::<OugiFinishParam>().ok().unwrap()).into() },
            NuccBinaryType::PlayerDoubleEffectParam => { (*boxed.downcast::<PlayerDoubleEffectParam>().ok().unwrap()).into() },
            NuccBinaryType::PlayerSettingParam => { (*boxed.downcast::<PlayerSettingParam>().ok().unwrap()).into() },
            NuccBinaryType::PlayerIcon => { (*boxed.downcast::<PlayerIcon>().ok().unwrap()).into() },
            NuccBinaryType::Png => { (*boxed.downcast::<Png>().ok().unwrap()).into() },

            NuccBinaryType::PrmBas => {
                let mut prm_bas = Cursor::new(Vec::new());
                prm_bas.write_le(&*boxed.downcast::<PrmBas>().ok().unwrap()).unwrap();
                prm_bas.into_inner()
            },

            NuccBinaryType::PrmLoad => {
                let mut prm_load = Cursor::new(Vec::new());
                prm_load.write_le(&*boxed.downcast::<PrmLoad>().ok().unwrap()).unwrap();
                prm_load.into_inner()
            },

            NuccBinaryType::ProhibitedSubstringParam => { (*boxed.downcast::<ProhibitedSubstringParam>().ok().unwrap()).into() },

            NuccBinaryType::SkillIndexSettingParam => {
                let mut skill_index_setting_param = Cursor::new(Vec::new());
                skill_index_setting_param.write_le(&*boxed.downcast::<SkillIndexSettingParam>().ok().unwrap()).unwrap();
                skill_index_setting_param.into_inner()
            },

            NuccBinaryType::Snd => {
                let mut snd = Cursor::new(Vec::new());
                snd.write_le(&*boxed.downcast::<Snd>().ok().unwrap()).unwrap();
                snd.into_inner()
            },

            NuccBinaryType::StaffRollTextParam => { (*boxed.downcast::<StaffRollTextParam>().ok().unwrap()).into() },
            NuccBinaryType::SupportActionParam => {
                let mut support_action_param = Cursor::new(Vec::new());
                support_action_param.write_le(&*boxed.downcast::<SupportActionParam>().ok().unwrap()).unwrap();
                support_action_param.into_inner()
            },

            NuccBinaryType::SupportSkillRecoverySpeedParam => {
                let mut support_skill_recovery_speed_param = Cursor::new(Vec::new());
                support_skill_recovery_speed_param.write_le(&*boxed.downcast::<SupportSkillRecoverySpeedParam>().ok().unwrap()).unwrap();
                support_skill_recovery_speed_param.into_inner()
            },

            NuccBinaryType::UpdateInfoParam => { (*boxed.downcast::<UpdateInfoParam>().ok().unwrap()).into() },
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
            NuccBinaryType::AccessoriesParam => Box::new(AccessoriesParam::deserialize(&data)),
            NuccBinaryType::AccessoryExceptionParam => Box::new(AccessoryExceptionParam::deserialize(&data)),
            NuccBinaryType::AccessoryParam => Box::new(AccessoryParam::deserialize(&data)),
            NuccBinaryType::AnimeSongBgmParam => Box::new(AnimeSongBgmParam::deserialize(&data)),
            NuccBinaryType::Anmofs => Box::new(Anmofs::deserialize(&data)),
            NuccBinaryType::BodAcc => Box::new(BodAcc::deserialize(&data)),
            NuccBinaryType::Characode => Box::new(Characode::deserialize(&data)),
            NuccBinaryType::CharaPoseParam => Box::new(CharaPoseParam::deserialize(&data)),
            NuccBinaryType::CharacterSelectParam => Box::new(CharacterSelectParam::deserialize(&data)),
            NuccBinaryType::ComboPrm => Box::new(ComboPrm::deserialize(&data)),
            NuccBinaryType::CommandListParam => Box::new(CommandListParam::deserialize(&data)),
            NuccBinaryType::CostumeBreakParam => Box::new(CostumeBreakParam::deserialize(&data)),
            NuccBinaryType::CostumeParam => Box::new(CostumeParam::deserialize(&data)),
            NuccBinaryType::Dds => Box::new(Dds::deserialize(&data)),
            NuccBinaryType::DictionaryCharacterParam => Box::new(DictionaryCharacterParam::deserialize(&data)),
            NuccBinaryType::DlcInfoParam => Box::new(DlcInfoParam::deserialize(&data)),
            NuccBinaryType::EffectPrm => Box::new(EffectPrm::deserialize(&data)),
            NuccBinaryType::Ev => Box::new(Ev::deserialize(&data)),
            NuccBinaryType::EvSpl => Box::new(EvSpl::deserialize(&data)),
            NuccBinaryType::FinalSpSkillCutIn => Box::new(FinalSpSkillCutIn::deserialize(&data)),
            NuccBinaryType::Lua => Box::new(Lua::deserialize(&data)),
            NuccBinaryType::MessageInfo => Box::new(MessageInfo::deserialize(&data)),
            NuccBinaryType::OugiFinishParam => Box::new(OugiFinishParam::deserialize(&data)),
            NuccBinaryType::PlayerDoubleEffectParam => Box::new(PlayerDoubleEffectParam::deserialize(&data)),
            NuccBinaryType::PlayerSettingParam => Box::new(PlayerSettingParam::deserialize(&data)),
            NuccBinaryType::PlayerIcon => Box::new(PlayerIcon::deserialize(&data)),
            NuccBinaryType::Png => Box::new(Png::deserialize(&data)),
            NuccBinaryType::PrmBas => Box::new(PrmBas::deserialize(&data)),
            NuccBinaryType::PrmLoad => Box::new(PrmLoad::deserialize(&data)),
            NuccBinaryType::ProhibitedSubstringParam => Box::new(ProhibitedSubstringParam::deserialize(&data)),
            NuccBinaryType::SkillIndexSettingParam => Box::new(SkillIndexSettingParam::deserialize(&data)),
            NuccBinaryType::Snd => Box::new(Snd::deserialize(&data)),
            NuccBinaryType::StaffRollTextParam => Box::new(StaffRollTextParam::deserialize(&data)),
            NuccBinaryType::SupportActionParam => Box::new(SupportActionParam::deserialize(&data)),
            NuccBinaryType::SupportSkillRecoverySpeedParam => Box::new(SupportSkillRecoverySpeedParam::deserialize(&data)),
            NuccBinaryType::UpdateInfoParam => Box::new(UpdateInfoParam::deserialize(&data)),
            NuccBinaryType::Xml => Box::new(Xml::deserialize(&data))
        }
    }
}