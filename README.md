# nuccbin
A tool to serialize/deserialize several nuccChunkBinary .xfbin's from Ultimate Ninja STORM CONNECTIONS.

## Usage
- Get the latest version from [releases]().
- Drag and drop any of the .xfbin files in the list onto the nuccbin.exe.
- Add the changes to your .json file by adding, removing, or editing entries.
- Apply your changes to the .xfbin file by dragging and dropping your .meta.json onto the nuccbin.exe.

## Formats
nuccbin supports a number of in game nuccChunkBinary param / bin formats. All formats support serializing. While some may not support deserializing.
| File | Serialize | Deserialize | Extension |
| --- | --- | --- | --- |
| [accessoryParam](https://github.com/ScanMountGoat/xc3_lib/blob/main/xc3_lib/src/apmd.rs) | ✔️ | ✔️ | `json` |
| [animeSongBgmParam](https://github.com/ScanMountGoat/xc3_lib/blob/main/xc3_lib/src/bc.rs) | ✔️ | ✔️ | `json` |
| [characode](https://github.com/ScanMountGoat/xc3_lib/blob/main/xc3_lib/src/dhal.rs) | ✔️ | ✔️  | `json` |
| [CharaPoseParam](https://github.com/ScanMountGoat/xc3_lib/blob/main/xc3_lib/src/eva.rs) | ✔️ | ✔️ | `json` |
| [characterSelectParam](https://github.com/ScanMountGoat/xc3_lib/blob/main/xc3_lib/src/ltpc.rs) | ✔️ | ✔️ | `json` |
| [costumeParam](https://github.com/ScanMountGoat/xc3_lib/blob/main/xc3_lib/src/mibl.rs) | ✔️ | ✔️ | `json` |
| [dds](https://github.com/ScanMountGoat/xc3_lib/blob/main/xc3_lib/src/msmd.rs) | ✔️ | ✔️ |  `dds` |
| [DlcInfoParam](https://github.com/ScanMountGoat/xc3_lib/blob/main/xc3_lib/src/msrd.rs) | ✔️ | ✔️ | `json` |
| [effectprm](https://github.com/ScanMountGoat/xc3_lib/blob/main/xc3_lib/src/mxmd.rs) | ✔️ | ✔️ | `json` | 
| [ev](https://github.com/ScanMountGoat/xc3_lib/blob/main/xc3_lib/src/sar1.rs) | ✔️ | ✔️ | `json` | 
| [lua](https://github.com/ScanMountGoat/xc3_lib/blob/main/xc3_lib/src/spch.rs) | ✔️ | ✔️ | `lua` | 
| [messageInfo](https://github.com/ScanMountGoat/xc3_lib/blob/main/xc3_lib/src/xbc1.rs) | ✔️ | ✔️ | `json` | 
| [playerDoubleEffectParam](https://github.com/ScanMountGoat/xc3_lib/blob/main/xc3_lib/src/xbc1.rs) | ✔️ | ✔️ | `json` |
| [playerSettingParam](https://github.com/ScanMountGoat/xc3_lib/blob/main/xc3_lib/src/xbc1.rs) | ✔️ | ✔️ | `json` | 
| [player_icon](https://github.com/ScanMountGoat/xc3_lib/blob/main/xc3_lib/src/xbc1.rs) | ✔️ | ✔️ | `json` | 
| [prm_load](https://github.com/ScanMountGoat/xc3_lib/blob/main/xc3_lib/src/xbc1.rs) | ✔️ | ✔️ | `json` || 
| [staffRollTextParam](https://github.com/ScanMountGoat/xc3_lib/blob/main/xc3_lib/src/xbc1.rs) | ✔️ | ✔️ | `json` | 
| [supportActionParam](https://github.com/ScanMountGoat/xc3_lib/blob/main/xc3_lib/src/xbc1.rs) | ✔️ | ✔️ | `json` | 
| [xml](https://github.com/ScanMountGoat/xc3_lib/blob/main/xc3_lib/src/xbc1.rs) | ✔️ | ✔️ | `xml` | 
\*

## Credits
This project is based on the [initial work](https://github.com/SutandoTsukai181/xfbin-nucc-binary) by SutandoTsukai181 on the original nuccChunkBinary parser for the All Star Battle R series.

Special thanks goes to several members including:
* [EliteAce170]() for reversing several formats.
* [HydraBladeZ](https://github.com/Al-Hydra) for reversing several formats.
* [TheLeonX](https://github.com/TheLeonX) for work on the Character Manager which was used as inspiration and several formats.
* [PortableProductions](https://www.youtube.com/@PortableProductions) for help on several formats and information.
* [Kuroha Saenoki](https://www.youtube.com/@KurohaSaenoki) for information on some formats.
* and [SutandoTsukai181](https://github.com/SutandoTsukai181) for his initial work on the tool.
