const BASE_PALETTE: [(u8, u8, u8); 30] = [
    (31, 119, 180),  // #1f77b4 (Blue)
    (255, 127, 14),  // #ff7f0e (Orange)
    (44, 160, 44),   // #2ca02c (Green)
    (214, 39, 40),   // #d62728 (Red)
    (148, 103, 189), // #9467bd (Purple)
    (140, 86, 75),   // #8c564b (Brown)
    (227, 119, 194), // #e377c2 (Pink)
    (127, 127, 127), // #7f7f7f (Gray)
    (188, 189, 34),  // #bcbd22 (Olive)
    (23, 190, 207),  // #17becf (Teal)
    (174, 199, 232), // #aec7e8 (Light Blue)
    (255, 187, 120), // #ffbb78 (Light Orange)
    (152, 223, 138), // #98df8a (Light Green)
    (255, 152, 150), // #ff9896 (Light Red)
    (197, 176, 213), // #c5b0d5 (Light Purple)
    (196, 156, 148), // #c49c94 (Light Brown)
    (247, 182, 210), // #f7b6d2 (Light Pink)
    (199, 199, 199), // #c7c7c7 (Light Gray)
    (219, 219, 141), // #dbdb8d (Light Olive)
    (158, 218, 229), // #9edae5 (Light Teal)
    (255, 255, 51),  // #ffff33 (Bright Yellow)
    (0, 128, 128),   // #008080 (Teal)
    (128, 0, 128),   // #800080 (Purple)
    (255, 165, 0),   // #ffa500 (Orange)
    (0, 255, 0),     // #00ff00 (Lime)
    (128, 128, 128), // #808080 (Gray)
    (255, 20, 147),  // #ff1493 (Deep Pink)
    (75, 0, 130),    // #4b0082 (Indigo)
    (255, 69, 0),    // #ff4500 (Orange Red)
    (0, 191, 255),   // #00bfff (Deep Sky Blue)
];
const FACTOR: f32 = 0.2;

// Function to generate color variants
pub(crate) fn generate_colors(amount: usize) -> Vec<String> {
    let mut factor = 0.0;
    let mut colors = Vec::new();

    for chunk in Vec::from_iter(0..amount).chunks(BASE_PALETTE.len()) {
        for (i, _) in chunk.iter().enumerate() {
            let (r, g, b) = BASE_PALETTE[i % BASE_PALETTE.len()];

            let variant_r = ((r as f32 + factor * 255.0) % 256.0) as u8;
            let variant_g = ((g as f32 + factor * 255.0) % 256.0) as u8;
            let variant_b = ((b as f32 + factor * 255.0) % 256.0) as u8;

            colors.push(rgb_to_hex(variant_r, variant_g, variant_b));
        }

        factor += FACTOR; // Slightly increase the factor over time to generate more variation
    }

    colors
}

// Function to convert RGB to hex
fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

#[cfg(test)]
mod tests {
    use crate::graph::node::color::{rgb_to_hex, BASE_PALETTE};

    use super::generate_colors;

    #[test]
    fn generates_colors_dynamically() {
        // when
        let colors = generate_colors(40);

        // then
        let mut expected_colors: Vec<String> = Vec::from(BASE_PALETTE)
            .into_iter()
            .map(|(r, g, b)| rgb_to_hex(r, g, b))
            .collect();

        expected_colors.extend(vec![
            "#52AAE7".to_string(),
            "#32B241".to_string(),
            "#5FD35F".to_string(),
            "#095A5B".to_string(),
            "#C79AF0".to_string(),
            "#BF897E".to_string(),
            "#16AAF5".to_string(),
            "#B2B2B2".to_string(),
            "#EFF055".to_string(),
            "#4AF102".to_string(),
        ]);

        assert_eq!(colors, expected_colors)
    }
}
