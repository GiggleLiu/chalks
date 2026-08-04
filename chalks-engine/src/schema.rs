use serde::{Deserialize, Serialize};

fn d_smooth() -> f64 {
    0.7
}
fn d_rough() -> f64 {
    1.0
}
fn d_width() -> f64 {
    1.2
}
fn d_taper() -> f64 {
    0.5
}
fn d_passes() -> u32 {
    1
}
fn d_pattern() -> String {
    "hachure".into()
}
fn d_angle() -> f64 {
    45.0
}
fn d_spacing() -> f64 {
    4.0
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StrokeStyle {
    #[serde(default = "d_smooth")]
    pub smoothness: f64,
    #[serde(default = "d_rough")]
    pub roughness: f64,
    #[serde(default = "d_width")]
    pub width: f64,
    #[serde(default = "d_taper")]
    pub taper: f64,
    #[serde(default = "d_passes")]
    pub passes: u32,
}

impl Default for StrokeStyle {
    fn default() -> Self {
        StrokeStyle {
            smoothness: d_smooth(),
            roughness: d_rough(),
            width: d_width(),
            taper: d_taper(),
            passes: d_passes(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FillStyle {
    #[serde(default = "d_smooth")]
    pub smoothness: f64,
    #[serde(default = "d_rough")]
    pub roughness: f64,
    #[serde(default = "d_width")]
    pub width: f64,
    #[serde(default = "d_pattern")]
    pub pattern: String,
    #[serde(default = "d_angle")]
    pub angle: f64,
    #[serde(default = "d_spacing")]
    pub spacing: f64,
}

impl Default for FillStyle {
    fn default() -> Self {
        FillStyle {
            smoothness: d_smooth(),
            roughness: d_rough(),
            width: d_width(),
            pattern: d_pattern(),
            angle: d_angle(),
            spacing: d_spacing(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StrokeRequest {
    pub points: Vec<[f64; 2]>,
    #[serde(default)]
    pub closed: bool,
    #[serde(default)]
    pub style: StrokeStyle,
    #[serde(default)]
    pub seed: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FillRequest {
    pub boundaries: Vec<Vec<[f64; 2]>>,
    #[serde(default)]
    pub style: FillStyle,
    #[serde(default)]
    pub seed: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Subpath {
    pub start: [f64; 2],
    pub cubics: Vec<[[f64; 2]; 3]>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Path {
    pub subpaths: Vec<Subpath>,
    pub weight: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub paths: Vec<Path>,
}

fn unit(name: &str, v: f64) -> Result<(), String> {
    if (0.0..=1.0).contains(&v) {
        Ok(())
    } else {
        Err(format!("chalks-engine: {name} must be in [0, 1], got {v}"))
    }
}

impl StrokeRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.points.len() < 2 {
            return Err("chalks-engine: stroke needs at least 2 points".into());
        }
        unit("smoothness", self.style.smoothness)?;
        unit("taper", self.style.taper)?;
        if self.style.width <= 0.0 {
            return Err("chalks-engine: width must be positive".into());
        }
        if self.style.passes < 1 {
            return Err("chalks-engine: passes must be >= 1".into());
        }
        Ok(())
    }
}

impl FillRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.boundaries.is_empty() {
            return Err("chalks-engine: fill needs at least one boundary".into());
        }
        for b in &self.boundaries {
            if b.len() < 3 {
                return Err("chalks-engine: fill boundary needs at least 3 points".into());
            }
        }
        unit("smoothness", self.style.smoothness)?;
        if self.style.width <= 0.0 {
            return Err("chalks-engine: width must be positive".into());
        }
        if self.style.spacing <= 0.0 {
            return Err("chalks-engine: spacing must be positive".into());
        }
        if !["hachure", "scribble", "shade"].contains(&self.style.pattern.as_str()) {
            return Err(format!(
                "chalks-engine: unknown fill pattern: {} (expected hachure, scribble, or shade)",
                self.style.pattern
            ));
        }
        Ok(())
    }
}
