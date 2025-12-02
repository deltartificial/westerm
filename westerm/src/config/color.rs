use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer, Serialize};

use westerm_config_derive::ConfigDeserialize;

use crate::display::color::{CellRgb, Rgb};

#[derive(ConfigDeserialize, Serialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct Colors {
    pub primary: PrimaryColors,
    pub cursor: InvertedCellColors,
    pub vi_mode_cursor: InvertedCellColors,
    pub selection: InvertedCellColors,
    pub normal: NormalColors,
    pub bright: BrightColors,
    pub dim: Option<DimColors>,
    pub indexed_colors: Vec<IndexedColor>,
    pub search: SearchColors,
    pub line_indicator: LineIndicatorColors,
    pub hints: HintColors,
    pub transparent_background_colors: bool,
    pub draw_bold_text_with_bright_colors: bool,
    footer_bar: BarColors,
}

impl Colors {
    pub fn footer_bar_foreground(&self) -> Rgb {
        self.footer_bar.foreground.unwrap_or(self.primary.background)
    }

    pub fn footer_bar_background(&self) -> Rgb {
        self.footer_bar.background.unwrap_or(self.primary.foreground)
    }
}

#[derive(ConfigDeserialize, Serialize, Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct LineIndicatorColors {
    pub foreground: Option<Rgb>,
    pub background: Option<Rgb>,
}

#[derive(ConfigDeserialize, Serialize, Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct HintColors {
    pub start: HintStartColors,
    pub end: HintEndColors,
}

#[derive(ConfigDeserialize, Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub struct HintStartColors {
    pub foreground: CellRgb,
    pub background: CellRgb,
}

impl Default for HintStartColors {
    fn default() -> Self {
        Self {
            foreground: CellRgb::Rgb(Rgb::new(0x1a, 0x12, 0x0f)),  // Dark background color
            background: CellRgb::Rgb(Rgb::new(0xff, 0xb3, 0x66)),  // Bright orange
        }
    }
}

#[derive(ConfigDeserialize, Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub struct HintEndColors {
    pub foreground: CellRgb,
    pub background: CellRgb,
}

impl Default for HintEndColors {
    fn default() -> Self {
        Self {
            foreground: CellRgb::Rgb(Rgb::new(0x1a, 0x12, 0x0f)),  // Dark background color
            background: CellRgb::Rgb(Rgb::new(0xe6, 0x5a, 0x3d)),  // Burnt orange-red
        }
    }
}

#[derive(Deserialize, Serialize, Copy, Clone, Default, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct IndexedColor {
    pub color: Rgb,

    index: ColorIndex,
}

impl IndexedColor {
    #[inline]
    pub fn index(&self) -> u8 {
        self.index.0
    }
}

#[derive(Serialize, Copy, Clone, Default, Debug, PartialEq, Eq)]
struct ColorIndex(u8);

impl<'de> Deserialize<'de> for ColorIndex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let index = u8::deserialize(deserializer)?;

        if index < 16 {
            Err(SerdeError::custom(
                "Config error: indexed_color's index is {}, but a value bigger than 15 was \
                 expected; ignoring setting",
            ))
        } else {
            Ok(Self(index))
        }
    }
}

#[derive(ConfigDeserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct InvertedCellColors {
    #[config(alias = "text")]
    pub foreground: CellRgb,
    #[config(alias = "cursor")]
    pub background: CellRgb,
}

impl Default for InvertedCellColors {
    fn default() -> Self {
        Self { foreground: CellRgb::CellBackground, background: CellRgb::CellForeground }
    }
}

#[derive(ConfigDeserialize, Serialize, Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct SearchColors {
    pub focused_match: FocusedMatchColors,
    pub matches: MatchColors,
}

#[derive(ConfigDeserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct FocusedMatchColors {
    pub foreground: CellRgb,
    pub background: CellRgb,
}

impl Default for FocusedMatchColors {
    fn default() -> Self {
        Self {
            background: CellRgb::Rgb(Rgb::new(0xff, 0xb3, 0x66)),  // Bright orange
            foreground: CellRgb::Rgb(Rgb::new(0x1a, 0x12, 0x0f)),  // Dark background
        }
    }
}

#[derive(ConfigDeserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct MatchColors {
    pub foreground: CellRgb,
    pub background: CellRgb,
}

impl Default for MatchColors {
    fn default() -> Self {
        Self {
            background: CellRgb::Rgb(Rgb::new(0xe6, 0x5a, 0x3d)),  // Burnt orange-red
            foreground: CellRgb::Rgb(Rgb::new(0x1a, 0x12, 0x0f)),  // Dark background
        }
    }
}

#[derive(ConfigDeserialize, Serialize, Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct BarColors {
    foreground: Option<Rgb>,
    background: Option<Rgb>,
}

#[derive(ConfigDeserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct PrimaryColors {
    pub foreground: Rgb,
    pub background: Rgb,
    pub bright_foreground: Option<Rgb>,
    pub dim_foreground: Option<Rgb>,
}

impl Default for PrimaryColors {
    fn default() -> Self {
        PrimaryColors {
            // Westerm signature: dark warm background with neutral foreground
            background: Rgb::new(0x1a, 0x12, 0x0f),  // Deep dark brown-black
            foreground: Rgb::new(0xd8, 0xd8, 0xd8),  // Light gray (neutral)
            bright_foreground: Some(Rgb::new(0xf8, 0xf8, 0xf8)),  // Bright white
            dim_foreground: Some(Rgb::new(0xa0, 0xa0, 0xa0)),     // Dim gray
        }
    }
}

#[derive(ConfigDeserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct NormalColors {
    pub black: Rgb,
    pub red: Rgb,
    pub green: Rgb,
    pub yellow: Rgb,
    pub blue: Rgb,
    pub magenta: Rgb,
    pub cyan: Rgb,
    pub white: Rgb,
}

impl Default for NormalColors {
    fn default() -> Self {
        NormalColors {
            // Westerm warm orange-toned palette
            black: Rgb::new(0x2a, 0x1f, 0x1a),      // Warm dark brown
            red: Rgb::new(0xe6, 0x5a, 0x3d),        // Bright burnt orange-red (Tmux orange)
            green: Rgb::new(0xa8, 0xb5, 0x6a),      // Muted warm green
            yellow: Rgb::new(0xe6, 0x5a, 0x3d),     // Tmux orange for path highlighting
            blue: Rgb::new(0x7d, 0x9f, 0xb5),       // Muted warm blue
            magenta: Rgb::new(0xd9, 0x8a, 0x7d),    // Dusty rose-orange
            cyan: Rgb::new(0x8f, 0xb5, 0xa8),       // Warm teal
            white: Rgb::new(0xd8, 0xd8, 0xd8),      // Light gray (neutral)
        }
    }
}

#[derive(ConfigDeserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct BrightColors {
    pub black: Rgb,
    pub red: Rgb,
    pub green: Rgb,
    pub yellow: Rgb,
    pub blue: Rgb,
    pub magenta: Rgb,
    pub cyan: Rgb,
    pub white: Rgb,
}

impl Default for BrightColors {
    fn default() -> Self {
        // Westerm bright warm palette - enhanced brightness while maintaining orange tone
        BrightColors {
            black: Rgb::new(0x6b, 0x6b, 0x6b),      // Medium gray
            red: Rgb::new(0xff, 0x7a, 0x5c),        // Vibrant coral-orange (brighter Tmux)
            green: Rgb::new(0xc4, 0xd4, 0x88),      // Bright warm green
            yellow: Rgb::new(0xff, 0x7a, 0x5c),     // Bright Tmux orange for path
            blue: Rgb::new(0x9f, 0xc4, 0xd9),       // Light warm blue
            magenta: Rgb::new(0xff, 0xaa, 0x99),    // Bright salmon-pink
            cyan: Rgb::new(0xad, 0xd4, 0xc4),       // Bright warm cyan
            white: Rgb::new(0xf8, 0xf8, 0xf8),      // Bright white
        }
    }
}

#[derive(ConfigDeserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct DimColors {
    pub black: Rgb,
    pub red: Rgb,
    pub green: Rgb,
    pub yellow: Rgb,
    pub blue: Rgb,
    pub magenta: Rgb,
    pub cyan: Rgb,
    pub white: Rgb,
}

impl Default for DimColors {
    fn default() -> Self {
        // Westerm dim warm palette - subdued orange tones
        DimColors {
            black: Rgb::new(0x1a, 0x12, 0x0f),      // Very dark warm brown
            red: Rgb::new(0x99, 0x3d, 0x2a),        // Dim burnt orange-red (dim Tmux)
            green: Rgb::new(0x6f, 0x77, 0x45),      // Dim olive
            yellow: Rgb::new(0x99, 0x3d, 0x2a),     // Dim Tmux orange for path
            blue: Rgb::new(0x52, 0x68, 0x77),       // Dim steel blue
            magenta: Rgb::new(0x8f, 0x5a, 0x52),    // Dim dusty rose
            cyan: Rgb::new(0x5f, 0x77, 0x6f),       // Dim teal
            white: Rgb::new(0x8e, 0x8e, 0x8e),      // Dim gray
        }
    }
}
