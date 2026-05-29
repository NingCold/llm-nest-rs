use std::fs::File;
use std::io::{BufReader, Read, Seek};
use std::path::Path;

use anyhow::{Context, Result, bail};
use byteorder::{LittleEndian, ReadBytesExt};

use super::enums::QuantType;

const GGUF_MAGIC: u32 = 0x46475547; // "GGUF" in little-endian

#[derive(Debug, Clone)]
pub struct GgufMetadata {
    pub version: u32,
    pub tensor_count: u64,
    pub arch: String,
    pub context_length: u64,
    pub vocab_size: u64,
    pub chat_template: String,
    pub embedding_length: u64,
    pub block_count: u64,
    pub quant_type: QuantType,
}

impl GgufMetadata {
    pub fn parse(path: &Path) -> Result<Self> {
        let file = File::open(path)
            .with_context(|| format!("Failed to open GGUF file: {}", path.display()))?;
        let mut reader = BufReader::new(file);

        let magic = reader.read_u32::<LittleEndian>()?;
        if magic != GGUF_MAGIC {
            bail!(
                "Not a GGUF file: {} (magic: 0x{:08X})",
                path.display(),
                magic
            );
        }

        let version = reader.read_u32::<LittleEndian>()?;
        let tensor_count = reader.read_u64::<LittleEndian>()?;
        let metadata_kv_count = reader.read_u64::<LittleEndian>()?;

        let mut arch = String::new();
        let mut context_length = 0u64;
        let mut vocab_size = 0u64;
        let mut chat_template = String::new();
        let mut embedding_length = 0u64;
        let mut block_count = 0u64;

        for _ in 0..metadata_kv_count {
            let key = read_gguf_string(&mut reader)?;
            let value_type = reader.read_u32::<LittleEndian>()?;
            let _value = read_gguf_value(&mut reader, value_type)?;

            match key.as_str() {
                "general.architecture" => {
                    if let GgufValue::String(s) = &_value {
                        arch = s.clone();
                    }
                }
                k if k == format!("{arch}.context_length") => {
                    context_length = gguf_value_to_u64(&_value);
                }
                k if k == format!("{arch}.vocab_size") => {
                    vocab_size = gguf_value_to_u64(&_value);
                }
                "tokenizer.chat_template" => {
                    if let GgufValue::String(s) = &_value {
                        chat_template = s.clone();
                    }
                }
                k if k == format!("{arch}.embedding_length") => {
                    embedding_length = gguf_value_to_u64(&_value);
                }
                k if k == format!("{arch}.block_count") => {
                    block_count = gguf_value_to_u64(&_value);
                }
                _ => {}
            }
        }

        let quant_type = if tensor_count > 0 {
            infer_quant_type(&mut reader).unwrap_or(QuantType::Unknown)
        } else {
            QuantType::Unknown
        };

        Ok(Self {
            version,
            tensor_count,
            arch,
            context_length,
            vocab_size,
            chat_template,
            embedding_length,
            block_count,
            quant_type,
        })
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum GgufValue {
    Uint8(u8),
    Int8(i8),
    Uint16(u16),
    Int16(i16),
    Uint32(u32),
    Int32(i32),
    Float32(f32),
    Bool(bool),
    String(String),
    Array(Vec<GgufValue>),
    Uint64(u64),
    Int64(i64),
    Float64(f64),
}

fn gguf_value_to_u64(value: &GgufValue) -> u64 {
    match value {
        GgufValue::Uint8(v) => *v as u64,
        GgufValue::Int8(v) => *v as u64,
        GgufValue::Uint16(v) => *v as u64,
        GgufValue::Int16(v) => *v as u64,
        GgufValue::Uint32(v) => *v as u64,
        GgufValue::Int32(v) => *v as u64,
        GgufValue::Uint64(v) => *v,
        GgufValue::Int64(v) => *v as u64,
        GgufValue::Float32(v) => *v as u64,
        GgufValue::Float64(v) => *v as u64,
        _ => 0,
    }
}

fn read_gguf_string(reader: &mut impl Read) -> Result<String> {
    let length = reader.read_u64::<LittleEndian>()?;
    let mut buf = vec![0u8; length as usize];
    reader.read_exact(&mut buf)?;
    Ok(String::from_utf8_lossy(&buf).into_owned())
}

fn read_gguf_value(reader: &mut (impl Read + Seek), value_type: u32) -> Result<GgufValue> {
    match value_type {
        0 => Ok(GgufValue::Uint8(reader.read_u8()?)),
        1 => Ok(GgufValue::Int8(reader.read_i8()?)),
        2 => Ok(GgufValue::Uint16(reader.read_u16::<LittleEndian>()?)),
        3 => Ok(GgufValue::Int16(reader.read_i16::<LittleEndian>()?)),
        4 => Ok(GgufValue::Uint32(reader.read_u32::<LittleEndian>()?)),
        5 => Ok(GgufValue::Int32(reader.read_i32::<LittleEndian>()?)),
        6 => Ok(GgufValue::Float32(reader.read_f32::<LittleEndian>()?)),
        7 => Ok(GgufValue::Bool(reader.read_u8()? != 0)),
        8 => Ok(GgufValue::String(read_gguf_string(reader)?)),
        9 => {
            let arr_type = reader.read_u32::<LittleEndian>()?;
            let arr_len = reader.read_u64::<LittleEndian>()?;
            let mut items = Vec::with_capacity(arr_len as usize);
            for _ in 0..arr_len {
                items.push(read_gguf_value(reader, arr_type)?);
            }
            Ok(GgufValue::Array(items))
        }
        10 => Ok(GgufValue::Uint64(reader.read_u64::<LittleEndian>()?)),
        11 => Ok(GgufValue::Int64(reader.read_i64::<LittleEndian>()?)),
        12 => Ok(GgufValue::Float64(reader.read_f64::<LittleEndian>()?)),
        _ => bail!("Unknown GGUF value type: {}", value_type),
    }
}

fn infer_quant_type(reader: &mut (impl Read + Seek)) -> Result<QuantType> {
    let _name = read_gguf_string(reader)?;
    let n_dims = reader.read_u32::<LittleEndian>()?;
    for _ in 0..n_dims {
        reader.read_u64::<LittleEndian>()?;
    }
    let tensor_type = reader.read_u32::<LittleEndian>()?;

    let quant = match tensor_type {
        0 => QuantType::F32,
        1 => QuantType::F16,
        2 => QuantType::Q4_0,
        3 => QuantType::Q4_1,
        7 => QuantType::Q5_0,
        8 => QuantType::Q5_1,
        10 => QuantType::Q8_0,
        11 => QuantType::Q8_1,
        14 => QuantType::Q2_K,
        15 => QuantType::Q3_K_S,
        16 => QuantType::Q3_K_M,
        17 => QuantType::Q4_K_S,
        18 => QuantType::Q4_K_M,
        19 => QuantType::Q5_K_S,
        20 => QuantType::Q5_K_M,
        21 => QuantType::Q6_K,
        30 => QuantType::IQ2_XXS,
        31 => QuantType::IQ3_XXS,
        36 => QuantType::IQ4_XS,
        _ => QuantType::Unknown,
    };

    Ok(quant)
}
