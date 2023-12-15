use strum_macros::{EnumString, EnumIter, Display};
use regex::Regex;

#[derive(Debug, Copy, Clone, EnumString, EnumIter, Display, PartialEq)]
pub enum NuccBinaryType {
    AccessoriesParam,
    AccessoryExceptionParam,
    AccessoryParam,
    AnimeSongBgmParam,
    #[strum(ascii_case_insensitive)]
    Characode,
    CharaPoseParam,
    CharacterSelectParam,
    ComboPrm,
    CommandListParam,
    CostumeParam,
    Dds,
    DlcInfoParam,
    EffectPrm,
    Ev,
    FinalSpSkillCutIn,
    Lua,
    MessageInfo,
    PlayerDoubleEffectParam,
    PlayerSettingParam,
    PlayerIcon,
    Png,
    PrmLoad,
    ProhibitedSubstringParam,
    SkillIndexSettingParam,
    StaffRollTextParam,
    SupportActionParam,
    SupportSkillRecoverySpeedParam,
    Xml,
}

impl NuccBinaryType {
    pub fn patterns(&self) -> Regex {

        match self {
            NuccBinaryType::AccessoriesParam => { Regex::new(r"(accessoriesParam\.bin)$").unwrap() },
            NuccBinaryType::AccessoryExceptionParam=> { Regex::new(r"(accessoryExceptionParam\.bin)$").unwrap() },
            NuccBinaryType::AccessoryParam => { Regex::new(r"(accessoryParam\.bin)$").unwrap() },
            NuccBinaryType::AnimeSongBgmParam => { Regex::new(r"(animeSongBgmParam\.bin)$").unwrap() },
            NuccBinaryType::Characode => { Regex::new(r"(characode\.bin)$").unwrap() },
            NuccBinaryType::CharaPoseParam => { Regex::new(r"(CharaPoseParam\.bin)$").unwrap() },
            NuccBinaryType::CharacterSelectParam => { Regex::new(r"(characterSelectParam\.bin)$").unwrap() },
            NuccBinaryType::ComboPrm => { Regex::new(r"(comboPrm\.bin)$").unwrap() },
            NuccBinaryType::CommandListParam => { Regex::new(r"(commandListParam\.bin)$").unwrap() },
            NuccBinaryType::CostumeParam => { Regex::new(r"(costumeParam\.bin)$").unwrap() },
            NuccBinaryType::Dds => { Regex::new(r"(\.dds)$").unwrap() },
            NuccBinaryType::DlcInfoParam => { Regex::new(r"(DlcInfoParam\.bin)$").unwrap() },
            NuccBinaryType::EffectPrm => { Regex::new(r"(effectprm.*\.bin)$").unwrap() },
            NuccBinaryType::Ev => { Regex::new(r"(ev.*\.bin)$").unwrap() },
            NuccBinaryType::FinalSpSkillCutIn => { Regex::new(r"(finalSpSkillCutIn\.bin)$").unwrap() },
            NuccBinaryType::Lua => { Regex::new(r"(\.lua)$").unwrap() },
            NuccBinaryType::MessageInfo => { Regex::new(r"(messageInfo\.bin)$").unwrap() },
            NuccBinaryType::PlayerDoubleEffectParam => { Regex::new(r"(playerDoubleEffectParam\.bin)$").unwrap() },
            NuccBinaryType::PlayerSettingParam => { Regex::new(r"(playerSettingParam\.bin)$").unwrap() },
            NuccBinaryType::PlayerIcon => { Regex::new(r"(player_icon\.bin)$").unwrap() },
            NuccBinaryType::Png => { Regex::new(r"(\.png)$").unwrap() },
            NuccBinaryType::PrmLoad => { Regex::new(r"(prm_load\.bin)$").unwrap() },
            NuccBinaryType::ProhibitedSubstringParam => { Regex::new(r"(prohibitedSubstringParam\.bin)$").unwrap() },
            NuccBinaryType::SkillIndexSettingParam => { Regex::new(r"(skillIndexSettingParam\.bin)$").unwrap() },
            NuccBinaryType::StaffRollTextParam => { Regex::new(r"(staffRollTextParam\.bin)$").unwrap() },
            NuccBinaryType::SupportActionParam => { Regex::new(r"(supportActionParam\.bin)$").unwrap() },
            NuccBinaryType::SupportSkillRecoverySpeedParam => { Regex::new(r"(supportSkillRecoverySpeedParam\.bin)$").unwrap() },
            NuccBinaryType::Xml => { Regex::new(r"(\.xml)$").unwrap() }
        }
    }
}