use khaiii_sys as raw;

use khaiii_sys::khaiii_word_t_;
use std::ffi::{CStr, CString};

pub mod errors;

#[derive(Debug)]
pub struct KhaiiiApi {
    handle: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct KhaiiiWord {
    lex: String,
    begin: i32,
    length: i32,
    morphs: Vec<KhaiiiMorph>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct KhaiiiMorph {
    lex: String,
    tag: String,
    begin: i32,
    length: i32,
}

impl KhaiiiApi {
    pub fn new(rsc_dir: String, opt_str: String, log_level: String) -> Result<KhaiiiApi, errors::Error> {
        let ret = KhaiiiApi { handle: -1 };

        ret.set_log_level("all".to_string(), log_level)?;

        match ret.open(rsc_dir, opt_str) {
            Ok(ret) => Ok(ret),
            Err(e) => Err(e),
        }
    }

    fn set_log_level(&self, name: String, level: String) -> Result<(), errors::Error> {
        let name = CString::new(name).unwrap();
        let level = CString::new(level).unwrap();

        let ret = unsafe { raw::khaiii_set_log_level(name.as_ptr(), level.as_ptr()) };
        if ret < 0 {
            return Err(self.last_error());
        }

        Ok(())
    }

    fn last_error(&self) -> errors::Error {
        let error = unsafe { CStr::from_ptr(raw::khaiii_last_error(self.handle)) };

        match error.to_str() {
            Ok(err) => {
                errors::Error::KhaiiiExcept(String::from(err))
            },
            Err(_) => {
                errors::Error::Unknown
            }
        }
    }

    fn open(mut self, rsc_dir: String, opt_str: String) -> Result<Self, errors::Error> {
        self.close();

        let opt_str_cstring = CString::new(opt_str).unwrap();
        let rsc_dir_cstring = if rsc_dir.is_empty() {
            let path = std::env::current_dir().unwrap();
            let local = path.join("share/khaiii");

            if local.exists() {
                CString::new(path.join("share/khaiii").to_str().unwrap()).unwrap()
            } else {
                CString::new("/usr/local/share/khaiii").unwrap()
            }
        } else {
            CString::new(rsc_dir).unwrap()
        };

        let handle =
            unsafe { raw::khaiii_open(rsc_dir_cstring.as_ptr(), opt_str_cstring.as_ptr()) };

        if handle < 0 {
            return Err(self.last_error());
        }

        self.handle = handle;

        Ok(self)
    }

    fn close(&mut self) {
        if self.handle >= 0 {
            unsafe {
                raw::khaiii_close(self.handle);
            };
        }

        self.handle = -1;
    }

    pub fn version(&self) -> String {
        let version_cstring = unsafe { CStr::from_ptr(raw::khaiii_version()) };

        String::from(version_cstring.to_str().unwrap_or("Failed to get version"))
    }

    pub fn analyze(&self, input: String) -> Result<Vec<KhaiiiWord>, errors::Error> {
        let input_cstring = CString::new(input.clone()).unwrap();
        let opt_str_cstring = CString::new("").unwrap();

        if self.handle < 0 {
            return Err(errors::Error::ResourcesFailure);
        }

        let results: *const raw::khaiii_word_t_ = unsafe {
            raw::khaiii_analyze(
                self.handle,
                input_cstring.as_ptr(),
                opt_str_cstring.as_ptr(),
            )
        };

        if results.is_null() {
            return Err(self.last_error());
        }

        let words = self.make_words(input, results);
        self.free_results(results);

        Ok(words)
    }

    #[allow(clippy::needless_range_loop)]
    fn get_align(&self, input: &str) -> Vec<i32> {
        if input.is_empty() {
            return Vec::new();
        }

        let mut char_indices = input.char_indices();

        let total_length = input.len();
        let mut vec: Vec<i32> = vec![0; total_length];
        let mut idx = 0;
        let (mut previous_pos, _) = char_indices.next().unwrap();

        for (pos, _) in char_indices {
            for n in previous_pos..pos {
                vec[n] = idx;
            }

            previous_pos = pos;
            idx += 1;
        }

        for n in previous_pos..total_length {
            vec[n] = idx;
        }

        vec
    }

    fn free_results(&self, results: *const raw::khaiii_word_t_) {
        if self.handle >= 0 {
            unsafe { raw::khaiii_free_results(self.handle, results); }
        }
    }

    fn make_words(&self, input: String, results: *const raw::khaiii_word_t_) -> Vec<KhaiiiWord> {
        let align = self.get_align(&input);
        let mut words: Vec<KhaiiiWord> = Vec::new();
        let bytes = input.as_bytes();

        let mut ptr: *const khaiii_word_t_ = results;
        while !ptr.is_null() {
            let result: raw::khaiii_word_t_ = unsafe { *ptr };

            let word = KhaiiiWord::from(result, bytes, &align);

            words.push(word);

            ptr = result.next;
        }

        words
    }
}

impl Default for KhaiiiApi {
    fn default() -> Self {
        KhaiiiApi::new("".to_string(), "".to_string(), "warn".to_string()).unwrap()
    }
}

impl Drop for KhaiiiApi {
    fn drop(&mut self) {
        self.close();
    }
}

impl KhaiiiWord {
    pub fn new() -> KhaiiiWord {
        KhaiiiWord {
            lex: "".to_string(),
            begin: -1,
            length: -1,
            morphs: Vec::new(),
        }
    }

    pub fn from(value: raw::khaiii_word_t_, input: &[u8], align: &[i32]) -> KhaiiiWord {
        let b = value.begin as usize;
        let e = (value.begin + value.length) as usize;
        let begin = align[b];
        let end = align[e - 1] + 1;

        let bytes = input[b..e].to_vec();
        let lex = String::from_utf8(bytes).unwrap();

        KhaiiiWord {
            lex,
            begin,
            length: end - begin,
            morphs: KhaiiiWord::make_morphs(value.morphs, align)
        }
    }

    fn make_morphs(value: *const raw::khaiii_morph_t_, align: &[i32]) -> Vec<KhaiiiMorph> {
        let mut morphs: Vec<KhaiiiMorph> = Vec::new();

        let mut ptr: *const raw::khaiii_morph_t_ = value;
        while !ptr.is_null() {
            let result: raw::khaiii_morph_t_ = unsafe { *ptr };

            let morph = KhaiiiMorph::from(result, align);

            morphs.push(morph);

            ptr = result.next;
        }

        morphs
    }
}

impl Default for KhaiiiWord {
    fn default() -> Self {
        KhaiiiWord::new()
    }
}

impl KhaiiiMorph {
    pub fn new() -> KhaiiiMorph {
        KhaiiiMorph {
            lex: "".to_string(),
            tag: "".to_string(),
            begin: -1,
            length: -1,
        }
    }

    pub fn from(value: raw::khaiii_morph_t_, align: &[i32]) -> KhaiiiMorph {
        let lex_cstring = unsafe { CStr::from_ptr(value.lex) };
        let tag_cstring = unsafe { CStr::from_ptr(value.tag) };

        let lex = String::from(lex_cstring.to_str().unwrap_or(""));
        let tag = String::from(tag_cstring.to_str().unwrap_or(""));

        let b = value.begin as usize;
        let e = (value.begin + value.length) as usize;
        let begin = align[b];
        let end = align[e - 1] + 1;

        KhaiiiMorph {
            lex,
            tag,
            begin,
            length: end - begin,
        }
    }
}

impl Default for KhaiiiMorph {
    fn default() -> Self {
        KhaiiiMorph::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{KhaiiiMorph, KhaiiiWord};
    use super::KhaiiiApi;

    #[test]
    fn version() {
        let api = KhaiiiApi::default();

        assert_eq!(api.version(), "0.4");
    }

    #[test]
    fn open() {
        let api = KhaiiiApi::new("/not/existing/dir".to_string(), "".to_string(), "warn".to_string());

        assert_eq!(api.unwrap_err(), crate::errors::Error::KhaiiiExcept("resource directory not found: /not/existing/dir".to_string()))
    }

    #[test]
    fn analyze() {
        let api = KhaiiiApi::default();

        let results = api.analyze("안녕하세요, 세상!".to_string()).unwrap();
        let expected_results: Vec<KhaiiiWord> = vec![KhaiiiWord{
            begin: 0,
            length: 6,
            lex: "안녕하세요,".to_string(),
            morphs: vec![KhaiiiMorph{
                begin: 0,
                length: 2,
                lex: "안녕".to_string(),
                tag: "NNG".to_string()
            }, KhaiiiMorph{
                begin: 2,
                length: 1,
                lex: "하".to_string(),
                tag: "XSA".to_string()
            }, KhaiiiMorph{
                begin: 3,
                length: 1,
                lex: "시".to_string(),
                tag: "EP".to_string()
            }, KhaiiiMorph{
                begin: 3,
                length: 2,
                lex: "어요".to_string(),
                tag: "EC".to_string()
            }, KhaiiiMorph{
                begin: 5,
                length: 1,
                lex: ",".to_string(),
                tag: "SP".to_string()
            }]
        }, KhaiiiWord{
            begin: 7,
            length: 3,
            lex: "세상!".to_string(),
            morphs: vec![KhaiiiMorph{
                begin: 7,
                length: 2,
                lex: "세상".to_string(),
                tag: "NNG".to_string()
            }, KhaiiiMorph{
                begin: 9,
                length: 1,
                lex: "!".to_string(),
                tag: "SF".to_string()
            }]
        }];

        assert_eq!(results, expected_results);
    }
}
