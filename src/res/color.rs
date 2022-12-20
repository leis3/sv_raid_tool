use super::Type;

// (r, g, b, a)
pub fn type_color(type_: Type) -> (u8, u8, u8, u8) {
    match type_ {
        Normal => (159,161,159,1),
        Fighting => (255,128,0,1),
        Flying => (129,185,239,1),
        Poison => (145,65,203,1),
        Ground => (145,81,33,1),
        Rock => (175,169,129,1),
        Bug => (145,161,25,1),
        Ghost => (112,65,112,1),
        Steel => (96,161,184,1),
        Fire => (230,40,41,1),
        Water => (41,128,239,1),
        Grass => (63,161,41,1),
        Electric => (250,192,0,1),
        Psychic => (239,65,121,1),
        Ice => (63,216,255,1),
        Dragon => (80,97,225,1),
        Dark => (80,65,63,1),
        Fairy => (239,113,239,1)
    }
}