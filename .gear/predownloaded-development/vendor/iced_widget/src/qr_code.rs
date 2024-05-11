//! Encode and display information in a QR code.
use crate::canvas;
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer::{self, Renderer as _};
use crate::core::widget::tree::{self, Tree};
use crate::core::{
    Element, Layout, Length, Point, Rectangle, Size, Vector, Widget,
};
use crate::graphics::geometry::Renderer as _;
use crate::Renderer;

use std::cell::RefCell;
use thiserror::Error;

pub use crate::style::qr_code::{Appearance, StyleSheet};

const DEFAULT_CELL_SIZE: u16 = 4;
const QUIET_ZONE: usize = 2;

/// A type of matrix barcode consisting of squares arranged in a grid which
/// can be read by an imaging device, such as a camera.
#[derive(Debug)]
pub struct QRCode<'a, Theme = crate::Theme>
where
    Theme: StyleSheet,
{
    data: &'a Data,
    cell_size: u16,
    style: Theme::Style,
}

impl<'a, Theme> QRCode<'a, Theme>
where
    Theme: StyleSheet,
{
    /// Creates a new [`QRCode`] with the provided [`Data`].
    pub fn new(data: &'a Data) -> Self {
        Self {
            data,
            cell_size: DEFAULT_CELL_SIZE,
            style: Default::default(),
        }
    }

    /// Sets the size of the squares of the grid cell of the [`QRCode`].
    pub fn cell_size(mut self, cell_size: u16) -> Self {
        self.cell_size = cell_size;
        self
    }

    /// Sets the style of the [`QRCode`].
    pub fn style(mut self, style: impl Into<Theme::Style>) -> Self {
        self.style = style.into();
        self
    }
}

impl<'a, Message, Theme> Widget<Message, Theme, Renderer> for QRCode<'a, Theme>
where
    Theme: StyleSheet,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        let side_length = (self.data.width + 2 * QUIET_ZONE) as f32
            * f32::from(self.cell_size);

        layout::Node::new(Size::new(side_length, side_length))
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();

        let bounds = layout.bounds();
        let side_length = self.data.width + 2 * QUIET_ZONE;

        let appearance = theme.appearance(&self.style);
        let mut last_appearance = state.last_appearance.borrow_mut();

        if Some(appearance) != *last_appearance {
            self.data.cache.clear();

            *last_appearance = Some(appearance);
        }

        // Reuse cache if possible
        let geometry = self.data.cache.draw(renderer, bounds.size(), |frame| {
            // Scale units to cell size
            frame.scale(self.cell_size);

            // Draw background
            frame.fill_rectangle(
                Point::ORIGIN,
                Size::new(side_length as f32, side_length as f32),
                appearance.background,
            );

            // Avoid drawing on the quiet zone
            frame.translate(Vector::new(QUIET_ZONE as f32, QUIET_ZONE as f32));

            // Draw contents
            self.data
                .contents
                .iter()
                .enumerate()
                .filter(|(_, value)| **value == qrcode::Color::Dark)
                .for_each(|(index, _)| {
                    let row = index / self.data.width;
                    let column = index % self.data.width;

                    frame.fill_rectangle(
                        Point::new(column as f32, row as f32),
                        Size::UNIT,
                        appearance.cell,
                    );
                });
        });

        renderer.with_translation(
            bounds.position() - Point::ORIGIN,
            |renderer| {
                renderer.draw(vec![geometry]);
            },
        );
    }
}

impl<'a, Message, Theme> From<QRCode<'a, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Theme: StyleSheet + 'a,
{
    fn from(qr_code: QRCode<'a, Theme>) -> Self {
        Self::new(qr_code)
    }
}

/// The data of a [`QRCode`].
///
/// It stores the contents that will be displayed.
#[derive(Debug)]
pub struct Data {
    contents: Vec<qrcode::Color>,
    width: usize,
    cache: canvas::Cache,
}

impl Data {
    /// Creates a new [`Data`] with the provided data.
    ///
    /// This method uses an [`ErrorCorrection::Medium`] and chooses the smallest
    /// size to display the data.
    pub fn new(data: impl AsRef<[u8]>) -> Result<Self, Error> {
        let encoded = qrcode::QrCode::new(data)?;

        Ok(Self::build(encoded))
    }

    /// Creates a new [`Data`] with the provided [`ErrorCorrection`].
    pub fn with_error_correction(
        data: impl AsRef<[u8]>,
        error_correction: ErrorCorrection,
    ) -> Result<Self, Error> {
        let encoded = qrcode::QrCode::with_error_correction_level(
            data,
            error_correction.into(),
        )?;

        Ok(Self::build(encoded))
    }

    /// Creates a new [`Data`] with the provided [`Version`] and
    /// [`ErrorCorrection`].
    pub fn with_version(
        data: impl AsRef<[u8]>,
        version: Version,
        error_correction: ErrorCorrection,
    ) -> Result<Self, Error> {
        let encoded = qrcode::QrCode::with_version(
            data,
            version.into(),
            error_correction.into(),
        )?;

        Ok(Self::build(encoded))
    }

    fn build(encoded: qrcode::QrCode) -> Self {
        let width = encoded.width();
        let contents = encoded.into_colors();

        Self {
            contents,
            width,
            cache: canvas::Cache::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The size of a [`QRCode`].
///
/// The higher the version the larger the grid of cells, and therefore the more
/// information the [`QRCode`] can carry.
pub enum Version {
    /// A normal QR code version. It should be between 1 and 40.
    Normal(u8),

    /// A micro QR code version. It should be between 1 and 4.
    Micro(u8),
}

impl From<Version> for qrcode::Version {
    fn from(version: Version) -> Self {
        match version {
            Version::Normal(v) => qrcode::Version::Normal(i16::from(v)),
            Version::Micro(v) => qrcode::Version::Micro(i16::from(v)),
        }
    }
}

/// The error correction level.
///
/// It controls the amount of data that can be damaged while still being able
/// to recover the original information.
///
/// A higher error correction level allows for more corrupted data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCorrection {
    /// Low error correction. 7% of the data can be restored.
    Low,
    /// Medium error correction. 15% of the data can be restored.
    Medium,
    /// Quartile error correction. 25% of the data can be restored.
    Quartile,
    /// High error correction. 30% of the data can be restored.
    High,
}

impl From<ErrorCorrection> for qrcode::EcLevel {
    fn from(ec_level: ErrorCorrection) -> Self {
        match ec_level {
            ErrorCorrection::Low => qrcode::EcLevel::L,
            ErrorCorrection::Medium => qrcode::EcLevel::M,
            ErrorCorrection::Quartile => qrcode::EcLevel::Q,
            ErrorCorrection::High => qrcode::EcLevel::H,
        }
    }
}

/// An error that occurred when building a [`Data`] for a [`QRCode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum Error {
    /// The data is too long to encode in a QR code for the chosen [`Version`].
    #[error(
        "The data is too long to encode in a QR code for the chosen version"
    )]
    DataTooLong,

    /// The chosen [`Version`] and [`ErrorCorrection`] combination is invalid.
    #[error(
        "The chosen version and error correction level combination is invalid."
    )]
    InvalidVersion,

    /// One or more characters in the provided data are not supported by the
    /// chosen [`Version`].
    #[error(
        "One or more characters in the provided data are not supported by the \
        chosen version"
    )]
    UnsupportedCharacterSet,

    /// The chosen ECI designator is invalid. A valid designator should be
    /// between 0 and 999999.
    #[error(
        "The chosen ECI designator is invalid. A valid designator should be \
        between 0 and 999999."
    )]
    InvalidEciDesignator,

    /// A character that does not belong to the character set was found.
    #[error("A character that does not belong to the character set was found")]
    InvalidCharacter,
}

impl From<qrcode::types::QrError> for Error {
    fn from(error: qrcode::types::QrError) -> Self {
        use qrcode::types::QrError;

        match error {
            QrError::DataTooLong => Error::DataTooLong,
            QrError::InvalidVersion => Error::InvalidVersion,
            QrError::UnsupportedCharacterSet => Error::UnsupportedCharacterSet,
            QrError::InvalidEciDesignator => Error::InvalidEciDesignator,
            QrError::InvalidCharacter => Error::InvalidCharacter,
        }
    }
}

#[derive(Default)]
struct State {
    last_appearance: RefCell<Option<Appearance>>,
}
