#![allow(dead_code)]
//! Key codes

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

pub const VC_A: u16 = 0xA096;
pub const VC_B: u16 = 0xA097;
pub const VC_C: u16 = 0xA098;
pub const VC_D: u16 = 0xA099;
pub const VC_E: u16 = 0xA09A;
pub const VC_F: u16 = 0xA09B;
pub const VC_G: u16 = 0xA09C;
pub const VC_H: u16 = 0xA09D;
pub const VC_I: u16 = 0xA09E;
pub const VC_J: u16 = 0xA09F;
pub const VC_K: u16 = 0xA0A0;
pub const VC_L: u16 = 0xA0A1;
pub const VC_M: u16 = 0xA0A2;
pub const VC_N: u16 = 0xA0A3;
pub const VC_O: u16 = 0xA0A4;
pub const VC_P: u16 = 0xA0A5;
pub const VC_Q: u16 = 0xA0A6;
pub const VC_R: u16 = 0xA0A7;
pub const VC_S: u16 = 0xA0A8;
pub const VC_T: u16 = 0xA0A9;
pub const VC_U: u16 = 0xA0AA;
pub const VC_V: u16 = 0xA0AB;
pub const VC_W: u16 = 0xA0AC;
pub const VC_X: u16 = 0xA0AD;
pub const VC_Y: u16 = 0xA0AE;
pub const VC_Z: u16 = 0xA0AF;

pub const VC_A_SHIFT: u16 = 0xA0B4;
pub const VC_B_SHIFT: u16 = 0xA0B5;
pub const VC_C_SHIFT: u16 = 0xA0B6;
pub const VC_D_SHIFT: u16 = 0xA0B7;
pub const VC_E_SHIFT: u16 = 0xA0B8;
pub const VC_F_SHIFT: u16 = 0xA0B9;
pub const VC_G_SHIFT: u16 = 0xA0BA;
pub const VC_H_SHIFT: u16 = 0xA0BB;
pub const VC_I_SHIFT: u16 = 0xA0BC;
pub const VC_J_SHIFT: u16 = 0xA0BD;
pub const VC_K_SHIFT: u16 = 0xA0BE;
pub const VC_L_SHIFT: u16 = 0xA0BF;
pub const VC_M_SHIFT: u16 = 0xA0C0;
pub const VC_N_SHIFT: u16 = 0xA0C1;
pub const VC_O_SHIFT: u16 = 0xA0C2;
pub const VC_P_SHIFT: u16 = 0xA0C3;
pub const VC_Q_SHIFT: u16 = 0xA0C4;
pub const VC_R_SHIFT: u16 = 0xA0C5;
pub const VC_S_SHIFT: u16 = 0xA0C6;
pub const VC_T_SHIFT: u16 = 0xA0C7;
pub const VC_U_SHIFT: u16 = 0xA0C8;
pub const VC_V_SHIFT: u16 = 0xA0C9;
pub const VC_W_SHIFT: u16 = 0xA0CA;
pub const VC_X_SHIFT: u16 = 0xA0CB;
pub const VC_Y_SHIFT: u16 = 0xA0CC;
pub const VC_Z_SHIFT: u16 = 0xA0CD;

pub const VC_BRACKET_LEFT: u16 = 0x001A; // '['
pub const VC_BRACKET_RIGHT: u16 = 0x001B; // ']'
pub const VC_BACK_SLASH: u16 = 0x002B; // '\'

pub const VC_BRACE_LEFT: u16 = 0x005B; // '{'
pub const VC_BRACE_RIGHT: u16 = 0x005C; // '}'
pub const VC_BAR: u16 = 0x005D; // '|'

pub const VC_SEMICOLON: u16 = 0x0027; // ';'
pub const VC_APOSTROPHE: u16 = 0x0028; // '''

pub const VC_COMMA: u16 = 0x0033; // ','
pub const VC_PERIOD: u16 = 0x0034; // '.'
pub const VC_SLASH: u16 = 0x0035; // '/'

pub const VC_COLON: u16 = 0x0063; // ':'
pub const VC_QUOTE: u16 = 0x0064; // '"'
pub const VC_LESS: u16 = 0x0065; // '<'
pub const VC_GREATER: u16 = 0x0066; // '>'
pub const VC_QUESTION: u16 = 0x0067; // '?'

// End Alphanumeric Zone

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

pub(crate) fn keycode_to_char(key: u16) -> char {
    match key {
        // Alphanumeric keys
        VC_GRAVE => '`',
        VC_TILDE => '~',
        VC_0 => '0',
        VC_PAREN_RIGHT => ')',
        VC_1 => '1',
        VC_EXCLAIM => '!',
        VC_2 => '2',
        VC_AT => '@',
        VC_3 => '3',
        VC_HASH => '#',
        VC_4 => '4',
        VC_DOLLAR => '$',
        VC_5 => '5',
        VC_PERCENT => '%',
        VC_6 => '6',
        VC_CIRCUM => '^',
        VC_7 => '7',
        VC_AMPERSAND => '&',
        VC_8 => '8',
        VC_ASTERISK => '*',
        VC_9 => '9',
        VC_PAREN_LEFT => '(',

        // Alphabet Keys
        VC_Q_SHIFT => 'Q',
        VC_Q => 'q',
        VC_W_SHIFT => 'W',
        VC_W => 'w',
        VC_E_SHIFT => 'E',
        VC_E => 'e',
        VC_R_SHIFT => 'R',
        VC_R => 'r',
        VC_T_SHIFT => 'T',
        VC_T => 't',
        VC_Y_SHIFT => 'Y',
        VC_Y => 'y',
        VC_U_SHIFT => 'U',
        VC_U => 'u',
        VC_I_SHIFT => 'I',
        VC_I => 'i',
        VC_O_SHIFT => 'O',
        VC_O => 'o',
        VC_P_SHIFT => 'P',
        VC_P => 'p',
        VC_A_SHIFT => 'A',
        VC_A => 'a',
        VC_S_SHIFT => 'S',
        VC_S => 's',
        VC_D_SHIFT => 'D',
        VC_D => 'd',
        VC_F_SHIFT => 'F',
        VC_F => 'f',
        VC_G_SHIFT => 'G',
        VC_G => 'g',
        VC_H_SHIFT => 'H',
        VC_H => 'h',
        VC_J_SHIFT => 'J',
        VC_J => 'j',
        VC_K_SHIFT => 'K',
        VC_K => 'k',
        VC_L_SHIFT => 'L',
        VC_L => 'l',
        VC_Z_SHIFT => 'Z',
        VC_Z => 'z',
        VC_X_SHIFT => 'X',
        VC_X => 'x',
        VC_C_SHIFT => 'C',
        VC_C => 'c',
        VC_V_SHIFT => 'V',
        VC_V => 'v',
        VC_B_SHIFT => 'B',
        VC_B => 'b',
        VC_N_SHIFT => 'N',
        VC_N => 'n',
        VC_M_SHIFT => 'M',
        VC_M => 'm',

        VC_MINUS => '-',
        VC_UNDERSCORE => '_',
        VC_EQUALS => '=',
        VC_PLUS => '+',

        VC_BRACKET_LEFT => '[',
        VC_BRACKET_RIGHT => ']',
        VC_BRACE_LEFT => '{',
        VC_BRACE_RIGHT => '}',
        VC_BACK_SLASH => '\\',
        VC_BAR => '|',

        VC_SEMICOLON => ';',
        VC_COLON => ':',
        VC_APOSTROPHE => '\'',
        VC_QUOTE => '\"',

        VC_COMMA => ',',
        VC_LESS => '<',
        VC_PERIOD => '.',
        VC_GREATER => '>',
        VC_SLASH => '/',
        VC_QUESTION => '?',

        // Keypad keys
        VC_KP_0 => '0',
        VC_KP_1 => '1',
        VC_KP_2 => '2',
        VC_KP_3 => '3',
        VC_KP_4 => '4',
        VC_KP_5 => '5',
        VC_KP_6 => '6',
        VC_KP_7 => '7',
        VC_KP_8 => '8',
        VC_KP_9 => '9',

        VC_KP_DIVIDE => '/',
        VC_KP_MULTIPLY => '*',
        VC_KP_SUBTRACT => '-',
        VC_KP_ADD => '+',
        VC_KP_DECIMAL => '.',

        _ => panic!("Got unknown key!"),
    }
}
