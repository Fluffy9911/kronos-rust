use crate::col::cols::Col8;

impl Col8 {
    // Basic colors
    pub const TRANSPARENT: Self = Self::new_a(0, 0, 0, 0);
    pub const BLACK: Self = Self::new(0, 0, 0);
    pub const WHITE: Self = Self::new(255, 255, 255);
    pub const RED: Self = Self::new(255, 0, 0);
    pub const GREEN: Self = Self::new(0, 255, 0);
    pub const BLUE: Self = Self::new(0, 0, 255);
    pub const YELLOW: Self = Self::new(255, 255, 0);
    pub const CYAN: Self = Self::new(0, 255, 255);
    pub const MAGENTA: Self = Self::new(255, 0, 255);

    // Greyscale
    pub const GRAY: Self = Self::new(128, 128, 128);
    pub const GREY: Self = Self::GRAY;
    pub const DARK_GRAY: Self = Self::new(64, 64, 64);
    pub const LIGHT_GRAY: Self = Self::new(192, 192, 192);
    pub const DIM_GRAY: Self = Self::new(105, 105, 105);
    pub const SILVER: Self = Self::new(192, 192, 192);
    pub const GAINSBORO: Self = Self::new(220, 220, 220);
    pub const WHITE_SMOKE: Self = Self::new(245, 245, 245);

    // Reds and pinks
    pub const DARK_RED: Self = Self::new(139, 0, 0);
    pub const MAROON: Self = Self::new(128, 0, 0);
    pub const CRIMSON: Self = Self::new(220, 20, 60);
    pub const FIREBRICK: Self = Self::new(178, 34, 34);
    pub const INDIAN_RED: Self = Self::new(205, 92, 92);
    pub const SALMON: Self = Self::new(250, 128, 114);
    pub const LIGHT_CORAL: Self = Self::new(240, 128, 128);
    pub const PINK: Self = Self::new(255, 192, 203);
    pub const HOT_PINK: Self = Self::new(255, 105, 180);
    pub const DEEP_PINK: Self = Self::new(255, 20, 147);

    // Oranges and browns
    pub const ORANGE: Self = Self::new(255, 165, 0);
    pub const DARK_ORANGE: Self = Self::new(255, 140, 0);
    pub const CORAL: Self = Self::new(255, 127, 80);
    pub const TOMATO: Self = Self::new(255, 99, 71);
    pub const GOLD: Self = Self::new(255, 215, 0);
    pub const GOLDENROD: Self = Self::new(218, 165, 32);
    pub const BROWN: Self = Self::new(165, 42, 42);
    pub const SADDLE_BROWN: Self = Self::new(139, 69, 19);
    pub const CHOCOLATE: Self = Self::new(210, 105, 30);
    pub const TAN: Self = Self::new(210, 180, 140);
    pub const BEIGE: Self = Self::new(245, 245, 220);
    pub const WHEAT: Self = Self::new(245, 222, 179);

    // Greens
    pub const DARK_GREEN: Self = Self::new(0, 100, 0);
    pub const LIME: Self = Self::new(0, 255, 0);
    pub const LIME_GREEN: Self = Self::new(50, 205, 50);
    pub const FOREST_GREEN: Self = Self::new(34, 139, 34);
    pub const SEA_GREEN: Self = Self::new(46, 139, 87);
    pub const OLIVE: Self = Self::new(128, 128, 0);
    pub const OLIVE_DRAB: Self = Self::new(107, 142, 35);
    pub const SPRING_GREEN: Self = Self::new(0, 255, 127);
    pub const MEDIUM_SEA_GREEN: Self = Self::new(60, 179, 113);
    pub const PALE_GREEN: Self = Self::new(152, 251, 152);
    pub const DARK_SEA_GREEN: Self = Self::new(143, 188, 143);
    pub const MINT: Self = Self::new(189, 252, 201);

    // Blues
    pub const NAVY: Self = Self::new(0, 0, 128);
    pub const DARK_BLUE: Self = Self::new(0, 0, 139);
    pub const MEDIUM_BLUE: Self = Self::new(0, 0, 205);
    pub const ROYAL_BLUE: Self = Self::new(65, 105, 225);
    pub const STEEL_BLUE: Self = Self::new(70, 130, 180);
    pub const DODGER_BLUE: Self = Self::new(30, 144, 255);
    pub const CORNFLOWER_BLUE: Self = Self::new(100, 149, 237);
    pub const SKY_BLUE: Self = Self::new(135, 206, 235);
    pub const LIGHT_BLUE: Self = Self::new(173, 216, 230);
    pub const DEEP_SKY_BLUE: Self = Self::new(0, 191, 255);
    pub const TEAL: Self = Self::new(0, 128, 128);
    pub const TURQUOISE: Self = Self::new(64, 224, 208);
    pub const AQUA: Self = Self::new(0, 255, 255);
    pub const MIDNIGHT_BLUE: Self = Self::new(25, 25, 112);

    // Purples
    pub const PURPLE: Self = Self::new(128, 0, 128);
    pub const INDIGO: Self = Self::new(75, 0, 130);
    pub const VIOLET: Self = Self::new(238, 130, 238);
    pub const DARK_VIOLET: Self = Self::new(148, 0, 211);
    pub const BLUE_VIOLET: Self = Self::new(138, 43, 226);
    pub const PLUM: Self = Self::new(221, 160, 221);
    pub const ORCHID: Self = Self::new(218, 112, 214);
    pub const LAVENDER: Self = Self::new(230, 230, 250);
    pub const REBECCA_PURPLE: Self = Self::new(102, 51, 153);

    // Common web colors
    pub const ALICE_BLUE: Self = Self::new(240, 248, 255);
    pub const ANTIQUE_WHITE: Self = Self::new(250, 235, 215);
    pub const AZURE: Self = Self::new(240, 255, 255);
    pub const IVORY: Self = Self::new(255, 255, 240);
    pub const KHAKI: Self = Self::new(240, 230, 140);
    pub const LEMON_CHIFFON: Self = Self::new(255, 250, 205);
    pub const LAVENDER_BLUSH: Self = Self::new(255, 240, 245);
    pub const MISTY_ROSE: Self = Self::new(255, 228, 225);
    pub const OLD_LACE: Self = Self::new(253, 245, 230);
    pub const PAPAYA_WHIP: Self = Self::new(255, 239, 213);
    pub const PEACH_PUFF: Self = Self::new(255, 218, 185);
    pub const SEA_SHELL: Self = Self::new(255, 245, 238);
    pub const SNOW: Self = Self::new(255, 250, 250);
    pub const HONEYDEW: Self = Self::new(240, 255, 240);

    // Handy renderer colors
    pub const CLEAR: Self = Self::TRANSPARENT;
    pub const ERROR: Self = Self::CRIMSON;
    pub const SUCCESS: Self = Self::FOREST_GREEN;
    pub const WARNING: Self = Self::GOLD;
    pub const INFO: Self = Self::DODGER_BLUE;
}
