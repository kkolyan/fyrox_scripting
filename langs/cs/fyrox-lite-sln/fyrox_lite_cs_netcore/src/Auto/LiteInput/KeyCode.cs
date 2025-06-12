// ReSharper disable InconsistentNaming
// ReSharper disable RedundantUnsafeContext
// ReSharper disable UnusedMember.Global
// ReSharper disable RedundantUsingDirective
using FyroxLite;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite;

// fyrox_lite::lite_input::LiteKeyCode

/// <para>Basically a copy of <c>winit</c>’s <see href="https://docs.rs/winit/0.29.15/i686-pc-windows-msvc/winit/keyboard/enum.KeyCode.html"><c>KeyCode</c></see>, which is mostly inspired by UI Events Specification’s <see href="https://w3c.github.io/uievents-code/#code-value-tables"><c>KeyboardEvent.code</c></see>.</para>
public enum KeyCode
{
    
    /// <para><c>`</c> on a US keyboard. This is also called a backtick or grave.
    /// This is the <c>半角</c>/<c>全角</c>/<c>漢字</c>
    /// (hankaku/zenkaku/kanji) key on Japanese keyboards</para>
    Backquote,
    
    /// <para>Used for both the US <c>\</c> (on the 101-key layout) and also for the key
    /// located between the <c>“</c> and <c>Enter</c> keys on row C of the 102-,
    /// 104- and 106-key layouts.
    /// Labeled <c>#</c> on a UK (102) keyboard.</para>
    Backslash,
    
    /// <para><c>[</c> on a US keyboard.</para>
    BracketLeft,
    
    /// <para><c>]</c> on a US keyboard.</para>
    BracketRight,
    
    /// <para><c>,</c> on a US keyboard.</para>
    Comma,
    
    /// <para><c>0</c> on a US keyboard.</para>
    Digit0,
    
    /// <para><c>1</c> on a US keyboard.</para>
    Digit1,
    
    /// <para><c>2</c> on a US keyboard.</para>
    Digit2,
    
    /// <para><c>3</c> on a US keyboard.</para>
    Digit3,
    
    /// <para><c>4</c> on a US keyboard.</para>
    Digit4,
    
    /// <para><c>5</c> on a US keyboard.</para>
    Digit5,
    
    /// <para><c>6</c> on a US keyboard.</para>
    Digit6,
    
    /// <para><c>7</c> on a US keyboard.</para>
    Digit7,
    
    /// <para><c>8</c> on a US keyboard.</para>
    Digit8,
    
    /// <para><c>9</c> on a US keyboard.</para>
    Digit9,
    
    /// <para><c>=</c> on a US keyboard.</para>
    Equal,
    
    /// <para>Located between the left <c>Shift</c> and <c>Z</c> keys.
    /// Labeled <c>\</c> on a UK keyboard.</para>
    IntlBackslash,
    
    /// <para>Located between the <c>/</c> and right <c>Shift</c> keys.
    /// Labeled <c>\</c> (ro) on a Japanese keyboard.</para>
    IntlRo,
    
    /// <para>Located between the <c>=</c> and <c>Backspace</c> keys.
    /// Labeled <c>¥</c> (yen) on a Japanese keyboard. <c>\</c> on a
    /// Russian keyboard.</para>
    IntlYen,
    
    /// <para><c>a</c> on a US keyboard.
    /// Labeled <c>q</c> on an AZERTY (e.g., French) keyboard.</para>
    A,
    
    /// <para><c>b</c> on a US keyboard.</para>
    B,
    
    /// <para><c>c</c> on a US keyboard.</para>
    C,
    
    /// <para><c>d</c> on a US keyboard.</para>
    D,
    
    /// <para><c>e</c> on a US keyboard.</para>
    E,
    
    /// <para><c>f</c> on a US keyboard.</para>
    F,
    
    /// <para><c>g</c> on a US keyboard.</para>
    G,
    
    /// <para><c>h</c> on a US keyboard.</para>
    H,
    
    /// <para><c>i</c> on a US keyboard.</para>
    I,
    
    /// <para><c>j</c> on a US keyboard.</para>
    J,
    
    /// <para><c>k</c> on a US keyboard.</para>
    K,
    
    /// <para><c>l</c> on a US keyboard.</para>
    L,
    
    /// <para><c>m</c> on a US keyboard.</para>
    M,
    
    /// <para><c>n</c> on a US keyboard.</para>
    N,
    
    /// <para><c>o</c> on a US keyboard.</para>
    O,
    
    /// <para><c>p</c> on a US keyboard.</para>
    P,
    
    /// <para><c>q</c> on a US keyboard.
    /// Labeled <c>a</c> on an AZERTY (e.g., French) keyboard.</para>
    Q,
    
    /// <para><c>r</c> on a US keyboard.</para>
    R,
    
    /// <para><c>s</c> on a US keyboard.</para>
    S,
    
    /// <para><c>t</c> on a US keyboard.</para>
    T,
    
    /// <para><c>u</c> on a US keyboard.</para>
    U,
    
    /// <para><c>v</c> on a US keyboard.</para>
    V,
    
    /// <para><c>w</c> on a US keyboard.
    /// Labeled <c>z</c> on an AZERTY (e.g., French) keyboard.</para>
    W,
    
    /// <para><c>x</c> on a US keyboard.</para>
    X,
    
    /// <para><c>y</c> on a US keyboard.
    /// Labeled <c>z</c> on a QWERTZ (e.g., German) keyboard.</para>
    Y,
    
    /// <para><c>z</c> on a US keyboard.
    /// Labeled <c>w</c> on an AZERTY (e.g., French) keyboard, and <c>y</c> on a
    /// QWERTZ (e.g., German) keyboard.</para>
    Z,
    
    /// <para><c>-</c> on a US keyboard.</para>
    Minus,
    
    /// <para><c>.</c> on a US keyboard.</para>
    Period,
    
    /// <para><c>’</c> on a US keyboard.</para>
    Quote,
    
    /// <para><c>;</c> on a US keyboard.</para>
    Semicolon,
    
    /// <para><c>/</c> on a US keyboard.</para>
    Slash,
    
    /// <para><c>Alt</c>, <c>Option</c>, or <c>⌥</c>.</para>
    AltLeft,
    
    /// <para><c>Alt</c>, <c>Option</c>, or <c>⌥</c>.
    /// This is labeled <c>AltGr</c> on many keyboard layouts.</para>
    AltRight,
    
    /// <para><c>Backspace</c> or <c>⌫</c>.
    /// Labeled <c>Delete</c> on Apple keyboards.</para>
    Backspace,
    
    /// <para><c>CapsLock</c> or <c>⇪</c></para>
    CapsLock,
    
    /// <para>The application context menu key, which is typically found between the right
    /// <c>Super</c> key and the right <c>Control</c> key.</para>
    ContextMenu,
    
    /// <para><c>Control</c> or <c>⌃</c></para>
    ControlLeft,
    
    /// <para><c>Control</c> or <c>⌃</c></para>
    ControlRight,
    
    /// <para><c>Enter</c> or <c>↵</c>. Labeled <c>Return</c> on Apple keyboards.</para>
    Enter,
    
    /// <para>The Windows, <c>⌘</c>, <c>Command</c>, or other OS symbol key.</para>
    SuperLeft,
    
    /// <para>The Windows, <c>⌘</c>, <c>Command</c>, or other OS symbol key.</para>
    SuperRight,
    
    /// <para><c>Shift</c> or <c>⇧</c></para>
    ShiftLeft,
    
    /// <para><c>Shift</c> or <c>⇧</c></para>
    ShiftRight,
    
    /// <para><c> </c> (space)</para>
    Space,
    
    /// <para><c>Tab</c> or <c>⇥</c></para>
    Tab,
    
    /// <para>Japanese: <c>変</c> (henkan)</para>
    Convert,
    
    /// <para>Japanese: <c>カタカナ</c>/<c>ひらがな</c>/<c>ローマ字</c> (katakana/hiragana/romaji)</para>
    KanaMode,
    
    /// <para>Korean: HangulMode <c>한/영</c> (han/yeong)</para><para>Japanese (Mac keyboard): <c>か</c> (kana)</para>
    Lang1,
    
    /// <para>Korean: Hanja <c>한</c> (hanja)</para><para>Japanese (Mac keyboard): <c>英</c> (eisu)</para>
    Lang2,
    
    /// <para>Japanese (word-processing keyboard): Katakana</para>
    Lang3,
    
    /// <para>Japanese (word-processing keyboard): Hiragana</para>
    Lang4,
    
    /// <para>Japanese (word-processing keyboard): Zenkaku/Hankaku</para>
    Lang5,
    
    /// <para>Japanese: <c>無変換</c> (muhenkan)</para>
    NonConvert,
    
    /// <para><c>⌦</c>. The forward delete key.
    /// Note that on Apple keyboards, the key labelled <c>Delete</c> on the main part of
    /// the keyboard is encoded as <see href="Self::Backspace"><c>Backspace</c></see>.</para>
    Delete,
    
    /// <para><c>Page Down</c>, <c>End</c>, or <c>↘</c></para>
    End,
    
    /// <para><c>Help</c>. Not present on standard PC keyboards.</para>
    Help,
    
    /// <para><c>Home</c> or <c>↖</c></para>
    Home,
    
    /// <para><c>Insert</c> or <c>Ins</c>. Not present on Apple keyboards.</para>
    Insert,
    
    /// <para><c>Page Down</c>, <c>PgDn</c>, or <c>⇟</c></para>
    PageDown,
    
    /// <para><c>Page Up</c>, <c>PgUp</c>, or <c>⇞</c></para>
    PageUp,
    
    /// <para><c>↓</c></para>
    ArrowDown,
    
    /// <para><c>←</c></para>
    ArrowLeft,
    
    /// <para><c>→</c></para>
    ArrowRight,
    
    /// <para><c>↑</c></para>
    ArrowUp,
    
    /// <para>On the Mac, this is used for the numpad <c>Clear</c> key.</para>
    NumLock,
    
    /// <para><c>0 Ins</c> on a keyboard. <c>0</c> on a phone or remote control</para>
    Numpad0,
    
    /// <para><c>1 End</c> on a keyboard. <c>1</c> or <c>1 QZ</c> on a phone or remote control</para>
    Numpad1,
    
    /// <para><c>2 ↓</c> on a keyboard. <c>2 ABC</c> on a phone or remote control</para>
    Numpad2,
    
    /// <para><c>3 PgDn</c> on a keyboard. <c>3 DEF</c> on a phone or remote control</para>
    Numpad3,
    
    /// <para><c>4 ←</c> on a keyboard. <c>4 GHI</c> on a phone or remote control</para>
    Numpad4,
    
    /// <para><c>5</c> on a keyboard. <c>5 JKL</c> on a phone or remote control</para>
    Numpad5,
    
    /// <para><c>6 →</c> on a keyboard. <c>6 MNO</c> on a phone or remote control</para>
    Numpad6,
    
    /// <para><c>7 Home</c> on a keyboard. <c>7 PQRS</c> or <c>7 PRS</c> on a phone
    /// or remote control</para>
    Numpad7,
    
    /// <para><c>8 ↑</c> on a keyboard. <c>8 TUV</c> on a phone or remote control</para>
    Numpad8,
    
    /// <para><c>9 PgUp</c> on a keyboard. <c>9 WXYZ</c> or <c>9 WXY</c> on a phone
    /// or remote control</para>
    Numpad9,
    
    /// <para><c>+</c></para>
    NumpadAdd,
    
    /// <para>Found on the Microsoft Natural Keyboard.</para>
    NumpadBackspace,
    
    /// <para><c>C</c> or <c>A</c> (All Clear). Also for use with numpads that have a
    /// <c>Clear</c> key that is separate from the <c>NumLock</c> key. On the Mac, the
    /// numpad <c>Clear</c> key is encoded as <see href="Self::NumLock"><c>NumLock</c></see>.</para>
    NumpadClear,
    
    /// <para><c>C</c> (Clear Entry)</para>
    NumpadClearEntry,
    
    /// <para><c>,</c> (thousands separator). For locales where the thousands separator
    /// is a “.” (e.g., Brazil), this key may generate a <c>.</c>.</para>
    NumpadComma,
    
    /// <para><c>. Del</c>. For locales where the decimal separator is “,” (e.g.,
    /// Brazil), this key may generate a <c>,</c>.</para>
    NumpadDecimal,
    
    /// <para><c>/</c></para>
    NumpadDivide,
    
    NumpadEnter,
    
    /// <para><c>=</c></para>
    NumpadEqual,
    
    /// <para><c>#</c> on a phone or remote control device. This key is typically found
    /// below the <c>9</c> key and to the right of the <c>0</c> key.</para>
    NumpadHash,
    
    /// <para><c>M</c> Add current entry to the value stored in memory.</para>
    NumpadMemoryAdd,
    
    /// <para><c>M</c> Clear the value stored in memory.</para>
    NumpadMemoryClear,
    
    /// <para><c>M</c> Replace the current entry with the value stored in memory.</para>
    NumpadMemoryRecall,
    
    /// <para><c>M</c> Replace the value stored in memory with the current entry.</para>
    NumpadMemoryStore,
    
    /// <para><c>M</c> Subtract current entry from the value stored in memory.</para>
    NumpadMemorySubtract,
    
    /// <para><c>*</c> on a keyboard. For use with numpads that provide mathematical
    /// operations (<c>+</c>, <c>-</c> <c>*</c> and <c>/</c>).</para><para>Use <c>NumpadStar</c> for the <c>*</c> key on phones and remote controls.</para>
    NumpadMultiply,
    
    /// <para><c>(</c> Found on the Microsoft Natural Keyboard.</para>
    NumpadParenLeft,
    
    /// <para><c>)</c> Found on the Microsoft Natural Keyboard.</para>
    NumpadParenRight,
    
    /// <para><c>*</c> on a phone or remote control device.</para><para>This key is typically found below the <c>7</c> key and to the left of
    /// the <c>0</c> key.</para><para>Use <c>“NumpadMultiply”</c> for the <c>*</c> key on
    /// numeric keypads.</para>
    NumpadStar,
    
    /// <para><c>-</c></para>
    NumpadSubtract,
    
    /// <para><c>Esc</c> or <c>⎋</c></para>
    Escape,
    
    /// <para><c>Fn</c> This is typically a hardware key that does not generate a separate code.</para>
    Fn,
    
    /// <para><c>FLock</c> or <c>FnLock</c>. Function Lock key. Found on the Microsoft
    /// Natural Keyboard.</para>
    FnLock,
    
    /// <para><c>PrtScr SysRq</c> or <c>Print Screen</c></para>
    PrintScreen,
    
    /// <para><c>Scroll Lock</c></para>
    ScrollLock,
    
    /// <para><c>Pause Break</c></para>
    Pause,
    
    /// <para>Some laptops place this key to the left of the <c>↑</c> key.</para><para>This also the “back” button (triangle) on Android.</para>
    BrowserBack,
    
    BrowserFavorites,
    
    /// <para>Some laptops place this key to the right of the <c>↑</c> key.</para>
    BrowserForward,
    
    /// <para>The “home” button on Android.</para>
    BrowserHome,
    
    BrowserRefresh,
    
    BrowserSearch,
    
    BrowserStop,
    
    /// <para><c>Eject</c> or <c>⏏</c>. This key is placed in the function section on some Apple
    /// keyboards.</para>
    Eject,
    
    /// <para>Sometimes labelled <c>My Computer</c> on the keyboard</para>
    LaunchApp1,
    
    /// <para>Sometimes labelled <c>Calculator</c> on the keyboard</para>
    LaunchApp2,
    
    LaunchMail,
    
    MediaPlayPause,
    
    MediaSelect,
    
    MediaStop,
    
    MediaTrackNext,
    
    MediaTrackPrevious,
    
    /// <para>This key is placed in the function section on some Apple keyboards, replacing the
    /// <c>Eject</c> key.</para>
    Power,
    
    Sleep,
    
    AudioVolumeDown,
    
    AudioVolumeMute,
    
    AudioVolumeUp,
    
    WakeUp,
    
    Meta,
    
    Hyper,
    
    Turbo,
    
    Abort,
    
    Resume,
    
    Suspend,
    
    /// <para>Found on Sun’s USB keyboard.</para>
    Again,
    
    /// <para>Found on Sun’s USB keyboard.</para>
    Copy,
    
    /// <para>Found on Sun’s USB keyboard.</para>
    Cut,
    
    /// <para>Found on Sun’s USB keyboard.</para>
    Find,
    
    /// <para>Found on Sun’s USB keyboard.</para>
    Open,
    
    /// <para>Found on Sun’s USB keyboard.</para>
    Paste,
    
    /// <para>Found on Sun’s USB keyboard.</para>
    Props,
    
    /// <para>Found on Sun’s USB keyboard.</para>
    Select,
    
    /// <para>Found on Sun’s USB keyboard.</para>
    Undo,
    
    /// <para>Use for dedicated <c>ひらがな</c> key found on some Japanese word processing keyboards.</para>
    Hiragana,
    
    /// <para>Use for dedicated <c>カタカナ</c> key found on some Japanese word processing keyboards.</para>
    Katakana,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F1,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F2,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F3,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F4,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F5,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F6,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F7,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F8,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F9,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F10,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F11,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F12,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F13,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F14,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F15,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F16,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F17,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F18,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F19,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F20,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F21,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F22,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F23,
    
    /// <para>General-purpose function key.
    /// Usually found at the top of the keyboard.</para>
    F24,
    
    /// <para>General-purpose function key.</para>
    F25,
    
    /// <para>General-purpose function key.</para>
    F26,
    
    /// <para>General-purpose function key.</para>
    F27,
    
    /// <para>General-purpose function key.</para>
    F28,
    
    /// <para>General-purpose function key.</para>
    F29,
    
    /// <para>General-purpose function key.</para>
    F30,
    
    /// <para>General-purpose function key.</para>
    F31,
    
    /// <para>General-purpose function key.</para>
    F32,
    
    /// <para>General-purpose function key.</para>
    F33,
    
    /// <para>General-purpose function key.</para>
    F34,
    
    /// <para>General-purpose function key.</para>
    F35,
}