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

// fyrox_lite::lite_ui::LiteText

/// <para>Text is a simple widget that allows you to print text on screen. It has various options like word wrapping, text
/// alignment, and so on.</para><para><b>How to create</b></para><para>An instance of the [<c>Text</c>] widget could be created like so:</para><code><c class="language-rust"># use fyrox_ui::{
/// #     core::pool::Handle,
/// #     text::TextBuilder, widget::WidgetBuilder, UiNode, UserInterface
/// # };
/// fn create_text(ui: &amp;mut UserInterface, text: &amp;str) -&gt; Handle&lt;UiNode&gt; {
/// TextBuilder::new(WidgetBuilder::new())
/// .with_text(text)
/// .build(&amp;mut ui.build_ctx())
/// }
/// </c></code><para><b>Text alignment and word wrapping</b></para><para>There are various text alignment options for both vertical and horizontal axes. Typical alignment values are:
/// [<c>HorizontalAlignment::Left</c>], [<c>HorizontalAlignment::Center</c>], [<c>HorizontalAlignment::Right</c>] for horizontal axis,
/// and [<c>VerticalAlignment::Top</c>], [<c>VerticalAlignment::Center</c>], [<c>VerticalAlignment::Bottom</c>] for vertical axis. An
/// instance of centered text could be created like so:</para><code><c class="language-rust,no_run"># use fyrox_ui::{
/// #     core::pool::Handle,
/// #     text::TextBuilder, widget::WidgetBuilder, HorizontalAlignment, UiNode, UserInterface,
/// #     VerticalAlignment,
/// # };
/// fn create_centered_text(ui: &amp;mut UserInterface, text: &amp;str) -&gt; Handle&lt;UiNode&gt; {
/// TextBuilder::new(WidgetBuilder::new())
/// .with_horizontal_text_alignment(HorizontalAlignment::Center)
/// .with_vertical_text_alignment(VerticalAlignment::Center)
/// .with_text(text)
/// .build(&amp;mut ui.build_ctx())
/// }
/// </c></code><para>What’s the difference between widget’s alignment and text-specific? Widget’s alignment operates on a bounding rectangle
/// of the text and text-specific alignment operates on line-basis. This means that if you set [<c>HorizontalAlignment::Center</c>]
/// as widget’s alignment, your text lines won’t be centered, instead they’ll be aligned at the left and the entire text block
/// will be aligned at center.</para><para>Long text is usually needs to wrap on available bounds, there are three possible options for word wrapping:
/// [<c>WrapMode::NoWrap</c>], [<c>WrapMode::Letter</c>], [<c>WrapMode::Word</c>]. An instance of text with word-based wrapping could
/// be created like so:</para><code><c class="language-rust,no_run"># use fyrox_ui::{
/// #     core::pool::Handle,
/// #     formatted_text::WrapMode, text::TextBuilder, widget::WidgetBuilder, UiNode,
/// #     UserInterface,
/// # };
/// fn create_text_with_word_wrap(ui: &amp;mut UserInterface, text: &amp;str) -&gt; Handle&lt;UiNode&gt; {
/// TextBuilder::new(WidgetBuilder::new())
/// .with_wrap(WrapMode::Word)
/// .with_text(text)
/// .build(&amp;mut ui.build_ctx())
/// }
/// </c></code><para><b>Background</b></para><para>If you need to have a text with some background, you should use [<c>crate::border::Border</c>] widget as a parent widget of your
/// text. <b>Caveat:</b> [<c>WidgetBuilder::with_background</c>] is ignored for [<c>Text</c>] widget!</para><code><c class="language-rust,no_run"># use fyrox_ui::{
/// #     core::{color::Color, pool::Handle},
/// #     border::BorderBuilder, brush::Brush, text::TextBuilder, widget::WidgetBuilder, UiNode,
/// #     UserInterface,
/// # };
/// #
/// fn create_text_with_background(ui: &amp;mut UserInterface, text: &amp;str) -&gt; Handle&lt;UiNode&gt; {
/// let text_widget =
/// TextBuilder::new(WidgetBuilder::new().with_foreground(Brush::Solid(Color::RED).into()))
/// .with_text(text)
/// .build(&amp;mut ui.build_ctx());
/// BorderBuilder::new(
/// WidgetBuilder::new()
/// .with_child(text_widget) // &lt;-- Text is now a child of the border
/// .with_background(Brush::Solid(Color::opaque(50, 50, 50)).into()),
/// )
/// .build(&amp;mut ui.build_ctx())
/// }
/// </c></code><para>Keep in mind that now the text widget is a child widget of the border, so if you need to position the text, you should
/// position the border, not the text.</para><para><b>Fonts and colors</b></para><para>To set a color of the text just use [<c>WidgetBuilder::with_foreground</c>] while building the text instance:</para><code><c class="language-rust,no_run"># use fyrox_ui::{
/// #     core::{color::Color, pool::Handle},
/// #     brush::Brush, text::TextBuilder, widget::WidgetBuilder, UiNode, UserInterface
/// # };
/// fn create_text(ui: &amp;mut UserInterface, text: &amp;str) -&gt; Handle&lt;UiNode&gt; {
/// //               vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
/// TextBuilder::new(WidgetBuilder::new().with_foreground(Brush::Solid(Color::RED).into()))
/// .with_text(text)
/// .build(&amp;mut ui.build_ctx())
/// }
/// </c></code><para>By default, text is created with default font, however it is possible to set any custom font:</para><code><c class="language-rust"># use fyrox_resource::manager::ResourceManager;
/// # use fyrox_ui::{
/// #     core::{futures::executor::block_on, pool::Handle},
/// #     text::TextBuilder,
/// #     font::{Font, FontResource},
/// #     widget::WidgetBuilder,
/// #     UiNode, UserInterface,
/// # };
/// 
/// fn create_text(ui: &amp;mut UserInterface, resource_manager: &amp;ResourceManager, text: &amp;str) -&gt; Handle&lt;UiNode&gt; {
/// TextBuilder::new(WidgetBuilder::new())
/// .with_font(resource_manager.request::&lt;Font&gt;("path/to/your/font.ttf"))
/// .with_text(text)
/// .with_font_size(20.0f32.into())
/// .build(&amp;mut ui.build_ctx())
/// }
/// </c></code><para>Please refer to [<c>crate::font::Font</c>] chapter to learn more about fonts.</para><para><b>Font size</b></para><para>Use [<c>fyrox::gui::text::TextBuilder::with_font_size</c>] or send [<c>TextMessage::font_size</c>] to your Text widget instance
/// to set the font size of it.</para><para><b>Shadows</b></para><para>Text widget supports shadows effect to add contrast to your text, which could be useful to make text readable independent
/// on the background colors. This effect could be used for subtitles. Shadows are pretty easy to add, all you need to do
/// is to enable them, setup desired thickness, offset and brush (solid color or gradient).</para><code><c class="language-rust,no_run"># use fyrox_ui::{
/// #     core::{algebra::Vector2, color::Color, pool::Handle},
/// #     brush::Brush, text::TextBuilder, widget::WidgetBuilder, UiNode, UserInterface
/// # };
/// #
/// fn create_red_text_with_black_shadows(ui: &amp;mut UserInterface, text: &amp;str) -&gt; Handle&lt;UiNode&gt; {
/// TextBuilder::new(WidgetBuilder::new().with_foreground(Brush::Solid(Color::RED).into()))
/// .with_text(text)
/// // Enable shadows.
/// .with_shadow(true)
/// // Black shadows.
/// .with_shadow_brush(Brush::Solid(Color::BLACK))
/// // 1px thick.
/// .with_shadow_dilation(1.0)
/// // Offset the shadow slightly to the right-bottom.
/// .with_shadow_offset(Vector2::new(1.0, 1.0))
/// .build(&amp;mut ui.build_ctx())
/// }
/// </c></code><para><b>Messages</b></para><para>Text widget can accept the following list of messages at runtime (respective constructors are name with small letter -
/// <c>TextMessage::Text -&gt; TextMessage::text(widget_handle, direction, text)</c>):</para><ul>
/// <li>[<c>TextMessage::Text</c>] - sets new text for a <c>Text</c> widget.</li>
/// <li>[<c>TextMessage::Wrap</c>] - sets new <see href="Text#text-alignment-and-word-wrapping">wrapping mode</see>.</li>
/// <li>[<c>TextMessage::Font</c>] - sets new <see href="Text#fonts-and-colors">font</see></li>
/// <li>[<c>TextMessage::VerticalAlignment</c>] and <c>TextMessage::HorizontalAlignment</c> sets
/// <see href="Text#text-alignment-and-word-wrapping">vertical and horizontal</see> text alignment respectively.</li>
/// <li>[<c>TextMessage::Shadow</c>] - enables or disables <see href="Text#shadows">shadow casting</see></li>
/// <li>[<c>TextMessage::ShadowDilation</c>] - sets “thickness” of the shadows under the tex.</li>
/// <li>[<c>TextMessage::ShadowBrush</c>] - sets shadow brush (allows you to change color and even make shadow with color gradients).</li>
/// <li>[<c>TextMessage::ShadowOffset</c>] - sets offset of the shadows.</li>
/// </ul><para>An example of changing text at runtime could be something like this:</para><code><c class="language-rust"># use fyrox_ui::{
/// #     core::pool::Handle,
/// #     message::{MessageDirection},
/// #     UiNode, UserInterface,
/// #     text::TextMessage
/// # };
/// fn request_change_text(ui: &amp;UserInterface, text_widget_handle: Handle&lt;UiNode&gt;, text: &amp;str) {
/// ui.send_message(TextMessage::text(
/// text_widget_handle,
/// MessageDirection::ToWidget,
/// text.to_owned(),
/// ))
/// }
/// </c></code><para>Please keep in mind, that like any other situation when you “changing” something via messages, you should remember
/// that the change is <b>not</b> immediate.</para>
public partial struct LiteText : IEquatable<LiteText>
{
    #region internal fields and constructor
    private readonly NativeHandle handle;

    internal LiteText(NativeHandle handle)
    {
        this.handle = handle;
    }
    #endregion
    
    /// <para>sets the text of UI element. applied at the end of frame.</para>
    public string TextAsync
    {
        set
        {
            #region native call
            unsafe {
                var _value = NativeString.FromFacade(value);
                fyrox_lite_lite_ui_LiteText_set_text_async(this, _value);
            }
            #endregion
        }
    }
    
    /// <para>current value (<c>set_text_async</c> doesn’t affect this immediately)</para>
    public string Text
    {
        get
        {
            #region native call
            unsafe {
                var __ret = fyrox_lite_lite_ui_LiteText_get_text(this);
                return NativeString.ToFacade(__ret);
            }
            #endregion
        }
    }

    
    public static LiteText New(TextBuilder state)
    {
        #region native call
        unsafe {
            var _state = state;
            var __ret = fyrox_lite_lite_ui_LiteText_new(&_state);
            return __ret;
        }
        #endregion
    }

    #region native internal methods

    [LibraryImport(FyroxDll.Name, StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_ui_LiteText_set_text_async(LiteText self, NativeString text);

    [LibraryImport(FyroxDll.Name, StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeString fyrox_lite_lite_ui_LiteText_get_text(LiteText self);

    [LibraryImport(FyroxDll.Name, StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial LiteText fyrox_lite_lite_ui_LiteText_new(TextBuilder* state);
    #endregion


    public bool Equals(LiteText other)
    {
        return handle.Equals(other.handle);
    }

    public override bool Equals(object? obj)
    {
        return obj is LiteText other && Equals(other);
    }

    public override int GetHashCode()
    {
        return handle.GetHashCode();
    }

    public static bool operator ==(LiteText left, LiteText right)
    {
        return left.Equals(right);
    }

    public static bool operator !=(LiteText left, LiteText right)
    {
        return !left.Equals(right);
    }
}
#region internal type wrappers


[StructLayout(LayoutKind.Sequential)]
internal struct LiteText_optional
{
    internal LiteText value;
    internal int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static LiteText? ToFacade(in LiteText_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static LiteText_optional FromFacade(in LiteText? value)
    {
        if (value == null)
        {
            return new LiteText_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = __item;
        return new LiteText_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct LiteText_result
{
    internal int ok;
    internal LiteText_result_value value;

    internal static unsafe LiteText ToFacade(in LiteText_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value.ok;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.value.err));
    }

    internal static LiteText_result FromFacade(in LiteText self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new LiteText_result {ok = 1, value = new LiteText_result_value { ok = __item_from_facade } };
    }

    internal static LiteText_result FromFacadeError(in string err)
    {
        return new LiteText_result {ok = 0, value = new LiteText_result_value { err = NativeString.FromFacade(err) } };
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct LiteText_result_value
{
    [FieldOffset(0)]
    internal LiteText ok;

    [FieldOffset(0)]
    internal NativeString err;
}

[StructLayout(LayoutKind.Sequential)]
internal struct LiteText_optional_result
{
    internal int ok;
    internal LiteText_optional_result_value value;

    internal static unsafe LiteText? ToFacade(in LiteText_optional_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value.ok;
            var __item_to_facade = LiteText_optional.ToFacade(__item);
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.value.err));
    }

    internal static LiteText_optional_result FromFacade(in LiteText? self)
    {
        var __item = self;
        var __item_from_facade = LiteText_optional.FromFacade(__item);
        return new LiteText_optional_result {ok = 1, value = new LiteText_optional_result_value { ok = __item_from_facade } };
    }

    internal static LiteText_optional_result FromFacadeError(in string err)
    {
        return new LiteText_optional_result {ok = 0, value = new LiteText_optional_result_value { err = NativeString.FromFacade(err) } };
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct LiteText_optional_result_value
{
    [FieldOffset(0)]
    internal LiteText_optional ok;

    [FieldOffset(0)]
    internal NativeString err;
}
#endregion