pub enum Query<T, U> {
    Pending,
    Loaded(T),
    Error(U),
}

pub async fn read_file(
    pathname: std::path::PathBuf,
) -> std::result::Result<String, std::io::ErrorKind> {
    tokio::fs::read_to_string(pathname)
        .await
        .map_err(|e| e.kind())
}

pub const ICON_DELETE: char = '\u{e801}';
pub const ICON_NEW_FOLDER: char = '\u{e802}';
pub const ICON_EDIT: char = '\u{e803}';
pub const ICON_DOWN_SMALL: char = '\u{e800}';
pub const ICON_RIGHT_SMALL: char = '\u{e804}';
pub const ICON_DOWN: char = '\u{e805}';
pub const ICON_RIGHT: char = '\u{e807}';

pub fn icon<'a, Message>(code_point: char) -> iced::Element<'a, Message> {
    const ICON_FONT: iced::Font = iced::Font::with_name("darkstone-icons");
    iced::widget::text(code_point).font(ICON_FONT).into()
}

pub fn button_no_bg_active(
    theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    match status {
        iced::widget::button::Status::Active => iced::widget::button::Style {
            background: Some(theme.extended_palette().background.strong.color.into()),
            text_color: theme.extended_palette().background.base.text.into(),
            border: iced::Border {
                width: 0.0,
                color: theme.extended_palette().background.base.color.into(),
                radius: 4.0.into(),
            },
            ..Default::default()
        },
        iced::widget::button::Status::Hovered => iced::widget::button::Style {
            background: Some(theme.extended_palette().background.strong.color.into()),
            text_color: theme.extended_palette().background.base.text.into(),
            border: iced::Border {
                width: 0.0,
                color: theme.extended_palette().background.base.color.into(),
                radius: 4.0.into(),
            },
            ..Default::default()
        },
        iced::widget::button::Status::Pressed => iced::widget::button::Style {
            background: Some(theme.palette().background.into()),
            text_color: theme.extended_palette().background.base.text.into(),
            border: iced::Border {
                width: 0.0,
                color: theme.extended_palette().background.base.color.into(),
                radius: 4.0.into(),
            },
            ..Default::default()
        },
        _ => Default::default(),
    }
}
pub fn button_no_bg(
    theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    match status {
        iced::widget::button::Status::Active => iced::widget::button::Style {
            background: None,
            text_color: theme.extended_palette().background.base.text.into(),
            border: iced::Border {
                width: 0.0,
                color: theme.extended_palette().background.base.color.into(),
                radius: 4.0.into(),
            },
            ..Default::default()
        },
        iced::widget::button::Status::Hovered => iced::widget::button::Style {
            background: Some(theme.extended_palette().background.strong.color.into()),
            text_color: theme.extended_palette().background.base.text.into(),
            border: iced::Border {
                width: 0.0,
                color: theme.extended_palette().background.base.color.into(),
                radius: 4.0.into(),
            },
            ..Default::default()
        },
        iced::widget::button::Status::Pressed => iced::widget::button::Style {
            background: Some(theme.palette().background.into()),
            text_color: theme.extended_palette().background.base.text.into(),
            border: iced::Border {
                width: 0.0,
                color: theme.extended_palette().background.base.color.into(),
                radius: 4.0.into(),
            },
            ..Default::default()
        },
        _ => Default::default(),
    }
}

pub fn button_secondary(
    theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    match status {
        iced::widget::button::Status::Active => iced::widget::button::Style {
            background: Some(theme.extended_palette().secondary.strong.color.into()),
            text_color: theme.extended_palette().secondary.strong.text.into(),
            border: iced::Border {
                width: 0.0,
                color: theme.extended_palette().secondary.base.color.into(),
                radius: 4.0.into(),
            },
            ..Default::default()
        },
        iced::widget::button::Status::Hovered => iced::widget::button::Style {
            background: Some(theme.extended_palette().primary.base.text.into()),
            text_color: theme.extended_palette().secondary.base.text.into(),
            border: iced::Border {
                width: 0.0,
                color: theme.extended_palette().secondary.base.color.into(),
                radius: 4.0.into(),
            },
            ..Default::default()
        },
        iced::widget::button::Status::Pressed => iced::widget::button::Style {
            background: Some(theme.palette().background.into()),
            text_color: theme.extended_palette().background.base.text.into(),
            border: iced::Border {
                width: 0.0,
                color: theme.extended_palette().secondary.base.color.into(),
                radius: 4.0.into(),
            },
            ..Default::default()
        },
        _ => Default::default(),
    }
}
