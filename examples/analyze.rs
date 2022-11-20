use khaiii_rs::{KhaiiiApi, KhaiiiWord};

fn main() {
    let api = KhaiiiApi::default();

    let results: Vec<KhaiiiWord> = api.analyze("안녕하세요, 세상!".to_string()).unwrap();

    dbg!(results);
}
