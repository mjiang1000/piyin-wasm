mod utils;
extern crate regex;


use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use regex::Regex;

mod dictdata;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, piyin-wasm!");
}


struct Dict {
    dict: HashMap<u32, String>
}


impl Dict {
    pub fn new() -> Dict {
        let dict :HashMap<u32, String> = dictdata::get_dict_data();
        Dict { dict }
    }
     
    pub fn get_pinyin(&self, han: &str) -> Vec<&str>{
        let mut pinyins: Vec<&str> = Vec::new();
        for c in han.chars() {
            let code = c as u32;
            match self.dict.get(&code) {
                Some(pinyin) => pinyins.push(pinyin),
                _ => pinyins.push(""),
            }
        }

        return pinyins;
    }

    pub fn get_pinyin_without_tone(&self, han: &str) -> Vec<String>{
        let pys_origin = self.get_pinyin(han);
        let mut pys: Vec<String> = Vec::new();
        let mut phonetic_symbol: HashMap<&str, &str> = HashMap::new();
        phonetic_symbol.insert("ā", "a1");
        phonetic_symbol.insert("á", "a2");
        phonetic_symbol.insert("ǎ", "a3");
        phonetic_symbol.insert("à", "a4");
        phonetic_symbol.insert("ē", "e1");
        phonetic_symbol.insert("é", "e2");
        phonetic_symbol.insert("ě", "e3");
        phonetic_symbol.insert("è", "e4");
        phonetic_symbol.insert("ō", "o1");
        phonetic_symbol.insert("ó", "o2");
        phonetic_symbol.insert("ǒ", "o3");
        phonetic_symbol.insert("ò", "o4");
        phonetic_symbol.insert("ī", "i1");
        phonetic_symbol.insert("í", "i2");
        phonetic_symbol.insert("ǐ", "i3");
        phonetic_symbol.insert("ì", "i4");
        phonetic_symbol.insert("ū", "u1");
        phonetic_symbol.insert("ú", "u2");
        phonetic_symbol.insert("ǔ", "u3");
        phonetic_symbol.insert("ù", "u4");
        phonetic_symbol.insert("ü", "v0");
        phonetic_symbol.insert("ǘ", "v2");
        phonetic_symbol.insert("ǚ", "v3");
        phonetic_symbol.insert("ǜ", "v4");
        phonetic_symbol.insert("ń", "n2");
        phonetic_symbol.insert("ň", "n3");
        phonetic_symbol.insert("", "m2");
        let reg = Regex::new(r"(?P<vo>[aeoiuvnm])(?P<to>[0-4])$").unwrap();

        for py in pys_origin {
            let mut py_str: String = "".to_string();
            for c in py.chars() {
                let c_str: &str = &c.to_string();
                match phonetic_symbol.get(c_str) {
                    Some(s) => py_str = py_str.to_string() + &reg.replace_all(s, "$vo"),
                    None => py_str = py_str + &c.to_string(),
                }
            }
            
            pys.push(py_str);
        }

        return pys;
    }
}



#[wasm_bindgen]
pub struct Pinyin {
    dict: Dict,    
}

#[wasm_bindgen]
impl Pinyin {
    pub fn new() -> Pinyin {
        Pinyin {
            dict: Dict::new(),
        }
    }
    pub fn covert(&self, han: &str, style: u32) -> String {
        let dv :&str = "|";
        if style == 1 {
            let result = self.dict.get_pinyin_without_tone(han);
            let mut py_str: String = String::new();
            let len = result.len();
            let mut i = 0;
            println!("{}", len);
            for s in result.iter() {
                py_str.push_str(s);
                i += 1;
                if i < len {
                    py_str.push_str(dv);
                }
            }

            return py_str;
        } else {
            let result = self.dict.get_pinyin(han);
            let mut py_str: String = String::new();
            let len = result.len();
            let mut i = 0;
            for s in result.iter() {
                py_str.push_str(&s);
                i += 1;
                if i < len {
                    py_str.push_str(dv);
                }
            }
            return py_str;
        }
    }
}
