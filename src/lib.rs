#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const PI: f64 = std::f64::consts::PI;

// Some Basic Colors
// NOTE: Custom raylib color palette for amazing visuals on WHITE background
macro_rules! raylib_color {
    ($name:ident, { $r:literal, $g:literal, $b:literal, $a:literal }) => {
        pub const $name: Color = Color {
            r: $r,
            g: $g,
            b: $b,
            a: $a,
        };
    };
}
raylib_color!(LIGHTGRAY , { 200, 200, 200, 255 } ); // Light Gray
raylib_color!(GRAY      , { 130, 130, 130, 255 } ); // Gray
raylib_color!(DARKGRAY  , { 80, 80, 80, 255 } ); // Dark Gray
raylib_color!(YELLOW    , { 253, 249, 0, 255 } ); // Yellow
raylib_color!(GOLD      , { 255, 203, 0, 255 } ); // Gold
raylib_color!(ORANGE    , { 255, 161, 0, 255 } ); // Orange
raylib_color!(PINK      , { 255, 109, 194, 255 } ); // Pink
raylib_color!(RED       , { 230, 41, 55, 255 } ); // Red
raylib_color!(MAROON    , { 190, 33, 55, 255 } ); // Maroon
raylib_color!(GREEN     , { 0, 228, 48, 255 } ); // Green
raylib_color!(LIME      , { 0, 158, 47, 255 } ); // Lime
raylib_color!(DARKGREEN , { 0, 117, 44, 255 } ); // Dark Green
raylib_color!(SKYBLUE   , { 102, 191, 255, 255 } ); // Sky Blue
raylib_color!(BLUE      , { 0, 121, 241, 255 } ); // Blue
raylib_color!(DARKBLUE  , { 0, 82, 172, 255 } ); // Dark Blue
raylib_color!(PURPLE    , { 200, 122, 255, 255 } ); // Purple
raylib_color!(VIOLET    , { 135, 60, 190, 255 } ); // Violet
raylib_color!(DARKPURPLE, { 112, 31, 126, 255 } ); // Dark Purple
raylib_color!(BEIGE     , { 211, 176, 131, 255 } ); // Beige
raylib_color!(BROWN     , { 127, 106, 79, 255 } ); // Brown
raylib_color!(DARKBROWN , { 76, 63, 47, 255 } ); // Dark Brown
raylib_color!(WHITE     , { 255, 255, 255, 255 } ); // White
raylib_color!(BLACK     , { 0, 0, 0, 255 } ); // Black
raylib_color!(BLANK     , { 0, 0, 0, 0 } ); // Blank (Transparent)
raylib_color!(MAGENTA   , { 255, 0, 255, 255 } ); // Magenta
raylib_color!(RAYWHITE  , { 245, 245, 245, 255 } ); // Raysan's own White (raylib logo)
