//! # Windows key codes
//!
//! `win_key_codes` is a translation of all available windows virtual
//! key codes in Rust

/// Left mouse button
pub const VK_LBUTTON: i32 = 0x01;
/// Right mouse button
pub const VK_RBUTTON: i32 = 0x02;
/// Control-break processing
pub const VK_CANCEL: i32 = 0x03;
/// Middle mouse button (three-button mouse)
pub const VK_MBUTTON: i32 = 0x04;
/// X1 mouse button
pub const VK_XBUTTON1: i32 = 0x05;
/// X2 mouse button
pub const VK_XBUTTON2: i32 = 0x06;
/// BACKSPACE key
pub const VK_BACK: i32 = 0x08;
/// TAB key
pub const VK_TAB: i32 = 0x09;
/// CLEAR key
pub const VK_CLEAR: i32 = 0x0C;
/// ENTER key
pub const VK_RETURN: i32 = 0x0D;
/// SHIFT key
pub const VK_SHIFT: i32 = 0x10;
/// CTRL key
pub const VK_CONTROL: i32 = 0x11;
/// ALT key
pub const VK_MENU: i32 = 0x12;
/// PAUSE key
pub const VK_PAUSE: i32 = 0x13;
/// CAPS LOCK key
pub const VK_CAPITAL: i32 = 0x14;
/// IME Kana mode
pub const VK_KANA: i32 = 0x15;
/// IME Hanguel mode (maintained for compatibility; use VK_HANGUL)
pub const VK_HANGUEL: i32 = 0x15;
/// IME Hangul mode
pub const VK_HANGUL: i32 = 0x15;
/// IME Junja mode
pub const VK_JUNJA: i32 = 0x17;
/// IME final mode
pub const VK_FINAL: i32 = 0x18;
/// IME Hanja mode
pub const VK_HANJA: i32 = 0x19;
/// IME Kanji mode
pub const VK_KANJI: i32 = 0x19;
/// ESC key
pub const VK_ESCAPE: i32 = 0x1B;
/// IME convert
pub const VK_CONVERT: i32 = 0x1C;
/// IME nonconvert
pub const VK_NONCONVERT: i32 = 0x1D;
/// IME accept
pub const VK_ACCEPT: i32 = 0x1E;
/// IME mode change request
pub const VK_MODECHANGE: i32 = 0x1F;
/// SPACEBAR
pub const VK_SPACE: i32 = 0x20;
/// PAGE UP key
pub const VK_PRIOR: i32 = 0x21;
/// PAGE DOWN key
pub const VK_NEXT: i32 = 0x22;
/// END key
pub const VK_END: i32 = 0x23;
/// HOME key
pub const VK_HOME: i32 = 0x24;
/// LEFT ARROW key
pub const VK_LEFT: i32 = 0x25;
/// UP ARROW key
pub const VK_UP: i32 = 0x26;
/// RIGHT ARROW key
pub const VK_RIGHT: i32 = 0x27;
/// DOWN ARROW key
pub const VK_DOWN: i32 = 0x28;
/// SELECT key
pub const VK_SELECT: i32 = 0x29;
/// PRINT key
pub const VK_PRINT: i32 = 0x2A;
/// EXECUTE key
pub const VK_EXECUTE: i32 = 0x2B;
/// PRINT SCREEN key
pub const VK_SNAPSHOT: i32 = 0x2C;
/// INS key
pub const VK_INSERT: i32 = 0x2D;
/// DEL key
pub const VK_DELETE: i32 = 0x2E;
/// HELP key
pub const VK_HELP: i32 = 0x2F;
/// 0 key
pub const VK_0: i32 = 0x30;
/// 1 key
pub const VK_1: i32 = 0x31;
/// 2 key
pub const VK_2: i32 = 0x32;
/// 3 key
pub const VK_3: i32 = 0x33;
/// 4 key
pub const VK_4: i32 = 0x34;
/// 5 key
pub const VK_5: i32 = 0x35;
/// 6 key
pub const VK_6: i32 = 0x36;
/// 7 key
pub const VK_7: i32 = 0x37;
/// 8 key
pub const VK_8: i32 = 0x38;
/// 9 key
pub const VK_9: i32 = 0x39;
/// A key
pub const VK_A: i32 = 0x41;
/// B key
pub const VK_B: i32 = 0x42;
/// C key
pub const VK_C: i32 = 0x43;
/// D key
pub const VK_D: i32 = 0x44;
/// E key
pub const VK_E: i32 = 0x45;
/// F key
pub const VK_F: i32 = 0x46;
/// G key
pub const VK_G: i32 = 0x47;
/// H key
pub const VK_H: i32 = 0x48;
/// I key
pub const VK_I: i32 = 0x49;
/// J key
pub const VK_J: i32 = 0x4A;
/// K key
pub const VK_K: i32 = 0x4B;
/// L key
pub const VK_L: i32 = 0x4C;
/// M key
pub const VK_M: i32 = 0x4D;
/// N key
pub const VK_N: i32 = 0x4E;
/// O key
pub const VK_O: i32 = 0x4F;
/// P key
pub const VK_P: i32 = 0x50;
/// Q key
pub const VK_Q: i32 = 0x51;
/// R key
pub const VK_R: i32 = 0x52;
/// S key
pub const VK_S: i32 = 0x53;
/// T key
pub const VK_T: i32 = 0x54;
/// U key
pub const VK_U: i32 = 0x55;
/// V key
pub const VK_V: i32 = 0x56;
/// W key
pub const VK_W: i32 = 0x57;
/// X key
pub const VK_X: i32 = 0x58;
/// Y key
pub const VK_Y: i32 = 0x59;
/// Z key
pub const VK_Z: i32 = 0x5A;
/// Left Windows key (Natural keyboard)
pub const VK_LWIN: i32 = 0x5B;
/// Right Windows key (Natural keyboard)
pub const VK_RWIN: i32 = 0x5C;
/// Applications key (Natural keyboard)
pub const VK_APPS: i32 = 0x5D;
/// Computer Sleep key
pub const VK_SLEEP: i32 = 0x5F;
/// Numeric keypad 0 key
pub const VK_NUMPAD0: i32 = 0x60;
/// Numeric keypad 1 key
pub const VK_NUMPAD1: i32 = 0x61;
/// Numeric keypad 2 key
pub const VK_NUMPAD2: i32 = 0x62;
/// Numeric keypad 3 key
pub const VK_NUMPAD3: i32 = 0x63;
/// Numeric keypad 4 key
pub const VK_NUMPAD4: i32 = 0x64;
/// Numeric keypad 5 key
pub const VK_NUMPAD5: i32 = 0x65;
/// Numeric keypad 6 key
pub const VK_NUMPAD6: i32 = 0x66;
/// Numeric keypad 7 key
pub const VK_NUMPAD7: i32 = 0x67;
/// Numeric keypad 8 key
pub const VK_NUMPAD8: i32 = 0x68;
/// Numeric keypad 9 key
pub const VK_NUMPAD9: i32 = 0x69;
/// Multiply key
pub const VK_MULTIPLY: i32 = 0x6A;
/// Add key
pub const VK_ADD: i32 = 0x6B;
/// Separator key
pub const VK_SEPARATOR: i32 = 0x6C;
/// Subtract key
pub const VK_SUBTRACT: i32 = 0x6D;
/// Decimal key
pub const VK_DECIMAL: i32 = 0x6E;
/// Divide key
pub const VK_DIVIDE: i32 = 0x6F;
/// F1 key
pub const VK_F1: i32 = 0x70;
/// F2 key
pub const VK_F2: i32 = 0x71;
/// F3 key
pub const VK_F3: i32 = 0x72;
/// F4 key
pub const VK_F4: i32 = 0x73;
/// F5 key
pub const VK_F5: i32 = 0x74;
/// F6 key
pub const VK_F6: i32 = 0x75;
/// F7 key
pub const VK_F7: i32 = 0x76;
/// F8 key
pub const VK_F8: i32 = 0x77;
/// F9 key
pub const VK_F9: i32 = 0x78;
/// F10 key
pub const VK_F10: i32 = 0x79;
/// F11 key
pub const VK_F11: i32 = 0x7A;
/// F12 key
pub const VK_F12: i32 = 0x7B;
/// F13 key
pub const VK_F13: i32 = 0x7C;
/// F14 key
pub const VK_F14: i32 = 0x7D;
/// F15 key
pub const VK_F15: i32 = 0x7E;
/// F16 key
pub const VK_F16: i32 = 0x7F;
/// F17 key
pub const VK_F17: i32 = 0x80;
/// F18 key
pub const VK_F18: i32 = 0x81;
/// F19 key
pub const VK_F19: i32 = 0x82;
/// F20 key
pub const VK_F20: i32 = 0x83;
/// F21 key
pub const VK_F21: i32 = 0x84;
/// F22 key
pub const VK_F22: i32 = 0x85;
/// F23 key
pub const VK_F23: i32 = 0x86;
/// F24 key
pub const VK_F24: i32 = 0x87;
/// NUM LOCK key
pub const VK_NUMLOCK: i32 = 0x90;
/// SCROLL LOCK key
pub const VK_SCROLL: i32 = 0x91;
/// Left SHIFT key
pub const VK_LSHIFT: i32 = 0xA0;
/// Right SHIFT key
pub const VK_RSHIFT: i32 = 0xA1;
/// Left CONTROL key
pub const VK_LCONTROL: i32 = 0xA2;
/// Right CONTROL key
pub const VK_RCONTROL: i32 = 0xA3;
/// Left MENU key
pub const VK_LMENU: i32 = 0xA4;
/// Right MENU key
pub const VK_RMENU: i32 = 0xA5;
/// Browser Back key
pub const VK_BROWSER_BACK: i32 = 0xA6;
/// Browser Forward key
pub const VK_BROWSER_FORWARD: i32 = 0xA7;
/// Browser Refresh key
pub const VK_BROWSER_REFRESH: i32 = 0xA8;
/// Browser Stop key
pub const VK_BROWSER_STOP: i32 = 0xA9;
/// Browser Search key
pub const VK_BROWSER_SEARCH: i32 = 0xAA;
/// Browser Favorites key
pub const VK_BROWSER_FAVORITES: i32 = 0xAB;
/// Browser Start and Home key
pub const VK_BROWSER_HOME: i32 = 0xAC;
/// Volume Mute key
pub const VK_VOLUME_MUTE: i32 = 0xAD;
/// Volume Down key
pub const VK_VOLUME_DOWN: i32 = 0xAE;
/// Volume Up key
pub const VK_VOLUME_UP: i32 = 0xAF;
/// Next Track key
pub const VK_MEDIA_NEXT_TRACK: i32 = 0xB0;
/// Previous Track key
pub const VK_MEDIA_PREV_TRACK: i32 = 0xB1;
/// Stop Media key
pub const VK_MEDIA_STOP: i32 = 0xB2;
/// Play/Pause Media key
pub const VK_MEDIA_PLAY_PAUSE: i32 = 0xB3;
/// Start Mail key
pub const VK_LAUNCH_MAIL: i32 = 0xB4;
/// Select Media key
pub const VK_LAUNCH_MEDIA_SELECT: i32 = 0xB5;
/// Start Application 1 key
pub const VK_LAUNCH_APP1: i32 = 0xB6;
/// Start Application 2 key
pub const VK_LAUNCH_APP2: i32 = 0xB7;
/// Used for miscellaneous characters; it can vary by keyboard.
pub const VK_OEM_1: i32 = 0xBA;
/// For any country/region, the '+' key
pub const VK_OEM_PLUS: i32 = 0xBB;
/// For any country/region, the ',' key
pub const VK_OEM_COMMA: i32 = 0xBC;
/// For any country/region, the '-' key
pub const VK_OEM_MINUS: i32 = 0xBD;
/// For any country/region, the '.' key
pub const VK_OEM_PERIOD: i32 = 0xBE;
/// Used for miscellaneous characters; it can vary by keyboard.
pub const VK_OEM_2: i32 = 0xBF;
/// Used for miscellaneous characters; it can vary by keyboard.
pub const VK_OEM_3: i32 = 0xC0;
/// Used for miscellaneous characters; it can vary by keyboard.
pub const VK_OEM_4: i32 = 0xDB;
/// Used for miscellaneous characters; it can vary by keyboard.
pub const VK_OEM_5: i32 = 0xDC;
/// Used for miscellaneous characters; it can vary by keyboard.
pub const VK_OEM_6: i32 = 0xDD;
/// Used for miscellaneous characters; it can vary by keyboard.
pub const VK_OEM_7: i32 = 0xDE;
/// Used for miscellaneous characters; it can vary by keyboard.
pub const VK_OEM_8: i32 = 0xDF;
/// Either the angle bracket key or the backslash key on the RT 102-key keyboard
pub const VK_OEM_102: i32 = 0xE2;
/// IME PROCESS key
pub const VK_PROCESSKEY: i32 = 0xE5;
/// Used to pass Unicode characters as if they were keystrokes.
pub const VK_PACKET: i32 = 0xE7;
/// Attn key
pub const VK_ATTN: i32 = 0xF6;
/// CrSel key
pub const VK_CRSEL: i32 = 0xF7;
/// ExSel key
pub const VK_EXSEL: i32 = 0xF8;
/// Erase EOF key
pub const VK_EREOF: i32 = 0xF9;
/// Play key
pub const VK_PLAY: i32 = 0xFA;
/// Zoom key
pub const VK_ZOOM: i32 = 0xFB;
/// PA1 key
pub const VK_PA1: i32 = 0xFD;
/// Clear key
pub const VK_OEM_CLEAR: i32 = 0xFE;

