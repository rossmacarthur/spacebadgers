use std::borrow::Cow;

use crate::{Badge, ColorPalette};

/// [Badge] builder.
pub struct BadgeBuilder {
    color_palette: Cow<'static, ColorPalette>,
    status: Option<Cow<'static, str>>,
    label: Option<Cow<'static, str>>,
    color: Option<Cow<'static, str>>,
    label_color: Option<Cow<'static, str>>,
    icon: Option<Cow<'static, str>>,
    icon_width: Option<u32>,
    scale: f32,
}

impl BadgeBuilder {
    /// Construct a new [BadgeBuilder] with default values.
    pub fn new() -> Self {
        let color_palette = Cow::Borrowed(&crate::color_palettes::BADGEN);
        let scale = 1.0;
        Self {
            color_palette,
            status: None,
            label: None,
            color: None,
            label_color: None,
            icon: None,
            icon_width: None,
            scale,
        }
    }

    /// Set the [ColorPalette].
    pub fn color_palette(mut self, color_palette: impl Into<Cow<'static, ColorPalette>>) -> Self {
        self.color_palette = color_palette.into();
        self
    }

    /// Set the status text.
    pub fn status(mut self, status: impl Into<Cow<'static, str>>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Set the label text.
    pub fn label(mut self, label: impl Into<Cow<'static, str>>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the color.
    pub fn color(mut self, color: impl Into<Cow<'static, str>>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Set an optional color.
    pub fn color_option(mut self, color: Option<impl Into<Cow<'static, str>>>) -> Self {
        self.color = color.map(Into::into);
        self
    }

    /// Set the label color.
    pub fn label_color(mut self, label_color: impl Into<Cow<'static, str>>) -> Self {
        self.label_color = Some(label_color.into());
        self
    }

    /// Set the badge scale.
    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    /// Build the [Badge].
    pub fn build(self) -> Badge {
        Badge {
            color_palette: self.color_palette,
            status: self.status.unwrap_or_default(),
            label: self.label,
            color: self.color,
            label_color: self.label_color,
            icon: self.icon,
            icon_width: self.icon_width,
            scale: self.scale,
        }
    }
}
