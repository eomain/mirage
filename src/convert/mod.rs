
/// Convert from and into an SVG
pub mod svg;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_test()
    {
        let svg = r#"
            <svg>
                <line x1="4" y1="4" x2="4" y2="4" />
                <rect width="100" height="100" />
                <line x1="0" y1="0" x2="3" y2="3" />
            </svg>
        "#;

        println!("{:?}", svg::into::string(svg).unwrap());
    }
}
