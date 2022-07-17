#[derive(Debug, Clone)]
pub struct RGBA {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: f64,
}

impl std::fmt::Display for RGBA {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "rgba({},{},{},{})",
            self.r, self.g, self.b, self.a
        )
    }
}

impl RGBA {
    pub fn new(
        r: u32,
        g: u32,
        b: u32,
        a: f64,
    ) -> RGBA {
        RGBA { r, g, b, a }
    }
}
