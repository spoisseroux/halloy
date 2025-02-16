#![allow(dead_code)]
use data::theme::randomize_color;
use data::user::NickColor;
use data::{message, Config};
use iced::widget::span;
use iced::{alignment, border};

use crate::{font, Theme};

pub use self::anchored_overlay::anchored_overlay;
pub use self::color_picker::color_picker;
pub use self::combo_box::combo_box;
pub use self::context_menu::context_menu;
pub use self::decorate::decorate;
pub use self::double_pass::double_pass;
pub use self::key_press::key_press;
pub use self::modal::modal;
pub use self::selectable_rich_text::selectable_rich_text;
pub use self::selectable_text::selectable_text;
pub use self::shortcut::shortcut;
pub use self::tooltip::tooltip;

pub mod anchored_overlay;
pub mod collection;
pub mod color_picker;
pub mod combo_box;
pub mod context_menu;
pub mod decorate;
pub mod double_click;
pub mod double_pass;
pub mod hover;
pub mod key_press;
pub mod modal;
pub mod selectable_rich_text;
pub mod selectable_text;
pub mod shortcut;
pub mod tooltip;

pub type Renderer = iced::Renderer;
pub type Element<'a, Message> = iced::Element<'a, Message, Theme, Renderer>;
pub type Content<'a, Message> = iced::widget::pane_grid::Content<'a, Message, Theme, Renderer>;
pub type TitleBar<'a, Message> = iced::widget::pane_grid::TitleBar<'a, Message, Theme, Renderer>;
pub type Column<'a, Message> = iced::widget::Column<'a, Message, Theme, Renderer>;
pub type Row<'a, Message> = iced::widget::Row<'a, Message, Theme, Renderer>;
pub type Text<'a> = iced::widget::Text<'a, Theme, Renderer>;
pub type Container<'a, Message> = iced::widget::Container<'a, Message, Theme, Renderer>;
pub type Button<'a, Message> = iced::widget::Button<'a, Message, Theme>;

pub fn message_content<'a, M: 'a>(
    content: &'a message::Content,
    theme: &'a Theme,
    on_link: impl Fn(message::Link) -> M + 'a,
    style: impl Fn(&Theme) -> selectable_text::Style + 'a,
    config: &Config,
) -> Element<'a, M> {
    match content {
        data::message::Content::Plain(text) => selectable_text(text).style(style).into(),
        data::message::Content::Fragments(fragments) => selectable_rich_text(
            fragments
                .iter()
                .map(|fragment| match fragment {
                    data::message::Fragment::Text(s) => span(s),
                    data::message::Fragment::User(user) => {
                        let color_kind = &config.buffer.channel.message.nickname_color;

                        let NickColor { seed, color } =
                            user.nick_color(theme.colors(), *color_kind);

                        let color = match seed {
                            Some(seed) => randomize_color(color, &seed),
                            None => theme.colors().text.primary,
                        };

                        span(user.nickname().to_string())
                            .color(color)
                            .link(message::Link::User(user.clone()))
                    }
                    data::message::Fragment::Url(s) => span(s.as_str())
                        .color(theme.colors().buffer.url)
                        .link(message::Link::Url(s.as_str().to_string())),
                    data::message::Fragment::Formatted { text, formatting } => {
                        let mut span = span(text)
                            .color_maybe(
                                formatting
                                    .fg
                                    .and_then(|color| color.into_iced(theme.colors())),
                            )
                            .background_maybe(
                                formatting
                                    .bg
                                    .and_then(|color| color.into_iced(theme.colors())),
                            )
                            .underline(formatting.underline)
                            .strikethrough(formatting.strikethrough);

                        if formatting.monospace {
                            span = span
                                .padding([0, 4])
                                .color(theme.colors().buffer.code)
                                .border(
                                    border::rounded(3)
                                        .color(theme.colors().general.border)
                                        .width(1),
                                );
                        }

                        match (formatting.bold, formatting.italics) {
                            (true, true) => {
                                span = span.font(font::MONO_BOLD_ITALICS.clone());
                            }
                            (true, false) => {
                                span = span.font(font::MONO_BOLD.clone());
                            }
                            (false, true) => {
                                span = span.font(font::MONO_ITALICS.clone());
                            }
                            (false, false) => {}
                        }

                        span
                    }
                })
                .collect::<Vec<_>>(),
        )
        .on_link(on_link)
        .style(style)
        .into(),
    }
}

pub fn message_marker<'a, M: 'a>(
    width: Option<f32>,
    style: impl Fn(&Theme) -> selectable_text::Style + 'a,
) -> Element<'a, M> {
    let marker = selectable_text(MESSAGE_MARKER_TEXT);

    if let Some(width) = width {
        marker
            .width(width)
            .horizontal_alignment(alignment::Horizontal::Right)
    } else {
        marker
    }
    .style(style)
    .into()
}

pub const MESSAGE_MARKER_TEXT: &str = " ∙";
