use strum_macros::{EnumString, EnumIter, Display};
use regex::Regex;

#[derive(Debug, Copy, Clone, EnumString, EnumIter, Display, PartialEq)]
pub enum NuccBinaryType {
    AccessoryParam,
    AnimeSongBgmParam,
    #[strum(ascii_case_insensitive)]
    Characode,
    CharaPoseParam,
    CharacterSelectParam,
    CostumeParam,
    Dds,
    DlcInfoParam,
    EffectPrm,
    Ev,
    Lua,
    MessageInfo,
    PlayerDoubleEffectParam,
    PlayerSettingParam,
    PlayerIcon,
    Png,
    PrmLoad,
    StaffRollTextParam,
    SupportActionParam,
    Xml,
}

impl NuccBinaryType {
    pub fn patterns(&self) -> Regex {

        match self {
            NuccBinaryType::AccessoryParam => { Regex::new(r"(accessoryParam\.bin)$").unwrap() },
            NuccBinaryType::AnimeSongBgmParam => { Regex::new(r"(animeSongBgmParam\.bin)$").unwrap() },
            NuccBinaryType::Characode => { Regex::new(r"(characode\.bin)$").unwrap() },
            NuccBinaryType::CharaPoseParam => { Regex::new(r"(CharaPoseParam\.bin)$").unwrap() },
            NuccBinaryType::CharacterSelectParam => { Regex::new(r"(characterSelectParam\.bin)$").unwrap() },
            NuccBinaryType::CostumeParam => { Regex::new(r"(costumeParam\.bin)$").unwrap() },
            NuccBinaryType::Dds => { Regex::new(r"(\.dds)$").unwrap() },
            NuccBinaryType::DlcInfoParam => { Regex::new(r"(DlcInfoParam\.bin)$").unwrap() },
            NuccBinaryType::EffectPrm => { Regex::new(r"(effectprm.*\.bin)$").unwrap() },
            NuccBinaryType::Ev => { Regex::new(r"(ev.*\.bin)$").unwrap() },
            NuccBinaryType::Lua => { Regex::new(r"(\.lua)$").unwrap() },
            NuccBinaryType::MessageInfo => { Regex::new(r"(messageInfo\.bin)$").unwrap() },
            NuccBinaryType::PlayerDoubleEffectParam => { Regex::new(r"(playerDoubleEffectParam\.bin)$").unwrap() },
            NuccBinaryType::PlayerSettingParam => { Regex::new(r"(playerSettingParam\.bin)$").unwrap() },
            NuccBinaryType::PlayerIcon => { Regex::new(r"(player_icon\.bin)$").unwrap() },
            NuccBinaryType::Png => { Regex::new(r"(\.png)$").unwrap() },
            NuccBinaryType::PrmLoad => { Regex::new(r"(prm_load\.bin)$").unwrap() },
            NuccBinaryType::StaffRollTextParam => { Regex::new(r"(staffRollTextParam\.bin)$").unwrap() },
            NuccBinaryType::SupportActionParam => { Regex::new(r"(supportActionParam\.bin)$").unwrap() },
            NuccBinaryType::Xml => { Regex::new(r"(\.xml)$").unwrap() }
        }
    }
}