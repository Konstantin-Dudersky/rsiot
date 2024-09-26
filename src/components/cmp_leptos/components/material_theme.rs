#[allow(dead_code, non_camel_case_types, missing_docs)]
#[derive(Clone)]
pub enum MaterialTheme {
    sys_color_primary,
    sys_color_surface_tint,
    sys_color_on_primary,
    sys_color_primary_container,
    sys_color_on_primary_container,
    sys_color_secondary,
    sys_color_on_secondary,
    sys_color_secondary_container,
    sys_color_on_secondary_container,
    sys_color_tertiary,
    sys_color_on_tertiary,
    sys_color_tertiary_container,
    sys_color_on_tertiary_container,
    sys_color_error,
    sys_color_on_error,
    sys_color_error_container,
    sys_color_on_error_container,
    sys_color_background,
    sys_color_on_background,
    sys_color_surface,
    sys_color_on_surface,
    sys_color_surface_variant,
    sys_color_on_surface_variant,
    sys_color_outline,
    sys_color_outline_variant,
    sys_color_shadow,
    sys_color_scrim,
    sys_color_inverse_surface,
    sys_color_inverse_on_surface,
    sys_color_inverse_primary,
    sys_color_primary_fixed,
    sys_color_on_primary_fixed,
    sys_color_primary_fixed_dim,
    sys_color_on_primary_fixed_variant,
    sys_color_secondary_fixed,
    sys_color_on_secondary_fixed,
    sys_color_secondary_fixed_dim,
    sys_color_on_secondary_fixed_variant,
    sys_color_tertiary_fixed,
    sys_color_on_tertiary_fixed,
    sys_color_tertiary_fixed_dim,
    sys_color_on_tertiary_fixed_variant,
    sys_color_surface_dim,
    sys_color_surface_bright,
    sys_color_surface_container_lowest,
    sys_color_surface_container_low,
    sys_color_surface_container,
    sys_color_surface_container_high,
    sys_color_surface_container_highest,
    extended_color_green_color,
    extended_color_green_on_color,
    extended_color_green_color_container,
    extended_color_green_on_color_container,
    extended_color_yellow_color,
    extended_color_yellow_on_color,
    extended_color_yellow_color_container,
    extended_color_yellow_on_color_container,
}

impl MaterialTheme {
    /// Преобразование варианта перечисления в переменную CSS
    pub fn css_var(&self) -> String {
        match self {
            MaterialTheme::sys_color_primary => "var(--md-sys-color-primary)",
            MaterialTheme::sys_color_surface_tint => "var(--md-sys-color-surface-tint)",
            MaterialTheme::sys_color_on_primary => "var(--md-sys-color-on-primary)",
            MaterialTheme::sys_color_primary_container => "var(--md-sys-color-primary-container)",
            MaterialTheme::sys_color_on_primary_container => {
                "var(--md-sys-color-on-primary-container)"
            }
            MaterialTheme::sys_color_secondary => "var(--md-sys-color-secondary)",
            MaterialTheme::sys_color_on_secondary => "var(--md-sys-color-on-secondary)",
            MaterialTheme::sys_color_secondary_container => {
                "var(--md-sys-color-secondary-container)"
            }
            MaterialTheme::sys_color_on_secondary_container => {
                "var(--md-sys-color-on-secondary-container)"
            }
            MaterialTheme::sys_color_tertiary => "var(--md-sys-color-tertiary)",
            MaterialTheme::sys_color_on_tertiary => "var(--md-sys-color-on-tertiary)",
            MaterialTheme::sys_color_tertiary_container => "var(--md-sys-color-tertiary-container)",
            MaterialTheme::sys_color_on_tertiary_container => {
                "var(--md-sys-color-on-tertiary-container)"
            }
            MaterialTheme::sys_color_error => "var(--md-sys-color-error)",
            MaterialTheme::sys_color_on_error => "var(--md-sys-color-on-error)",
            MaterialTheme::sys_color_error_container => "var(--md-sys-color-error-container)",
            MaterialTheme::sys_color_on_error_container => "var(--md-sys-color-on-error-container)",
            MaterialTheme::sys_color_background => "var(--md-sys-color-background)",
            MaterialTheme::sys_color_on_background => "var(--md-sys-color-on-background)",
            MaterialTheme::sys_color_surface => "var(--md-sys-color-surface)",
            MaterialTheme::sys_color_on_surface => "var(--md-sys-color-on-surface)",
            MaterialTheme::sys_color_surface_variant => "var(--md-sys-color-surface-variant)",
            MaterialTheme::sys_color_on_surface_variant => "var(--md-sys-color-on-surface-variant)",
            MaterialTheme::sys_color_outline => "var(--md-sys-color-outline)",
            MaterialTheme::sys_color_outline_variant => "var(--md-sys-color-outline-variant)",
            MaterialTheme::sys_color_shadow => "var(--md-sys-color-shadow)",
            MaterialTheme::sys_color_scrim => "var(--md-sys-color-scrim)",
            MaterialTheme::sys_color_inverse_surface => "var(--md-sys-color-inverse-surface)",
            MaterialTheme::sys_color_inverse_on_surface => "var(--md-sys-color-inverse-on-surface)",
            MaterialTheme::sys_color_inverse_primary => "var(--md-sys-color-inverse-primary)",
            MaterialTheme::sys_color_primary_fixed => "var(--md-sys-color-primary-fixed)",
            MaterialTheme::sys_color_on_primary_fixed => "var(--md-sys-color-on-primary-fixed)",
            MaterialTheme::sys_color_primary_fixed_dim => "var(--md-sys-color-primary-fixed-dim)",
            MaterialTheme::sys_color_on_primary_fixed_variant => {
                "var(--md-sys-color-on-primary-fixed-variant)"
            }
            MaterialTheme::sys_color_secondary_fixed => "var(--md-sys-color-secondary-fixed)",
            MaterialTheme::sys_color_on_secondary_fixed => "var(--md-sys-color-on-secondary-fixed)",
            MaterialTheme::sys_color_secondary_fixed_dim => {
                "var(--md-sys-color-secondary-fixed-dim)"
            }
            MaterialTheme::sys_color_on_secondary_fixed_variant => {
                "var(--md-sys-color-on-secondary-fixed-variant)"
            }
            MaterialTheme::sys_color_tertiary_fixed => "var(--md-sys-color-tertiary-fixed)",
            MaterialTheme::sys_color_on_tertiary_fixed => "var(--md-sys-color-on-tertiary-fixed)",
            MaterialTheme::sys_color_tertiary_fixed_dim => "var(--md-sys-color-tertiary-fixed-dim)",
            MaterialTheme::sys_color_on_tertiary_fixed_variant => {
                "var(--md-sys-color-on-tertiary-fixed-variant)"
            }
            MaterialTheme::sys_color_surface_dim => "var(--md-sys-color-surface-dim)",
            MaterialTheme::sys_color_surface_bright => "var(--md-sys-color-surface-bright)",
            MaterialTheme::sys_color_surface_container_lowest => {
                "var(--md-sys-color-surface-container-lowest)"
            }
            MaterialTheme::sys_color_surface_container_low => {
                "var(--md-sys-color-surface-container-low)"
            }
            MaterialTheme::sys_color_surface_container => "var(--md-sys-color-surface-container)",
            MaterialTheme::sys_color_surface_container_high => {
                "var(--md-sys-color-surface-container-high)"
            }
            MaterialTheme::sys_color_surface_container_highest => {
                "var(--md-sys-color-surface-container-highest)"
            }
            MaterialTheme::extended_color_green_color => "var(--md-extended-color-green-color)",
            MaterialTheme::extended_color_green_on_color => {
                "var(--md-extended-color-green-on-color)"
            }
            MaterialTheme::extended_color_green_color_container => {
                "var(--md-extended-color-green-color-container)"
            }
            MaterialTheme::extended_color_green_on_color_container => {
                "var(--md-extended-color-green-on-color-container)"
            }
            MaterialTheme::extended_color_yellow_color => "var(--md-extended-color-yellow-color)",
            MaterialTheme::extended_color_yellow_on_color => {
                "var(--md-extended-color-yellow-on-color)"
            }
            MaterialTheme::extended_color_yellow_color_container => {
                "var(--md-extended-color-yellow-color-container)"
            }
            MaterialTheme::extended_color_yellow_on_color_container => {
                "var(--md-extended-color-yellow-on-color-container)"
            }
        }
        .to_string()
    }
}
