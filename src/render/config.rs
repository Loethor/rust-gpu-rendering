/// Knobs for the renderer. Every "feature" is just a parameter.
#[derive(Debug, Clone, Copy)]
pub struct RenderConfig {
    /// Shadow-ray occlusion test for direct lighting.
    pub shadows: bool,
    /// How many surface interactions a ray may have.
    /// 1 = direct view + direct light (metal appears black),
    /// 2+ = reflections. (Diffuse bounces/GI arrive with randomness.)
    pub max_bounces: u32,
    /// Rays per pixel. Wired now, but visually alive only once
    /// we add randomness (Phase 3): then it becomes anti-aliasing.
    pub samples_per_pixel: u32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            shadows: true,
            max_bounces: 4,
            samples_per_pixel: 1,
        }
    }
}

impl RenderConfig {
    // Builder-style setters: RenderConfig::default().shadows(false).bounces(2)
    pub fn shadows(mut self, on: bool) -> Self {
        self.shadows = on;
        self
    }

    pub fn bounces(mut self, n: u32) -> Self {
        self.max_bounces = n;
        self
    }

    pub fn samples(mut self, n: u32) -> Self {
        self.samples_per_pixel = n;
        self
    }
}
