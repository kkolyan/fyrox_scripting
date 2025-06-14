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
use crate::lite_color::LiteColor;

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

#[lite_api(class=Text)]

/// Text is a simple widget that allows you to print text on screen. It has various options like word wrapping, text
/// alignment, and so on.
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
    pub solid_color: Option<LiteColor>,

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
    pub color: LiteColor,
}

impl From<GradientPoint> for brush::GradientPoint {
    fn from(value: GradientPoint) -> Self {
        Self {
            stop: value.stop,
            color: value.color.into(),
        }
    }
}
