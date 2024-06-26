//! A compositor is responsible for initializing a renderer and managing window
//! surfaces.
use crate::{Error, Viewport};

use crate::core::Color;
use crate::futures::{MaybeSend, MaybeSync};

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use thiserror::Error;

/// A graphics compositor that can draw to windows.
pub trait Compositor: Sized {
    /// The settings of the backend.
    type Settings: Default;

    /// The iced renderer of the backend.
    type Renderer: iced_core::Renderer;

    /// The surface of the backend.
    type Surface;

    /// Creates a new [`Compositor`].
    fn new<W: Window + Clone>(
        settings: Self::Settings,
        compatible_window: W,
    ) -> Result<Self, Error>;

    /// Creates a [`Self::Renderer`] for the [`Compositor`].
    fn create_renderer(&self) -> Self::Renderer;

    /// Crates a new [`Surface`] for the given window.
    ///
    /// [`Surface`]: Self::Surface
    fn create_surface<W: Window + Clone>(
        &mut self,
        window: W,
        width: u32,
        height: u32,
    ) -> Self::Surface;

    /// Configures a new [`Surface`] with the given dimensions.
    ///
    /// [`Surface`]: Self::Surface
    fn configure_surface(
        &mut self,
        surface: &mut Self::Surface,
        width: u32,
        height: u32,
    );

    /// Returns [`Information`] used by this [`Compositor`].
    fn fetch_information(&self) -> Information;

    /// Presents the [`Renderer`] primitives to the next frame of the given [`Surface`].
    ///
    /// [`Renderer`]: Self::Renderer
    /// [`Surface`]: Self::Surface
    fn present<T: AsRef<str>>(
        &mut self,
        renderer: &mut Self::Renderer,
        surface: &mut Self::Surface,
        viewport: &Viewport,
        background_color: Color,
        overlay: &[T],
    ) -> Result<(), SurfaceError>;

    /// Screenshots the current [`Renderer`] primitives to an offscreen texture, and returns the bytes of
    /// the texture ordered as `RGBA` in the `sRGB` color space.
    ///
    /// [`Renderer`]: Self::Renderer
    fn screenshot<T: AsRef<str>>(
        &mut self,
        renderer: &mut Self::Renderer,
        surface: &mut Self::Surface,
        viewport: &Viewport,
        background_color: Color,
        overlay: &[T],
    ) -> Vec<u8>;
}

/// A window that can be used in a [`Compositor`].
///
/// This is just a convenient super trait of the `raw-window-handle`
/// traits.
pub trait Window:
    HasWindowHandle + HasDisplayHandle + MaybeSend + MaybeSync + 'static
{
}

impl<T> Window for T where
    T: HasWindowHandle + HasDisplayHandle + MaybeSend + MaybeSync + 'static
{
}

/// Result of an unsuccessful call to [`Compositor::present`].
#[derive(Clone, PartialEq, Eq, Debug, Error)]
pub enum SurfaceError {
    /// A timeout was encountered while trying to acquire the next frame.
    #[error(
        "A timeout was encountered while trying to acquire the next frame"
    )]
    Timeout,
    /// The underlying surface has changed, and therefore the surface must be updated.
    #[error(
        "The underlying surface has changed, and therefore the surface must be updated."
    )]
    Outdated,
    /// The swap chain has been lost and needs to be recreated.
    #[error("The surface has been lost and needs to be recreated")]
    Lost,
    /// There is no more memory left to allocate a new frame.
    #[error("There is no more memory left to allocate a new frame")]
    OutOfMemory,
}

/// Contains information about the graphics (e.g. graphics adapter, graphics backend).
#[derive(Debug)]
pub struct Information {
    /// Contains the graphics adapter.
    pub adapter: String,
    /// Contains the graphics backend.
    pub backend: String,
}
