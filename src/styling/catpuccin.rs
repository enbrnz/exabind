use ratatui::style::Color;

pub const CATPPUCCIN: Catppuccin = Catppuccin::new();

#[allow(unused)]
pub struct Catppuccin {
    pub rosewater: Color,
    pub flamingo: Color,
    pub pink: Color,
    pub mauve: Color,
    pub red: Color,
    pub maroon: Color,
    pub peach: Color,
    pub yellow: Color,
    pub green: Color,
    pub teal: Color,
    pub sky: Color,
    pub sapphire: Color,
    pub blue: Color,
    pub lavender: Color,
    pub text: Color,
    pub subtext1: Color,
    pub subtext0: Color,
    pub overlay2: Color,
    pub overlay1: Color,
    pub overlay0: Color,
    pub surface2: Color,
    pub surface1: Color,
    pub surface0: Color,
    pub base: Color,
    pub mantle: Color,
    pub crust: Color,
}

impl Catppuccin {
    pub const fn new() -> Self {
        Self {
            rosewater: Color::from_u32(0xf5e0dc),
            flamingo: Color::from_u32(0xf2cdcd),
            pink: Color::from_u32(0xf5c2e7),
            mauve: Color::from_u32(0xcba6f7),
            red: Color::from_u32(0xf38ba8),
            maroon: Color::from_u32(0xeba0ac),
            peach: Color::from_u32(0xfab387),
            yellow: Color::from_u32(0xf9e2af),
            green: Color::from_u32(0xa6e3a1),
            teal: Color::from_u32(0x94e2d5),
            sky: Color::from_u32(0x89dceb),
            sapphire: Color::from_u32(0x74c7ec),
            blue: Color::from_u32(0x89b4fa),
            lavender: Color::from_u32(0xb4befe),
            text: Color::from_u32(0xcdd6f4),
            subtext1: Color::from_u32(0xbac2de),
            subtext0: Color::from_u32(0xa6adc8),
            overlay2: Color::from_u32(0x9399b2),
            overlay1: Color::from_u32(0x7f849c),
            overlay0: Color::from_u32(0x6c7086),
            surface2: Color::from_u32(0x585b70),
            surface1: Color::from_u32(0x45475a),
            surface0: Color::from_u32(0x313244),
            base: Color::from_u32(0x1e1e2e),
            mantle: Color::from_u32(0x181825),
            crust: Color::from_u32(0x11111b),
        }
    }

    pub fn named_colors(&self) -> Vec<(&str, Color)> {
        vec![
            ("rosewater", self.rosewater),
            ("flamingo", self.flamingo),
            ("pink", self.pink),
            ("mauve", self.mauve),
            ("red", self.red),
            ("maroon", self.maroon),
            ("peach", self.peach),
            ("yellow", self.yellow),
            ("green", self.green),
            ("teal", self.teal),
            ("sky", self.sky),
            ("sapphire", self.sapphire),
            ("blue", self.blue),
            ("lavender", self.lavender),
            ("text", self.text),
            ("subtext1", self.subtext1),
            ("subtext0", self.subtext0),
            ("overlay2", self.overlay2),
            ("overlay1", self.overlay1),
            ("overlay0", self.overlay0),
            ("surface2", self.surface2),
            ("surface1", self.surface1),
            ("surface0", self.surface0),
            ("base", self.base),
            ("mantle", self.mantle),
            ("crust", self.crust),
        ]
    }
}