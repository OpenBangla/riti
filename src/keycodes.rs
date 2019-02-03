// Begin Alphanumeric Zone
const VC_GRAVE: u16 =           0x0029;  // '`'
const VC_TILDE: u16 =           0x0001;  // '~'

const VC_1: u16 =               0x0002;
const VC_2: u16 =               0x0003;
const VC_3: u16 =               0x0004;
const VC_4: u16 =               0x0005;
const VC_5: u16 =               0x0006;
const VC_6: u16 =               0x0007;
const VC_7: u16 =               0x0008;
const VC_8: u16 =               0x0009;
const VC_9: u16 =               0x000A;
const VC_0: u16 =               0x000B;

const VC_EXCLAIM: u16 =         0x003B;
const VC_AT: u16 =              0x003C;
const VC_HASH: u16 =            0x003D;
const VC_DOLLAR: u16 =          0x003E;
const VC_PERCENT: u16 =         0x003F;
const VC_CIRCUM: u16 =          0x0040;
const VC_AMPERSAND: u16 =       0x0041;
const VC_ASTERISK: u16 =        0x0042;
const VC_PAREN_LEFT: u16 =      0x0043;
const VC_PAREN_RIGHT: u16 =     0x0044;
const VC_UNDERSCORE: u16 =      0x0057;
const VC_PLUS: u16 =            0x0058;

const VC_MINUS: u16 =           0x000C;    // '-'
const VC_EQUALS: u16 =          0x000D;    // '='
const VC_BACKSPACE: u16 =       0x000E;

const VC_TAB: u16 =             0x000F;

const VC_A: u16 =               0xA0B4;
const VC_B: u16 =               0xA0B5;
const VC_C: u16 =               0xA0B6;
const VC_D: u16 =               0xA0B7;
const VC_E: u16 =               0xA0B8;
const VC_F: u16 =               0xA0B9;
const VC_G: u16 =               0xA0BA;
const VC_H: u16 =               0xA0BB;
const VC_I: u16 =               0xA0BC;
const VC_J: u16 =               0xA0BD;
const VC_K: u16 =               0xA0BE;
const VC_L: u16 =               0xA0BF;
const VC_M: u16 =               0xA0C0;
const VC_N: u16 =               0xA0C1;
const VC_O: u16 =               0xA0C2;
const VC_P: u16 =               0xA0C3;
const VC_Q: u16 =               0xA0C4;
const VC_R: u16 =               0xA0C5;
const VC_S: u16 =               0xA0C6;
const VC_T: u16 =               0xA0C7;
const VC_U: u16 =               0xA0C8;
const VC_V: u16 =               0xA0C9;
const VC_W: u16 =               0xA0CA;
const VC_X: u16 =               0xA0CB;
const VC_Y: u16 =               0xA0CC;
const VC_Z: u16 =               0xA0CD;

const VC_BRACKET_LEFT: u16 =    0x001A;    // '['
const VC_BRACKET_RIGHT: u16 =   0x001B;    // ']'
const VC_BACK_SLASH: u16 =      0x002B;    // '\'

const VC_BRACE_LEFT: u16 =      0x005B;  // '{'
const VC_BRACE_RIGHT: u16 =     0x005C;  // '}'
const VC_BAR: u16 =             0x005D;  // '|'

const VC_SEMICOLON: u16 =       0x0027;    // ';'
const VC_APOSTROPHE: u16 =      0x0028;  // '''
const VC_ENTER: u16 =           0x001C;

const VC_COMMA: u16 =           0x0033;    // ','
const VC_PERIOD: u16 =          0x0034;    // '.'
const VC_SLASH: u16 =           0x0035;    // '/'

const VC_COLON: u16 =           0x0063; // ':'
const VC_QUOTE: u16 =           0x0064; // '"'
const VC_LESS: u16 =            0x0065; // '<'
const VC_GREATER: u16 =         0x0066; // '>'
const VC_QUESTION: u16 =        0x0067; // '?'

const VC_SPACE: u16 =           0x0039;
// End Alphanumeric Zone

const VC_UNKNOWN: u16 =         0x0046;

// Begin Edit Key Zone
const VC_INSERT: u16 =          0x0E52;
const VC_DELETE: u16 =          0x0E53;
const VC_HOME: u16 =            0x0E47;
const VC_END: u16 =             0x0E4F;
const VC_PAGE_UP: u16 =         0x0E49;
const VC_PAGE_DOWN: u16 =       0x0E51;
// End Edit Key Zone


// Begin Cursor Key Zone
const VC_UP: u16 =              0xE048;
const VC_LEFT: u16 =            0xE04B;
const VC_RIGHT: u16 =           0xE04D;
const VC_DOWN: u16 =            0xE050;
// End Cursor Key Zone


// Begin Numeric Zone
const VC_KP_DIVIDE: u16 =       0x0E35;
const VC_KP_MULTIPLY: u16 =     0x0037;
const VC_KP_SUBTRACT: u16 =     0x004A;
const VC_KP_EQUALS: u16 =       0x0E0D;
const VC_KP_ADD: u16 =          0x004E;
const VC_KP_ENTER: u16 =        0x0E1C;
const VC_KP_DECIMAL: u16 =      0x0053;

const VC_KP_1: u16 =            0x004F;
const VC_KP_2: u16 =            0x0050;
const VC_KP_3: u16 =            0x0051;
const VC_KP_4: u16 =            0x004B;
const VC_KP_5: u16 =            0x004C;
const VC_KP_6: u16 =            0x004D;
const VC_KP_7: u16 =            0x0047;
const VC_KP_8: u16 =            0x0048;
const VC_KP_9: u16 =            0x0049;
const VC_KP_0: u16 =            0x0052;
// End Numeric Zone

const VC_SHIFT: u16 =           0x002A;
const VC_CONTROL: u16 =         0x001D;
const VC_ALT: u16 =             0x0038;