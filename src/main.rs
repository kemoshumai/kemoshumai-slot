use rand::seq::SliceRandom;
const UPPER: &[&str] = &["けも", "にく", "すし", "かも", "さけ", "にせ", "ねこ", "いぬ", "えび", "かに", "たこ", "いか", "ひも", "へび", "とら", "めか"];
const LOWER: &[&str] = &["シューマイ", "ラーメン", "ギョウザ", "チャーハン", "ヤクザ", "マラカス", "チワワ", "ニワトリ", "ニャンコ", "ドラゴン"];
fn main() {
    let mut rng = rand::thread_rng();
    println!("{}{}", UPPER.choose(&mut rng).unwrap(), LOWER.choose(&mut rng).unwrap());
}
