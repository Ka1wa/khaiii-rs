use khaiii_rs::{KhaiiiApi, KhaiiiWord};

fn main() {
    let api = KhaiiiApi::new("/tmp/share/khaiii".to_string(), "".to_string(), "warn".to_string()).expect("Couldn't initialize khaiii API");

    let results: Vec<KhaiiiWord> = api.analyze("안녕하세요, 세상!".to_string()).unwrap();

    dbg!(results);
}
