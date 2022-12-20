use std::convert::From;

pub fn type_list() -> Vec<String> {
    vec!["ノーマル", "かくとう", "ひこう", "どく", "じめん", "いわ", "むし", "ゴースト", "はがね", "ほのお", "みず", "くさ", "でんき", "エスパー", "こおり", "ドラゴン", "あく", "フェアリー"]
        .into_iter().map(|e| e.to_string()).collect::<Vec<_>>()
}

/// ポケモンのタイプ
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type {
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        use Type::*;
        match value {
            "ノーマル" => Normal,
            "かくとう" => Fighting,
            "ひこう" => Flying,
            "どく" => Poison,
            "じめん" => Ground,
            "いわ" => Rock,
            "むし" => Bug,
            "ゴースト" => Ghost,
            "はがね" => Steel,
            "ほのお" => Fire,
            "みず" => Water,
            "くさ" => Grass,
            "でんき" => Electric,
            "エスパー" => Psychic,
            "こおり" => Ice,
            "ドラゴン" => Dragon,
            "あく" => Dark,
            "フェアリー" => Fairy,
            _ => panic!("unexpected type name")
        }
    }
}