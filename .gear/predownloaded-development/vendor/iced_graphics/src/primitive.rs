//! Draw using different graphical primitives.
use crate::core::alignment;
use crate::core::image;
use crate::core::svg;
use crate::core::text;
use crate::core::{
    Background, Border, Color, Font, Pixels, Point, Rectangle, Shadow,
    Transformation, Vector,
};
use crate::text::editor;
use crate::text::paragraph;

use std::sync::Arc;

/// A rendering primitive.
#[derive(Debug, Clone, PartialEq)]
pub enum Primitive<T> {
    /// A text primitive
    Text {
        /// The contents of the text.
        content: String,
        /// The bounds of the text.
        bounds: Rectangle,
        /// The color of the text.
        color: Color,
        /// The size of the text in logical pixels.
        size: Pixels,
        /// The line height of the text.
        line_height: text::LineHeight,
        /// The font of the text.
        font: Font,
        /// The horizontal alignment of the text.
        horizontal_alignment: alignment::Horizontal,
        /// The vertical alignment of the text.
        vertical_alignment: alignment::Vertical,
        /// The shaping strategy of the text.
        shaping: text::Shaping,
        /// The clip bounds of the text.
        clip_bounds: Rectangle,
    },
    /// A paragraph primitive
    Paragraph {
        /// The [`paragraph::Weak`] reference.
        paragraph: paragraph::Weak,
        /// The position of the paragraph.
        position: Point,
        /// The color of the paragraph.
        color: Color,
        /// The clip bounds of the paragraph.
        clip_bounds: Rectangle,
    },
    /// An editor primitive
    Editor {
        /// The [`editor::Weak`] reference.
        editor: editor::Weak,
        /// The position of the editor.
        position: Point,
        /// The color of the editor.
        color: Color,
        /// The clip bounds of the editor.
        clip_bounds: Rectangle,
    },
    /// A raw `cosmic-text` primitive
    RawText(crate::text::Raw),
    /// A quad primitive
    Quad {
        /// The bounds of the quad
        bounds: Rectangle,
        /// The background of the quad
        background: Background,
        /// The [`Border`] of the quad
        border: Border,
        /// The [`Shadow`] of the quad
        shadow: Shadow,
    },
    /// An image primitive
    Image {
        /// The handle of the image
        handle: image::Handle,
        /// The filter method of the image
        filter_method: image::FilterMethod,
        /// The bounds of the image
        bounds: Rectangle,
    },
    /// An SVG primitive
    Svg {
        /// The path of the SVG file
        handle: svg::Handle,

        /// The [`Color`] filter
        color: Option<Color>,

        /// The bounds of the viewport
        bounds: Rectangle,
    },
    /// A group of primitives
    Group {
        /// The primitives of the group
        primitives: Vec<Primitive<T>>,
    },
    /// A clip primitive
    Clip {
        /// The bounds of the clip
        bounds: Rectangle,
        /// The content of the clip
        content: Box<Primitive<T>>,
    },
    /// A primitive that applies a [`Transformation`]
    Transform {
        /// The [`Transformation`]
        transformation: Transformation,

        /// The primitive to transform
        content: Box<Primitive<T>>,
    },
    /// A cached primitive.
    ///
    /// This can be useful if you are implementing a widget where primitive
    /// generation is expensive.
    Cache {
        /// The cached primitive
        content: Arc<Primitive<T>>,
    },
    /// A backend-specific primitive.
    Custom(T),
}

impl<T> Primitive<T> {
    /// Groups the current [`Primitive`].
    pub fn group(primitives: Vec<Self>) -> Self {
        Self::Group { primitives }
    }

    /// Clips the current [`Primitive`].
    pub fn clip(self, bounds: Rectangle) -> Self {
        Self::Clip {
            bounds,
            content: Box::new(self),
        }
    }

    /// Translates the current [`Primitive`].
    pub fn translate(self, translation: Vector) -> Self {
        Self::Transform {
            transformation: Transformation::translate(
                translation.x,
                translation.y,
            ),
            content: Box::new(self),
        }
    }

    /// Transforms the current [`Primitive`].
    pub fn transform(self, transformation: Transformation) -> Self {
        Self::Transform {
            transformation,
            content: Box::new(self),
        }
    }
}
