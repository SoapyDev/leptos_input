use codee::string::FromToStringCodec;
use csscolorparser::Color;
use leptos::{component, provide_context, use_context, SignalSet, SignalUpdate};
use leptos::{view, Children, IntoView, RwSignal, SignalGet};
use leptos_use::{use_cookie, use_css_var};
use std::fmt::{Display, Formatter};

/// The selected theme of the application.
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Display for Theme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::Light => write!(f, "light"),
            Theme::Dark => write!(f, "dark"),
        }
    }
}
impl Default for Theme {
    fn default() -> Self {
        let (cookie, _) = leptos_use::use_cookie::<String, FromToStringCodec>("theme_mode");

        if cookie.get().is_some() {
            if cookie.get().unwrap() == "dark" {
                return Theme::Dark;
            }
            return Theme::Light;
        }
        Theme::Dark
    }
}
/// For any given variable, a light and dark color must be
/// specified. The color will be selected based on the theme.
#[derive(Clone, Debug, PartialEq)]
pub struct ThemeColor {
    /// The value for the light theme. Can be any valid CSS color value.
    /// For example: "#fff", "rgb(255, 255, 255)", "hsl(0, 0%, 100%)"
    /// or transparent.
    pub light: Color,
    pub dark: Color,
}

impl ThemeColor {
    pub fn hex(&self, theme: &Theme) -> String {
        match theme {
            Theme::Light => self.light.to_hex_string(),
            Theme::Dark => self.dark.to_hex_string(),
        }
    }

    pub fn rgb(&self, theme: &Theme) -> String {
        match theme {
            Theme::Light => self.light.to_rgb_string(),
            Theme::Dark => self.dark.to_rgb_string(),
        }
    }

    pub fn hsla(&self, theme: &Theme) -> [f32; 4] {
        match theme {
            Theme::Light => self.light.to_hsla(),
            Theme::Dark => self.dark.to_hsla(),
        }
    }

    pub fn lighten(&self, theme: &Theme, amount: f32) -> String {
        let base = self.hsla(theme);

        Color::from_hsla(base[0], base[1], base[2] + amount, base[3]).to_hex_string()
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Shadow {
    pub light: String,
    pub dark: String,
}

impl Shadow {
    pub fn shadow(&self, theme: &Theme) -> String {
        match theme {
            Theme::Light => self.light.clone(),
            Theme::Dark => self.dark.clone(),
        }
    }
}

/// A distance or size value.
#[derive(Clone, Debug, PartialEq)]
pub struct Sizes {
    pub value: f64,
    pub unit: String,
}

impl Display for Sizes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{};", self.value, self.unit)
    }
}

impl Default for Sizes {
    fn default() -> Self {
        Sizes {
            value: 1.0,
            unit: String::from("px"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BreakPoints {
    pub small_form: Sizes,
    pub mobile: Sizes,
    pub tablet: Sizes,
    pub desktop: Sizes,
}

impl Default for BreakPoints {
    fn default() -> Self {
        BreakPoints {
            small_form: Sizes {
                value: 400.0,
                unit: String::from("px"),
            },
            mobile: Sizes {
                value: 600.0,
                unit: String::from("px"),
            },
            tablet: Sizes {
                value: 900.0,
                unit: String::from("px"),
            },
            desktop: Sizes {
                value: 1200.0,
                unit: String::from("px"),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Radius {
    pub input_radius: Sizes,
    pub box_radius: Sizes,
}

impl Default for Radius {
    fn default() -> Self {
        Radius {
            input_radius: Sizes {
                value: 8.0,
                unit: String::from("px"),
            },
            box_radius: Sizes {
                value: 16.0,
                unit: String::from("px"),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Spacing {
    small: Sizes,
    medium: Sizes,
    large: Sizes,
}

impl Default for Spacing {
    fn default() -> Self {
        Spacing {
            small: Sizes {
                value: 1.0,
                unit: String::from("rem"),
            },
            medium: Sizes {
                value: 2.0,
                unit: String::from("rem"),
            },
            large: Sizes {
                value: 4.0,
                unit: String::from("rem"),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fonts {
    pub family: String,
    pub small: Sizes,
    pub medium: Sizes,
    pub large: Sizes,
    pub very_large: Sizes,
}

impl Default for Fonts {
    fn default() -> Self {
        Fonts {
            family: String::from("Roboto, Inter, Helvetica, sans-serif, serif"),
            small: Sizes {
                value: 0.8,
                unit: String::from("rem"),
            },
            medium: Sizes {
                value: 1.0,
                unit: String::from("rem"),
            },
            large: Sizes {
                value: 1.5,
                unit: String::from("rem"),
            },
            very_large: Sizes {
                value: 2.5,
                unit: String::from("rem"),
            },
        }
    }
}

/// The theme of the application. One of Light or Dark with
/// the associated colors.
#[derive(Clone, Debug, PartialEq)]
pub struct GlobalTheme {
    pub theme: Theme,
    /// Colors
    pub white: ThemeColor,
    pub black: ThemeColor,
    pub gray: ThemeColor,
    pub text: ThemeColor,
    pub text_hover: ThemeColor,
    pub background: ThemeColor,
    pub emphasis: ThemeColor,
    pub secondary: ThemeColor,
    pub shadow_small: Shadow,
    pub shadow_medium: Shadow,
    pub shadow_large: Shadow,
    pub shadow_x_large: Shadow,
    pub success: ThemeColor,
    pub error: ThemeColor,

    /// Fonts and sizes
    pub fonts: Fonts,
    /// Spacing
    pub spacing: Spacing,
    /// Radius
    pub radius: Radius,
    /// Breakpoints
    pub breakpoints: BreakPoints,
}

impl Default for GlobalTheme {
    fn default() -> Self {
        GlobalTheme {
            theme: Theme::default(),
            white: ThemeColor {
                light: "#E1E1E1".parse().unwrap(),
                dark: "#E1E1E1".parse().unwrap(),
            },
            black: ThemeColor {
                light: "#121212".parse().unwrap(),
                dark: "#121212".parse().unwrap(),
            },
            gray: ThemeColor {
                light: "#B1B1B1".parse().unwrap(),
                dark: "#B1B1B1".parse().unwrap(),
            },
            text: ThemeColor {
                light: "#121212".parse().unwrap(),
                dark: "#E1E1E1".parse().unwrap(),
            },
            text_hover: ThemeColor {
                light: "#E1E1E1".parse().unwrap(),
                dark: "#121212".parse().unwrap(),
            },
            background: ThemeColor {
                light: "#D1D1D1".parse().unwrap(),
                dark: "#181818".parse().unwrap(),
            },
            emphasis: ThemeColor {
                light: "#9457e1".parse().unwrap(),
                dark: "#8544d9".parse().unwrap(),
            },
            secondary: ThemeColor {
                light: "#15b6a6".parse().unwrap(),
                dark: "#03DAC5".parse().unwrap(),
            },
            shadow_small: Shadow {
                light: String::from("rgba(17, 17, 26, 0.1) 0px 4px 12px, rgba(17, 17, 26, 0.05) 0px 8px 24px"),
                dark: String::from("rgba(248, 248, 229, 0.1) 0px 4px 12px, rgba(248, 248, 229, 0.05) 0px 8px 24px"),
            },
            shadow_medium: Shadow {
                light: String::from("rgba(17, 17, 26, 0.1) 0px 1px 0px, rgba(17, 17, 26, 0.1) 0px 8px 24px, rgba(17, 17, 26, 0.1) 0px 16px 48px"),
                dark: String::from("rgba(248, 248, 229, 0.1) 0px 1px 0px, rgba(248, 248, 229, 0.1) 0px 8px 24px, rgba(248, 248, 229, 0.1) 0px 16px 48px"),
            },
            shadow_large: Shadow {
                light: String::from("rgba(17, 17, 26, 0.1) 0px 4px 16px, rgba(17, 17, 26, 0.1) 0px 8px 24px, rgba(17, 17, 26, 0.1) 0px 16px 56px)"),
                dark: String::from("rgba(248, 248, 229, 0.1) 0px 4px 16px, rgba(248, 248, 229, 0.1) 0px 8px 24px, rgba(248, 248, 229, 0.1) 0px 16px 56px"),
            },
            shadow_x_large: Shadow {
                light: String::from("rgba(17, 17, 26, 0.1) 0px 8px 24px, rgba(17, 17, 26, 0.1) 0px 16px 56px, rgba(17, 17, 26, 0.1) 0px 24px 80px"),
                dark: String::from("rgba(248, 248, 229, 0.1) 0px 8px 24px, rgba(248, 248, 229, 0.1) 0px 16px 56px, rgba(248, 248, 229, 0.1) 0px 24px 80px"),
            },
            success: ThemeColor {
                light: "#4caf50".parse().unwrap(),
                dark: "#4caf50".parse().unwrap(),
            },
            error: ThemeColor {
                light: "#c94444".parse().unwrap(),
                dark: "#c94444".parse().unwrap(),
            },
            fonts: Fonts::default(),
            spacing: Spacing::default(),
            radius: Radius::default(),
            breakpoints: BreakPoints::default(),
        }
    }
}

impl GlobalTheme {
    pub fn apply(&mut self) {
        let (_, set_white) = use_css_var("--white");
        let (_, set_black) = use_css_var("--black");
        let (_, set_gray) = use_css_var("--gray");
        let (_, set_text) = use_css_var("--text");
        let (_, set_text_hover) = use_css_var("--text-hover");

        let (_, set_background) = use_css_var("--dp-0");
        let (_, set_dp_1) = use_css_var("--dp-1");
        let (_, set_dp_2) = use_css_var("--dp-2");
        let (_, set_dp_3) = use_css_var("--dp-3");
        let (_, set_dp_4) = use_css_var("--dp-4");
        let (_, set_dp_6) = use_css_var("--dp-6");
        let (_, set_dp_8) = use_css_var("--dp-8");
        let (_, set_dp_12) = use_css_var("--dp-12");
        let (_, set_dp_16) = use_css_var("--dp-16");
        let (_, set_dp_24) = use_css_var("--dp-24");

        let (_, set_emphasis) = use_css_var("--emphasis");
        let (_, set_emphasis_hover) = use_css_var("--emphasis-hover");
        let (_, set_secondary) = use_css_var("--secondary");
        let (_, set_secondary_hover) = use_css_var("--secondary-hover");

        let (_, set_shadow_small) = use_css_var("--shadow-small");
        let (_, set_shadow_medium) = use_css_var("--shadow-medium");
        let (_, set_shadow_large) = use_css_var("--shadow-large");
        let (_, set_shadow_x_large) = use_css_var("--shadow-x-large");

        let (_, set_success) = use_css_var("--success");
        let (_, set_error) = use_css_var("--error");

        let (_, set_font_family) = use_css_var("--font-family");
        let (_, set_font_size) = use_css_var("--font-size-small");
        let (_, set_font_size_medium) = use_css_var("--font-size-medium");
        let (_, set_font_size_large) = use_css_var("--font-size-large");
        let (_, set_font_size_x_large) = use_css_var("--font-size-x-large");

        let (_, set_spacing_small) = use_css_var("--spacing-small");
        let (_, set_spacing_medium) = use_css_var("--spacing-medium");
        let (_, set_spacing_large) = use_css_var("--spacing-large");

        let (_, set_radius_input) = use_css_var("--radius-input");
        let (_, set_radius_box) = use_css_var("--radius-box");

        let (_, set_breakpoint_small) = use_css_var("--breakpoint-small");
        let (_, set_breakpoint_medium) = use_css_var("--breakpoint-mobile");
        let (_, set_breakpoint_large) = use_css_var("--breakpoint-tablet");
        let (_, set_breakpoint_x_large) = use_css_var("--breakpoint-desktop");

        let theme = self.theme;

        set_white.set(self.white.hex(&theme));
        set_black.set(self.black.hex(&theme));
        set_gray.set(self.gray.hex(&theme));
        set_text.set(self.text.hex(&theme));
        set_text_hover.set(self.text_hover.hex(&theme));

        let background = self.background.hex(&theme);
        set_background.set(background.to_owned());

        set_emphasis.set(self.emphasis.hex(&theme));
        set_secondary.set(self.secondary.hex(&theme));

        if theme == Theme::Light {
            set_dp_1.set(background.to_owned());
            set_dp_2.set(background.to_owned());
            set_dp_3.set(background.to_owned());
            set_dp_4.set(background.to_owned());
            set_dp_6.set(background.to_owned());
            set_dp_8.set(background.to_owned());
            set_dp_12.set(background.to_owned());
            set_dp_16.set(background.to_owned());
            set_dp_24.set(background.to_owned());
            set_emphasis_hover.set(self.emphasis.lighten(&theme, -0.1));
            set_secondary_hover.set(self.secondary.lighten(&theme, -0.1));
        } else {
            set_dp_1.set(self.background.lighten(&theme, 0.05));
            set_dp_2.set(self.background.lighten(&theme, 0.07));
            set_dp_3.set(self.background.lighten(&theme, 0.08));
            set_dp_4.set(self.background.lighten(&theme, 0.09));
            set_dp_6.set(self.background.lighten(&theme, 0.11));
            set_dp_8.set(self.background.lighten(&theme, 0.12));
            set_dp_12.set(self.background.lighten(&theme, 0.14));
            set_dp_16.set(self.background.lighten(&theme, 0.15));
            set_dp_24.set(self.background.lighten(&theme, 0.16));
            set_emphasis_hover.set(self.emphasis.lighten(&theme, 0.1));
            set_secondary_hover.set(self.secondary.lighten(&theme, 0.1));
        }

        set_shadow_small.set(self.shadow_small.shadow(&theme));
        set_shadow_medium.set(self.shadow_medium.shadow(&theme));
        set_shadow_large.set(self.shadow_large.shadow(&theme));
        set_shadow_x_large.set(self.shadow_x_large.shadow(&theme));

        set_success.set(self.success.hex(&theme));
        set_error.set(self.error.hex(&theme));

        set_font_family.set(self.fonts.family.clone());
        set_font_size.set(self.fonts.small.to_string());
        set_font_size_medium.set(self.fonts.medium.to_string());
        set_font_size_large.set(self.fonts.large.to_string());
        set_font_size_x_large.set(self.fonts.very_large.to_string());

        set_spacing_small.set(self.spacing.small.to_string());
        set_spacing_medium.set(self.spacing.medium.to_string());
        set_spacing_large.set(self.spacing.large.to_string());

        set_radius_input.set(self.radius.input_radius.to_string());
        set_radius_box.set(self.radius.box_radius.to_string());

        set_breakpoint_small.set(self.breakpoints.small_form.to_string());
        set_breakpoint_medium.set(self.breakpoints.mobile.to_string());
        set_breakpoint_large.set(self.breakpoints.tablet.to_string());
        set_breakpoint_x_large.set(self.breakpoints.desktop.to_string());
    }

    pub fn toggle(&mut self) {
        let new_theme = match self.theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        };

        let (_, set_theme) = use_cookie::<String, FromToStringCodec>("theme_mode");

        set_theme.set(Option::from(new_theme.to_string()));

        self.theme = new_theme;
        self.apply();
    }
}

#[component]
pub fn ThemeToggler() -> impl IntoView {
    let global_theme = use_context::<RwSignal<GlobalTheme>>().unwrap();

    // https://codepen.io/ghaste/pen/WNOjQJN
    view! {
        <button
        on:click=move |_| global_theme.update(|t| t.toggle())
        id="switch-theme" aria-label="Switch theme">
            <svg xmlns="http://www.w3.org/2000/svg" width="30" height="30" viewBox="0 0 472.39 472.39">
                <g class="toggle-sun">
                    <path d="M403.21,167V69.18H305.38L236.2,0,167,69.18H69.18V167L0,236.2l69.18,69.18v97.83H167l69.18,69.18,69.18-69.18h97.83V305.38l69.18-69.18Zm-167,198.17a129,129,0,1,1,129-129A129,129,0,0,1,236.2,365.19Z" />
                </g>
                <g class="toggle-circle">
                    <circle class="cls-1" cx="236.2" cy="236.2" r="103.78" />
                </g>
            </svg>
        </button>
    }
}

#[component]
pub fn GlobalThemeProvider(
    #[prop(default = RwSignal::new(GlobalTheme::default()))] global_theme: RwSignal<GlobalTheme>,
    children: Children,
) -> impl IntoView {
    global_theme.get().apply();
    provide_context(global_theme);

    view! {
        <body
            class:dark-theme=move || global_theme.get().theme == Theme::Dark
            class:light-theme=move || global_theme.get().theme == Theme::Light
        >
            {children()}
        </body>
    }
}
