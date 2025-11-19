
pub fn week_color(week: u32) -> String {
    let week = week.clamp(1, 52);

    // Start hue for light blue ~200° (HSL)
    // End hue for dark purple ~270°
    let start_hue = 200.0;
    let end_hue = 270.0;

    // Interpolate hue linearly based on week number
    let hue = start_hue + (end_hue - start_hue) * ((week - 1) as f32 / 51.0);

    // Fixed saturation and lightness for consistent appearance
    let s = 0.65;
    let l = 0.55;

    // Convert HSL to RGB
    let (r, g, b) = hsl_to_rgb(hue, s, l);

    // Convert RGB to hex string
    format!("{:02x}{:02x}{:02x}", (r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

/// Convert HSL to RGB
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    let (r1, g1, b1) = match h_prime as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        5 | 6 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };
    let m = l - c / 2.0;
    (r1 + m, g1 + m, b1 + m)
}