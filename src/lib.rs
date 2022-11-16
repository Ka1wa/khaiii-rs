use khaiii_sys as raw;
use std::ffi::{CStr, CString};

pub struct KhaiiiApi {
    handle: i32,
}

pub struct KhaiiiWord {
    lex: String,
    begin: i32,
    length: i32,
    morphs: Vec<KhaiiiMorph>,
}

pub struct KhaiiiMorph {
    lex: String,
    tag: String,
    begin: i32,
    length: i32,
}

impl KhaiiiApi {
    pub fn new() -> Result<KhaiiiApi, String> {
        let ret = KhaiiiApi { handle: -1 };

        ret.set_log_level("all".to_string(), "warn".to_string())
            .expect("Failed to set log level");

        match ret.open("".to_string(), "".to_string()) {
            Ok(ret) => Ok(ret),
            Err(e) => Err(e),
        }
    }

    fn set_log_level(&self, name: String, level: String) -> Result<(), String> {
        let name = CString::new(name).unwrap();
        let level = CString::new(level).unwrap();

        let ret = unsafe { raw::khaiii_set_log_level(name.as_ptr(), level.as_ptr()) };
        if ret < 0 {
            return Err(self.last_error());
        }

        Ok(())
    }

    fn last_error(&self) -> String {
        let error = unsafe { CStr::from_ptr(raw::khaiii_last_error(self.handle)) };

        String::from(error.to_str().unwrap_or("Failed to get last error"))
    }

    fn open(mut self, rsc_dir: String, opt_str: String) -> Result<Self, String> {
        self = self.close();

        let opt_str_cstring = CString::new(opt_str).unwrap();
        let rsc_dir_cstring = if rsc_dir.is_empty() {
            CString::new("/usr/local/share/khaiii").unwrap()
        } else {
            let path = std::env::current_dir().unwrap();
            CString::new(path.join("share/khaiii").to_str().unwrap()).unwrap()
        };

        let handle =
            unsafe { raw::khaiii_open(rsc_dir_cstring.as_ptr(), opt_str_cstring.as_ptr()) };

        if handle < 0 {
            return Err(self.last_error());
        }

        self.handle = handle;

        Ok(self)
    }

    fn close(mut self) -> Self {
        if self.handle >= 0 {
            unsafe {
                raw::khaiii_close(self.handle);
            };
        }

        self.handle = -1;

        self
    }

    pub fn version(&self) -> String {
        let version = unsafe { CStr::from_ptr(raw::khaiii_version()) };

        String::from(version.to_str().unwrap_or("Failed to get version"))
    }

    pub fn analyze(self, input: String) -> Result<Vec<KhaiiiWord>, String> {
        let input_cstring = CString::new(input.clone()).unwrap();
        let opt_str_cstring = CString::new("").unwrap();

        if self.handle < 0 {
            return Err("Failed to analyze input".to_string());
        }

        let results = unsafe {
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
        //self.free_results(results);

        Ok(words)
    }

    fn get_align(input: String) {}

    fn make_words(&self, input: String, results: *const raw::khaiii_word_t) -> Vec<KhaiiiWord> {
        let words: Vec<KhaiiiWord> = Vec::new();

        words
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

    pub fn set(mut self, word: raw::khaiii_word_t, in_str: String, align: String) {}

    fn make_morphs(&self) -> Vec<KhaiiiMorph> {
        Vec::new()
    }
}

impl Default for KhaiiiWord {
    fn default() -> Self {
        KhaiiiWord::new()
    }
}

#[cfg(test)]
mod tests {
    use super::KhaiiiApi;

    #[test]
    fn version() {
        let api = KhaiiiApi::new().unwrap();

        assert_eq!(api.version(), "0.4");
    }
}
