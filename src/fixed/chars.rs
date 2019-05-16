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
pub(crate) const B_SIGN_ANJI: &str = "\u{0980}";
pub(crate) const B_CHANDRA: &str = "\u{0981}";
pub(crate) const B_ANUSHAR: &str = "\u{0982}"; // BENGALI SIGN ANUSVARA
pub(crate) const B_BISHARGA: &str = "\u{0983}"; // BENGALI SIGN VISARGA

/* Independent vowels */
pub(crate) const B_A: &str = "\u{0985}";
pub(crate) const B_AA: &str = "\u{0986}";
pub(crate) const B_I: &str = "\u{0987}";
pub(crate) const B_II: &str = "\u{0988}";
pub(crate) const B_U: &str = "\u{0989}";
pub(crate) const B_UU: &str = "\u{098A}";
pub(crate) const B_RRI: &str = "\u{098B}"; // BENGALI LETTER VOCALIC R
pub(crate) const B_VOCALIC_L: &str = "\u{098C}";
pub(crate) const B_E: &str = "\u{098F}";
pub(crate) const B_OI: &str = "\u{0990}"; // BENGALI LETTER AI
pub(crate) const B_O: &str = "\u{0993}";
pub(crate) const B_OU: &str = "\u{0994}";

/* Consonants */
pub(crate) const B_K: &str = "\u{0995}";
pub(crate) const B_KH: &str = "\u{0996}";
pub(crate) const B_G: &str = "\u{0997}";
pub(crate) const B_GH: &str = "\u{0998}";
pub(crate) const B_NGA: &str = "\u{0999}";
pub(crate) const B_C: &str = "\u{099A}";
pub(crate) const B_CH: &str = "\u{099B}";
pub(crate) const B_J: &str = "\u{099C}";
pub(crate) const B_JH: &str = "\u{099D}";
pub(crate) const B_NYA: &str = "\u{099E}";
pub(crate) const B_TT: &str = "\u{099F}";
pub(crate) const B_TTH: &str = "\u{09A0}";
pub(crate) const B_DD: &str = "\u{09A1}";
pub(crate) const B_DDH: &str = "\u{09A2}";
pub(crate) const B_NN: &str = "\u{09A3}";
pub(crate) const B_T: &str = "\u{09A4}";
pub(crate) const B_TH: &str = "\u{09A5}";
pub(crate) const B_D: &str = "\u{09A6}";
pub(crate) const B_DH: &str = "\u{09A7}";
pub(crate) const B_N: &str = "\u{09A8}";
pub(crate) const B_P: &str = "\u{09AA}";
pub(crate) const B_PH: &str = "\u{09AB}";
pub(crate) const B_B: &str = "\u{09AC}";
pub(crate) const B_BH: &str = "\u{09AD}";
pub(crate) const B_M: &str = "\u{09AE}";
pub(crate) const B_Z: &str = "\u{09AF}";
pub(crate) const B_R: &str = "\u{09B0}";
pub(crate) const B_L: &str = "\u{09B2}";
pub(crate) const B_SH: &str = "\u{09B6}";
pub(crate) const B_SS: &str = "\u{09B7}";
pub(crate) const B_S: &str = "\u{09B8}";
pub(crate) const B_H: &str = "\u{09B9}";

/* Various signs */
pub(crate) const B_SIGN_NUKTA: &str = "\u{09BC}"; // for extending the alphabet to new letters
pub(crate) const B_SIGN_AVAGRAHA: &str = "\u{09BD}";

/* Dependent vowel signs (kars) */
pub(crate) const B_AA_KAR: &str = "\u{09BE}";
pub(crate) const B_I_KAR: &str = "\u{09BF}";
pub(crate) const B_II_KAR: &str = "\u{09C0}";
pub(crate) const B_U_KAR: &str = "\u{09C1}";
pub(crate) const B_UU_KAR: &str = "\u{09C2}";
pub(crate) const B_RRI_KAR: &str = "\u{09C3}"; // BENGALI VOWEL SIGN VOCALIC R
pub(crate) const B_VOCALIC_RR: &str = "\u{09C4}"; // BENGALI VOWEL SIGN VOCALIC RR
pub(crate) const B_E_KAR: &str = "\u{09C7}";
pub(crate) const B_OI_KAR: &str = "\u{09C8}";

/* Two-part dependent vowel signs */
pub(crate) const B_O_KAR: &str = "\u{09CB}";
pub(crate) const B_OU_KAR: &str = "\u{09CC}"; // BENGALI VOWEL SIGN AU

/* Virama or Hasant */
pub(crate) const B_HASANTA: &str = "\u{09CD}";

/* Additional consonant */
pub(crate) const B_KHANDATTA: &str = "\u{09CE}";

/* Sign */
pub(crate) const B_LENGTH_MARK: &str = "\u{09D7}"; // BENGALI AU LENGTH MARK

/* Additional consonants */
pub(crate) const B_RR: &str = "\u{09DC}"; // BENGALI LETTER RRA
pub(crate) const B_RH: &str = "\u{09DD}"; // BENGALI LETTER RHA
pub(crate) const B_Y: &str = "\u{09DF}"; // BENGALI LETTER YYA

/* Additional vowels for Sanskrit */
pub(crate) const B_SANSKRIT_RR: &str = "\u{09E0}"; // BENGALI LETTER VOCALIC RR
pub(crate) const B_SANSKRIT_LL: &str = "\u{09E1}"; // BENGALI LETTER VOCALIC LL
pub(crate) const B_SIGN_L: &str = "\u{09E2}"; // BENGALI VOWEL SIGN VOCALIC L
pub(crate) const B_SIGN_LL: &str = "\u{09E3}"; // BENGALI VOWEL SIGN VOCALIC LL

/* Reserved */
/****************************************************************
 * For viram punctuation, use the generic Indic 0964 and 0965.  *
 * Note that these punctuation marks are referred to as dahri   *
 * and double dahri in Bangla.                                  *
 ****************************************************************/
pub(crate) const B_DARI: &str = "\u{0964}";
pub(crate) const B_DDARI: &str = "\u{0965}";

/* Digits */
pub(crate) const B_0: &str = "\u{09E6}";
pub(crate) const B_1: &str = "\u{09E7}";
pub(crate) const B_2: &str = "\u{09E8}";
pub(crate) const B_3: &str = "\u{09E9}";
pub(crate) const B_4: &str = "\u{09EA}";
pub(crate) const B_5: &str = "\u{09EB}";
pub(crate) const B_6: &str = "\u{09EC}";
pub(crate) const B_7: &str = "\u{09ED}";
pub(crate) const B_8: &str = "\u{09EE}";
pub(crate) const B_9: &str = "\u{09EF}";

/* Additions for Assamese */
pub(crate) const B_RM: &str = "\u{09F0}"; // BENGALI LETTER RA WITH MIDDLE DIAGONAL
pub(crate) const B_RL: &str = "\u{09F1}"; // BENGALI LETTER RA WITH LOWER DIAGONAL

/* Currency signs */
pub(crate) const B_CRTAKA_M: &str = "\u{09F2}"; // BENGALI RUPEE MARK = taka
pub(crate) const B_CRTAKA: &str = "\u{09F3}"; // BENGALI RUPEE SIGN = Bangladeshi taka

/* Historic symbols for fractional values */
pub(crate) const B_CURRENCYNUMERATOR_ONE: &str = "\u{09F4}";
pub(crate) const B_CURRENCYNUMERATOR_TWO: &str = "\u{09F5}";
pub(crate) const B_CURRENCYNUMERATOR_THREE: &str = "\u{09F6}";
pub(crate) const B_CURRENCYNUMERATOR_FOUR: &str = "\u{09F7}";
pub(crate) const B_CURRENCYNUMERATOR_LESS: &str = "\u{09F8}";
pub(crate) const B_CURRENCYNUMERATOR_SIXTEEN: &str = "\u{09F9}";

/* Sign */
pub(crate) const B_SIGN_ISSHAR: &str = "\u{09FA}";

/* Historic currency sign */
pub(crate) const B_CURRENCYGANDA: &str = "\u{09FB}";

/* Unicode Addition */
pub(crate) const ZWJ: &str = "\u{200D}";
pub(crate) const ZWNJ: &str = "\u{200C}";