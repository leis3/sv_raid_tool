use super::Type;

// (r, g, b, a)
pub fn type_color(type_: Type) -> (u8, u8, u8, f64) {
    match type_ {
        Type::Normal => (159,161,159,0.4),
        Type::Fighting => (255,128,0,0.4),
        Type::Flying => (129,185,239,0.4),
        Type::Poison => (145,65,203,0.4),
        Type::Ground => (145,81,33,0.4),
        Type::Rock => (175,169,129,0.4),
        Type::Bug => (145,161,25,0.4),
        Type::Ghost => (112,65,112,0.4),
        Type::Steel => (96,161,184,0.4),
        Type::Fire => (230,40,41,0.4),
        Type::Water => (41,128,239,0.4),
        Type::Grass => (63,161,41,0.4),
        Type::Electric => (250,192,0,0.4),
        Type::Psychic => (239,65,121,0.4),
        Type::Ice => (63,216,255,0.4),
        Type::Dragon => (80,97,225,0.4),
        Type::Dark => (80,65,63,0.4),
        Type::Fairy => (239,113,239,0.4)
    }
}