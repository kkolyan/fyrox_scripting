---@meta
-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
-- More about Lua annotations format:
-- https://luals.github.io/wiki/annotations
-- https://github.com/LuaLS/lua-language-server/wiki/Annotations
--
-- This file is auto-generated, do not edit it manually.

---@diagnostic disable: missing-return, lowercase-global, missing-fields

-----------------------------------------------------------
------ fyrox_lite::lite_input::LiteKeyCode
-----------------------------------------------------------

--- <p>Basically a copy of <code>winit</code>’s <a href="https://docs.rs/winit/0.29.15/i686-pc-windows-msvc/winit/keyboard/enum.KeyCode.html"><code>KeyCode</code></a>, which is mostly inspired by UI Events Specification’s <a href="https://w3c.github.io/uievents-code/#code-value-tables"><code>KeyboardEvent.code</code></a>.</p>
---@class KeyCode_static
KeyCode = {}

--- <p><kbd>`</kbd> on a US keyboard. This is also called a backtick or grave.
--- This is the <kbd>半角</kbd>/<kbd>全角</kbd>/<kbd>漢字</kbd>
--- (hankaku/zenkaku/kanji) key on Japanese keyboards</p>
---@type KeyCode
KeyCode.Backquote = {}

--- <p>Used for both the US <kbd>\</kbd> (on the 101-key layout) and also for the key
--- located between the <kbd>“</kbd> and <kbd>Enter</kbd> keys on row C of the 102-,
--- 104- and 106-key layouts.
--- Labeled <kbd>#</kbd> on a UK (102) keyboard.</p>
---@type KeyCode
KeyCode.Backslash = {}

--- <p><kbd>[</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.BracketLeft = {}

--- <p><kbd>]</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.BracketRight = {}

--- <p><kbd>,</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.Comma = {}

--- <p><kbd>0</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.Digit0 = {}

--- <p><kbd>1</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.Digit1 = {}

--- <p><kbd>2</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.Digit2 = {}

--- <p><kbd>3</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.Digit3 = {}

--- <p><kbd>4</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.Digit4 = {}

--- <p><kbd>5</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.Digit5 = {}

--- <p><kbd>6</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.Digit6 = {}

--- <p><kbd>7</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.Digit7 = {}

--- <p><kbd>8</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.Digit8 = {}

--- <p><kbd>9</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.Digit9 = {}

--- <p><kbd>=</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.Equal = {}

--- <p>Located between the left <kbd>Shift</kbd> and <kbd>Z</kbd> keys.
--- Labeled <kbd>\</kbd> on a UK keyboard.</p>
---@type KeyCode
KeyCode.IntlBackslash = {}

--- <p>Located between the <kbd>/</kbd> and right <kbd>Shift</kbd> keys.
--- Labeled <kbd>\</kbd> (ro) on a Japanese keyboard.</p>
---@type KeyCode
KeyCode.IntlRo = {}

--- <p>Located between the <kbd>=</kbd> and <kbd>Backspace</kbd> keys.
--- Labeled <kbd>¥</kbd> (yen) on a Japanese keyboard. <kbd>\</kbd> on a
--- Russian keyboard.</p>
---@type KeyCode
KeyCode.IntlYen = {}

--- <p><kbd>a</kbd> on a US keyboard.
--- Labeled <kbd>q</kbd> on an AZERTY (e.g., French) keyboard.</p>
---@type KeyCode
KeyCode.A = {}

--- <p><kbd>b</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.B = {}

--- <p><kbd>c</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.C = {}

--- <p><kbd>d</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.D = {}

--- <p><kbd>e</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.E = {}

--- <p><kbd>f</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.F = {}

--- <p><kbd>g</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.G = {}

--- <p><kbd>h</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.H = {}

--- <p><kbd>i</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.I = {}

--- <p><kbd>j</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.J = {}

--- <p><kbd>k</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.K = {}

--- <p><kbd>l</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.L = {}

--- <p><kbd>m</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.M = {}

--- <p><kbd>n</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.N = {}

--- <p><kbd>o</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.O = {}

--- <p><kbd>p</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.P = {}

--- <p><kbd>q</kbd> on a US keyboard.
--- Labeled <kbd>a</kbd> on an AZERTY (e.g., French) keyboard.</p>
---@type KeyCode
KeyCode.Q = {}

--- <p><kbd>r</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.R = {}

--- <p><kbd>s</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.S = {}

--- <p><kbd>t</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.T = {}

--- <p><kbd>u</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.U = {}

--- <p><kbd>v</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.V = {}

--- <p><kbd>w</kbd> on a US keyboard.
--- Labeled <kbd>z</kbd> on an AZERTY (e.g., French) keyboard.</p>
---@type KeyCode
KeyCode.W = {}

--- <p><kbd>x</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.X = {}

--- <p><kbd>y</kbd> on a US keyboard.
--- Labeled <kbd>z</kbd> on a QWERTZ (e.g., German) keyboard.</p>
---@type KeyCode
KeyCode.Y = {}

--- <p><kbd>z</kbd> on a US keyboard.
--- Labeled <kbd>w</kbd> on an AZERTY (e.g., French) keyboard, and <kbd>y</kbd> on a
--- QWERTZ (e.g., German) keyboard.</p>
---@type KeyCode
KeyCode.Z = {}

--- <p><kbd>-</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.Minus = {}

--- <p><kbd>.</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.Period = {}

--- <p><kbd>’</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.Quote = {}

--- <p><kbd>;</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.Semicolon = {}

--- <p><kbd>/</kbd> on a US keyboard.</p>
---@type KeyCode
KeyCode.Slash = {}

--- <p><kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>.</p>
---@type KeyCode
KeyCode.AltLeft = {}

--- <p><kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>.
--- This is labeled <kbd>AltGr</kbd> on many keyboard layouts.</p>
---@type KeyCode
KeyCode.AltRight = {}

--- <p><kbd>Backspace</kbd> or <kbd>⌫</kbd>.
--- Labeled <kbd>Delete</kbd> on Apple keyboards.</p>
---@type KeyCode
KeyCode.Backspace = {}

--- <p><kbd>CapsLock</kbd> or <kbd>⇪</kbd></p>
---@type KeyCode
KeyCode.CapsLock = {}

--- <p>The application context menu key, which is typically found between the right
--- <kbd>Super</kbd> key and the right <kbd>Control</kbd> key.</p>
---@type KeyCode
KeyCode.ContextMenu = {}

--- <p><kbd>Control</kbd> or <kbd>⌃</kbd></p>
---@type KeyCode
KeyCode.ControlLeft = {}

--- <p><kbd>Control</kbd> or <kbd>⌃</kbd></p>
---@type KeyCode
KeyCode.ControlRight = {}

--- <p><kbd>Enter</kbd> or <kbd>↵</kbd>. Labeled <kbd>Return</kbd> on Apple keyboards.</p>
---@type KeyCode
KeyCode.Enter = {}

--- <p>The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key.</p>
---@type KeyCode
KeyCode.SuperLeft = {}

--- <p>The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key.</p>
---@type KeyCode
KeyCode.SuperRight = {}

--- <p><kbd>Shift</kbd> or <kbd>⇧</kbd></p>
---@type KeyCode
KeyCode.ShiftLeft = {}

--- <p><kbd>Shift</kbd> or <kbd>⇧</kbd></p>
---@type KeyCode
KeyCode.ShiftRight = {}

--- <p><kbd> </kbd> (space)</p>
---@type KeyCode
KeyCode.Space = {}

--- <p><kbd>Tab</kbd> or <kbd>⇥</kbd></p>
---@type KeyCode
KeyCode.Tab = {}

--- <p>Japanese: <kbd>変</kbd> (henkan)</p>
---@type KeyCode
KeyCode.Convert = {}

--- <p>Japanese: <kbd>カタカナ</kbd>/<kbd>ひらがな</kbd>/<kbd>ローマ字</kbd> (katakana/hiragana/romaji)</p>
---@type KeyCode
KeyCode.KanaMode = {}

--- <p>Korean: HangulMode <kbd>한/영</kbd> (han/yeong)</p>
--- <p>Japanese (Mac keyboard): <kbd>か</kbd> (kana)</p>
---@type KeyCode
KeyCode.Lang1 = {}

--- <p>Korean: Hanja <kbd>한</kbd> (hanja)</p>
--- <p>Japanese (Mac keyboard): <kbd>英</kbd> (eisu)</p>
---@type KeyCode
KeyCode.Lang2 = {}

--- <p>Japanese (word-processing keyboard): Katakana</p>
---@type KeyCode
KeyCode.Lang3 = {}

--- <p>Japanese (word-processing keyboard): Hiragana</p>
---@type KeyCode
KeyCode.Lang4 = {}

--- <p>Japanese (word-processing keyboard): Zenkaku/Hankaku</p>
---@type KeyCode
KeyCode.Lang5 = {}

--- <p>Japanese: <kbd>無変換</kbd> (muhenkan)</p>
---@type KeyCode
KeyCode.NonConvert = {}

--- <p><kbd>⌦</kbd>. The forward delete key.
--- Note that on Apple keyboards, the key labelled <kbd>Delete</kbd> on the main part of
--- the keyboard is encoded as <a href="Self::Backspace"><code>Backspace</code></a>.</p>
---@type KeyCode
KeyCode.Delete = {}

--- <p><kbd>Page Down</kbd>, <kbd>End</kbd>, or <kbd>↘</kbd></p>
---@type KeyCode
KeyCode.End = {}

--- <p><kbd>Help</kbd>. Not present on standard PC keyboards.</p>
---@type KeyCode
KeyCode.Help = {}

--- <p><kbd>Home</kbd> or <kbd>↖</kbd></p>
---@type KeyCode
KeyCode.Home = {}

--- <p><kbd>Insert</kbd> or <kbd>Ins</kbd>. Not present on Apple keyboards.</p>
---@type KeyCode
KeyCode.Insert = {}

--- <p><kbd>Page Down</kbd>, <kbd>PgDn</kbd>, or <kbd>⇟</kbd></p>
---@type KeyCode
KeyCode.PageDown = {}

--- <p><kbd>Page Up</kbd>, <kbd>PgUp</kbd>, or <kbd>⇞</kbd></p>
---@type KeyCode
KeyCode.PageUp = {}

--- <p><kbd>↓</kbd></p>
---@type KeyCode
KeyCode.ArrowDown = {}

--- <p><kbd>←</kbd></p>
---@type KeyCode
KeyCode.ArrowLeft = {}

--- <p><kbd>→</kbd></p>
---@type KeyCode
KeyCode.ArrowRight = {}

--- <p><kbd>↑</kbd></p>
---@type KeyCode
KeyCode.ArrowUp = {}

--- <p>On the Mac, this is used for the numpad <kbd>Clear</kbd> key.</p>
---@type KeyCode
KeyCode.NumLock = {}

--- <p><kbd>0 Ins</kbd> on a keyboard. <kbd>0</kbd> on a phone or remote control</p>
---@type KeyCode
KeyCode.Numpad0 = {}

--- <p><kbd>1 End</kbd> on a keyboard. <kbd>1</kbd> or <kbd>1 QZ</kbd> on a phone or remote control</p>
---@type KeyCode
KeyCode.Numpad1 = {}

--- <p><kbd>2 ↓</kbd> on a keyboard. <kbd>2 ABC</kbd> on a phone or remote control</p>
---@type KeyCode
KeyCode.Numpad2 = {}

--- <p><kbd>3 PgDn</kbd> on a keyboard. <kbd>3 DEF</kbd> on a phone or remote control</p>
---@type KeyCode
KeyCode.Numpad3 = {}

--- <p><kbd>4 ←</kbd> on a keyboard. <kbd>4 GHI</kbd> on a phone or remote control</p>
---@type KeyCode
KeyCode.Numpad4 = {}

--- <p><kbd>5</kbd> on a keyboard. <kbd>5 JKL</kbd> on a phone or remote control</p>
---@type KeyCode
KeyCode.Numpad5 = {}

--- <p><kbd>6 →</kbd> on a keyboard. <kbd>6 MNO</kbd> on a phone or remote control</p>
---@type KeyCode
KeyCode.Numpad6 = {}

--- <p><kbd>7 Home</kbd> on a keyboard. <kbd>7 PQRS</kbd> or <kbd>7 PRS</kbd> on a phone
--- or remote control</p>
---@type KeyCode
KeyCode.Numpad7 = {}

--- <p><kbd>8 ↑</kbd> on a keyboard. <kbd>8 TUV</kbd> on a phone or remote control</p>
---@type KeyCode
KeyCode.Numpad8 = {}

--- <p><kbd>9 PgUp</kbd> on a keyboard. <kbd>9 WXYZ</kbd> or <kbd>9 WXY</kbd> on a phone
--- or remote control</p>
---@type KeyCode
KeyCode.Numpad9 = {}

--- <p><kbd>+</kbd></p>
---@type KeyCode
KeyCode.NumpadAdd = {}

--- <p>Found on the Microsoft Natural Keyboard.</p>
---@type KeyCode
KeyCode.NumpadBackspace = {}

--- <p><kbd>C</kbd> or <kbd>A</kbd> (All Clear). Also for use with numpads that have a
--- <kbd>Clear</kbd> key that is separate from the <kbd>NumLock</kbd> key. On the Mac, the
--- numpad <kbd>Clear</kbd> key is encoded as <a href="Self::NumLock"><code>NumLock</code></a>.</p>
---@type KeyCode
KeyCode.NumpadClear = {}

--- <p><kbd>C</kbd> (Clear Entry)</p>
---@type KeyCode
KeyCode.NumpadClearEntry = {}

--- <p><kbd>,</kbd> (thousands separator). For locales where the thousands separator
--- is a “.” (e.g., Brazil), this key may generate a <kbd>.</kbd>.</p>
---@type KeyCode
KeyCode.NumpadComma = {}

--- <p><kbd>. Del</kbd>. For locales where the decimal separator is “,” (e.g.,
--- Brazil), this key may generate a <kbd>,</kbd>.</p>
---@type KeyCode
KeyCode.NumpadDecimal = {}

--- <p><kbd>/</kbd></p>
---@type KeyCode
KeyCode.NumpadDivide = {}

---@type KeyCode
KeyCode.NumpadEnter = {}

--- <p><kbd>=</kbd></p>
---@type KeyCode
KeyCode.NumpadEqual = {}

--- <p><kbd>#</kbd> on a phone or remote control device. This key is typically found
--- below the <kbd>9</kbd> key and to the right of the <kbd>0</kbd> key.</p>
---@type KeyCode
KeyCode.NumpadHash = {}

--- <p><kbd>M</kbd> Add current entry to the value stored in memory.</p>
---@type KeyCode
KeyCode.NumpadMemoryAdd = {}

--- <p><kbd>M</kbd> Clear the value stored in memory.</p>
---@type KeyCode
KeyCode.NumpadMemoryClear = {}

--- <p><kbd>M</kbd> Replace the current entry with the value stored in memory.</p>
---@type KeyCode
KeyCode.NumpadMemoryRecall = {}

--- <p><kbd>M</kbd> Replace the value stored in memory with the current entry.</p>
---@type KeyCode
KeyCode.NumpadMemoryStore = {}

--- <p><kbd>M</kbd> Subtract current entry from the value stored in memory.</p>
---@type KeyCode
KeyCode.NumpadMemorySubtract = {}

--- <p><kbd>*</kbd> on a keyboard. For use with numpads that provide mathematical
--- operations (<kbd>+</kbd>, <kbd>-</kbd> <kbd>*</kbd> and <kbd>/</kbd>).</p>
--- <p>Use <code>NumpadStar</code> for the <kbd>*</kbd> key on phones and remote controls.</p>
---@type KeyCode
KeyCode.NumpadMultiply = {}

--- <p><kbd>(</kbd> Found on the Microsoft Natural Keyboard.</p>
---@type KeyCode
KeyCode.NumpadParenLeft = {}

--- <p><kbd>)</kbd> Found on the Microsoft Natural Keyboard.</p>
---@type KeyCode
KeyCode.NumpadParenRight = {}

--- <p><kbd>*</kbd> on a phone or remote control device.</p>
--- <p>This key is typically found below the <kbd>7</kbd> key and to the left of
--- the <kbd>0</kbd> key.</p>
--- <p>Use <kbd>“NumpadMultiply”</kbd> for the <kbd>*</kbd> key on
--- numeric keypads.</p>
---@type KeyCode
KeyCode.NumpadStar = {}

--- <p><kbd>-</kbd></p>
---@type KeyCode
KeyCode.NumpadSubtract = {}

--- <p><kbd>Esc</kbd> or <kbd>⎋</kbd></p>
---@type KeyCode
KeyCode.Escape = {}

--- <p><kbd>Fn</kbd> This is typically a hardware key that does not generate a separate code.</p>
---@type KeyCode
KeyCode.Fn = {}

--- <p><kbd>FLock</kbd> or <kbd>FnLock</kbd>. Function Lock key. Found on the Microsoft
--- Natural Keyboard.</p>
---@type KeyCode
KeyCode.FnLock = {}

--- <p><kbd>PrtScr SysRq</kbd> or <kbd>Print Screen</kbd></p>
---@type KeyCode
KeyCode.PrintScreen = {}

--- <p><kbd>Scroll Lock</kbd></p>
---@type KeyCode
KeyCode.ScrollLock = {}

--- <p><kbd>Pause Break</kbd></p>
---@type KeyCode
KeyCode.Pause = {}

--- <p>Some laptops place this key to the left of the <kbd>↑</kbd> key.</p>
--- <p>This also the “back” button (triangle) on Android.</p>
---@type KeyCode
KeyCode.BrowserBack = {}

---@type KeyCode
KeyCode.BrowserFavorites = {}

--- <p>Some laptops place this key to the right of the <kbd>↑</kbd> key.</p>
---@type KeyCode
KeyCode.BrowserForward = {}

--- <p>The “home” button on Android.</p>
---@type KeyCode
KeyCode.BrowserHome = {}

---@type KeyCode
KeyCode.BrowserRefresh = {}

---@type KeyCode
KeyCode.BrowserSearch = {}

---@type KeyCode
KeyCode.BrowserStop = {}

--- <p><kbd>Eject</kbd> or <kbd>⏏</kbd>. This key is placed in the function section on some Apple
--- keyboards.</p>
---@type KeyCode
KeyCode.Eject = {}

--- <p>Sometimes labelled <kbd>My Computer</kbd> on the keyboard</p>
---@type KeyCode
KeyCode.LaunchApp1 = {}

--- <p>Sometimes labelled <kbd>Calculator</kbd> on the keyboard</p>
---@type KeyCode
KeyCode.LaunchApp2 = {}

---@type KeyCode
KeyCode.LaunchMail = {}

---@type KeyCode
KeyCode.MediaPlayPause = {}

---@type KeyCode
KeyCode.MediaSelect = {}

---@type KeyCode
KeyCode.MediaStop = {}

---@type KeyCode
KeyCode.MediaTrackNext = {}

---@type KeyCode
KeyCode.MediaTrackPrevious = {}

--- <p>This key is placed in the function section on some Apple keyboards, replacing the
--- <kbd>Eject</kbd> key.</p>
---@type KeyCode
KeyCode.Power = {}

---@type KeyCode
KeyCode.Sleep = {}

---@type KeyCode
KeyCode.AudioVolumeDown = {}

---@type KeyCode
KeyCode.AudioVolumeMute = {}

---@type KeyCode
KeyCode.AudioVolumeUp = {}

---@type KeyCode
KeyCode.WakeUp = {}

---@type KeyCode
KeyCode.Meta = {}

---@type KeyCode
KeyCode.Hyper = {}

---@type KeyCode
KeyCode.Turbo = {}

---@type KeyCode
KeyCode.Abort = {}

---@type KeyCode
KeyCode.Resume = {}

---@type KeyCode
KeyCode.Suspend = {}

--- <p>Found on Sun’s USB keyboard.</p>
---@type KeyCode
KeyCode.Again = {}

--- <p>Found on Sun’s USB keyboard.</p>
---@type KeyCode
KeyCode.Copy = {}

--- <p>Found on Sun’s USB keyboard.</p>
---@type KeyCode
KeyCode.Cut = {}

--- <p>Found on Sun’s USB keyboard.</p>
---@type KeyCode
KeyCode.Find = {}

--- <p>Found on Sun’s USB keyboard.</p>
---@type KeyCode
KeyCode.Open = {}

--- <p>Found on Sun’s USB keyboard.</p>
---@type KeyCode
KeyCode.Paste = {}

--- <p>Found on Sun’s USB keyboard.</p>
---@type KeyCode
KeyCode.Props = {}

--- <p>Found on Sun’s USB keyboard.</p>
---@type KeyCode
KeyCode.Select = {}

--- <p>Found on Sun’s USB keyboard.</p>
---@type KeyCode
KeyCode.Undo = {}

--- <p>Use for dedicated <kbd>ひらがな</kbd> key found on some Japanese word processing keyboards.</p>
---@type KeyCode
KeyCode.Hiragana = {}

--- <p>Use for dedicated <kbd>カタカナ</kbd> key found on some Japanese word processing keyboards.</p>
---@type KeyCode
KeyCode.Katakana = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F1 = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F2 = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F3 = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F4 = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F5 = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F6 = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F7 = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F8 = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F9 = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F10 = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F11 = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F12 = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F13 = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F14 = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F15 = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F16 = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F17 = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F18 = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F19 = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F20 = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F21 = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F22 = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F23 = {}

--- <p>General-purpose function key.
--- Usually found at the top of the keyboard.</p>
---@type KeyCode
KeyCode.F24 = {}

--- <p>General-purpose function key.</p>
---@type KeyCode
KeyCode.F25 = {}

--- <p>General-purpose function key.</p>
---@type KeyCode
KeyCode.F26 = {}

--- <p>General-purpose function key.</p>
---@type KeyCode
KeyCode.F27 = {}

--- <p>General-purpose function key.</p>
---@type KeyCode
KeyCode.F28 = {}

--- <p>General-purpose function key.</p>
---@type KeyCode
KeyCode.F29 = {}

--- <p>General-purpose function key.</p>
---@type KeyCode
KeyCode.F30 = {}

--- <p>General-purpose function key.</p>
---@type KeyCode
KeyCode.F31 = {}

--- <p>General-purpose function key.</p>
---@type KeyCode
KeyCode.F32 = {}

--- <p>General-purpose function key.</p>
---@type KeyCode
KeyCode.F33 = {}

--- <p>General-purpose function key.</p>
---@type KeyCode
KeyCode.F34 = {}

--- <p>General-purpose function key.</p>
---@type KeyCode
KeyCode.F35 = {}
KeyCode = {}

---@class KeyCode
KeyCode_instance = {}

