/*
Copyright 2020 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

/*

HIBOU Color Palette
My color picks for a palette with:
 - a constant Lightness L
 - a constant Saturation S
in the HSL representation of colors

All colors have :
    - a lightness of :
        + 20 for the "Dark" version
        + 30 for the "Standard" version
        + 50 for the "Light" version
        + 65 for the "Bright" version
    - a saturation of :
        + 70 for the "Dark" version
        + 70 for the "Standard" version
        + 70 for the "Light" version
        + 90 for the "Bright" version

Different hues were selected :
    - 0 for "Red"
    - 30 for "Orange"
    - 60 for "Yellow"
    - 120 for "Green"
    - 180 for "Cyan"
    - 240 for "Blue"
    - 280 for "Purple"
    - 310 for "Pink"

For use with image::Rgb from the "image" crate

*/

pub const HCP_WHITE : [u8;3] = [255u8,  255u8,  255u8];
pub const HCP_BLACK : [u8;3] = [0u8, 0u8, 0u8];

pub const HCP_DARK_RED : [u8;3] = [86u8, 15u8, 15u8];
pub const HCP_STANDARD_RED : [u8;3] = [130u8, 22u8, 22u8];
pub const HCP_LIGHT_RED : [u8;3] = [216u8, 38u8, 38u8];
pub const HCP_BRIGHT_RED : [u8;3] = [246u8, 85u8, 85u8];
pub const HCP_DARK_ORANGE : [u8;3] = [86u8, 51u8, 15u8];
pub const HCP_STANDARD_ORANGE : [u8;3] = [130u8, 76u8, 22u8];
pub const HCP_LIGHT_ORANGE : [u8;3] = [216u8, 127u8, 38u8];
pub const HCP_BRIGHT_ORANGE : [u8;3] = [246u8, 165u8, 85u8];
pub const HCP_DARK_YELLOW : [u8;3] = [86u8, 86u8, 15u8];
pub const HCP_STANDARD_YELLOW : [u8;3] = [130u8, 130u8, 22u8];
pub const HCP_LIGHT_YELLOW : [u8;3] = [216u8, 216u8, 38u8];
pub const HCP_BRIGHT_YELLOW : [u8;3] = [246u8, 246u8, 85u8];
pub const HCP_DARK_GREEN : [u8;3] = [15u8, 86u8, 15u8];
pub const HCP_STANDARD_GREEN : [u8;3] = [22u8, 130u8, 22u8];
pub const HCP_LIGHT_GREEN : [u8;3] = [38u8, 216u8, 38u8];
pub const HCP_BRIGHT_GREEN : [u8;3] = [85u8, 246u8, 85u8];
pub const HCP_DARK_CYAN : [u8;3] = [15u8, 86u8, 86u8];
pub const HCP_STANDARD_CYAN : [u8;3] = [22u8, 130u8, 130u8];
pub const HCP_LIGHT_CYAN : [u8;3] = [38u8, 216u8, 216u8];
pub const HCP_BRIGHT_CYAN : [u8;3] = [85u8, 246u8, 246u8];
pub const HCP_DARK_BLUE : [u8;3] = [15u8, 15u8, 86u8];
pub const HCP_STANDARD_BLUE : [u8;3] = [22u8, 22u8, 130u8];
pub const HCP_LIGHT_BLUE : [u8;3] = [38u8, 38u8, 216u8];
pub const HCP_BRIGHT_BLUE : [u8;3] = [85u8, 85u8, 246u8];
pub const HCP_DARK_PURPLE : [u8;3] = [62u8, 15u8, 86u8];
pub const HCP_STANDARD_PURPLE : [u8;3] = [94u8, 22u8, 130u8];
pub const HCP_LIGHT_PURPLE : [u8;3] = [157u8, 38u8, 216u8];
pub const HCP_BRIGHT_PURPLE : [u8;3] = [192u8, 85u8, 246u8];
pub const HCP_DARK_PINK : [u8;3] = [86u8, 15u8, 74u8];
pub const HCP_STANDARD_PINK : [u8;3] = [130u8, 22u8, 112u8];
pub const HCP_LIGHT_PINK : [u8;3] = [216u8, 38u8, 186u8];
pub const HCP_BRIGHT_PINK : [u8;3] = [246u8, 85u8, 219u8];

pub const HCP_DARK_GRAY : [u8;3] = [51u8, 51u8, 51u8];
pub const HCP_STANDARD_GRAY : [u8;3] = [76u8, 76u8, 76u8];
pub const HCP_LIGHT_GRAY : [u8;3] = [127u8, 127u8, 127u8];
pub const HCP_BRIGHT_GRAY : [u8;3] = [165u8, 165u8, 165u8];


pub const HC_LIFELINE : [u8;3] = HCP_STANDARD_BLUE;
pub const HC_GATE : [u8;3] = HCP_STANDARD_PURPLE;
pub const HC_MESSAGE : [u8;3] = HCP_DARK_GREEN;
pub const HC_GRAMMAR_SYMBOL : [u8;3] = HCP_BLACK;

