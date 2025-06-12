use fyrox::{
    core::{color, pool::Handle},
    gui::{brush, message::MessageDirection, text, widget, UiNode},
};
use fyrox::gui::{HorizontalAlignment, VerticalAlignment};
use fyrox::gui::formatted_text::WrapMode;
use fyrox::gui::text::{Text, TextMessage};
use fyrox::gui::widget::WidgetBuilder;
use lite_macro::lite_api;

use crate::{
    externalizable::Externalizable, lite_math::PodVector2, script_context::with_script_context,
};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LiteUiNode {
    handle: Handle<UiNode>,
}

impl LiteUiNode {
    pub fn new(handle: Handle<UiNode>) -> Self {
        Self { handle }
    }

    pub fn inner(&self) -> Handle<UiNode> {
        self.handle
    }
}

#[lite_api(class=UiNode)]
impl LiteUiNode {}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LiteText {
    handle: Handle<UiNode>,
}

impl LiteText {
    pub fn inner(&self) -> Handle<UiNode> {
        self.handle
    }
}

impl From<Handle<UiNode>> for LiteText {
    fn from(handle: Handle<UiNode>) -> Self {
        Self { handle }
    }
}

#[lite_api(class=LiteText)]

/// Text is a simple widget that allows you to print text on screen. It has various options like word wrapping, text
/// alignment, and so on.
///
/// ## How to create
///
/// An instance of the [`Text`] widget could be created like so:
///
/// ```rust
/// # use fyrox_ui::{
/// #     core::pool::Handle,
/// #     text::TextBuilder, widget::WidgetBuilder, UiNode, UserInterface
/// # };
/// fn create_text(ui: &mut UserInterface, text: &str) -> Handle<UiNode> {
///     TextBuilder::new(WidgetBuilder::new())
///         .with_text(text)
///         .build(&mut ui.build_ctx())
/// }
/// ```
///
/// ## Text alignment and word wrapping
///
/// There are various text alignment options for both vertical and horizontal axes. Typical alignment values are:
/// [`HorizontalAlignment::Left`], [`HorizontalAlignment::Center`], [`HorizontalAlignment::Right`] for horizontal axis,
/// and [`VerticalAlignment::Top`], [`VerticalAlignment::Center`], [`VerticalAlignment::Bottom`] for vertical axis. An
/// instance of centered text could be created like so:
///
/// ```rust,no_run
/// # use fyrox_ui::{
/// #     core::pool::Handle,
/// #     text::TextBuilder, widget::WidgetBuilder, HorizontalAlignment, UiNode, UserInterface,
/// #     VerticalAlignment,
/// # };
/// fn create_centered_text(ui: &mut UserInterface, text: &str) -> Handle<UiNode> {
///     TextBuilder::new(WidgetBuilder::new())
///         .with_horizontal_text_alignment(HorizontalAlignment::Center)
///         .with_vertical_text_alignment(VerticalAlignment::Center)
///     .with_text(text)
///     .build(&mut ui.build_ctx())
/// }
/// ```
///
/// What's the difference between widget's alignment and text-specific? Widget's alignment operates on a bounding rectangle
/// of the text and text-specific alignment operates on line-basis. This means that if you set [`HorizontalAlignment::Center`]
/// as widget's alignment, your text lines won't be centered, instead they'll be aligned at the left and the entire text block
/// will be aligned at center.
///
/// Long text is usually needs to wrap on available bounds, there are three possible options for word wrapping:
/// [`WrapMode::NoWrap`], [`WrapMode::Letter`], [`WrapMode::Word`]. An instance of text with word-based wrapping could
/// be created like so:
///
/// ```rust,no_run
/// # use fyrox_ui::{
/// #     core::pool::Handle,
/// #     formatted_text::WrapMode, text::TextBuilder, widget::WidgetBuilder, UiNode,
/// #     UserInterface,
/// # };
/// fn create_text_with_word_wrap(ui: &mut UserInterface, text: &str) -> Handle<UiNode> {
///     TextBuilder::new(WidgetBuilder::new())
///         .with_wrap(WrapMode::Word)
///         .with_text(text)
///         .build(&mut ui.build_ctx())
/// }
/// ```
///
/// ## Background
///
/// If you need to have a text with some background, you should use [`crate::border::Border`] widget as a parent widget of your
/// text. **Caveat:** [`WidgetBuilder::with_background`] is ignored for [`Text`] widget!
///
/// ```rust,no_run
/// # use fyrox_ui::{
/// #     core::{color::Color, pool::Handle},
/// #     border::BorderBuilder, brush::Brush, text::TextBuilder, widget::WidgetBuilder, UiNode,
/// #     UserInterface,
/// # };
/// #
/// fn create_text_with_background(ui: &mut UserInterface, text: &str) -> Handle<UiNode> {
///     let text_widget =
///         TextBuilder::new(WidgetBuilder::new().with_foreground(Brush::Solid(Color::RED).into()))
///             .with_text(text)
///             .build(&mut ui.build_ctx());
///     BorderBuilder::new(
///         WidgetBuilder::new()
///             .with_child(text_widget) // <-- Text is now a child of the border
///             .with_background(Brush::Solid(Color::opaque(50, 50, 50)).into()),
///     )
///     .build(&mut ui.build_ctx())
/// }
/// ```
///
/// Keep in mind that now the text widget is a child widget of the border, so if you need to position the text, you should
/// position the border, not the text.
///
/// ## Fonts and colors
///
/// To set a color of the text just use [`WidgetBuilder::with_foreground`] while building the text instance:
///
/// ```rust,no_run
/// # use fyrox_ui::{
/// #     core::{color::Color, pool::Handle},
/// #     brush::Brush, text::TextBuilder, widget::WidgetBuilder, UiNode, UserInterface
/// # };
/// fn create_text(ui: &mut UserInterface, text: &str) -> Handle<UiNode> {
///     //               vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
///     TextBuilder::new(WidgetBuilder::new().with_foreground(Brush::Solid(Color::RED).into()))
///         .with_text(text)
///         .build(&mut ui.build_ctx())
/// }
/// ```
///
/// By default, text is created with default font, however it is possible to set any custom font:
///
/// ```rust
/// # use fyrox_resource::manager::ResourceManager;
/// # use fyrox_ui::{
/// #     core::{futures::executor::block_on, pool::Handle},
/// #     text::TextBuilder,
/// #     font::{Font, FontResource},
/// #     widget::WidgetBuilder,
/// #     UiNode, UserInterface,
/// # };
///
/// fn create_text(ui: &mut UserInterface, resource_manager: &ResourceManager, text: &str) -> Handle<UiNode> {
///     TextBuilder::new(WidgetBuilder::new())
///         .with_font(resource_manager.request::<Font>("path/to/your/font.ttf"))
///         .with_text(text)
///         .with_font_size(20.0f32.into())
///         .build(&mut ui.build_ctx())
/// }
/// ```
///
/// Please refer to [`crate::font::Font`] chapter to learn more about fonts.
///
/// ### Font size
///
/// Use [`fyrox::gui::text::TextBuilder::with_font_size`] or send [`TextMessage::font_size`] to your Text widget instance
/// to set the font size of it.
///
/// ## Shadows
///
/// Text widget supports shadows effect to add contrast to your text, which could be useful to make text readable independent
/// on the background colors. This effect could be used for subtitles. Shadows are pretty easy to add, all you need to do
/// is to enable them, setup desired thickness, offset and brush (solid color or gradient).
///
/// ```rust,no_run
/// # use fyrox_ui::{
/// #     core::{algebra::Vector2, color::Color, pool::Handle},
/// #     brush::Brush, text::TextBuilder, widget::WidgetBuilder, UiNode, UserInterface
/// # };
/// #
/// fn create_red_text_with_black_shadows(ui: &mut UserInterface, text: &str) -> Handle<UiNode> {
///     TextBuilder::new(WidgetBuilder::new().with_foreground(Brush::Solid(Color::RED).into()))
///         .with_text(text)
///         // Enable shadows.
///         .with_shadow(true)
///         // Black shadows.
///         .with_shadow_brush(Brush::Solid(Color::BLACK))
///         // 1px thick.
///         .with_shadow_dilation(1.0)
///         // Offset the shadow slightly to the right-bottom.
///         .with_shadow_offset(Vector2::new(1.0, 1.0))
///         .build(&mut ui.build_ctx())
/// }
/// ```
///
/// ## Messages
///
/// Text widget can accept the following list of messages at runtime (respective constructors are name with small letter -
/// `TextMessage::Text -> TextMessage::text(widget_handle, direction, text)`):
///
/// - [`TextMessage::Text`] - sets new text for a `Text` widget.
/// - [`TextMessage::Wrap`] - sets new [wrapping mode](Text#text-alignment-and-word-wrapping).
/// - [`TextMessage::Font`] - sets new [font](Text#fonts-and-colors)
/// - [`TextMessage::VerticalAlignment`] and `TextMessage::HorizontalAlignment` sets
/// [vertical and horizontal](Text#text-alignment-and-word-wrapping) text alignment respectively.
/// - [`TextMessage::Shadow`] - enables or disables [shadow casting](Text#shadows)
/// - [`TextMessage::ShadowDilation`] - sets "thickness" of the shadows under the tex.
/// - [`TextMessage::ShadowBrush`] - sets shadow brush (allows you to change color and even make shadow with color gradients).
/// - [`TextMessage::ShadowOffset`] - sets offset of the shadows.
///
/// An example of changing text at runtime could be something like this:
///
/// ```rust
/// # use fyrox_ui::{
/// #     core::pool::Handle,
/// #     message::{MessageDirection},
/// #     UiNode, UserInterface,
/// #     text::TextMessage
/// # };
/// fn request_change_text(ui: &UserInterface, text_widget_handle: Handle<UiNode>, text: &str) {
///     ui.send_message(TextMessage::text(
///         text_widget_handle,
///         MessageDirection::ToWidget,
///         text.to_owned(),
///     ))
/// }
/// ```
///
/// Please keep in mind, that like any other situation when you "changing" something via messages, you should remember
/// that the change is **not** immediate.
impl LiteText {
    /// sets the text of UI element. applied at the end of frame.
    pub fn set_text_async(&self, text: String) {
        with_script_context(|ctx| {
            ctx.ui().first_mut().send_message(text::TextMessage::text(
                self.handle,
                MessageDirection::ToWidget,
                text,
            ));
        })
    }

    /// current value (`set_text_async` doesn't affect this immediately)
    pub fn get_text(&self) -> String {
        with_script_context(|ctx| {
            ctx.ui().first()
                .nodes()[self.handle]
                .cast::<Text>()
                .as_ref()
                .unwrap()
                .formatted_text
                .borrow()
                .text()
        })
    }

    pub fn new(state: TextBuilder) -> LiteText {
        with_script_context(|ctx| {
            let mut wb = widget::WidgetBuilder::new();
            if let Some(foreground) = state.foreground {
                wb = wb.with_foreground(fyrox::gui::brush::Brush::from(foreground).into());
            }
            let mut builder = text::TextBuilder::new(wb);
            if let Some(font_size) = state.font_size {
                builder = builder.with_font_size(font_size.into());
            }
            LiteText {
                handle: builder.build(&mut ctx.ui().first_mut().build_ctx()),
            }
        })
    }
}

impl Externalizable for LiteText {
    fn to_external(&self) -> u128 {
        self.handle.encode_to_u128()
    }

    fn from_external(handle: u128) -> Self {
        Self {
            handle: Handle::decode_from_u128(handle),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq)]
#[lite_api]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<Color> for color::Color {
    fn from(value: Color) -> Self {
        color::Color::from_rgba(value.r, value.g, value.b, value.a)
    }
}

impl From<Brush> for brush::Brush {
    fn from(value: Brush) -> Self {
        if let Some(color) = value.solid_color {
            return brush::Brush::Solid(color.into());
        }
        if let Some(LinearGradient { from, to, stops }) = value.linear_gradient {
            return brush::Brush::LinearGradient {
                from: from.into(),
                to: to.into(),
                stops: stops.into_iter().map(|it| it.into()).collect(),
            }
        }
        if let Some(RadialGradient { center, stops }) = value.radial_gradient {
            return brush::Brush::RadialGradient {
                center: center.into(),
                stops: stops.into_iter().map(|it| it.into()).collect(),
            };
        }
        panic!("Unsupported brush type: {:?}", value)
    }
}

#[derive(Debug, Clone)]
#[lite_api(class=TextBuilder)]
pub struct TextBuilder {
    pub foreground: Option<Brush>,
    pub font_size: Option<f32>,
}

/// Brush defines a way to fill an arbitrary surface.
#[derive(Debug, Clone, PartialEq)]
#[lite_api]
pub struct Brush {
    /// A brush, that fills a surface with a solid color.
    pub solid_color: Option<Color>,

    /// A brush, that fills a surface with a linear gradient, which is defined by two points in local coordinates
    /// and a set of stop points. See [`GradientPoint`] for more info.
    pub linear_gradient: Option<LinearGradient>,
    
    /// A brush, that fills a surface with a radial gradient, which is defined by a center point in local coordinates
    /// and a set of stop points. See [`GradientPoint`] for more info.
    pub radial_gradient: Option<RadialGradient>,
}
#[derive(Debug, Clone, PartialEq)]
#[lite_api]
pub struct LinearGradient  {
    /// Beginning of the gradient in local coordinates.
    pub from: PodVector2,
    /// End of the gradient in local coordinates.
    pub to: PodVector2,
    /// Stops of the gradient.
    pub stops: Vec<GradientPoint>,
}

#[derive(Debug, Clone, PartialEq)]
#[lite_api]
pub struct RadialGradient {
    /// Center of the gradient in local coordinates.
    pub center: PodVector2,
    /// Stops of the gradient.
    pub stops: Vec<GradientPoint>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[lite_api(class=GradientPoint)]
pub struct GradientPoint {
    /// A distance from an origin of the gradient.
    pub stop: f32,
    /// Color of the point.
    pub color: Color,
}

impl From<GradientPoint> for brush::GradientPoint {
    fn from(value: GradientPoint) -> Self {
        Self {
            stop: value.stop,
            color: value.color.into(),
        }
    }
}
