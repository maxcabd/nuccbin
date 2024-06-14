# nuccbin
A tool to serialize/deserialize several nuccChunkBinary .xfbin's from Ultimate Ninja STORM CONNECTIONS.

## Usage
- Get the latest version from [releases](https://github.com/maxcabd/nuccbin/releases).
- Drag and drop any of the .xfbin files in the list onto the nuccbin.exe.
- Add the changes to your .json file(s) by adding, removing, or editing entries.
- Apply your changes to the .xfbin file by dragging and dropping the newly created folder onto nuccbin.exe.

## Formats
nuccbin supports a number of in game nuccChunkBinary param / bin formats. All formats support serializing. While some may not support deserializing.
| File | Serialize | Deserialize | Extension |
| --- | --- | --- | --- |
| [accessoriesParam](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/accessories_param.rs) | ✔️ | ✔️ | `json` |
| [accessoryParam](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/accessory_param.rs) | ✔️ | ✔️ | `json` |
| [animeSongBgmParam](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/anime_song_bgm_param.rs) | ✔️ | ✔️ | `json` |
| [anmofs](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/anm_offset.rs) | ✔️ | ✔️ | `json` |
| [characode](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/characode.rs) | ✔️ | ✔️  | `json` |
| [CharaPoseParam](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/chara_pose_param.rs) | ✔️ | ✔️ | `json` |
| [characterSelectParam](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/character_select_param.rs) | ✔️ | ✔️ | `json` |
| [comboPrm](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/combo_prm.rs) | ✔️ | ✔️ | `json` |
| [commmandListParam](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/command_list_param.rs) | ✔️ | ✔️ | `json` |
| [costumeBreakParam](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/costume_break_param.rs) | ✔️ | ✔️ | `json` |
| [costumeParam](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/costume_param.rs) | ✔️ | ✔️ | `json` |
| [dds](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/dds.rs) | ✔️ | ✔️ |  `dds` |
| [DictionaryCharacterParam](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/dictionary_character_param.rs) | ✔️ | ✔️ | `json` |
| [DlcInfoParam](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/dlc_info_param.rs) | ✔️ | ✔️ | `json` |
| [effectprm](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/effectprm.rs) | ✔️ | ✔️ | `json` | 
| [ev](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/ev.rs) | ✔️ | ✔️ | `json` | 
| [lua](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/lua.rs) | ✔️ | ✔️ | `lua` | 
| [messageInfo](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/message_info.rs) | ✔️ | ✔️ | `json` | 
| [OugiFinishParam](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/message_info.rs) | ✔️ | ✔️ | `json` | 
| [playerDoubleEffectParam](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/player_double_effect_param.rs) | ✔️ | ✔️ | `json` |
| [playerSettingParam](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/player_setting_param.rs) | ✔️ | ✔️ | `json` | 
| [player_icon](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/player_icon.rs) | ✔️ | ✔️ | `json` | 
| [prm_load](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/player_icon.rs) | ✔️ | ✔️ | `json` |
| [prohibitedSubstringParam](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/prohibited_substring_param) | ✔️ | ✔️ | `json` |
| [skillIndexSettingParam](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/skill_index_setting_param.rs) | ✔️ | ✔️ | `json` |
| [snd](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/snd.rs) | ✔️ | ✔️ | `json` |
| [staffRollTextParam](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/staff_roll_text_param.rs) | ✔️ | ❌ | `json` | 
| [supportActionParam](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/support_action_param.rs) | ✔️ | ✔️ | `json` |
| [supportSkillRecoverySpeedParam](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/support_skill_recovery_speed_param.rs) | ✔️ | ✔️ | `json` |
| [xml](https://github.com/maxcabd/nuccbin/blob/main/src/nucc_binary/xml.rs) | ✔️ | ✔️ | `xml` | 


## Credits
This project is based on the [initial work](https://github.com/SutandoTsukai181/xfbin-nucc-binary) by SutandoTsukai181 on the original nuccChunkBinary parser for the All Star Battle R series.

Special thanks goes to several members including:
* [EliteAce170](https://www.youtube.com/@EliteAce) for reversing some formats.
* [HydraBladeZ](https://github.com/Al-Hydra) for reversing some formats.
* [TheLeonX](https://github.com/TheLeonX) for work on the Character Manager which was used as inspiration and some formats.
* [PortableProductions](https://www.youtube.com/@PortableProductions) for reversing some formats.
* [Kuroha Saenoki](https://www.youtube.com/@KurohaSaenoki) for reversing some formats.
* [Valant96](https://www.youtube.com/@valant96) for information on some formats.
* [Xact](https://www.youtube.com/channel/UCluz3KlVGPDYNnJhNvOW_AA) for reversing some formats.
* and [SutandoTsukai181](https://github.com/SutandoTsukai181) for his initial work on the tool.
