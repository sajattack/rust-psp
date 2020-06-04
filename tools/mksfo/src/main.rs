use clap::{App, Arg};
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

#[repr(C,packed)]
struct SfoHeader {
    magic: u32,
    version: u32,
    key_offset: u32,
    val_offset: u32,
    count: u32,
}

impl SfoHeader {
    fn to_le_bytes(self) -> [u8;20] {
        let mut buf = [0u8;20];
        buf[0..=3].copy_from_slice(&self.magic.to_le_bytes());
        buf[4..=7].copy_from_slice(&self.version.to_le_bytes());
        buf[8..=11].copy_from_slice(&self.key_offset.to_le_bytes());
        buf[12..=15].copy_from_slice(&self.val_offset.to_le_bytes());
        buf[16..=19].copy_from_slice(&self.count.to_le_bytes());

        buf
    }
}

#[repr(C,packed)]
#[derive(Default, Copy, Clone)]
struct SfoEntry {
    key_offset: u16,
    alignment: u8,
    type_: u8,
    val_size: u32,
    total_size: u32,
    data_offset: u32,
}

impl SfoEntry {
    fn to_le_bytes(self) -> [u8;16] {
        let mut buf = [0u8;16]; 
        buf[0..=1].copy_from_slice(&self.key_offset.to_le_bytes());
        buf[2..=2].copy_from_slice(&self.alignment.to_le_bytes());
        buf[3..=3].copy_from_slice(&self.type_.to_le_bytes());
        buf[4..=7].copy_from_slice(&self.val_size.to_le_bytes());
        buf[8..=11].copy_from_slice(&self.total_size.to_le_bytes());
        buf[12..=15].copy_from_slice(&self.data_offset.to_le_bytes());

        buf
    }
}

#[repr(u8)]
enum EntryType {
    Binary = 0,
    Text = 2,
    Integer = 4,
}

const MAX_OPTIONS: usize = 256;
const PSF_MAGIC: u32 = 0x46535000;
const PSF_VERSION: u32 = 0x00000101;

fn main() {
    let matches = App::new("mksfo")
        .version("0.1")
        .author("Paul Sajna <sajattack@gmail.com>")
        .about("Creates SFO files used for building Sony PSP EBOOT executables")
        .arg(Arg::with_name("dword")
            .short("d")
            .long("dword")
            .help("key=VALUE Add a new DWORD value")
            .multiple(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("string")
            .short("s")
            .long("string")
            .help("key=STRING Add a new string value")
            .multiple(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("TITLE")
            .takes_value(true)
            .required(true)
            .help("Display title")
        )
        .arg(Arg::with_name("output")
            .takes_value(true)
            .required(true)
            .help("Output file name")
        )
    .get_matches();

    let mut strings: HashMap<String, String> = HashMap::new();
    let mut dwords: HashMap<String, u32> = HashMap::new(); 

    let title = matches.value_of("TITLE").unwrap();
    strings.insert("TITLE".to_string(), title.to_string());

    // Default Values
    strings.insert("CATEGORY".to_string(), "MG".to_string());
    strings.insert("DISC_ID".to_string(), "UCJS10041".to_string());
    strings.insert("DISC_VERSION".to_string(), "1.00".to_string());
    strings.insert("PSP_SYSTEM_VER".to_string(), "1.00".to_string());

    dwords.insert("BOOTABLE".to_string(), 1);
    dwords.insert("PARENTAL_LEVEL".to_string(), 1);
    dwords.insert("REGION".to_string(), 0x8000);

    if matches.values_of("string").is_some() {
        for s in matches.values_of("string").unwrap() {
            let key_value_pair: Vec<String> = 
                s.split("=").map(|s: &str| s.to_string()).collect();
            strings.insert(key_value_pair[0].clone(), key_value_pair[1].clone());
        }
    }

    if matches.values_of("dword").is_some() {
        for s in matches.values_of("dword").unwrap() {
            let key_value_pair: Vec<String> = 
                s.split("=").map(|s: &str| s.to_string()).collect();
            dwords.insert(
                key_value_pair[0].clone(),
                str::parse::<u32>(&key_value_pair[1]).unwrap()
            );
        }
    }

    let outpath = Path::new(matches.value_of("output").unwrap());

    let mut header = SfoHeader {
        magic: PSF_MAGIC,
        version: PSF_VERSION,
        key_offset: 0,
        val_offset: 0,
        count: 0,
    };

    let num_options = dwords.len() + strings.len();
    if num_options > MAX_OPTIONS {
        panic!("Maximum number of options is {}, you have {}", MAX_OPTIONS, num_options);
    }

    let mut keys = [0u8; 8192];
    let mut data = [0u8; 8192];

    let mut key_offset = 0;
    let mut data_offset = 0;

    let mut sfo_entries: Vec<SfoEntry> = Vec::new();

    for (key, value) in dwords {
        header.count += 1;
        let mut sfo_entry = SfoEntry {
            key_offset,
            data_offset,
            alignment: 4,
            type_: EntryType::Integer as u8,
            ..Default::default()
        };
        let idx = key_offset as usize;
        &keys[idx..idx+key.len()].copy_from_slice(key.as_bytes());
        key_offset += key.len() as u16 + 1;
        sfo_entry.val_size = 4;
        sfo_entry.total_size = 4;
        let idx = data_offset as usize;
        data[idx..idx+4].copy_from_slice(&value.to_le_bytes());
        data_offset += 4;
        sfo_entries.push(sfo_entry);
    }

    for (key, value) in strings {
        header.count += 1;
        let mut sfo_entry = SfoEntry {
            key_offset,
            data_offset,
            alignment: 4,
            type_: EntryType::Text as u8,
            ..Default::default()
        };
        let idx = key_offset as usize;
        &keys[idx..idx+key.len()].copy_from_slice(key.as_bytes());
        key_offset += key.len() as u16 + 1;

        let val_size = value.len()+1;
        let total_size = (val_size + 3) & !3;
        sfo_entry.val_size = val_size as u32;
        sfo_entry.total_size = total_size as u32;
        let idx = data_offset as usize;
        data[idx..idx + value.len()].copy_from_slice(
            value.as_bytes()
        );
        data_offset += total_size as u32;
        sfo_entries.push(sfo_entry);
    }

    let mut file = File::create(outpath).unwrap();
    file.write_all(&header.to_le_bytes()).unwrap();
    for sfo_entry in sfo_entries {
        file.write_all(&sfo_entry.to_le_bytes()).unwrap();
    }
    file.write_all(&keys[0..key_offset as usize]).unwrap();
    file.write_all(&data[0..data_offset as usize]).unwrap();
}
