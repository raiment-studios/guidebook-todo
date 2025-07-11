// Apollo color palette from Lospec (https://lospec.com/palette-list/apollo)
// Created by AdamCYounis - used with appreciation

use ratatui::prelude::*;

/// Apollo color palette RGB values
pub struct ApolloRgb;

impl ApolloRgb {
    // Dark blues and cyans - good for background and secondary elements
    pub const MIDNIGHT: (u8, u8, u8) = (0x17, 0x20, 0x38); // #172038
    pub const DARK_BLUE: (u8, u8, u8) = (0x25, 0x3a, 0x5e); // #253a5e
    pub const OCEAN_BLUE: (u8, u8, u8) = (0x3c, 0x5e, 0x8b); // #3c5e8b
    pub const SKY_BLUE: (u8, u8, u8) = (0x4f, 0x8f, 0xba); // #4f8fba
    pub const LIGHT_CYAN: (u8, u8, u8) = (0x73, 0xbe, 0xd3); // #73bed3
    pub const PALE_CYAN: (u8, u8, u8) = (0xa4, 0xdd, 0xdb); // #a4dddb

    // Greens - good for success states and highlights
    pub const DARK_GREEN: (u8, u8, u8) = (0x19, 0x33, 0x2d); // #19332d
    pub const FOREST_GREEN: (u8, u8, u8) = (0x25, 0x56, 0x2e); // #25562e
    pub const BRIGHT_GREEN: (u8, u8, u8) = (0x46, 0x82, 0x32); // #468232
    pub const LIME_GREEN: (u8, u8, u8) = (0x75, 0xa7, 0x43); // #75a743
    pub const YELLOW_GREEN: (u8, u8, u8) = (0xa8, 0xca, 0x58); // #a8ca58
    pub const PALE_GREEN: (u8, u8, u8) = (0xd0, 0xda, 0x91); // #d0da91

    // Warm browns and oranges - good for text and neutral elements
    pub const DARK_BROWN: (u8, u8, u8) = (0x4d, 0x2b, 0x32); // #4d2b32
    pub const MED_BROWN: (u8, u8, u8) = (0x7a, 0x48, 0x41); // #7a4841
    pub const LIGHT_BROWN: (u8, u8, u8) = (0xad, 0x77, 0x57); // #ad7757
    pub const ORANGE_BROWN: (u8, u8, u8) = (0xc0, 0x94, 0x73); // #c09473
    pub const CREAM: (u8, u8, u8) = (0xd7, 0xb5, 0x94); // #d7b594
    pub const LIGHT_CREAM: (u8, u8, u8) = (0xe7, 0xd5, 0xb3); // #e7d5b3

    // Purples and magentas - good for special states and warnings
    pub const DARK_PURPLE: (u8, u8, u8) = (0x34, 0x1c, 0x27); // #341c27
    pub const DEEP_RED: (u8, u8, u8) = (0x60, 0x2c, 0x2c); // #602c2c
    pub const RUST_RED: (u8, u8, u8) = (0x88, 0x4b, 0x2b); // #884b2b
    pub const ORANGE: (u8, u8, u8) = (0xbe, 0x77, 0x2b); // #be772b
    pub const BRIGHT_ORANGE: (u8, u8, u8) = (0xde, 0x9e, 0x41); // #de9e41
    pub const YELLOW: (u8, u8, u8) = (0xe8, 0xc1, 0x70); // #e8c170

    // Deep purples and violets - good for accents and special states
    pub const DEEP_PURPLE: (u8, u8, u8) = (0x24, 0x15, 0x27); // #241527
    pub const VIOLET: (u8, u8, u8) = (0x41, 0x1d, 0x31); // #411d31
    pub const MAGENTA: (u8, u8, u8) = (0x75, 0x24, 0x38); // #752438
    pub const BRIGHT_MAGENTA: (u8, u8, u8) = (0xa2, 0x2c, 0x40); // #a22c40
    pub const PINK: (u8, u8, u8) = (0xde, 0x72, 0x77); // #de7277
    pub const LIGHT_PINK: (u8, u8, u8) = (0xec, 0xa8, 0xb0); // #eca8b0

    // Neutrals and whites
    pub const WHITE: (u8, u8, u8) = (0xff, 0xff, 0xff); // #ffffff
    pub const LIGHT_GRAY: (u8, u8, u8) = (0xc0, 0xcb, 0xdc); // #c0cbdc
    pub const MED_GRAY: (u8, u8, u8) = (0x8b, 0x9b, 0xb4); // #8b9bb4
    pub const DARK_GRAY: (u8, u8, u8) = (0x5a, 0x69, 0x88); // #5a6988
    pub const DARKER_GRAY: (u8, u8, u8) = (0x3a, 0x44, 0x66); // #3a4466
    pub const DARKEST_GRAY: (u8, u8, u8) = (0x26, 0x2b, 0x44); // #262b44
    pub const NEAR_BLACK: (u8, u8, u8) = (0x18, 0x14, 0x25); // #181425
}

/// Apollo-based color scheme for terminal UI
pub struct ApolloTheme;

impl ApolloTheme {
    // Primary colors for different UI states
    pub fn primary() -> Color {
        Color::Rgb(
            ApolloRgb::SKY_BLUE.0,
            ApolloRgb::SKY_BLUE.1,
            ApolloRgb::SKY_BLUE.2,
        )
    }

    pub fn secondary() -> Color {
        Color::Rgb(
            ApolloRgb::LIGHT_CYAN.0,
            ApolloRgb::LIGHT_CYAN.1,
            ApolloRgb::LIGHT_CYAN.2,
        )
    }

    pub fn accent() -> Color {
        Color::Rgb(
            ApolloRgb::YELLOW.0,
            ApolloRgb::YELLOW.1,
            ApolloRgb::YELLOW.2,
        )
    }

    pub fn success() -> Color {
        Color::Rgb(
            ApolloRgb::BRIGHT_GREEN.0,
            ApolloRgb::BRIGHT_GREEN.1,
            ApolloRgb::BRIGHT_GREEN.2,
        )
    }

    pub fn warning() -> Color {
        Color::Rgb(
            ApolloRgb::BRIGHT_ORANGE.0,
            ApolloRgb::BRIGHT_ORANGE.1,
            ApolloRgb::BRIGHT_ORANGE.2,
        )
    }

    pub fn error() -> Color {
        Color::Rgb(
            ApolloRgb::BRIGHT_MAGENTA.0,
            ApolloRgb::BRIGHT_MAGENTA.1,
            ApolloRgb::BRIGHT_MAGENTA.2,
        )
    }

    // Text colors
    pub fn text_primary() -> Color {
        Color::Rgb(
            ApolloRgb::LIGHT_CREAM.0,
            ApolloRgb::LIGHT_CREAM.1,
            ApolloRgb::LIGHT_CREAM.2,
        )
    }

    pub fn text_secondary() -> Color {
        Color::Rgb(
            ApolloRgb::LIGHT_GRAY.0,
            ApolloRgb::LIGHT_GRAY.1,
            ApolloRgb::LIGHT_GRAY.2,
        )
    }

    pub fn text_muted() -> Color {
        Color::Rgb(
            ApolloRgb::MED_GRAY.0,
            ApolloRgb::MED_GRAY.1,
            ApolloRgb::MED_GRAY.2,
        )
    }

    pub fn text_disabled() -> Color {
        Color::Rgb(
            ApolloRgb::DARK_GRAY.0,
            ApolloRgb::DARK_GRAY.1,
            ApolloRgb::DARK_GRAY.2,
        )
    }

    // Background colors
    pub fn background() -> Color {
        Color::Rgb(
            ApolloRgb::NEAR_BLACK.0,
            ApolloRgb::NEAR_BLACK.1,
            ApolloRgb::NEAR_BLACK.2,
        )
    }

    pub fn background_alt() -> Color {
        Color::Rgb(
            ApolloRgb::MIDNIGHT.0,
            ApolloRgb::MIDNIGHT.1,
            ApolloRgb::MIDNIGHT.2,
        )
    }

    pub fn surface() -> Color {
        Color::Rgb(
            ApolloRgb::DARKEST_GRAY.0,
            ApolloRgb::DARKEST_GRAY.1,
            ApolloRgb::DARKEST_GRAY.2,
        )
    }

    pub fn surface_alt() -> Color {
        Color::Rgb(
            ApolloRgb::DARKER_GRAY.0,
            ApolloRgb::DARKER_GRAY.1,
            ApolloRgb::DARKER_GRAY.2,
        )
    }

    // Interactive states
    pub fn focused() -> Color {
        Color::Rgb(
            ApolloRgb::YELLOW.0,
            ApolloRgb::YELLOW.1,
            ApolloRgb::YELLOW.2,
        )
    }

    pub fn selected() -> Color {
        Color::Rgb(
            ApolloRgb::DARKER_GRAY.0,
            ApolloRgb::DARKER_GRAY.1,
            ApolloRgb::DARKER_GRAY.2,
        )
    }

    pub fn hover() -> Color {
        Color::Rgb(
            ApolloRgb::DARK_BLUE.0,
            ApolloRgb::DARK_BLUE.1,
            ApolloRgb::DARK_BLUE.2,
        )
    }

    // Priority colors for TODOs
    pub fn priority_urgent() -> Color {
        Color::Rgb(
            ApolloRgb::LIGHT_PINK.0,
            ApolloRgb::LIGHT_PINK.1,
            ApolloRgb::LIGHT_PINK.2,
        )
    }

    pub fn priority_high() -> Color {
        Color::Rgb(ApolloRgb::PINK.0, ApolloRgb::PINK.1, ApolloRgb::PINK.2)
    }

    pub fn priority_medium() -> Color {
        Color::Rgb(
            ApolloRgb::YELLOW.0,
            ApolloRgb::YELLOW.1,
            ApolloRgb::YELLOW.2,
        )
    }

    pub fn priority_low() -> Color {
        Color::Rgb(
            ApolloRgb::LIME_GREEN.0,
            ApolloRgb::LIME_GREEN.1,
            ApolloRgb::LIME_GREEN.2,
        )
    }

    pub fn priority_wishlist() -> Color {
        Color::Rgb(
            ApolloRgb::LIGHT_CYAN.0,
            ApolloRgb::LIGHT_CYAN.1,
            ApolloRgb::LIGHT_CYAN.2,
        )
    }

    // Status colors for TODOs
    pub fn status_todo() -> Color {
        Self::text_primary()
    }

    pub fn status_in_progress() -> Color {
        Color::Rgb(
            ApolloRgb::SKY_BLUE.0,
            ApolloRgb::SKY_BLUE.1,
            ApolloRgb::SKY_BLUE.2,
        )
    }

    pub fn status_done() -> Color {
        Color::Rgb(
            ApolloRgb::BRIGHT_GREEN.0,
            ApolloRgb::BRIGHT_GREEN.1,
            ApolloRgb::BRIGHT_GREEN.2,
        )
    }



    pub fn status_archived() -> Color {
        Color::Rgb(
            ApolloRgb::MED_GRAY.0,
            ApolloRgb::MED_GRAY.1,
            ApolloRgb::MED_GRAY.2,
        )
    }
}

/// Compile-time configurable color theme
///
/// This allows users to modify colors at compile time by changing the implementation
/// of this trait. By default, it uses the Apollo theme.
pub trait ColorTheme {
    fn primary() -> Color {
        ApolloTheme::primary()
    }
    fn secondary() -> Color {
        ApolloTheme::secondary()
    }
    fn accent() -> Color {
        ApolloTheme::accent()
    }
    fn success() -> Color {
        ApolloTheme::success()
    }
    fn warning() -> Color {
        ApolloTheme::warning()
    }
    fn error() -> Color {
        ApolloTheme::error()
    }

    fn text_primary() -> Color {
        ApolloTheme::text_primary()
    }
    fn text_secondary() -> Color {
        ApolloTheme::text_secondary()
    }
    fn text_muted() -> Color {
        ApolloTheme::text_muted()
    }
    fn text_disabled() -> Color {
        ApolloTheme::text_disabled()
    }

    fn background() -> Color {
        ApolloTheme::background()
    }
    fn background_alt() -> Color {
        ApolloTheme::background_alt()
    }
    fn surface() -> Color {
        ApolloTheme::surface()
    }
    fn surface_alt() -> Color {
        ApolloTheme::surface_alt()
    }

    fn focused() -> Color {
        ApolloTheme::focused()
    }
    fn selected() -> Color {
        ApolloTheme::selected()
    }
    fn hover() -> Color {
        ApolloTheme::hover()
    }

    fn priority_urgent() -> Color {
        ApolloTheme::priority_urgent()
    }
    fn priority_high() -> Color {
        ApolloTheme::priority_high()
    }
    fn priority_medium() -> Color {
        ApolloTheme::priority_medium()
    }
    fn priority_low() -> Color {
        ApolloTheme::priority_low()
    }
    fn priority_wishlist() -> Color {
        ApolloTheme::priority_wishlist()
    }

    fn status_todo() -> Color {
        ApolloTheme::status_todo()
    }
    fn status_in_progress() -> Color {
        ApolloTheme::status_in_progress()
    }
    fn status_done() -> Color {
        ApolloTheme::status_done()
    }

    fn status_archived() -> Color {
        ApolloTheme::status_archived()
    }
}

/// Default theme implementation - users can override this
pub struct DefaultTheme;
impl ColorTheme for DefaultTheme {}

/// Type alias for the active theme - change this to use a different theme
pub type Theme = DefaultTheme;
