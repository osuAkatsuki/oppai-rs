bitflags::bitflags! {
    /// The mods available to osu!
    #[derive(Default)]
    pub struct Mods: libc::c_int {
        const NOMOD = 0;
        const NF = 1 << 0;
        const EZ = 1 << 1;
        const TD = 1 << 2;
        const HD = 1 << 3;
        const HR = 1 << 4;
        const SD = 1 << 5;
        const DT = 1 << 6;
        const RX = 1 << 7;
        const HT = 1 << 8;
        const NC = 1 << 9;
        const FL = 1 << 10;
        const AT = 1 << 11;
        const SO = 1 << 12;
        const AP = 1 << 13;
        const PF = 1 << 14;
        const KEY4 = 1 << 15; /* TODO: what are these abbreviated to? */
        const KEY5 = 1 << 16;
        const KEY6 = 1 << 17;
        const KEY7 = 1 << 18;
        const KEY8 = 1 << 19;
        const FADEIN = 1 << 20;
        const RANDOM = 1 << 21;
        const CINEMA = 1 << 22;
        const TARGET = 1 << 23;
        const KEY9 = 1 << 24;
        const KEYCOOP = 1 << 25;
        const KEY1 = 1 << 26;
        const KEY3 = 1 << 27;
        const KEY2 = 1 << 28;
        const SCOREV2 = 1 << 29;
        const TOUCH_DEVICE = Self::TD.bits;
        const NOVIDEO = Self::TD.bits; /* never forget */
        const SPEED_CHANGING = Self::DT.bits | Self::HT.bits | Self::NC.bits;
        const MAP_CHANGING = Self::HR.bits | Self::EZ.bits | Self::SPEED_CHANGING.bits;
    }
}
