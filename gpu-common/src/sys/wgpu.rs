#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

#[cfg(feature = "wgpu")]
pub use wgpu_ext::*;

use super::common::{ImageDimension, TextureFormat};

bitflags::bitflags! {
    #[cfg_attr(feature = "schema", derive(JsonSchema))]
    pub struct BufferUsages: u32 {
        const MAP_READ= 1 << 0;
        const MAP_WRITE= 1 << 1;
        const COPY_SRC= 1 << 2;
        const COPY_DST= 1 << 3;
        const INDEX= 1 << 4;
        const VERTEX = 1 << 5;
        const UNFIORM = 1 << 6;
        const STORAGE = 1 << 7;
        const INDIRECT = 1 << 8;
    }

    #[cfg_attr(feature = "schema", derive(JsonSchema))]
    pub struct TextureUsages: u32 {
        const COPY_SRC = 1 << 0;
        const COPY_DST = 1 << 1;
        const TEXTURE_BINDING = 1 << 2;
        const STORAGE_BINDING = 1 << 3;
        const RENDER_ATTACHMENT = 1 << 4;
    }
}

#[cfg(feature = "serialize")]
bitflags_serde_shim::impl_serde_for_bitflags!(BufferUsages);
#[cfg(feature = "serialize")]
bitflags_serde_shim::impl_serde_for_bitflags!(TextureUsages);

pub trait ResourceArguments {
    fn ident(&self) -> &str;
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(
    any(feature = "serialize", feature = "deserialize"),
    serde(tag = "ty", content = "c")
)]
pub enum ResourceArgs {
    Buffer(BufferArgs),
    Texture(TextureArgs),
    Sampler(SamplerArgs),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(
    any(feature = "serialize", feature = "deserialize"),
    serde(rename_all = "camelCase")
)]
pub struct BufferArgs {
    pub label: String,
    pub binding_type: BufferBindingType,
    pub usage: BufferUsages,
    pub mapped_at_creation: bool,
    pub size: u32,
}

impl ResourceArguments for BufferArgs {
    #[inline]
    fn ident(&self) -> &str {
        self.label.as_str()
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub enum BufferBindingType {
    Uniform,
    Storage,
    ReadonlyStorage,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct TextureArgs {
    pub label: String,
    pub size: [u32; 3],
    pub mip_level_count: u32,
    pub sample_count: u32,
    pub dimension: ImageDimension,
    pub format: TextureFormat,
    pub usage: TextureUsages,
}

impl ResourceArguments for TextureArgs {
    #[inline]
    fn ident(&self) -> &str {
        self.label.as_str()
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct SamplerArgs {}

#[cfg(feature = "wgpu")]
mod wgpu_ext {
    use std::num::NonZeroU32;

    use super::*;
    use crate::sys::common;

    impl<'a> From<&'a BufferArgs> for wgpu::BufferDescriptor<'a> {
        fn from(args: &'a BufferArgs) -> Self {
            Self {
                label: Some(&args.label),
                mapped_at_creation: args.mapped_at_creation,
                size: args.size as u64,
                usage: args.usage.into(),
            }
        }
    }

    impl From<BufferBindingType> for wgpu::BufferBindingType {
        fn from(ty: BufferBindingType) -> Self {
            match ty {
                BufferBindingType::Uniform => Self::Uniform,
                BufferBindingType::Storage => Self::Storage { read_only: false },
                BufferBindingType::ReadonlyStorage => Self::Storage { read_only: true },
            }
        }
    }

    impl From<BufferUsages> for wgpu::BufferUsages {
        fn from(usages: BufferUsages) -> Self {
            wgpu::BufferUsages::from_bits_truncate(usages.bits)
        }
    }

    impl<'a> From<&'a TextureArgs> for wgpu::TextureDescriptor<'a> {
        fn from(args: &'a TextureArgs) -> Self {
            Self {
                label: Some(&args.label),
                dimension: args.dimension.into(),
                format: args.format.into(),
                mip_level_count: args.mip_level_count,
                sample_count: args.sample_count,
                size: wgpu::Extent3d {
                    width: args.size[0],
                    height: args.size[1],
                    depth_or_array_layers: args.size[2],
                },
                usage: args.usage.into(),
            }
        }
    }

    impl From<ImageDimension> for wgpu::TextureDimension {
        fn from(dim: ImageDimension) -> Self {
            match dim {
                ImageDimension::D1 => Self::D1,
                ImageDimension::D2 | ImageDimension::Cube => Self::D2,
                ImageDimension::D3 => Self::D3,
            }
        }
    }

    impl<'a> From<&'a TextureArgs> for wgpu::TextureViewDescriptor<'a> {
        fn from(args: &'a TextureArgs) -> Self {
            Self {
                label: Some(&args.label),
                format: Some(args.format.into()),
                // TODO: add special arg fields for texture view
                dimension: None,
                aspect: wgpu::TextureAspect::All,
                base_mip_level: 0,
                mip_level_count: NonZeroU32::new(args.mip_level_count),
                base_array_layer: 0,
                array_layer_count: NonZeroU32::new(args.size[2]),
            }
        }
    }

    impl From<&TextureArgs> for wgpu::TextureViewDimension {
        fn from(args: &TextureArgs) -> Self {
            match (args.dimension, args.size[2]) {
                (ImageDimension::D1, _) => Self::D1,
                (ImageDimension::D2, depth) if depth == 1 => Self::D2,
                (ImageDimension::D2, _) => Self::D2Array,
                (ImageDimension::D3, _) => Self::D3,
                (ImageDimension::Cube, depth) if depth == 1 => Self::Cube,
                (ImageDimension::Cube, _) => Self::CubeArray,
            }
        }
    }

    impl From<TextureUsages> for wgpu::TextureUsages {
        fn from(usages: TextureUsages) -> Self {
            wgpu::TextureUsages::from_bits_truncate(usages.bits)
        }
    }

    impl From<common::TextureFormat> for wgpu::TextureFormat {
        fn from(format: common::TextureFormat) -> Self {
            match format {
                TextureFormat::R8Unorm => Self::R8Unorm,
                TextureFormat::R8Snorm => Self::R8Snorm,
                TextureFormat::R8Uint => Self::R8Uint,
                TextureFormat::R8Sint => Self::R8Sint,
                TextureFormat::R16Uint => Self::R16Uint,
                TextureFormat::R16Sint => Self::R16Sint,
                TextureFormat::R16Float => Self::R16Float,
                TextureFormat::Rg8Unorm => Self::Rg8Unorm,
                TextureFormat::Rg8Snorm => Self::Rg8Snorm,
                TextureFormat::Rg8Uint => Self::Rg8Uint,
                TextureFormat::Rg8Sint => Self::Rg8Sint,
                TextureFormat::R32Uint => Self::R32Uint,
                TextureFormat::R32Sint => Self::R32Sint,
                TextureFormat::R32Float => Self::R32Float,
                TextureFormat::Rg16Uint => Self::Rg16Uint,
                TextureFormat::Rg16Sint => Self::Rg16Sint,
                TextureFormat::Rg16Float => Self::Rg16Float,
                TextureFormat::Rgba8Unorm => Self::Rgba8Unorm,
                TextureFormat::Rgba8UnormSrgb => Self::Rgba8UnormSrgb,
                TextureFormat::Rgba8Snorm => Self::Rgba8Snorm,
                TextureFormat::Rgba8Uint => Self::Rgba8Uint,
                TextureFormat::Rgba8Sint => Self::Rgba8Sint,
                TextureFormat::Bgra8Unorm => Self::Bgra8Unorm,
                TextureFormat::Bgra8UnormSrgb => Self::Bgra8UnormSrgb,
                TextureFormat::Rgb10a2Unorm => Self::Rgb10a2Unorm,
                TextureFormat::Rg11b10Float => Self::Rg11b10Float,
                TextureFormat::Rg32Uint => Self::Rg32Uint,
                TextureFormat::Rg32Sint => Self::Rg32Sint,
                TextureFormat::Rg32Float => Self::Rg32Float,
                TextureFormat::Rgba16Uint => Self::Rgba16Uint,
                TextureFormat::Rgba16Sint => Self::Rgba16Sint,
                TextureFormat::Rgba16Float => Self::Rgba16Float,
                TextureFormat::Rgba32Uint => Self::Rgba32Uint,
                TextureFormat::Rgba32Sint => Self::Rgba32Sint,
                TextureFormat::Rgba32Float => Self::Rgba32Float,
                TextureFormat::Depth32Float => Self::Depth32Float,
                TextureFormat::Depth32FloatStencil8 => Self::Depth32FloatStencil8,
                TextureFormat::Depth24Plus => Self::Depth24Plus,
                TextureFormat::Depth24PlusStencil8 => Self::Depth24PlusStencil8,
                TextureFormat::Depth24UnormStencil8 => Self::Depth24UnormStencil8,
            }
        }
    }
    impl From<wgpu::TextureFormat> for common::TextureFormat {
        fn from(format: wgpu::TextureFormat) -> Self {
            match format {
                wgpu::TextureFormat::R8Unorm => Self::R8Unorm,
                wgpu::TextureFormat::R8Snorm => Self::R8Snorm,
                wgpu::TextureFormat::R8Uint => Self::R8Uint,
                wgpu::TextureFormat::R8Sint => Self::R8Sint,
                wgpu::TextureFormat::R16Uint => Self::R16Uint,
                wgpu::TextureFormat::R16Sint => Self::R16Sint,
                wgpu::TextureFormat::R16Float => Self::R16Float,
                wgpu::TextureFormat::Rg8Unorm => Self::Rg8Unorm,
                wgpu::TextureFormat::Rg8Snorm => Self::Rg8Snorm,
                wgpu::TextureFormat::Rg8Uint => Self::Rg8Uint,
                wgpu::TextureFormat::Rg8Sint => Self::Rg8Sint,
                wgpu::TextureFormat::R32Uint => Self::R32Uint,
                wgpu::TextureFormat::R32Sint => Self::R32Sint,
                wgpu::TextureFormat::R32Float => Self::R32Float,
                wgpu::TextureFormat::Rg16Uint => Self::Rg16Uint,
                wgpu::TextureFormat::Rg16Sint => Self::Rg16Sint,
                wgpu::TextureFormat::Rg16Float => Self::Rg16Float,
                wgpu::TextureFormat::Rgba8Unorm => Self::Rgba8Unorm,
                wgpu::TextureFormat::Rgba8UnormSrgb => Self::Rgba8UnormSrgb,
                wgpu::TextureFormat::Rgba8Snorm => Self::Rgba8Snorm,
                wgpu::TextureFormat::Rgba8Uint => Self::Rgba8Uint,
                wgpu::TextureFormat::Rgba8Sint => Self::Rgba8Sint,
                wgpu::TextureFormat::Bgra8Unorm => Self::Bgra8Unorm,
                wgpu::TextureFormat::Bgra8UnormSrgb => Self::Bgra8UnormSrgb,
                wgpu::TextureFormat::Rgb10a2Unorm => Self::Rgb10a2Unorm,
                wgpu::TextureFormat::Rg11b10Float => Self::Rg11b10Float,
                wgpu::TextureFormat::Rg32Uint => Self::Rg32Uint,
                wgpu::TextureFormat::Rg32Sint => Self::Rg32Sint,
                wgpu::TextureFormat::Rg32Float => Self::Rg32Float,
                wgpu::TextureFormat::Rgba16Uint => Self::Rgba16Uint,
                wgpu::TextureFormat::Rgba16Sint => Self::Rgba16Sint,
                wgpu::TextureFormat::Rgba16Float => Self::Rgba16Float,
                wgpu::TextureFormat::Rgba32Uint => Self::Rgba32Uint,
                wgpu::TextureFormat::Rgba32Sint => Self::Rgba32Sint,
                wgpu::TextureFormat::Rgba32Float => Self::Rgba32Float,
                wgpu::TextureFormat::Depth32Float => Self::Depth32Float,
                wgpu::TextureFormat::Depth32FloatStencil8 => Self::Depth32FloatStencil8,
                wgpu::TextureFormat::Depth24Plus => Self::Depth24Plus,
                wgpu::TextureFormat::Depth24PlusStencil8 => Self::Depth24PlusStencil8,
                wgpu::TextureFormat::Depth24UnormStencil8 => Self::Depth24UnormStencil8,
                _ => todo!(),
            }
        }
    }

    impl From<common::TextureFormat> for wgpu::TextureSampleType {
        fn from(format: common::TextureFormat) -> Self {
            match format {
                TextureFormat::R8Unorm
                | TextureFormat::R16Uint
                | TextureFormat::R8Uint
                | TextureFormat::Rg8Uint
                | TextureFormat::R32Uint
                | TextureFormat::Rg16Uint
                | TextureFormat::Rgba8Uint
                | TextureFormat::Rgba16Uint
                | TextureFormat::Rg32Uint
                | TextureFormat::Rgba32Uint => Self::Uint,
                TextureFormat::R8Sint
                | TextureFormat::Rg8Sint
                | TextureFormat::R16Sint
                | TextureFormat::R32Sint
                | TextureFormat::Rg16Sint
                | TextureFormat::Rgba8Sint
                | TextureFormat::Rg32Sint
                | TextureFormat::Rgba16Sint
                | TextureFormat::Rgba32Sint => Self::Sint,
                TextureFormat::R8Snorm
                | TextureFormat::Rg8Unorm
                | TextureFormat::Rgba8Unorm
                | TextureFormat::Rgba8UnormSrgb
                | TextureFormat::Bgra8Unorm
                | TextureFormat::Bgra8UnormSrgb
                | TextureFormat::Rgb10a2Unorm
                | TextureFormat::Rg8Snorm
                | TextureFormat::Rgba8Snorm
                | TextureFormat::R16Float
                | TextureFormat::R32Float
                | TextureFormat::Rg16Float
                | TextureFormat::Rg11b10Float
                | TextureFormat::Rg32Float
                | TextureFormat::Rgba16Float
                | TextureFormat::Rgba32Float => Self::Float { filterable: false },
                TextureFormat::Depth32Float
                | TextureFormat::Depth32FloatStencil8
                | TextureFormat::Depth24Plus
                | TextureFormat::Depth24PlusStencil8
                | TextureFormat::Depth24UnormStencil8 => Self::Depth,
            }
        }
    }
}
