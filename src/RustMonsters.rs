enum Species {
    Bulbasaur = 1,
    Ivysaur = 2,
    Venusaur = 3,
    Charmander = 4,
    Charmeleon = 5,
    Charizard = 6,
    Squirtle = 7,
    Wartortle = 8,
    Blastoise = 9,
    Pikachu = 10,
    Raichu = 11,
    Gastly = 12,
    Haunter = 13,
    Gengar = 14,
    Geodude = 15,
    Onix = 16,
    Dratini = 17,
    Dragonair = 18,
    Dragonite = 19,
}

impl Species {
    fn base_data(&self) -> BaseData {
        match self {
            Species:: Charmander => BaseData {
                name: "Charmander",
                base_stats: Stats { hp: 39, attack: 52, defense: 43, sp_attack: 60, sp_defense: 50, speed: 65 },
                typing: Typing { primary: Type::Fire, secondary: None },
                abilities: AbilitySet { primary: Ability::Placeholder, secondary: None, hidden: None }
            },
            _ => todo!()
        }
    }
}

struct Pokemon {
    name: String,
    nickname: String,
    pokedex: u32, // force this to be a tuple to match an arbitrary pokedex? (prevent missingno)
    types: Typing, // definitely force this to be a tuple, define TYPE as a separate struct?
    stats: Stats, // would this be a struct itself? speed, attack, spA, spD, defense, etc
    origtrainer: String,
    nature: Nature, // again, this is interesting because it would impact stats, but is rigidly define

}
enum Nature {
    Adamant, Bashful, Bold, Brave, Calm, Careful,
    Docile, Gentle, Hardy, Hasty, Impish, Jolly,
    Lax, Lonely, Mild, Modest, Naive, Naughty,
    Quiet, Quirky, Rash, Relaxed, Sassy, Timid,
}

enum Type {
    Fire, Water, Grass, Rock, Ground, Steel,
    Ghost, Dark, Psychic, Fighting, Electric,
    Bug, Fairy, Poison, Dragon, Ice, Flying,
}

struct Typing {
    primary: Type,
    secondary: Option<Type>
}

struct Stats {
    hp: u32,
    attack: u32,
    defense: u32,
    sp_attack: u32,
    sp_defense: u32,
    speed: u32,

}

enum Ability {
    Placeholder,
    Placeholder2,
}

struct AbilitySet {
    primary: Ability,
    secondary: Option<Ability>,
    hidden: Option<Ability>,
}

struct BaseData {
    name: &'static str,
    base_stats: Stats,
    typing: Typing,
    abilities: AbilitySet,
}