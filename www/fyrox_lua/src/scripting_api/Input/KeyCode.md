# KeyCode
enum in [FyroxLite](../../scripting_api.md).[Input](../Input.md)

## Description
<p>Basically a copy of <code>winit</code>’s <a href="https://docs.rs/winit/0.29.15/i686-pc-windows-msvc/winit/keyboard/enum.KeyCode.html"><code>KeyCode</code></a>, which is mostly inspired by UI Events Specification’s <a href="https://w3c.github.io/uievents-code/#code-value-tables"><code>KeyboardEvent.code</code></a>.</p>

## Properties
| Property | Description |
|---|---|
| `Backquote` | <p><kbd>`</kbd> on a US keyboard. This is also called a backtick or grave. This is the <kbd>半角</kbd>/<kbd>全角</kbd>/<kbd>漢字</kbd> (hankaku/zenkaku/kanji) key on Japanese keyboards</p> |
| `Backslash` | <p>Used for both the US <kbd>\\</kbd> (on the 101-key layout) and also for the key located between the <kbd>“</kbd> and <kbd>Enter</kbd> keys on row C of the 102-, 104- and 106-key layouts. Labeled <kbd>#</kbd> on a UK (102) keyboard.</p> |
| `BracketLeft` | <p><kbd>[</kbd> on a US keyboard.</p> |
| `BracketRight` | <p><kbd>]</kbd> on a US keyboard.</p> |
| `Comma` | <p><kbd>,</kbd> on a US keyboard.</p> |
| `Digit0` | <p><kbd>0</kbd> on a US keyboard.</p> |
| `Digit1` | <p><kbd>1</kbd> on a US keyboard.</p> |
| `Digit2` | <p><kbd>2</kbd> on a US keyboard.</p> |
| `Digit3` | <p><kbd>3</kbd> on a US keyboard.</p> |
| `Digit4` | <p><kbd>4</kbd> on a US keyboard.</p> |
| `Digit5` | <p><kbd>5</kbd> on a US keyboard.</p> |
| `Digit6` | <p><kbd>6</kbd> on a US keyboard.</p> |
| `Digit7` | <p><kbd>7</kbd> on a US keyboard.</p> |
| `Digit8` | <p><kbd>8</kbd> on a US keyboard.</p> |
| `Digit9` | <p><kbd>9</kbd> on a US keyboard.</p> |
| `Equal` | <p><kbd>=</kbd> on a US keyboard.</p> |
| `IntlBackslash` | <p>Located between the left <kbd>Shift</kbd> and <kbd>Z</kbd> keys. Labeled <kbd>\\</kbd> on a UK keyboard.</p> |
| `IntlRo` | <p>Located between the <kbd>/</kbd> and right <kbd>Shift</kbd> keys. Labeled <kbd>\\</kbd> (ro) on a Japanese keyboard.</p> |
| `IntlYen` | <p>Located between the <kbd>=</kbd> and <kbd>Backspace</kbd> keys. Labeled <kbd>¥</kbd> (yen) on a Japanese keyboard. <kbd>\\</kbd> on a Russian keyboard.</p> |
| `A` | <p><kbd>a</kbd> on a US keyboard. Labeled <kbd>q</kbd> on an AZERTY (e.g., French) keyboard.</p> |
| `B` | <p><kbd>b</kbd> on a US keyboard.</p> |
| `C` | <p><kbd>c</kbd> on a US keyboard.</p> |
| `D` | <p><kbd>d</kbd> on a US keyboard.</p> |
| `E` | <p><kbd>e</kbd> on a US keyboard.</p> |
| `F` | <p><kbd>f</kbd> on a US keyboard.</p> |
| `G` | <p><kbd>g</kbd> on a US keyboard.</p> |
| `H` | <p><kbd>h</kbd> on a US keyboard.</p> |
| `I` | <p><kbd>i</kbd> on a US keyboard.</p> |
| `J` | <p><kbd>j</kbd> on a US keyboard.</p> |
| `K` | <p><kbd>k</kbd> on a US keyboard.</p> |
| `L` | <p><kbd>l</kbd> on a US keyboard.</p> |
| `M` | <p><kbd>m</kbd> on a US keyboard.</p> |
| `N` | <p><kbd>n</kbd> on a US keyboard.</p> |
| `O` | <p><kbd>o</kbd> on a US keyboard.</p> |
| `P` | <p><kbd>p</kbd> on a US keyboard.</p> |
| `Q` | <p><kbd>q</kbd> on a US keyboard. Labeled <kbd>a</kbd> on an AZERTY (e.g., French) keyboard.</p> |
| `R` | <p><kbd>r</kbd> on a US keyboard.</p> |
| `S` | <p><kbd>s</kbd> on a US keyboard.</p> |
| `T` | <p><kbd>t</kbd> on a US keyboard.</p> |
| `U` | <p><kbd>u</kbd> on a US keyboard.</p> |
| `V` | <p><kbd>v</kbd> on a US keyboard.</p> |
| `W` | <p><kbd>w</kbd> on a US keyboard. Labeled <kbd>z</kbd> on an AZERTY (e.g., French) keyboard.</p> |
| `X` | <p><kbd>x</kbd> on a US keyboard.</p> |
| `Y` | <p><kbd>y</kbd> on a US keyboard. Labeled <kbd>z</kbd> on a QWERTZ (e.g., German) keyboard.</p> |
| `Z` | <p><kbd>z</kbd> on a US keyboard. Labeled <kbd>w</kbd> on an AZERTY (e.g., French) keyboard, and <kbd>y</kbd> on a QWERTZ (e.g., German) keyboard.</p> |
| `Minus` | <p><kbd>-</kbd> on a US keyboard.</p> |
| `Period` | <p><kbd>.</kbd> on a US keyboard.</p> |
| `Quote` | <p><kbd>’</kbd> on a US keyboard.</p> |
| `Semicolon` | <p><kbd>;</kbd> on a US keyboard.</p> |
| `Slash` | <p><kbd>/</kbd> on a US keyboard.</p> |
| `AltLeft` | <p><kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>.</p> |
| `AltRight` | <p><kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>. This is labeled <kbd>AltGr</kbd> on many keyboard layouts.</p> |
| `Backspace` | <p><kbd>Backspace</kbd> or <kbd>⌫</kbd>. Labeled <kbd>Delete</kbd> on Apple keyboards.</p> |
| `CapsLock` | <p><kbd>CapsLock</kbd> or <kbd>⇪</kbd></p> |
| `ContextMenu` | <p>The application context menu key, which is typically found between the right <kbd>Super</kbd> key and the right <kbd>Control</kbd> key.</p> |
| `ControlLeft` | <p><kbd>Control</kbd> or <kbd>⌃</kbd></p> |
| `ControlRight` | <p><kbd>Control</kbd> or <kbd>⌃</kbd></p> |
| `Enter` | <p><kbd>Enter</kbd> or <kbd>↵</kbd>. Labeled <kbd>Return</kbd> on Apple keyboards.</p> |
| `SuperLeft` | <p>The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key.</p> |
| `SuperRight` | <p>The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key.</p> |
| `ShiftLeft` | <p><kbd>Shift</kbd> or <kbd>⇧</kbd></p> |
| `ShiftRight` | <p><kbd>Shift</kbd> or <kbd>⇧</kbd></p> |
| `Space` | <p><kbd> </kbd> (space)</p> |
| `Tab` | <p><kbd>Tab</kbd> or <kbd>⇥</kbd></p> |
| `Convert` | <p>Japanese: <kbd>変</kbd> (henkan)</p> |
| `KanaMode` | <p>Japanese: <kbd>カタカナ</kbd>/<kbd>ひらがな</kbd>/<kbd>ローマ字</kbd> (katakana/hiragana/romaji)</p> |
| `Lang1` | <p>Korean: HangulMode <kbd>한/영</kbd> (han/yeong)</p> <p>Japanese (Mac keyboard): <kbd>か</kbd> (kana)</p> |
| `Lang2` | <p>Korean: Hanja <kbd>한</kbd> (hanja)</p> <p>Japanese (Mac keyboard): <kbd>英</kbd> (eisu)</p> |
| `Lang3` | <p>Japanese (word-processing keyboard): Katakana</p> |
| `Lang4` | <p>Japanese (word-processing keyboard): Hiragana</p> |
| `Lang5` | <p>Japanese (word-processing keyboard): Zenkaku/Hankaku</p> |
| `NonConvert` | <p>Japanese: <kbd>無変換</kbd> (muhenkan)</p> |
| `Delete` | <p><kbd>⌦</kbd>. The forward delete key. Note that on Apple keyboards, the key labelled <kbd>Delete</kbd> on the main part of the keyboard is encoded as <a href="Self::Backspace"><code>Backspace</code></a>.</p> |
| `End` | <p><kbd>Page Down</kbd>, <kbd>End</kbd>, or <kbd>↘</kbd></p> |
| `Help` | <p><kbd>Help</kbd>. Not present on standard PC keyboards.</p> |
| `Home` | <p><kbd>Home</kbd> or <kbd>↖</kbd></p> |
| `Insert` | <p><kbd>Insert</kbd> or <kbd>Ins</kbd>. Not present on Apple keyboards.</p> |
| `PageDown` | <p><kbd>Page Down</kbd>, <kbd>PgDn</kbd>, or <kbd>⇟</kbd></p> |
| `PageUp` | <p><kbd>Page Up</kbd>, <kbd>PgUp</kbd>, or <kbd>⇞</kbd></p> |
| `ArrowDown` | <p><kbd>↓</kbd></p> |
| `ArrowLeft` | <p><kbd>←</kbd></p> |
| `ArrowRight` | <p><kbd>→</kbd></p> |
| `ArrowUp` | <p><kbd>↑</kbd></p> |
| `NumLock` | <p>On the Mac, this is used for the numpad <kbd>Clear</kbd> key.</p> |
| `Numpad0` | <p><kbd>0 Ins</kbd> on a keyboard. <kbd>0</kbd> on a phone or remote control</p> |
| `Numpad1` | <p><kbd>1 End</kbd> on a keyboard. <kbd>1</kbd> or <kbd>1 QZ</kbd> on a phone or remote control</p> |
| `Numpad2` | <p><kbd>2 ↓</kbd> on a keyboard. <kbd>2 ABC</kbd> on a phone or remote control</p> |
| `Numpad3` | <p><kbd>3 PgDn</kbd> on a keyboard. <kbd>3 DEF</kbd> on a phone or remote control</p> |
| `Numpad4` | <p><kbd>4 ←</kbd> on a keyboard. <kbd>4 GHI</kbd> on a phone or remote control</p> |
| `Numpad5` | <p><kbd>5</kbd> on a keyboard. <kbd>5 JKL</kbd> on a phone or remote control</p> |
| `Numpad6` | <p><kbd>6 →</kbd> on a keyboard. <kbd>6 MNO</kbd> on a phone or remote control</p> |
| `Numpad7` | <p><kbd>7 Home</kbd> on a keyboard. <kbd>7 PQRS</kbd> or <kbd>7 PRS</kbd> on a phone or remote control</p> |
| `Numpad8` | <p><kbd>8 ↑</kbd> on a keyboard. <kbd>8 TUV</kbd> on a phone or remote control</p> |
| `Numpad9` | <p><kbd>9 PgUp</kbd> on a keyboard. <kbd>9 WXYZ</kbd> or <kbd>9 WXY</kbd> on a phone or remote control</p> |
| `NumpadAdd` | <p><kbd>+</kbd></p> |
| `NumpadBackspace` | <p>Found on the Microsoft Natural Keyboard.</p> |
| `NumpadClear` | <p><kbd>C</kbd> or <kbd>A</kbd> (All Clear). Also for use with numpads that have a <kbd>Clear</kbd> key that is separate from the <kbd>NumLock</kbd> key. On the Mac, the numpad <kbd>Clear</kbd> key is encoded as <a href="Self::NumLock"><code>NumLock</code></a>.</p> |
| `NumpadClearEntry` | <p><kbd>C</kbd> (Clear Entry)</p> |
| `NumpadComma` | <p><kbd>,</kbd> (thousands separator). For locales where the thousands separator is a “.” (e.g., Brazil), this key may generate a <kbd>.</kbd>.</p> |
| `NumpadDecimal` | <p><kbd>. Del</kbd>. For locales where the decimal separator is “,” (e.g., Brazil), this key may generate a <kbd>,</kbd>.</p> |
| `NumpadDivide` | <p><kbd>/</kbd></p> |
| `NumpadEnter` |  |
| `NumpadEqual` | <p><kbd>=</kbd></p> |
| `NumpadHash` | <p><kbd>#</kbd> on a phone or remote control device. This key is typically found below the <kbd>9</kbd> key and to the right of the <kbd>0</kbd> key.</p> |
| `NumpadMemoryAdd` | <p><kbd>M</kbd> Add current entry to the value stored in memory.</p> |
| `NumpadMemoryClear` | <p><kbd>M</kbd> Clear the value stored in memory.</p> |
| `NumpadMemoryRecall` | <p><kbd>M</kbd> Replace the current entry with the value stored in memory.</p> |
| `NumpadMemoryStore` | <p><kbd>M</kbd> Replace the value stored in memory with the current entry.</p> |
| `NumpadMemorySubtract` | <p><kbd>M</kbd> Subtract current entry from the value stored in memory.</p> |
| `NumpadMultiply` | <p><kbd>*</kbd> on a keyboard. For use with numpads that provide mathematical operations (<kbd>+</kbd>, <kbd>-</kbd> <kbd>*</kbd> and <kbd>/</kbd>).</p> <p>Use <code>NumpadStar</code> for the <kbd>*</kbd> key on phones and remote controls.</p> |
| `NumpadParenLeft` | <p><kbd>(</kbd> Found on the Microsoft Natural Keyboard.</p> |
| `NumpadParenRight` | <p><kbd>)</kbd> Found on the Microsoft Natural Keyboard.</p> |
| `NumpadStar` | <p><kbd>*</kbd> on a phone or remote control device.</p> <p>This key is typically found below the <kbd>7</kbd> key and to the left of the <kbd>0</kbd> key.</p> <p>Use <kbd>“NumpadMultiply”</kbd> for the <kbd>*</kbd> key on numeric keypads.</p> |
| `NumpadSubtract` | <p><kbd>-</kbd></p> |
| `Escape` | <p><kbd>Esc</kbd> or <kbd>⎋</kbd></p> |
| `Fn` | <p><kbd>Fn</kbd> This is typically a hardware key that does not generate a separate code.</p> |
| `FnLock` | <p><kbd>FLock</kbd> or <kbd>FnLock</kbd>. Function Lock key. Found on the Microsoft Natural Keyboard.</p> |
| `PrintScreen` | <p><kbd>PrtScr SysRq</kbd> or <kbd>Print Screen</kbd></p> |
| `ScrollLock` | <p><kbd>Scroll Lock</kbd></p> |
| `Pause` | <p><kbd>Pause Break</kbd></p> |
| `BrowserBack` | <p>Some laptops place this key to the left of the <kbd>↑</kbd> key.</p> <p>This also the “back” button (triangle) on Android.</p> |
| `BrowserFavorites` |  |
| `BrowserForward` | <p>Some laptops place this key to the right of the <kbd>↑</kbd> key.</p> |
| `BrowserHome` | <p>The “home” button on Android.</p> |
| `BrowserRefresh` |  |
| `BrowserSearch` |  |
| `BrowserStop` |  |
| `Eject` | <p><kbd>Eject</kbd> or <kbd>⏏</kbd>. This key is placed in the function section on some Apple keyboards.</p> |
| `LaunchApp1` | <p>Sometimes labelled <kbd>My Computer</kbd> on the keyboard</p> |
| `LaunchApp2` | <p>Sometimes labelled <kbd>Calculator</kbd> on the keyboard</p> |
| `LaunchMail` |  |
| `MediaPlayPause` |  |
| `MediaSelect` |  |
| `MediaStop` |  |
| `MediaTrackNext` |  |
| `MediaTrackPrevious` |  |
| `Power` | <p>This key is placed in the function section on some Apple keyboards, replacing the <kbd>Eject</kbd> key.</p> |
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
| `Again` | <p>Found on Sun’s USB keyboard.</p> |
| `Copy` | <p>Found on Sun’s USB keyboard.</p> |
| `Cut` | <p>Found on Sun’s USB keyboard.</p> |
| `Find` | <p>Found on Sun’s USB keyboard.</p> |
| `Open` | <p>Found on Sun’s USB keyboard.</p> |
| `Paste` | <p>Found on Sun’s USB keyboard.</p> |
| `Props` | <p>Found on Sun’s USB keyboard.</p> |
| `Select` | <p>Found on Sun’s USB keyboard.</p> |
| `Undo` | <p>Found on Sun’s USB keyboard.</p> |
| `Hiragana` | <p>Use for dedicated <kbd>ひらがな</kbd> key found on some Japanese word processing keyboards.</p> |
| `Katakana` | <p>Use for dedicated <kbd>カタカナ</kbd> key found on some Japanese word processing keyboards.</p> |
| `F1` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F2` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F3` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F4` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F5` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F6` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F7` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F8` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F9` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F10` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F11` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F12` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F13` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F14` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F15` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F16` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F17` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F18` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F19` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F20` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F21` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F22` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F23` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F24` | <p>General-purpose function key. Usually found at the top of the keyboard.</p> |
| `F25` | <p>General-purpose function key.</p> |
| `F26` | <p>General-purpose function key.</p> |
| `F27` | <p>General-purpose function key.</p> |
| `F28` | <p>General-purpose function key.</p> |
| `F29` | <p>General-purpose function key.</p> |
| `F30` | <p>General-purpose function key.</p> |
| `F31` | <p>General-purpose function key.</p> |
| `F32` | <p>General-purpose function key.</p> |
| `F33` | <p>General-purpose function key.</p> |
| `F34` | <p>General-purpose function key.</p> |
| `F35` | <p>General-purpose function key.</p> |

