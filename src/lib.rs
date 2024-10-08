use strum_macros::{EnumString, EnumIter, Display};
use regex::Regex;

pub mod args;
pub mod nucc_binary;


#[derive(Debug, Copy, Clone, EnumString, EnumIter, Display, PartialEq, Hash, Eq)]
pub enum NuccBinaryType {
    AccessoriesParam,
    AccessoryExceptionParam,
    AccessoryParam,
    AnimeSongBgmParam,
    Anmofs,
    BodAcc,
    #[strum(ascii_case_insensitive)]
    Characode,
    CharaPoseParam,
    CharacterSelectParam,
    ComboPrm,
    CommandListParam,
    CostumeBreakParam,
    CostumeParam,
    Dds,
    DictionaryCharacterParam,
    DlcInfoParam,
    EffectPrm,
    Ev,
    EvSpl,
    FinalSpSkillCutIn,
    Lua,
    MessageInfo,
    OugiFinishParam,
    PlayerDoubleEffectParam,
    PlayerSettingParam,
    PlayerIcon,
    Png,
    PrmBas,
    PrmLoad,
    ProhibitedSubstringParam,
    SkillIndexSettingParam,
    Snd,
    StaffRollTextParam,
    SupportActionParam,
    SupportSkillRecoverySpeedParam,
    UpdateInfoParam,
    Xml,
}

impl NuccBinaryType {
    pub fn patterns(&self) -> Regex {

        match self {
            NuccBinaryType::AccessoriesParam => { Regex::new(r"(accessoriesParam\.bin)$").unwrap() },
            NuccBinaryType::AccessoryExceptionParam=> { Regex::new(r"(accessoryExceptionParam\.bin)$").unwrap() },
            NuccBinaryType::AccessoryParam => { Regex::new(r"(accessoryParam\.bin)$").unwrap() },
            NuccBinaryType::AnimeSongBgmParam => { Regex::new(r"(animeSongBgmParam\.bin)$").unwrap() },
            NuccBinaryType::Anmofs => { Regex::new(r"(anm_offset)").unwrap() },
            NuccBinaryType::BodAcc => { Regex::new(r"(bod1acc\.bin)$").unwrap() },
            NuccBinaryType::Characode => { Regex::new(r"(characode\.bin)$").unwrap() },
            NuccBinaryType::CharaPoseParam => { Regex::new(r"(CharaPoseParam\.bin)$").unwrap() },
            NuccBinaryType::CharacterSelectParam => { Regex::new(r"(characterSelectParam\.bin)$").unwrap() },
            NuccBinaryType::ComboPrm => { Regex::new(r"(comboPrm\.bin)$").unwrap() },
            NuccBinaryType::CommandListParam => { Regex::new(r"(commandListParam\.bin)$").unwrap() },
            NuccBinaryType::CostumeBreakParam => { Regex::new(r"(costumeBreakParam\.bin)$").unwrap() },
            NuccBinaryType::CostumeParam => { Regex::new(r"(costumeParam\.bin)$").unwrap() },
            NuccBinaryType::Dds => { Regex::new(r"(\.dds)$").unwrap() },
            NuccBinaryType::DictionaryCharacterParam => { Regex::new(r"(DictionaryCharacterParam\.bin)$").unwrap() },
            NuccBinaryType::DlcInfoParam => { Regex::new(r"(DlcInfoParam\.bin)$").unwrap() },
            NuccBinaryType::EffectPrm => { Regex::new(r"(effectprm.*\.bin)$").unwrap() },
            NuccBinaryType::Ev => { Regex::new(r"(_ev.bin)").unwrap() },
            NuccBinaryType::EvSpl => { Regex::new(r"(_ev_spl\.bin)").unwrap() },
            NuccBinaryType::FinalSpSkillCutIn => { Regex::new(r"(finalSpSkillCutIn\.bin)$").unwrap() },
            NuccBinaryType::Lua => { Regex::new(r"(\.lua)$").unwrap() },
            NuccBinaryType::MessageInfo => { Regex::new(r"(messageInfo\.bin)$").unwrap() },
            NuccBinaryType::OugiFinishParam => { Regex::new(r"(OugiFinishParam\.bin)$").unwrap() },
            NuccBinaryType::PlayerDoubleEffectParam => { Regex::new(r"(playerDoubleEffectParam\.bin)$").unwrap() },
            NuccBinaryType::PlayerSettingParam => { Regex::new(r"(playerSettingParam\.bin)$").unwrap() },
            NuccBinaryType::PlayerIcon => { Regex::new(r"(player_icon\.bin)$").unwrap() },
            NuccBinaryType::Png => { Regex::new(r"(\.png)$").unwrap() },
            NuccBinaryType::PrmBas => { Regex::new(r"(prm_bas)").unwrap() },
            NuccBinaryType::PrmLoad => { Regex::new(r"(prm_load\.bin)$").unwrap() },
            NuccBinaryType::ProhibitedSubstringParam => { Regex::new(r"(prohibitedSubstringParam\.bin)$").unwrap() },
            NuccBinaryType::SkillIndexSettingParam => { Regex::new(r"(skillIndexSettingParam\.bin)$").unwrap() },
            NuccBinaryType::Snd => { Regex::new(r"(snd.*\.bin)$").unwrap() },
            NuccBinaryType::StaffRollTextParam => { Regex::new(r"(staffRollTextParam\.bin)$").unwrap() },
            NuccBinaryType::SupportActionParam => { Regex::new(r"(supportActionParam\.bin)$").unwrap() },
            NuccBinaryType::SupportSkillRecoverySpeedParam => { Regex::new(r"(supportSkillRecoverySpeedParam\.bin)$").unwrap() },
            NuccBinaryType::UpdateInfoParam => { Regex::new(r"(updateInfoParam\.bin)$").unwrap() }
            NuccBinaryType::Xml => { Regex::new(r"(\.xml)$").unwrap() }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::path::Path;
    use xfbin::read_xfbin;
    use super::NuccBinaryType;
    
    /*use super::nucc_binary::NuccBinaryParsed;

      
    #[test]
    fn characode_test() {
        let filepath = Path::new("9ind_x.xfbin");
        let xfbin = read_xfbin(filepath).unwrap();

        let _ = dbg!(xfbin.pages.len());

        for chunk in &xfbin.get_chunks_by_type("nuccChunkBinary") {
            let bytes = chunk.data.as_bytes();
            let reader = NuccBinaryParsedReader(NuccBinaryType::Characode, &bytes);
            let nucc_binary_parsed: Box<dyn NuccBinaryParsed> = reader.into();
            let characode: &Characode = nucc_binary_parsed.as_any().downcast_ref::<Characode>().unwrap();
            characode.serialize(); // Serialize to JSON
        }
    }*/
}