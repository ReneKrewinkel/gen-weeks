use rand::Rng;

// Generate random hex color
pub fn random_hex_color() -> String {
    let mut r#gen = rand::thread_rng();
    format!("{:02x}{:02x}{:02x}", r#gen.r#gen::<u8>(), r#gen.r#gen::<u8>(), r#gen.r#gen::<u8>())
}

/// Generate a hex color for a week number (1-52) that smoothly transitions
/// from light yellow (week 1) to dark purple (week 52)
pub fn week_color(week: u32) -> String {
    // Clamp week to 1..52
    let week = week.clamp(1, 52);

    // Map week to a hue value from yellow (~60°) to purple (~270°)
    // GitHub labels expect hex, so we'll use HSL -> RGB conversion
    let start_hue = 60.0;   // yellow
    let end_hue = 270.0;    // purple
    let hue = start_hue + (end_hue - start_hue) * ((week - 1) as f32 / 51.0);

    // Fixed saturation and lightness for nice label colors
    let s = 0.65; // 65% saturation
    let l = 0.55; // 55% lightness

    // Convert HSL to RGB
    let (r, g, b) = hsl_to_rgb(hue, s, l);

    // Convert to hex string
    format!("{:02x}{:02x}{:02x}", (r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

/// Convert HSL to RGB
/// h in degrees 0..360, s and l in 0..1
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
