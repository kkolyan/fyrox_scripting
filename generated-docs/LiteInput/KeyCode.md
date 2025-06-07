# KeyCode
enum in [FyroxLite](../README.md).[LiteInput](README.md)
## Description
Basically a copy of `winit`'s [`KeyCode`], which is mostly inspired by UI Events Specification’s [`KeyboardEvent.code`].

[`KeyCode`]: https://docs.rs/winit/0.29.15/i686-pc-windows-msvc/winit/keyboard/enum.KeyCode.html
[`KeyboardEvent.code`]: https://w3c.github.io/uievents-code/#code-value-tables
## Properties
| Property | Description |
|---|---|
| `Backquote` | <kbd>`</kbd> on a US keyboard. This is also called a backtick or grave.
This is the <kbd>半角</kbd>/<kbd>全角</kbd>/<kbd>漢字</kbd>
(hankaku/zenkaku/kanji) key on Japanese keyboards |
| `Backslash` | Used for both the US <kbd>\\</kbd> (on the 101-key layout) and also for the key
located between the <kbd>"</kbd> and <kbd>Enter</kbd> keys on row C of the 102-,
104- and 106-key layouts.
Labeled <kbd>#</kbd> on a UK (102) keyboard. |
| `BracketLeft` | <kbd>[</kbd> on a US keyboard. |
| `BracketRight` | <kbd>]</kbd> on a US keyboard. |
| `Comma` | <kbd>,</kbd> on a US keyboard. |
| `Digit0` | <kbd>0</kbd> on a US keyboard. |
| `Digit1` | <kbd>1</kbd> on a US keyboard. |
| `Digit2` | <kbd>2</kbd> on a US keyboard. |
| `Digit3` | <kbd>3</kbd> on a US keyboard. |
| `Digit4` | <kbd>4</kbd> on a US keyboard. |
| `Digit5` | <kbd>5</kbd> on a US keyboard. |
| `Digit6` | <kbd>6</kbd> on a US keyboard. |
| `Digit7` | <kbd>7</kbd> on a US keyboard. |
| `Digit8` | <kbd>8</kbd> on a US keyboard. |
| `Digit9` | <kbd>9</kbd> on a US keyboard. |
| `Equal` | <kbd>=</kbd> on a US keyboard. |
| `IntlBackslash` | Located between the left <kbd>Shift</kbd> and <kbd>Z</kbd> keys.
Labeled <kbd>\\</kbd> on a UK keyboard. |
| `IntlRo` | Located between the <kbd>/</kbd> and right <kbd>Shift</kbd> keys.
Labeled <kbd>\\</kbd> (ro) on a Japanese keyboard. |
| `IntlYen` | Located between the <kbd>=</kbd> and <kbd>Backspace</kbd> keys.
Labeled <kbd>¥</kbd> (yen) on a Japanese keyboard. <kbd>\\</kbd> on a
Russian keyboard. |
| `A` | <kbd>a</kbd> on a US keyboard.
Labeled <kbd>q</kbd> on an AZERTY (e.g., French) keyboard. |
| `B` | <kbd>b</kbd> on a US keyboard. |
| `C` | <kbd>c</kbd> on a US keyboard. |
| `D` | <kbd>d</kbd> on a US keyboard. |
| `E` | <kbd>e</kbd> on a US keyboard. |
| `F` | <kbd>f</kbd> on a US keyboard. |
| `G` | <kbd>g</kbd> on a US keyboard. |
| `H` | <kbd>h</kbd> on a US keyboard. |
| `I` | <kbd>i</kbd> on a US keyboard. |
| `J` | <kbd>j</kbd> on a US keyboard. |
| `K` | <kbd>k</kbd> on a US keyboard. |
| `L` | <kbd>l</kbd> on a US keyboard. |
| `M` | <kbd>m</kbd> on a US keyboard. |
| `N` | <kbd>n</kbd> on a US keyboard. |
| `O` | <kbd>o</kbd> on a US keyboard. |
| `P` | <kbd>p</kbd> on a US keyboard. |
| `Q` | <kbd>q</kbd> on a US keyboard.
Labeled <kbd>a</kbd> on an AZERTY (e.g., French) keyboard. |
| `R` | <kbd>r</kbd> on a US keyboard. |
| `S` | <kbd>s</kbd> on a US keyboard. |
| `T` | <kbd>t</kbd> on a US keyboard. |
| `U` | <kbd>u</kbd> on a US keyboard. |
| `V` | <kbd>v</kbd> on a US keyboard. |
| `W` | <kbd>w</kbd> on a US keyboard.
Labeled <kbd>z</kbd> on an AZERTY (e.g., French) keyboard. |
| `X` | <kbd>x</kbd> on a US keyboard. |
| `Y` | <kbd>y</kbd> on a US keyboard.
Labeled <kbd>z</kbd> on a QWERTZ (e.g., German) keyboard. |
| `Z` | <kbd>z</kbd> on a US keyboard.
Labeled <kbd>w</kbd> on an AZERTY (e.g., French) keyboard, and <kbd>y</kbd> on a
QWERTZ (e.g., German) keyboard. |
| `Minus` | <kbd>-</kbd> on a US keyboard. |
| `Period` | <kbd>.</kbd> on a US keyboard. |
| `Quote` | <kbd>'</kbd> on a US keyboard. |
| `Semicolon` | <kbd>;</kbd> on a US keyboard. |
| `Slash` | <kbd>/</kbd> on a US keyboard. |
| `AltLeft` | <kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>. |
| `AltRight` | <kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>.
This is labeled <kbd>AltGr</kbd> on many keyboard layouts. |
| `Backspace` | <kbd>Backspace</kbd> or <kbd>⌫</kbd>.
Labeled <kbd>Delete</kbd> on Apple keyboards. |
| `CapsLock` | <kbd>CapsLock</kbd> or <kbd>⇪</kbd> |
| `ContextMenu` | The application context menu key, which is typically found between the right
<kbd>Super</kbd> key and the right <kbd>Control</kbd> key. |
| `ControlLeft` | <kbd>Control</kbd> or <kbd>⌃</kbd> |
| `ControlRight` | <kbd>Control</kbd> or <kbd>⌃</kbd> |
| `Enter` | <kbd>Enter</kbd> or <kbd>↵</kbd>. Labeled <kbd>Return</kbd> on Apple keyboards. |
| `SuperLeft` | The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key. |
| `SuperRight` | The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key. |
| `ShiftLeft` | <kbd>Shift</kbd> or <kbd>⇧</kbd> |
| `ShiftRight` | <kbd>Shift</kbd> or <kbd>⇧</kbd> |
| `Space` | <kbd> </kbd> (space) |
| `Tab` | <kbd>Tab</kbd> or <kbd>⇥</kbd> |
| `Convert` | Japanese: <kbd>変</kbd> (henkan) |
| `KanaMode` | Japanese: <kbd>カタカナ</kbd>/<kbd>ひらがな</kbd>/<kbd>ローマ字</kbd> (katakana/hiragana/romaji) |
| `Lang1` | Korean: HangulMode <kbd>한/영</kbd> (han/yeong)

Japanese (Mac keyboard): <kbd>か</kbd> (kana) |
| `Lang2` | Korean: Hanja <kbd>한</kbd> (hanja)

Japanese (Mac keyboard): <kbd>英</kbd> (eisu) |
| `Lang3` | Japanese (word-processing keyboard): Katakana |
| `Lang4` | Japanese (word-processing keyboard): Hiragana |
| `Lang5` | Japanese (word-processing keyboard): Zenkaku/Hankaku |
| `NonConvert` | Japanese: <kbd>無変換</kbd> (muhenkan) |
| `Delete` | <kbd>⌦</kbd>. The forward delete key.
Note that on Apple keyboards, the key labelled <kbd>Delete</kbd> on the main part of
the keyboard is encoded as [`Backspace`].

[`Backspace`]: Self::Backspace |
| `End` | <kbd>Page Down</kbd>, <kbd>End</kbd>, or <kbd>↘</kbd> |
| `Help` | <kbd>Help</kbd>. Not present on standard PC keyboards. |
| `Home` | <kbd>Home</kbd> or <kbd>↖</kbd> |
| `Insert` | <kbd>Insert</kbd> or <kbd>Ins</kbd>. Not present on Apple keyboards. |
| `PageDown` | <kbd>Page Down</kbd>, <kbd>PgDn</kbd>, or <kbd>⇟</kbd> |
| `PageUp` | <kbd>Page Up</kbd>, <kbd>PgUp</kbd>, or <kbd>⇞</kbd> |
| `ArrowDown` | <kbd>↓</kbd> |
| `ArrowLeft` | <kbd>←</kbd> |
| `ArrowRight` | <kbd>→</kbd> |
| `ArrowUp` | <kbd>↑</kbd> |
| `NumLock` | On the Mac, this is used for the numpad <kbd>Clear</kbd> key. |
| `Numpad0` | <kbd>0 Ins</kbd> on a keyboard. <kbd>0</kbd> on a phone or remote control |
| `Numpad1` | <kbd>1 End</kbd> on a keyboard. <kbd>1</kbd> or <kbd>1 QZ</kbd> on a phone or remote control |
| `Numpad2` | <kbd>2 ↓</kbd> on a keyboard. <kbd>2 ABC</kbd> on a phone or remote control |
| `Numpad3` | <kbd>3 PgDn</kbd> on a keyboard. <kbd>3 DEF</kbd> on a phone or remote control |
| `Numpad4` | <kbd>4 ←</kbd> on a keyboard. <kbd>4 GHI</kbd> on a phone or remote control |
| `Numpad5` | <kbd>5</kbd> on a keyboard. <kbd>5 JKL</kbd> on a phone or remote control |
| `Numpad6` | <kbd>6 →</kbd> on a keyboard. <kbd>6 MNO</kbd> on a phone or remote control |
| `Numpad7` | <kbd>7 Home</kbd> on a keyboard. <kbd>7 PQRS</kbd> or <kbd>7 PRS</kbd> on a phone
or remote control |
| `Numpad8` | <kbd>8 ↑</kbd> on a keyboard. <kbd>8 TUV</kbd> on a phone or remote control |
| `Numpad9` | <kbd>9 PgUp</kbd> on a keyboard. <kbd>9 WXYZ</kbd> or <kbd>9 WXY</kbd> on a phone
or remote control |
| `NumpadAdd` | <kbd>+</kbd> |
| `NumpadBackspace` | Found on the Microsoft Natural Keyboard. |
| `NumpadClear` | <kbd>C</kbd> or <kbd>A</kbd> (All Clear). Also for use with numpads that have a
<kbd>Clear</kbd> key that is separate from the <kbd>NumLock</kbd> key. On the Mac, the
numpad <kbd>Clear</kbd> key is encoded as [`NumLock`].

[`NumLock`]: Self::NumLock |
| `NumpadClearEntry` | <kbd>C</kbd> (Clear Entry) |
| `NumpadComma` | <kbd>,</kbd> (thousands separator). For locales where the thousands separator
is a "." (e.g., Brazil), this key may generate a <kbd>.</kbd>. |
| `NumpadDecimal` | <kbd>. Del</kbd>. For locales where the decimal separator is "," (e.g.,
Brazil), this key may generate a <kbd>,</kbd>. |
| `NumpadDivide` | <kbd>/</kbd> |
| `NumpadEnter` |  |
| `NumpadEqual` | <kbd>=</kbd> |
| `NumpadHash` | <kbd>#</kbd> on a phone or remote control device. This key is typically found
below the <kbd>9</kbd> key and to the right of the <kbd>0</kbd> key. |
| `NumpadMemoryAdd` | <kbd>M</kbd> Add current entry to the value stored in memory. |
| `NumpadMemoryClear` | <kbd>M</kbd> Clear the value stored in memory. |
| `NumpadMemoryRecall` | <kbd>M</kbd> Replace the current entry with the value stored in memory. |
| `NumpadMemoryStore` | <kbd>M</kbd> Replace the value stored in memory with the current entry. |
| `NumpadMemorySubtract` | <kbd>M</kbd> Subtract current entry from the value stored in memory. |
| `NumpadMultiply` | <kbd>*</kbd> on a keyboard. For use with numpads that provide mathematical
operations (<kbd>+</kbd>, <kbd>-</kbd> <kbd>*</kbd> and <kbd>/</kbd>).

Use `NumpadStar` for the <kbd>*</kbd> key on phones and remote controls. |
| `NumpadParenLeft` | <kbd>(</kbd> Found on the Microsoft Natural Keyboard. |
| `NumpadParenRight` | <kbd>)</kbd> Found on the Microsoft Natural Keyboard. |
| `NumpadStar` | <kbd>*</kbd> on a phone or remote control device.

This key is typically found below the <kbd>7</kbd> key and to the left of
the <kbd>0</kbd> key.

Use <kbd>"NumpadMultiply"</kbd> for the <kbd>*</kbd> key on
numeric keypads. |
| `NumpadSubtract` | <kbd>-</kbd> |
| `Escape` | <kbd>Esc</kbd> or <kbd>⎋</kbd> |
| `Fn` | <kbd>Fn</kbd> This is typically a hardware key that does not generate a separate code. |
| `FnLock` | <kbd>FLock</kbd> or <kbd>FnLock</kbd>. Function Lock key. Found on the Microsoft
Natural Keyboard. |
| `PrintScreen` | <kbd>PrtScr SysRq</kbd> or <kbd>Print Screen</kbd> |
| `ScrollLock` | <kbd>Scroll Lock</kbd> |
| `Pause` | <kbd>Pause Break</kbd> |
| `BrowserBack` | Some laptops place this key to the left of the <kbd>↑</kbd> key.

This also the "back" button (triangle) on Android. |
| `BrowserFavorites` |  |
| `BrowserForward` | Some laptops place this key to the right of the <kbd>↑</kbd> key. |
| `BrowserHome` | The "home" button on Android. |
| `BrowserRefresh` |  |
| `BrowserSearch` |  |
| `BrowserStop` |  |
| `Eject` | <kbd>Eject</kbd> or <kbd>⏏</kbd>. This key is placed in the function section on some Apple
keyboards. |
| `LaunchApp1` | Sometimes labelled <kbd>My Computer</kbd> on the keyboard |
| `LaunchApp2` | Sometimes labelled <kbd>Calculator</kbd> on the keyboard |
| `LaunchMail` |  |
| `MediaPlayPause` |  |
| `MediaSelect` |  |
| `MediaStop` |  |
| `MediaTrackNext` |  |
| `MediaTrackPrevious` |  |
| `Power` | This key is placed in the function section on some Apple keyboards, replacing the
<kbd>Eject</kbd> key. |
| `Sleep` |  |
| `AudioVolumeDown` |  |
| `AudioVolumeMute` |  |
| `AudioVolumeUp` |  |
| `WakeUp` |  |
| `Meta` |  |
| `Hyper` |  |
| `Turbo` |  |
| `Abort` |  |
| `Resume` |  |
| `Suspend` |  |
| `Again` | Found on Sun’s USB keyboard. |
| `Copy` | Found on Sun’s USB keyboard. |
| `Cut` | Found on Sun’s USB keyboard. |
| `Find` | Found on Sun’s USB keyboard. |
| `Open` | Found on Sun’s USB keyboard. |
| `Paste` | Found on Sun’s USB keyboard. |
| `Props` | Found on Sun’s USB keyboard. |
| `Select` | Found on Sun’s USB keyboard. |
| `Undo` | Found on Sun’s USB keyboard. |
| `Hiragana` | Use for dedicated <kbd>ひらがな</kbd> key found on some Japanese word processing keyboards. |
| `Katakana` | Use for dedicated <kbd>カタカナ</kbd> key found on some Japanese word processing keyboards. |
| `F1` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F2` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F3` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F4` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F5` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F6` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F7` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F8` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F9` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F10` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F11` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F12` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F13` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F14` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F15` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F16` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F17` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F18` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F19` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F20` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F21` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F22` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F23` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F24` | General-purpose function key.
Usually found at the top of the keyboard. |
| `F25` | General-purpose function key. |
| `F26` | General-purpose function key. |
| `F27` | General-purpose function key. |
| `F28` | General-purpose function key. |
| `F29` | General-purpose function key. |
| `F30` | General-purpose function key. |
| `F31` | General-purpose function key. |
| `F32` | General-purpose function key. |
| `F33` | General-purpose function key. |
| `F34` | General-purpose function key. |
| `F35` | General-purpose function key. |

