#[rustfmt::skip]
/* The Unicode Standard 8.0
 * http://www.unicode.org/charts/
 * http://www.unicode.org/charts/PDF/U0980.pdf
 */

/***************************************************************
 * The Bengali script is also known as Bangla. In Assam, the   *
 * preferred name of the script is Asamiya or Assamese. The    *
 * Assamese language has also been written historically using  *
 * distinct regional scripts known as Kamrupi.                 *
 ***************************************************************/

/* We have changed some Character names according to Bangla language or phonetic equivalent.
 * Actual names are denoted by comments */

/* Various signs */
pub(crate) const B_SIGN_ANJI: char = '\u{0980}';
pub(crate) const B_CHANDRA: char = '\u{0981}';
pub(crate) const B_ANUSHAR: char = '\u{0982}'; // BENGALI SIGN ANUSVARA
pub(crate) const B_BISHARGA: char = '\u{0983}'; // BENGALI SIGN VISARGA

/* Independent vowels */
pub(crate) const B_A: char = '\u{0985}';
pub(crate) const B_AA: char = '\u{0986}';
pub(crate) const B_I: char = '\u{0987}';
pub(crate) const B_II: char = '\u{0988}';
pub(crate) const B_U: char = '\u{0989}';
pub(crate) const B_UU: char = '\u{098A}';
pub(crate) const B_RRI: char = '\u{098B}'; // BENGALI LETTER VOCALIC R
pub(crate) const B_VOCALIC_L: char = '\u{098C}';
pub(crate) const B_E: char = '\u{098F}';
pub(crate) const B_OI: char = '\u{0990}'; // BENGALI LETTER AI
pub(crate) const B_O: char = '\u{0993}';
pub(crate) const B_OU: char = '\u{0994}';

/* Consonants */
pub(crate) const B_K: char = '\u{0995}';
pub(crate) const B_KH: char = '\u{0996}';
pub(crate) const B_G: char = '\u{0997}';
pub(crate) const B_GH: char = '\u{0998}';
pub(crate) const B_NGA: char = '\u{0999}';
pub(crate) const B_C: char = '\u{099A}';
pub(crate) const B_CH: char = '\u{099B}';
pub(crate) const B_J: char = '\u{099C}';
pub(crate) const B_JH: char = '\u{099D}';
pub(crate) const B_NYA: char = '\u{099E}';
pub(crate) const B_TT: char = '\u{099F}';
pub(crate) const B_TTH: char = '\u{09A0}';
pub(crate) const B_DD: char = '\u{09A1}';
pub(crate) const B_DDH: char = '\u{09A2}';
pub(crate) const B_NN: char = '\u{09A3}';
pub(crate) const B_T: char = '\u{09A4}';
pub(crate) const B_TH: char = '\u{09A5}';
pub(crate) const B_D: char = '\u{09A6}';
pub(crate) const B_DH: char = '\u{09A7}';
pub(crate) const B_N: char = '\u{09A8}';
pub(crate) const B_P: char = '\u{09AA}';
pub(crate) const B_PH: char = '\u{09AB}';
pub(crate) const B_B: char = '\u{09AC}';
pub(crate) const B_BH: char = '\u{09AD}';
pub(crate) const B_M: char = '\u{09AE}';
pub(crate) const B_Z: char = '\u{09AF}';
pub(crate) const B_R: char = '\u{09B0}';
pub(crate) const B_L: char = '\u{09B2}';
pub(crate) const B_SH: char = '\u{09B6}';
pub(crate) const B_SS: char = '\u{09B7}';
pub(crate) const B_S: char = '\u{09B8}';
pub(crate) const B_H: char = '\u{09B9}';

/* Various signs */
pub(crate) const B_SIGN_NUKTA: char = '\u{09BC}'; // for extending the alphabet to new letters
pub(crate) const B_SIGN_AVAGRAHA: char = '\u{09BD}';

/* Dependent vowel signs (kars) */
pub(crate) const B_AAKAR: char = '\u{09BE}';
pub(crate) const B_IKAR: char = '\u{09BF}';
pub(crate) const B_IIKAR: char = '\u{09C0}';
pub(crate) const B_UKAR: char = '\u{09C1}';
pub(crate) const B_UUKAR: char = '\u{09C2}';
pub(crate) const B_RRIKAR: char = '\u{09C3}'; // BENGALI VOWEL SIGN VOCALIC R
pub(crate) const B_VOCALIC_RR: char = '\u{09C4}'; // BENGALI VOWEL SIGN VOCALIC RR
pub(crate) const B_EKAR: char = '\u{09C7}';
pub(crate) const B_OIKAR: char = '\u{09C8}';

/* Two-part dependent vowel signs */
pub(crate) const B_OKAR: char = '\u{09CB}';
pub(crate) const B_OUKAR: char = '\u{09CC}'; // BENGALI VOWEL SIGN AU

/* Virama or Hasant */
pub(crate) const B_HASANTA: char = '\u{09CD}';

/* Additional consonant */
pub(crate) const B_KHANDATTA: char = '\u{09CE}';

/* Sign */
pub(crate) const B_LENGTH_MARK: char = '\u{09D7}'; // BENGALI AU LENGTH MARK

/* Additional consonants */
pub(crate) const B_RR: char = '\u{09DC}'; // BENGALI LETTER RRA
pub(crate) const B_RH: char = '\u{09DD}'; // BENGALI LETTER RHA
pub(crate) const B_Y: char = '\u{09DF}'; // BENGALI LETTER YYA

/* Additional vowels for Sanskrit */
pub(crate) const B_SANSKRIT_RR: char = '\u{09E0}'; // BENGALI LETTER VOCALIC RR
pub(crate) const B_SANSKRIT_LL: char = '\u{09E1}'; // BENGALI LETTER VOCALIC LL
pub(crate) const B_SIGN_L: char = '\u{09E2}'; // BENGALI VOWEL SIGN VOCALIC L
pub(crate) const B_SIGN_LL: char = '\u{09E3}'; // BENGALI VOWEL SIGN VOCALIC LL

/* Reserved */
/****************************************************************
 * For viram punctuation, use the generic Indic 0964 and 0965.  *
 * Note that these punctuation marks are referred to as dahri   *
 * and double dahri in Bangla.                                  *
 ****************************************************************/
pub(crate) const B_DARI: char = '\u{0964}';
pub(crate) const B_DDARI: char = '\u{0965}';

/* Digits */
pub(crate) const B_0: char = '\u{09E6}';
pub(crate) const B_1: char = '\u{09E7}';
pub(crate) const B_2: char = '\u{09E8}';
pub(crate) const B_3: char = '\u{09E9}';
pub(crate) const B_4: char = '\u{09EA}';
pub(crate) const B_5: char = '\u{09EB}';
pub(crate) const B_6: char = '\u{09EC}';
pub(crate) const B_7: char = '\u{09ED}';
pub(crate) const B_8: char = '\u{09EE}';
pub(crate) const B_9: char = '\u{09EF}';

/* Additions for Assamese */
pub(crate) const B_RM: char = '\u{09F0}'; // BENGALI LETTER RA WITH MIDDLE DIAGONAL
pub(crate) const B_RL: char = '\u{09F1}'; // BENGALI LETTER RA WITH LOWER DIAGONAL

/* Currency signs */
pub(crate) const B_CRTAKA_M: char = '\u{09F2}'; // BENGALI RUPEE MARK = taka
pub(crate) const B_CRTAKA: char = '\u{09F3}'; // BENGALI RUPEE SIGN = Bangladeshi taka

/* Historic symbols for fractional values */
pub(crate) const B_CURRENCYNUMERATOR_ONE: char = '\u{09F4}';
pub(crate) const B_CURRENCYNUMERATOR_TWO: char = '\u{09F5}';
pub(crate) const B_CURRENCYNUMERATOR_THREE: char = '\u{09F6}';
pub(crate) const B_CURRENCYNUMERATOR_FOUR: char = '\u{09F7}';
pub(crate) const B_CURRENCYNUMERATOR_LESS: char = '\u{09F8}';
pub(crate) const B_CURRENCYNUMERATOR_SIXTEEN: char = '\u{09F9}';

/* Sign */
pub(crate) const B_SIGN_ISSHAR: char = '\u{09FA}';

/* Historic currency sign */
pub(crate) const B_CURRENCYGANDA: char = '\u{09FB}';

/* Unicode Addition */
pub(crate) const ZWJ: char = '\u{200D}';
pub(crate) const ZWNJ: char = '\u{200C}';