use clap::{App, Arg};
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use core::mem;

#[repr(C,packed)]
struct SfoHeader {
    magic: u32,
    version: u32,
    key_offset: u32,
    val_offset: u32,
    count: u32,
}

#[repr(C,packed)]
#[derive(Default, Copy, Clone)]
struct SfoEntry {
    name_offset: u16,
    alignment: u8,
    type_: u8,
    val_size: u32,
    total_size: u32,
    data_offset: u32,
}

#[derive(Copy, Clone)]
struct EntryContainer<'a> {
    name: &'a str,
    type_: EntryType,
    value: u32,
    data: Option<&'a[u8]>,
}

#[repr(i32)]
#[derive(Copy, Clone)]
enum EntryType {
    Binary = 0,
    String_ = 2,
    Value = 4,
}

static DEFAULTS: [EntryContainer; 7] = [
    EntryContainer { name: "BOOTABLE", type_: EntryType::Value, value: 1, data: None },
    EntryContainer { name: "CATEGORY", type_: EntryType::String_, value: 0, data: Some(b"MG") },
    EntryContainer { name: "DISC_ID", type_: EntryType::String_, value: 0, data: Some(b"UCJS10041") },
    EntryContainer { name: "DISC_VERSION", type_: EntryType::String_, value: 0, data: Some(b"1.00") },
    EntryContainer { name: "PARENTAL_LEVEL", type_: EntryType::Value, value: 1, data: None },
    EntryContainer { name: "PSP_SYSTEM_VER", type_: EntryType::Value, value: 0, data: Some(b"1.00") },
    EntryContainer { name: "REGION", type_: EntryType::Value, value: 0x8000, data: None },
];

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
            .help("NAME=VALUE Add a new DWORD value")
            .multiple(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("string")
            .short("s")
            .long("string")
            .help("NAME=STRING Add a new string value")
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
    if matches.values_of("string").is_some() {
        for s in matches.values_of("string").unwrap() {
            let key_value_pair: Vec<String> = 
                s.split("=").map(|s: &str| s.to_string()).collect();
            strings.insert(key_value_pair[0].clone(), key_value_pair[1].clone());
        }
    }

    let mut dwords: HashMap<String, u32> = HashMap::new(); 

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

    let title = matches.value_of("TITLE");
    let outpath = Path::new(matches.value_of("output").unwrap());

    let mut entries: Vec<EntryContainer> = Vec::new();

    let mut header = SfoHeader {
        magic: PSF_MAGIC,
        version: PSF_VERSION,
        key_offset: 0,
        val_offset: 0,
        count: 0,
    };

    for entry in DEFAULTS.iter() {
        if !(strings.contains_key(entry.name) || dwords.contains_key(entry.name)) {
            entries.push(*entry);
        }
    }

    entries.push(
        EntryContainer {
            name: "TITLE", type_: EntryType::String_, value: 0, data: Some(title.unwrap().as_bytes())
        }
    );

    for (key, value) in dwords.iter() {
        if entries.len() < MAX_OPTIONS {
            entries.push(
                EntryContainer {
                    name: key,
                    type_: EntryType::Value,
                    value: *value,
                    data: None,
                }
            )
        } else {
            panic!("Maximum options reached");
        }
    }


    for (key, value) in strings.iter() {
        if entries.len() < MAX_OPTIONS {
            entries.push(
                EntryContainer {
                    name: key,
                    type_: EntryType::String_,
                    value: 0,
                    data: Some(value.as_bytes()),

                }
            )
        } else {
            panic!("Maximum options reached");
        }
    }

    let mut head = [0u8; 8192]; 
    let mut keys = [0u8; 8192];
    let mut data = [0u8; 8192];

    let mut name_offset = 0;
    let mut data_offset = 0;

    let mut sfo_entries: Vec<SfoEntry> = Vec::new();

    for entry in entries {
        header.count += 1;
        let mut sfo_entry = SfoEntry {
            name_offset,
            data_offset,
            alignment: 4,
            type_: entry.type_ as u8,
            ..Default::default() 
        };
        let idx = name_offset as usize;
        &keys[idx..idx+entry.name.len()].copy_from_slice(entry.name.as_bytes());
        name_offset += entry.name.len() as u16 + 1;
        match entry.type_ {
            EntryType::Value => {
                sfo_entry.val_size = 4;
                sfo_entry.total_size = 4;
                let idx = data_offset as usize;
                data[idx..idx+4].copy_from_slice(&entry.value.to_le_bytes());
                data_offset += 4;
            },
            EntryType::String_ | EntryType::Binary => {
               let val_size = entry.data.unwrap().len()+1;
               let total_size = (val_size + 3) & !3;
               sfo_entry.val_size = val_size as u32;
               sfo_entry.total_size = total_size as u32;
               let idx = data_offset as usize;
               let unwrapped_data = entry.data.unwrap();
               data[idx..idx + unwrapped_data.len()].copy_from_slice(
                   unwrapped_data
                );
               data_offset += total_size as u32;
            },
        }
        sfo_entries.push(sfo_entry);
    }
    let mut file = File::create(outpath).unwrap();
    let temp = unsafe { mem::transmute::<_, [u8;20]>(header) };
    head[0..20].copy_from_slice(&temp);
    file.write_all(&head[0..20]);
    for entry in sfo_entries {
        file.write_all(unsafe { &mem::transmute::<_, [u8;16]>(entry) });
    }
    file.write_all(&keys[0..name_offset as usize]);
    file.write_all(&data[0..data_offset as usize]);
}
