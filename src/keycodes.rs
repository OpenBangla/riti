//! Key codes
#![allow(non_upper_case_globals)]

// Begin Alphanumeric Zone
pub const VC_GRAVE: u16 = 0x0029; // '`'
pub const VC_TILDE: u16 = 0x0001; // '~'

pub const VC_1: u16 = 0x0002;
pub const VC_2: u16 = 0x0003;
pub const VC_3: u16 = 0x0004;
pub const VC_4: u16 = 0x0005;
pub const VC_5: u16 = 0x0006;
pub const VC_6: u16 = 0x0007;
pub const VC_7: u16 = 0x0008;
pub const VC_8: u16 = 0x0009;
pub const VC_9: u16 = 0x000A;
pub const VC_0: u16 = 0x000B;

pub const VC_EXCLAIM: u16 = 0x003B;
pub const VC_AT: u16 = 0x003C;
pub const VC_HASH: u16 = 0x003D;
pub const VC_DOLLAR: u16 = 0x003E;
pub const VC_PERCENT: u16 = 0x003F;
pub const VC_CIRCUM: u16 = 0x0040;
pub const VC_AMPERSAND: u16 = 0x0041;
pub const VC_ASTERISK: u16 = 0x0042;
pub const VC_PAREN_LEFT: u16 = 0x0043;
pub const VC_PAREN_RIGHT: u16 = 0x0044;
pub const VC_UNDERSCORE: u16 = 0x0057;
pub const VC_PLUS: u16 = 0x0058;

pub const VC_MINUS: u16 = 0x000C; // '-'
pub const VC_EQUALS: u16 = 0x000D; // '='
pub const VC_BACKSPACE: u16 = 0x000E;

pub const VC_TAB: u16 = 0x000F;

pub const VC_a: u16 = 0xA096;
pub const VC_b: u16 = 0xA097;
pub const VC_c: u16 = 0xA098;
pub const VC_d: u16 = 0xA099;
pub const VC_e: u16 = 0xA09A;
pub const VC_f: u16 = 0xA09B;
pub const VC_g: u16 = 0xA09C;
pub const VC_h: u16 = 0xA09D;
pub const VC_i: u16 = 0xA09E;
pub const VC_j: u16 = 0xA09F;
pub const VC_k: u16 = 0xA0A0;
pub const VC_l: u16 = 0xA0A1;
pub const VC_m: u16 = 0xA0A2;
pub const VC_n: u16 = 0xA0A3;
pub const VC_o: u16 = 0xA0A4;
pub const VC_p: u16 = 0xA0A5;
pub const VC_q: u16 = 0xA0A6;
pub const VC_r: u16 = 0xA0A7;
pub const VC_s: u16 = 0xA0A8;
pub const VC_t: u16 = 0xA0A9;
pub const VC_u: u16 = 0xA0AA;
pub const VC_v: u16 = 0xA0AB;
pub const VC_w: u16 = 0xA0AC;
pub const VC_x: u16 = 0xA0AD;
pub const VC_y: u16 = 0xA0AE;
pub const VC_z: u16 = 0xA0AF;

pub const VC_A: u16 = 0xA0B4;
pub const VC_B: u16 = 0xA0B5;
pub const VC_C: u16 = 0xA0B6;
pub const VC_D: u16 = 0xA0B7;
pub const VC_E: u16 = 0xA0B8;
pub const VC_F: u16 = 0xA0B9;
pub const VC_G: u16 = 0xA0BA;
pub const VC_H: u16 = 0xA0BB;
pub const VC_I: u16 = 0xA0BC;
pub const VC_J: u16 = 0xA0BD;
pub const VC_K: u16 = 0xA0BE;
pub const VC_L: u16 = 0xA0BF;
pub const VC_M: u16 = 0xA0C0;
pub const VC_N: u16 = 0xA0C1;
pub const VC_O: u16 = 0xA0C2;
pub const VC_P: u16 = 0xA0C3;
pub const VC_Q: u16 = 0xA0C4;
pub const VC_R: u16 = 0xA0C5;
pub const VC_S: u16 = 0xA0C6;
pub const VC_T: u16 = 0xA0C7;
pub const VC_U: u16 = 0xA0C8;
pub const VC_V: u16 = 0xA0C9;
pub const VC_W: u16 = 0xA0CA;
pub const VC_X: u16 = 0xA0CB;
pub const VC_Y: u16 = 0xA0CC;
pub const VC_Z: u16 = 0xA0CD;

pub const VC_BRACKET_LEFT: u16 = 0x001A; // '['
pub const VC_BRACKET_RIGHT: u16 = 0x001B; // ']'
pub const VC_BACK_SLASH: u16 = 0x002B; // '\'

pub const VC_BRACE_LEFT: u16 = 0x005B; // '{'
pub const VC_BRACE_RIGHT: u16 = 0x005C; // '}'
pub const VC_BAR: u16 = 0x005D; // '|'

pub const VC_SEMICOLON: u16 = 0x0027; // ';'
pub const VC_APOSTROPHE: u16 = 0x0028; // '''
pub const VC_ENTER: u16 = 0x001C;

pub const VC_COMMA: u16 = 0x0033; // ','
pub const VC_PERIOD: u16 = 0x0034; // '.'
pub const VC_SLASH: u16 = 0x0035; // '/'

pub const VC_COLON: u16 = 0x0063; // ':'
pub const VC_QUOTE: u16 = 0x0064; // '"'
pub const VC_LESS: u16 = 0x0065; // '<'
pub const VC_GREATER: u16 = 0x0066; // '>'
pub const VC_QUESTION: u16 = 0x0067; // '?'

pub const VC_SPACE: u16 = 0x0039;
// End Alphanumeric Zone

pub const VC_UNKNOWN: u16 = 0x0046;

// Begin Edit Key Zone
pub const VC_INSERT: u16 = 0x0E52;
pub const VC_DELETE: u16 = 0x0E53;
pub const VC_HOME: u16 = 0x0E47;
pub const VC_END: u16 = 0x0E4F;
pub const VC_PAGE_UP: u16 = 0x0E49;
pub const VC_PAGE_DOWN: u16 = 0x0E51;
// End Edit Key Zone

// Begin Cursor Key Zone
pub const VC_UP: u16 = 0xE048;
pub const VC_LEFT: u16 = 0xE04B;
pub const VC_RIGHT: u16 = 0xE04D;
pub const VC_DOWN: u16 = 0xE050;
// End Cursor Key Zone

// Begin Numeric Zone
pub const VC_KP_DIVIDE: u16 = 0x0E35;
pub const VC_KP_MULTIPLY: u16 = 0x0037;
pub const VC_KP_SUBTRACT: u16 = 0x004A;
pub const VC_KP_EQUALS: u16 = 0x0E0D;
pub const VC_KP_ADD: u16 = 0x004E;
pub const VC_KP_ENTER: u16 = 0x0E1C;
pub const VC_KP_DECIMAL: u16 = 0x0053;

pub const VC_KP_1: u16 = 0x004F;
pub const VC_KP_2: u16 = 0x0050;
pub const VC_KP_3: u16 = 0x0051;
pub const VC_KP_4: u16 = 0x004B;
pub const VC_KP_5: u16 = 0x004C;
pub const VC_KP_6: u16 = 0x004D;
pub const VC_KP_7: u16 = 0x0047;
pub const VC_KP_8: u16 = 0x0048;
pub const VC_KP_9: u16 = 0x0049;
pub const VC_KP_0: u16 = 0x0052;
// End Numeric Zone

pub const VC_SHIFT: u16 = 0x002A;
pub const VC_CONTROL: u16 = 0x001D;
pub const VC_ALT: u16 = 0x0038;
