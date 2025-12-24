# Oplus EDL Toolkit

An Oplus EDL toolkit developed with Rust.

## 📱 Features:
- Supports free partition reading and writing.
- Supports GPT parsing and XML file generation.
- Multiple language support.

## ⏫️ Todo:
- Add support for more languages
- Linux platform support
- Support official EDL package

## ⚠️ Notes:
- Requirements: You need to provide your own Loader (Firehose) + digest + sign files.
- Flashing involves risks. Please ensure your important data is backed up.

## 📋 Build
* [Setup Tauri](https://v2.tauri.app/start/prerequisites/)
```bash
git clone https://github.com/snowwolf725/OPlus_EDL_Toolkit.git
cd OPlus_EDL_Toolkit
npm install
npm run tauri build
```

## 🔔 Translation
<!-- CROWDIN-TRANSLATIONS-PROGRESS-ACTION-START -->


#### Available

<table><tr><td align="center" valign="top"><img width="30px" height="30px" title="Chinese Simplified" alt="Chinese Simplified" src="https://raw.githubusercontent.com/benjaminjonard/crowdin-translations-progress-action/1.0/flags/zh-CN.png"></div><div align="center" valign="top">99%</td><td align="center" valign="top"><img width="30px" height="30px" title="Chinese Traditional" alt="Chinese Traditional" src="https://raw.githubusercontent.com/benjaminjonard/crowdin-translations-progress-action/1.0/flags/zh-TW.png"></div><div align="center" valign="top">99%</td><td align="center" valign="top"><img width="30px" height="30px" title="Russian" alt="Russian" src="https://raw.githubusercontent.com/benjaminjonard/crowdin-translations-progress-action/1.0/flags/ru.png"></div><div align="center" valign="top">96%</td></tr></table>
<!-- CROWDIN-TRANSLATIONS-PROGRESS-ACTION-END -->

## 🎉 Credit:
* Special thanks to 某贼@CoolAPK for the repost.
* [linux-msm/qdl](https://github.com/linux-msm/qdl) for the open C implementation of fh_loader
* [qualcomm/qdlrs](https://github.com/qualcomm/qdlrs/) for the open Rust implementation of fh_loader
