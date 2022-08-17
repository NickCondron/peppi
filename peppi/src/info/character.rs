pub mod external {
    use crate::model::enums::character::External;

    #[non_exhaustive]
    #[derive(Clone, Debug)]
    pub struct Info {
        pub external: External,
        pub short: &'static str,
        pub long: &'static str,
    }

    info!(External => Info {
        CAPTAIN_FALCON {
            external: External::CAPTAIN_FALCON,
            short: "Falcon",
            long: "Captain Falcon",
        };

        DONKEY_KONG {
            external: External::DONKEY_KONG,
            short: "DK",
            long: "Donkey Kong",
        };

        FOX {
            external: External::FOX,
            short: "Fox",
            long: "Fox",
        };

        GAME_AND_WATCH {
            external: External::GAME_AND_WATCH,
            short: "G&W",
            long: "Game and Watch",
        };

        KIRBY {
            external: External::KIRBY,
            short: "Kirby",
            long: "Kirby",
        };

        BOWSER {
            external: External::BOWSER,
            short: "Bowser",
            long: "Bowser",
        };

        LINK {
            external: External::LINK,
            short: "Link",
            long: "Link",
        };

        LUIGI {
            external: External::LUIGI,
            short: "Luigi",
            long: "Luigi",
        };

        MARIO {
            external: External::MARIO,
            short: "Mario",
            long: "Mario",
        };

        MARTH {
            external: External::MARTH,
            short: "Marth",
            long: "Marth",
        };

        MEWTWO {
            external: External::MEWTWO,
            short: "Mewtwo",
            long: "Mewtwo",
        };

        NESS {
            external: External::NESS,
            short: "Ness",
            long: "Ness",
        };

        PEACH {
            external: External::PEACH,
            short: "Peach",
            long: "Peach",
        };

        PIKACHU {
            external: External::PIKACHU,
            short: "Pika",
            long: "Pikachu",
        };

        ICE_CLIMBERS {
            external: External::ICE_CLIMBERS,
            short: "ICs",
            long: "Ice Climbers",
        };

        JIGGLYPUFF {
            external: External::JIGGLYPUFF,
            short: "Puff",
            long: "Jigglypuff",
        };

        SAMUS {
            external: External::SAMUS,
            short: "Samus",
            long: "Samus",
        };

        YOSHI {
            external: External::YOSHI,
            short: "Yoshi",
            long: "Yoshi",
        };

        ZELDA {
            external: External::ZELDA,
            short: "Zelda",
            long: "Zelda",
        };

        SHEIK {
            external: External::SHEIK,
            short: "Sheik",
            long: "Sheik",
        };

        FALCO {
            external: External::FALCO,
            short: "Falco",
            long: "Falco",
        };

        YOUNG_LINK {
            external: External::YOUNG_LINK,
            short: "YL",
            long: "Young Link",
        };

        DR_MARIO {
            external: External::DR_MARIO,
            short: "Doc",
            long: "Dr. Mario",
        };

        ROY {
            external: External::ROY,
            short: "Roy",
            long: "Roy",
        };

        PICHU {
            external: External::PICHU,
            short: "Pichu",
            long: "Pichu",
        };

        GANONDORF {
            external: External::GANONDORF,
            short: "Ganon",
            long: "Ganondorf",
        };

        MASTER_HAND {
            external: External::MASTER_HAND,
            short: "Master Hand",
            long: "Master Hand",
        };

        WIRE_FRAME_MALE {
            external: External::WIRE_FRAME_MALE,
            short: "Male Wireframe",
            long: "Male Wireframe",
        };

        WIRE_FRAME_FEMALE {
            external: External::WIRE_FRAME_FEMALE,
            short: "Female Wireframe",
            long: "Female Wireframe",
        };

        GIGA_BOWSER {
            external: External::GIGA_BOWSER,
            short: "Giga Bowser",
            long: "Giga Bowser",
        };

        CRAZY_HAND {
            external: External::CRAZY_HAND,
            short: "Crazy Hand",
            long: "Crazy Hand",
        };

        SANDBAG {
            external: External::SANDBAG,
            short: "Sandbag",
            long: "Sandbag",
        };

        POPO {
            external: External::POPO,
            short: "Popo",
            long: "Popo",
        };
    });


    info_regex!(Info {
        CAPTAIN_FALCON: r"(?i-u)^(capt(ain|\.)?[ _]?)?falcon$",
        DONKEY_KONG: r"(?i-u)^dk|(donkey[ _]?kong$",
        FOX: r"(?i-u)^fox$",
        GAME_AND_WATCH: r"(?i-u)^(g|game)[ _]?(and|&|n)[ _]?(w|watch)$",
        KIRBY: r"(?i-u)^kirby$",
        BOWSER: r"(?i-u)^bowser$",
        LINK: r"(?i-u)^link$",
        LUIGI: r"(?i-u)^luigi$",
        MARIO: r"(?i-u)^mario$",
        MARTH: r"(?i-u)^marth$",
        MEWTWO: r"(?i-u)^mewtwo|mew2|m2$",
        NESS: r"(?i-u)^ness$",
        PEACH: r"(?i-u)^peach$",
        PIKACHU: r"(?i-u)^pika(chu)?$",
        ICE_CLIMBERS: r"(?i-u)^ic(|s|ies|e[ _]?climbers)$",
        JIGGLYPUFF: r"(?i-u)^(jiggly)?puff|jiggs$",
        SAMUS: r"(?i-u)^samus$",
        YOSHI: r"(?i-u)^yoshi$",
        ZELDA: r"(?i-u)^zelda$",
        SHEIK: r"(?i-u)^sheik$",
        FALCO: r"(?i-u)^falco$",
        YOUNG_LINK: r"(?i-u)^(y\.?|young)[ _]?(l|link$",
        DR_MARIO: r"(?i-u)^doc|dr\.?[ _]?mario$",
        ROY: r"(?i-u)^roy$",
        PICHU: r"(?i-u)^pichu$",
        GANONDORF: r"(?i-u)^ganon(dorf)?$",
        MASTER_HAND: r"(?i-u)^master[ _]?hand$",
        WIRE_FRAME_MALE: r"(?i-u)^male[ _]?wire[ _]?frame|wire[ _]?frame[ _]?male$",
        WIRE_FRAME_FEMALE: r"(?i-u)^female[ _]?wire[ _]?frame|wire[ _]?frame[ _]?female$",
        GIGA_BOWSER: r"(?i-u)^giga[ _]?bowser$",
        CRAZY_HAND: r"(?i-u)^crazy[ _]?hand$",
        SANDBAG: r"(?i-u)^sandbag$",
        POPO: r"(?i-u)^popo$",
    });
}

pub mod internal {
    use crate::model::enums::character::Internal;

    #[non_exhaustive]
    #[derive(Clone, Debug)]
    pub struct Info {
        pub internal: Internal,
        pub jumpsquat: u8,
        pub empty_landing_lag: u8,
    }

    info!(Internal => Info {
        MARIO {
            internal: Internal::MARIO,
            jumpsquat: 4,
            empty_landing_lag: 4,
        };

        FOX {
            internal: Internal::FOX,
            jumpsquat: 3,
            empty_landing_lag: 4,
        };

        CAPTAIN_FALCON {
            internal: Internal::CAPTAIN_FALCON,
            jumpsquat: 4,
            empty_landing_lag: 4,
        };

        DONKEY_KONG {
            internal: Internal::DONKEY_KONG,
            jumpsquat: 5,
            empty_landing_lag: 5,
        };

        KIRBY {
            internal: Internal::KIRBY,
            jumpsquat: 3,
            empty_landing_lag: 4,
        };

        BOWSER {
            internal: Internal::BOWSER,
            jumpsquat: 8,
            empty_landing_lag: 6,
        };

        LINK {
            internal: Internal::LINK,
            jumpsquat: 6,
            empty_landing_lag: 4,
        };

        SHEIK {
            internal: Internal::SHEIK,
            jumpsquat: 3,
            empty_landing_lag: 4,
        };

        NESS {
            internal: Internal::NESS,
            jumpsquat: 4,
            empty_landing_lag: 4,
        };

        PEACH {
            internal: Internal::PEACH,
            jumpsquat: 5,
            empty_landing_lag: 4,
        };

        POPO {
            internal: Internal::POPO,
            jumpsquat: 3,
            empty_landing_lag: 4,
        };

        NANA {
            internal: Internal::NANA,
            jumpsquat: 3,
            empty_landing_lag: 4,
        };

        PIKACHU {
            internal: Internal::PIKACHU,
            jumpsquat: 3,
            empty_landing_lag: 4,
        };

        SAMUS {
            internal: Internal::SAMUS,
            jumpsquat: 3,
            empty_landing_lag: 4,
        };

        YOSHI {
            internal: Internal::YOSHI,
            jumpsquat: 5,
            empty_landing_lag: 4,
        };

        JIGGLYPUFF {
            internal: Internal::JIGGLYPUFF,
            jumpsquat: 5,
            empty_landing_lag: 4,
        };

        MEWTWO {
            internal: Internal::MEWTWO,
            jumpsquat: 5,
            empty_landing_lag: 4,
        };

        LUIGI {
            internal: Internal::LUIGI,
            jumpsquat: 4,
            empty_landing_lag: 4,
        };

        MARTH {
            internal: Internal::MARTH,
            jumpsquat: 4,
            empty_landing_lag: 4,
        };

        ZELDA {
            internal: Internal::ZELDA,
            jumpsquat: 6,
            empty_landing_lag: 4,
        };

        YOUNG_LINK {
            internal: Internal::YOUNG_LINK,
            jumpsquat: 4,
            empty_landing_lag: 4,
        };

        DR_MARIO {
            internal: Internal::DR_MARIO,
            jumpsquat: 4,
            empty_landing_lag: 4,
        };

        FALCO {
            internal: Internal::FALCO,
            jumpsquat: 5,
            empty_landing_lag: 4,
        };

        PICHU {
            internal: Internal::PICHU,
            jumpsquat: 3,
            empty_landing_lag: 2,
        };

        GAME_AND_WATCH {
            internal: Internal::GAME_AND_WATCH,
            jumpsquat: 4,
            empty_landing_lag: 4,
        };

        GANONDORF {
            internal: Internal::GANONDORF,
            jumpsquat: 6,
            empty_landing_lag: 5,
        };

        ROY {
            internal: Internal::ROY,
            jumpsquat: 5,
            empty_landing_lag: 4,
        };

        WIRE_FRAME_MALE {
            internal: Internal::WIRE_FRAME_MALE,
            jumpsquat: 7,
            empty_landing_lag: 15,
        };

        WIRE_FRAME_FEMALE {
            internal: Internal::WIRE_FRAME_FEMALE,
            jumpsquat: 7,
            empty_landing_lag: 15,
        };

        GIGA_BOWSER {
            internal: Internal::GIGA_BOWSER,
            jumpsquat: 6,
            empty_landing_lag: 30,
        };
    });
}
