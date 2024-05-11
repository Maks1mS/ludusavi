use iced::{alignment, widget::tooltip, Length};

use crate::{
    gui::{
        common::Message,
        style,
        widget::{text, Button, Container, Tooltip},
    },
    lang::TRANSLATOR,
    scan::ScanChange,
};

const CHANGE_BADGE_WIDTH: f32 = 10.0;

#[derive(Default)]
pub struct Badge {
    text: String,
    change: Option<ScanChange>,
    tooltip: Option<String>,
    on_press: Option<Message>,
    faded: bool,
    width: Option<Length>,
}

impl Badge {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            change: None,
            tooltip: None,
            on_press: None,
            faded: false,
            width: None,
        }
    }

    pub fn scan_change(change: ScanChange) -> Self {
        Self {
            text: change.symbol().to_string(),
            change: Some(change),
            tooltip: match change {
                ScanChange::New => Some(TRANSLATOR.new_tooltip()),
                ScanChange::Different => Some(TRANSLATOR.updated_tooltip()),
                ScanChange::Removed => Some(TRANSLATOR.removed_tooltip()),
                ScanChange::Same => None,
                ScanChange::Unknown => None,
            },
            width: Some(Length::Fixed(CHANGE_BADGE_WIDTH)),
            ..Default::default()
        }
    }

    pub fn new_entry() -> Self {
        Self::scan_change(ScanChange::New)
    }

    pub fn new_entry_with_count(count: usize) -> Self {
        Self {
            text: format!("{}{}", crate::lang::ADD_SYMBOL, count),
            change: Some(ScanChange::New),
            tooltip: Some(TRANSLATOR.new_tooltip()),
            ..Default::default()
        }
    }

    pub fn changed_entry() -> Self {
        Self::scan_change(ScanChange::Different)
    }

    pub fn removed_entry() -> Self {
        Self::scan_change(ScanChange::Removed)
    }

    pub fn changed_entry_with_count(count: usize) -> Self {
        Self {
            text: format!("{}{}", crate::lang::CHANGE_SYMBOL, count),
            change: Some(ScanChange::Different),
            tooltip: Some(TRANSLATOR.updated_tooltip()),
            ..Default::default()
        }
    }

    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    pub fn faded(mut self, faded: bool) -> Self {
        self.faded = faded;
        self
    }

    pub fn tooltip(mut self, tooltip: String) -> Self {
        self.tooltip = Some(tooltip);
        self
    }

    pub fn view(self) -> Container<'static> {
        Container::new({
            let content = Container::new(
                text(self.text)
                    .size(12)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .width(self.width.unwrap_or(Length::Shrink)),
            )
            .padding([2, 10, 2, 10])
            .style(match self.change {
                None => match self.on_press.as_ref() {
                    Some(Message::FilterDuplicates { game: None, .. }) => style::Container::BadgeActivated,
                    _ if self.faded => style::Container::BadgeFaded,
                    _ => style::Container::Badge,
                },
                Some(change) => style::Container::ChangeBadge(change),
            });

            let content = match self.tooltip {
                None => content,
                Some(tooltip) => Container::new(
                    Tooltip::new(content, text(tooltip).size(16), tooltip::Position::Top)
                        .gap(5)
                        .style(style::Container::Tooltip),
                ),
            };

            match self.on_press {
                Some(message) => Container::new(
                    Button::new(content)
                        .padding(0)
                        .on_press(message)
                        .style(style::Button::Badge),
                ),
                None => Container::new(content),
            }
        })
        .padding([1, 0, 0, 0])
        .center_x()
        .center_y()
    }
}
