use leptos::{component, provide_context, use_context, SignalSet};
use leptos::{view, Children, IntoView, RwSignal, SignalGet};
use leptos_use::use_css_var;
use std::fmt::{Display, Formatter};

/// The selected theme of the application.
#[derive(Clone, Debug, Copy, Default, PartialEq)]
pub enum Theme{
    Light,
    #[default]
    Dark
}

/// For any given variable, a light and dark color must be
/// specified. The color will be selected based on the theme.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ThemeColor{
    /// The value for the light theme. Can be any valid CSS color value.
    /// For example: "#fff", "rgb(255, 255, 255)", "hsl(0, 0%, 100%)"
    /// or transparent.
    pub light: String,
    pub dark: String
}

impl ThemeColor{
    pub fn color(&self, theme: &Theme) -> String{
        match theme {
            Theme::Light => self.light.clone(),
            Theme::Dark => self.dark.clone()
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Shadow{
    pub light: String,
    pub dark: String
}

impl Shadow{
    pub fn shadow(&self, theme: &Theme) -> String{
        match theme {
            Theme::Light => self.light.clone(),
            Theme::Dark => self.dark.clone()
        }
    }
}

/// A distance or size value.
#[derive(Clone, Debug, PartialEq)]
pub struct Sizes{
    pub value: f64,
    pub unit : String
}

impl Display for Sizes{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        
        write!(f, "{}{};", self.value, self.unit)
    }
}

impl Default for Sizes{
    fn default() -> Self {
        Sizes{
            value: 1.0,
            unit: String::from("px")
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct BreakPoints{
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
                unit: String::from("px")
            },
            mobile: Sizes {
                value: 600.0,
                unit: String::from("px")
            },
            tablet: Sizes {
                value: 900.0,
                unit: String::from("px")
            },
            desktop: Sizes {
                value: 1200.0,
                unit: String::from("px")
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Radius{
    pub input_radius: Sizes,
    pub box_radius: Sizes
}

impl Default for Radius{
    fn default() -> Self {
        Radius{
            input_radius: Sizes{
                value: 8.0,
                unit: String::from("px")
            },
            box_radius: Sizes{
                value: 16.0,
                unit: String::from("px")
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Spacing{
    small: Sizes,
    medium: Sizes,
    large: Sizes
}

impl Default for Spacing{
    fn default() -> Self {
        Spacing{
            small: Sizes{
                value: 1.0,
                unit: String::from("rem")
            },
            medium: Sizes{
                value: 2.0,
                unit: String::from("rem")
            },
            large: Sizes{
                value: 4.0,
                unit: String::from("rem")
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fonts{
    pub family: String,
    pub small: Sizes,
    pub medium: Sizes,
    pub large: Sizes,
    pub very_large: Sizes,
}

impl Default for Fonts{
    fn default() -> Self {
        Fonts{
            family: String::from("Roboto, Inter, Helvetica, sans-serif, serif"),
            small: Sizes{
                value: 0.8,
                unit: String::from("rem")
            },
            medium: Sizes{
                value: 1.0,
                unit: String::from("rem")
            },
            large: Sizes{
                value: 1.5,
                unit: String::from("rem")
            },
            very_large: Sizes{
                value: 2.5,
                unit: String::from("rem")
            }
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

impl Default for GlobalTheme{
    fn default() -> Self {
        GlobalTheme{
            theme: Theme::default(),
            white: ThemeColor{
                light: String::from("#E1E1E1"),
                dark: String::from("#E1E1E1")
            },
            black: ThemeColor{
                light: String::from("#121212"),
                dark: String::from("#121212")
            },
            gray: ThemeColor{
                light: String::from("#B1B1B1"),
                dark: String::from("#B1B1B1")
            },
            background: ThemeColor{
                light: String::from("#CCCCCC"),
                dark: String::from("#151515")
            },
            emphasis: ThemeColor{
                light: String::from("#9457e1"),
                dark: String::from("#9457e1")
            },
            secondary: ThemeColor{
                light: String::from("#15b6a6"),
                dark: String::from("#03DAC5")
            },
            shadow_small: Shadow{
                light: String::from("rgba(17, 17, 26, 0.1) 0px 4px 12px, rgba(17, 17, 26, 0.05) 0px 8px 24px"),
                dark: String::from("rgba(248, 248, 229, 0.1) 0px 4px 12px, rgba(248, 248, 229, 0.05) 0px 8px 24px")
            },
            shadow_medium: Shadow{
                light: String::from("rgba(17, 17, 26, 0.1) 0px 1px 0px, rgba(17, 17, 26, 0.1) 0px 8px 24px, rgba(17, 17, 26, 0.1) 0px 16px 48px"),
                dark: String::from("rgba(248, 248, 229, 0.1) 0px 1px 0px, rgba(248, 248, 229, 0.1) 0px 8px 24px, rgba(248, 248, 229, 0.1) 0px 16px 48px")
            },
            shadow_large: Shadow{
                light: String::from("rgba(17, 17, 26, 0.1) 0px 4px 16px, rgba(17, 17, 26, 0.1) 0px 8px 24px, rgba(17, 17, 26, 0.1) 0px 16px 56px)"),
                dark: String::from("rgba(248, 248, 229, 0.1) 0px 4px 16px, rgba(248, 248, 229, 0.1) 0px 8px 24px, rgba(248, 248, 229, 0.1) 0px 16px 56px")
            },
            shadow_x_large: Shadow{
                light: String::from("rgba(17, 17, 26, 0.1) 0px 8px 24px, rgba(17, 17, 26, 0.1) 0px 16px 56px, rgba(17, 17, 26, 0.1) 0px 24px 80px"),
                dark: String::from("rgba(248, 248, 229, 0.1) 0px 8px 24px, rgba(248, 248, 229, 0.1) 0px 16px 56px, rgba(248, 248, 229, 0.1) 0px 24px 80px")
            },
            success: ThemeColor{
                light: String::from("#4caf50"),
                dark: String::from("#4caf50")
            },
            error: ThemeColor{
                light: String::from("#c94444"),
                dark: String::from("#c94444")
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

        let (_, set_background) = use_css_var("--dp-0");
        let (_, set_emphasis) = use_css_var("--emphasis");
        let (_, set_secondary) = use_css_var("--secondary");

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

        set_white.set(self.white.color(&theme));
        set_black.set(self.black.color(&theme));
        set_gray.set(self.gray.color(&theme));

        set_background.set(self.background.color(&theme));
        set_emphasis.set(self.emphasis.color(&theme));
        set_secondary.set(self.secondary.color(&theme));

        set_shadow_small.set(self.shadow_small.shadow(&theme));
        set_shadow_medium.set(self.shadow_medium.shadow(&theme));
        set_shadow_large.set(self.shadow_large.shadow(&theme));
        set_shadow_x_large.set(self.shadow_x_large.shadow(&theme));

        set_success.set(self.success.color(&theme));
        set_error.set(self.error.color(&theme));

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
        self.theme = match self.theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light
        };

        self.apply();
    }
    
}

#[component]
pub fn ThemeToggler() -> impl IntoView {
    let global_theme = use_context::<RwSignal<GlobalTheme>>().unwrap();
    
    view!{
        <button on:click=move |_| global_theme.get().toggle()>
            {format!("{:?}", global_theme.get().theme)}
        </button>
    }
}

#[component]
pub fn GlobalThemeProvider(
    #[prop(default = RwSignal::new(GlobalTheme::default()))] global_theme: RwSignal<GlobalTheme>,
    children: Children,
) -> impl IntoView {
    
    provide_context(global_theme);
    
    view! {
        <body 
            class:dark=global_theme.get().theme == Theme::Dark
        >
            {children()}
        </body>
    }
}
