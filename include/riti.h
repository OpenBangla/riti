/* Text to put at the beginning of the generated file. Probably a license. */

#ifndef RITI_H
#define RITI_H

/* Generated with cbindgen:0.18.0 */

/* 
 * Warning, this file is autogenerated by cbindgen. Don't modify this manually.
 * Run this command to generate this file: cbindgen --config cbindgen.toml --output include/riti.h 
 */


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

/*
 Shift modifier key.

 Used by the [`get_suggestion_for_key()`](struct.RitiContext.html#method.get_suggestion_for_key) function.
 */
static const uint8_t MODIFIER_SHIFT = (1 << 0);

/*
 AltGr modifier key.

 Used by the [`get_suggestion_for_key()`](struct.RitiContext.html#method.get_suggestion_for_key) function.
 */
static const uint8_t MODIFIER_ALT_GR = (1 << 1);

static const uint16_t VC_GRAVE = 41;

static const uint16_t VC_TILDE = 1;

static const uint16_t VC_1 = 2;

static const uint16_t VC_2 = 3;

static const uint16_t VC_3 = 4;

static const uint16_t VC_4 = 5;

static const uint16_t VC_5 = 6;

static const uint16_t VC_6 = 7;

static const uint16_t VC_7 = 8;

static const uint16_t VC_8 = 9;

static const uint16_t VC_9 = 10;

static const uint16_t VC_0 = 11;

static const uint16_t VC_EXCLAIM = 59;

static const uint16_t VC_AT = 60;

static const uint16_t VC_HASH = 61;

static const uint16_t VC_DOLLAR = 62;

static const uint16_t VC_PERCENT = 63;

static const uint16_t VC_CIRCUM = 64;

static const uint16_t VC_AMPERSAND = 65;

static const uint16_t VC_ASTERISK = 66;

static const uint16_t VC_PAREN_LEFT = 67;

static const uint16_t VC_PAREN_RIGHT = 68;

static const uint16_t VC_UNDERSCORE = 87;

static const uint16_t VC_PLUS = 88;

static const uint16_t VC_MINUS = 12;

static const uint16_t VC_EQUALS = 13;

static const uint16_t VC_A = 41110;

static const uint16_t VC_B = 41111;

static const uint16_t VC_C = 41112;

static const uint16_t VC_D = 41113;

static const uint16_t VC_E = 41114;

static const uint16_t VC_F = 41115;

static const uint16_t VC_G = 41116;

static const uint16_t VC_H = 41117;

static const uint16_t VC_I = 41118;

static const uint16_t VC_J = 41119;

static const uint16_t VC_K = 41120;

static const uint16_t VC_L = 41121;

static const uint16_t VC_M = 41122;

static const uint16_t VC_N = 41123;

static const uint16_t VC_O = 41124;

static const uint16_t VC_P = 41125;

static const uint16_t VC_Q = 41126;

static const uint16_t VC_R = 41127;

static const uint16_t VC_S = 41128;

static const uint16_t VC_T = 41129;

static const uint16_t VC_U = 41130;

static const uint16_t VC_V = 41131;

static const uint16_t VC_W = 41132;

static const uint16_t VC_X = 41133;

static const uint16_t VC_Y = 41134;

static const uint16_t VC_Z = 41135;

static const uint16_t VC_A_SHIFT = 41140;

static const uint16_t VC_B_SHIFT = 41141;

static const uint16_t VC_C_SHIFT = 41142;

static const uint16_t VC_D_SHIFT = 41143;

static const uint16_t VC_E_SHIFT = 41144;

static const uint16_t VC_F_SHIFT = 41145;

static const uint16_t VC_G_SHIFT = 41146;

static const uint16_t VC_H_SHIFT = 41147;

static const uint16_t VC_I_SHIFT = 41148;

static const uint16_t VC_J_SHIFT = 41149;

static const uint16_t VC_K_SHIFT = 41150;

static const uint16_t VC_L_SHIFT = 41151;

static const uint16_t VC_M_SHIFT = 41152;

static const uint16_t VC_N_SHIFT = 41153;

static const uint16_t VC_O_SHIFT = 41154;

static const uint16_t VC_P_SHIFT = 41155;

static const uint16_t VC_Q_SHIFT = 41156;

static const uint16_t VC_R_SHIFT = 41157;

static const uint16_t VC_S_SHIFT = 41158;

static const uint16_t VC_T_SHIFT = 41159;

static const uint16_t VC_U_SHIFT = 41160;

static const uint16_t VC_V_SHIFT = 41161;

static const uint16_t VC_W_SHIFT = 41162;

static const uint16_t VC_X_SHIFT = 41163;

static const uint16_t VC_Y_SHIFT = 41164;

static const uint16_t VC_Z_SHIFT = 41165;

static const uint16_t VC_BRACKET_LEFT = 26;

static const uint16_t VC_BRACKET_RIGHT = 27;

static const uint16_t VC_BACK_SLASH = 43;

static const uint16_t VC_BRACE_LEFT = 91;

static const uint16_t VC_BRACE_RIGHT = 92;

static const uint16_t VC_BAR = 93;

static const uint16_t VC_SEMICOLON = 39;

static const uint16_t VC_APOSTROPHE = 40;

static const uint16_t VC_COMMA = 51;

static const uint16_t VC_PERIOD = 52;

static const uint16_t VC_SLASH = 53;

static const uint16_t VC_COLON = 99;

static const uint16_t VC_QUOTE = 100;

static const uint16_t VC_LESS = 101;

static const uint16_t VC_GREATER = 102;

static const uint16_t VC_QUESTION = 103;

static const uint16_t VC_KP_DIVIDE = 3637;

static const uint16_t VC_KP_MULTIPLY = 55;

static const uint16_t VC_KP_SUBTRACT = 74;

static const uint16_t VC_KP_EQUALS = 3597;

static const uint16_t VC_KP_ADD = 78;

static const uint16_t VC_KP_ENTER = 3612;

static const uint16_t VC_KP_DECIMAL = 83;

static const uint16_t VC_KP_1 = 79;

static const uint16_t VC_KP_2 = 80;

static const uint16_t VC_KP_3 = 81;

static const uint16_t VC_KP_4 = 75;

static const uint16_t VC_KP_5 = 76;

static const uint16_t VC_KP_6 = 77;

static const uint16_t VC_KP_7 = 71;

static const uint16_t VC_KP_8 = 72;

static const uint16_t VC_KP_9 = 73;

static const uint16_t VC_KP_0 = 82;

/*
 Config struct for configuring RitiContext.
 */
struct Config;

/*
 Context handle used for libRiti IM APIs
 */
struct RitiContext;

/*
 Suggestions which are intended to be shown by the IM's candidate window.
 Suggestion is of two variants, the 'Full' one includes a list of suggestion and
 the 'Single' one is just a String.
 */
struct Suggestion;

extern "C" {

/*
 Creates a new instance of RitiContext with a Config which is properly
 populated using `riti_config_set_*` set of functions.
 */
RitiContext *riti_context_new_with_config(const Config *ptr);

void riti_context_free(RitiContext *ptr);

Suggestion *riti_get_suggestion_for_key(RitiContext *ptr,
                                        uint16_t key,
                                        uint8_t modifier);

/*
 A candidate of the suggestion list was committed.

 `index`: index of the candidate.

 This function will end the ongoing input session.
 */
void riti_context_candidate_committed(RitiContext *ptr, uintptr_t index);

/*
 Update the suggestion making engine. This would also look for changes
 in layout selection and AutoCorrect database.
 */
void riti_context_update_engine(RitiContext *ptr, const Config *config);

/*
 Checks if there is an ongoing input session.
 */
bool riti_context_ongoing_input_session(RitiContext *ptr);

/*
 Finish the ongoing input session if any.
 */
void riti_context_finish_input_session(RitiContext *ptr);

/*
 A BackSpace event.

 Returns a new `suggestion` after applying the BackSpace event.

 If the internal buffer becomes empty, this function will
 end the ongoing input session.
 */
Suggestion *riti_context_backspace_event(RitiContext *ptr);

void riti_suggestion_free(Suggestion *ptr);

/*
 Get the suggestion of the `index` from suggestions.
 */
char *riti_suggestion_get_suggestion(const Suggestion *ptr, uintptr_t index);

/*
 Get the only suggestion of the *lonely* `Suggestion`.
 */
char *riti_suggestion_get_lonely_suggestion(const Suggestion *ptr);

char *riti_suggestion_get_auxiliary_text(const Suggestion *ptr);

/*
 Free the allocated string.
 */
void riti_string_free(char *ptr);

/*
 Returns index of the suggestion, which was previously selected.
 */
uintptr_t riti_suggestion_previously_selected_index(const Suggestion *ptr);

uintptr_t riti_suggestion_get_length(const Suggestion *ptr);

/*
 Returns `true` when the `Suggestion` struct is a **lonely** one, otherwise returns `false`.

 A *lonely* `Suggestion` struct means that the struct has only one suggestion.
 */
bool riti_suggestion_is_lonely(const Suggestion *ptr);

bool riti_suggestion_is_empty(const Suggestion *ptr);

/*
 Creates a new instance of Config which is used to initialize
 and to control the configuration of RitiContext.

 This function creates an instance of Config in an initial
 state which can't be used before populating the Config using
 `riti_config_set_*` set of functions.
 */
Config *riti_config_new();

/*
 Free the allocated Config struct.
 */
void riti_config_free(Config *ptr);

void riti_config_set_layout_file(Config *ptr, const char *path);

void riti_config_set_database_dir(Config *ptr, const char *path);

void riti_config_set_suggestion_include_english(Config *ptr, bool option);

void riti_config_set_phonetic_suggestion(Config *ptr, bool option);

void riti_config_set_fixed_suggestion(Config *ptr, bool option);

void riti_config_set_fixed_auto_vowel(Config *ptr, bool option);

void riti_config_set_fixed_auto_chandra(Config *ptr, bool option);

void riti_config_set_fixed_traditional_kar(Config *ptr, bool option);

void riti_config_set_fixed_old_reph(Config *ptr, bool option);

void riti_config_set_fixed_numpad(Config *ptr, bool option);

void riti_config_set_fixed_old_kar_order(Config *ptr, bool option);

} // extern "C"

#endif // RITI_H
