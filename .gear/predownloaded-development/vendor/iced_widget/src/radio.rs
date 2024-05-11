//! Create choices using radio buttons.
use crate::core::alignment;
use crate::core::event::{self, Event};
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::text;
use crate::core::touch;
use crate::core::widget;
use crate::core::widget::tree::{self, Tree};
use crate::core::{
    Border, Clipboard, Element, Layout, Length, Pixels, Rectangle, Shell, Size,
    Widget,
};

pub use iced_style::radio::{Appearance, StyleSheet};

/// A circular button representing a choice.
///
/// # Example
/// ```no_run
/// # type Radio<Message> =
/// #     iced_widget::Radio<Message, iced_widget::style::Theme, iced_widget::renderer::Renderer>;
/// #
/// # use iced_widget::column;
/// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// pub enum Choice {
///     A,
///     B,
///     C,
///     All,
/// }
///
/// #[derive(Debug, Clone, Copy)]
/// pub enum Message {
///     RadioSelected(Choice),
/// }
///
/// let selected_choice = Some(Choice::A);
///
/// let a = Radio::new(
///     "A",
///     Choice::A,
///     selected_choice,
///     Message::RadioSelected,
/// );
///
/// let b = Radio::new(
///     "B",
///     Choice::B,
///     selected_choice,
///     Message::RadioSelected,
/// );
///
/// let c = Radio::new(
///     "C",
///     Choice::C,
///     selected_choice,
///     Message::RadioSelected,
/// );
///
/// let all = Radio::new(
///     "All of the above",
///     Choice::All,
///     selected_choice,
///     Message::RadioSelected
/// );
///
/// let content = column![a, b, c, all];
/// ```
#[allow(missing_debug_implementations)]
pub struct Radio<Message, Theme = crate::Theme, Renderer = crate::Renderer>
where
    Theme: StyleSheet,
    Renderer: text::Renderer,
{
    is_selected: bool,
    on_click: Message,
    label: String,
    width: Length,
    size: f32,
    spacing: f32,
    text_size: Option<Pixels>,
    text_line_height: text::LineHeight,
    text_shaping: text::Shaping,
    font: Option<Renderer::Font>,
    style: Theme::Style,
}

impl<Message, Theme, Renderer> Radio<Message, Theme, Renderer>
where
    Message: Clone,
    Theme: StyleSheet,
    Renderer: text::Renderer,
{
    /// The default size of a [`Radio`] button.
    pub const DEFAULT_SIZE: f32 = 28.0;

    /// The default spacing of a [`Radio`] button.
    pub const DEFAULT_SPACING: f32 = 15.0;

    /// Creates a new [`Radio`] button.
    ///
    /// It expects:
    ///   * the value related to the [`Radio`] button
    ///   * the label of the [`Radio`] button
    ///   * the current selected value
    ///   * a function that will be called when the [`Radio`] is selected. It
    ///   receives the value of the radio and must produce a `Message`.
    pub fn new<F, V>(
        label: impl Into<String>,
        value: V,
        selected: Option<V>,
        f: F,
    ) -> Self
    where
        V: Eq + Copy,
        F: FnOnce(V) -> Message,
    {
        Radio {
            is_selected: Some(value) == selected,
            on_click: f(value),
            label: label.into(),
            width: Length::Shrink,
            size: Self::DEFAULT_SIZE,
            spacing: Self::DEFAULT_SPACING, //15
            text_size: None,
            text_line_height: text::LineHeight::default(),
            text_shaping: text::Shaping::Basic,
            font: None,
            style: Default::default(),
        }
    }

    /// Sets the size of the [`Radio`] button.
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = size.into().0;
        self
    }

    /// Sets the width of the [`Radio`] button.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the spacing between the [`Radio`] button and the text.
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into().0;
        self
    }

    /// Sets the text size of the [`Radio`] button.
    pub fn text_size(mut self, text_size: impl Into<Pixels>) -> Self {
        self.text_size = Some(text_size.into());
        self
    }

    /// Sets the text [`text::LineHeight`] of the [`Radio`] button.
    pub fn text_line_height(
        mut self,
        line_height: impl Into<text::LineHeight>,
    ) -> Self {
        self.text_line_height = line_height.into();
        self
    }

    /// Sets the [`text::Shaping`] strategy of the [`Radio`] button.
    pub fn text_shaping(mut self, shaping: text::Shaping) -> Self {
        self.text_shaping = shaping;
        self
    }

    /// Sets the text font of the [`Radio`] button.
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Sets the style of the [`Radio`] button.
    pub fn style(mut self, style: impl Into<Theme::Style>) -> Self {
        self.style = style.into();
        self
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Radio<Message, Theme, Renderer>
where
    Message: Clone,
    Theme: StyleSheet + crate::text::StyleSheet,
    Renderer: text::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<widget::text::State<Renderer::Paragraph>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(widget::text::State::<Renderer::Paragraph>::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::next_to_each_other(
            &limits.width(self.width),
            self.spacing,
            |_| layout::Node::new(Size::new(self.size, self.size)),
            |limits| {
                let state = tree
                    .state
                    .downcast_mut::<widget::text::State<Renderer::Paragraph>>();

                widget::text::layout(
                    state,
                    renderer,
                    limits,
                    self.width,
                    Length::Shrink,
                    &self.label,
                    self.text_line_height,
                    self.text_size,
                    self.font,
                    alignment::Horizontal::Left,
                    alignment::Vertical::Top,
                    self.text_shaping,
                )
            },
        )
    }

    fn on_event(
        &mut self,
        _state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor.is_over(layout.bounds()) {
                    shell.publish(self.on_click.clone());

                    return event::Status::Captured;
                }
            }
            _ => {}
        }

        event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let is_mouse_over = cursor.is_over(layout.bounds());

        let mut children = layout.children();

        let custom_style = if is_mouse_over {
            theme.hovered(&self.style, self.is_selected)
        } else {
            theme.active(&self.style, self.is_selected)
        };

        {
            let layout = children.next().unwrap();
            let bounds = layout.bounds();

            let size = bounds.width;
            let dot_size = size / 2.0;

            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        radius: (size / 2.0).into(),
                        width: custom_style.border_width,
                        color: custom_style.border_color,
                    },
                    ..renderer::Quad::default()
                },
                custom_style.background,
            );

            if self.is_selected {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x: bounds.x + dot_size / 2.0,
                            y: bounds.y + dot_size / 2.0,
                            width: bounds.width - dot_size,
                            height: bounds.height - dot_size,
                        },
                        border: Border::with_radius(dot_size / 2.0),
                        ..renderer::Quad::default()
                    },
                    custom_style.dot_color,
                );
            }
        }

        {
            let label_layout = children.next().unwrap();

            crate::text::draw(
                renderer,
                style,
                label_layout,
                tree.state.downcast_ref(),
                crate::text::Appearance {
                    color: custom_style.text_color,
                },
                viewport,
            );
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Radio<Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: StyleSheet + crate::text::StyleSheet + 'a,
    Renderer: 'a + text::Renderer,
{
    fn from(
        radio: Radio<Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(radio)
    }
}
