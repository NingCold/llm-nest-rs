use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum QuantType {
    F32,
    F16,
    Q8_0,
    Q8_1,
    Q8_K,
    Q6_K,
    Q5_0,
    Q5_1,
    Q5_K_M,
    Q5_K_S,
    Q4_0,
    Q4_1,
    Q4_K_M,
    Q4_K_S,
    Q3_K_M,
    Q3_K_S,
    Q2_K,
    IQ4_XS,
    IQ3_XXS,
    IQ2_XXS,
    Unknown,
}

impl fmt::Display for QuantType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::F32 => write!(f, "F32"),
            Self::F16 => write!(f, "F16"),
            Self::Q8_0 => write!(f, "Q8_0"),
            Self::Q8_1 => write!(f, "Q8_1"),
            Self::Q8_K => write!(f, "Q8_K"),
            Self::Q6_K => write!(f, "Q6_K"),
            Self::Q5_0 => write!(f, "Q5_0"),
            Self::Q5_1 => write!(f, "Q5_1"),
            Self::Q5_K_M => write!(f, "Q5_K_M"),
            Self::Q5_K_S => write!(f, "Q5_K_S"),
            Self::Q4_0 => write!(f, "Q4_0"),
            Self::Q4_1 => write!(f, "Q4_1"),
            Self::Q4_K_M => write!(f, "Q4_K_M"),
            Self::Q4_K_S => write!(f, "Q4_K_S"),
            Self::Q3_K_M => write!(f, "Q3_K_M"),
            Self::Q3_K_S => write!(f, "Q3_K_S"),
            Self::Q2_K => write!(f, "Q2_K"),
            Self::IQ4_XS => write!(f, "IQ4_XS"),
            Self::IQ3_XXS => write!(f, "IQ3_XXS"),
            Self::IQ2_XXS => write!(f, "IQ2_XXS"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl QuantType {
    pub fn from_gguf_type(type_id: u32) -> Self {
        match type_id {
            0 => Self::F32,
            1 => Self::F16,
            2 => Self::Q4_0,
            3 => Self::Q4_1,
            6 => Self::Q5_0,
            7 => Self::Q5_1,
            8 => Self::Q8_0,
            9 => Self::Q8_1,
            10 => Self::Q2_K,
            11 => Self::Q3_K_S,
            12 => Self::Q3_K_M,
            13 => Self::Q4_K_S,
            14 => Self::Q4_K_M,
            15 => Self::Q5_K_S,
            16 => Self::Q5_K_M,
            17 => Self::Q6_K,
            18 => Self::Q8_K,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelStatus {
    Available,
    Downloading,
    Loading,
    Loaded,
    Error,
}

impl fmt::Display for ModelStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Available => write!(f, "available"),
            Self::Downloading => write!(f, "downloading"),
            Self::Loading => write!(f, "loading"),
            Self::Loaded => write!(f, "loaded"),
            Self::Error => write!(f, "error"),
        }
    }
}
