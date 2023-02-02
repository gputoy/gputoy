//! Conversion from naga types to local types.
//! This is to avoid using naga types throughout gpu_common for two reasons.
//!     1. Naga is heavy, so importing it to gpu_common unconditionally will blow up
//!        the size of every other module (which only matters for wasm really).
//!     2. Naga types cannot be schema'd, so typescript would have no type checking when doing things with
//!        shader info in frontend.
//!
//! Compromises taken:
//!     * Arenas are vecs, this should be fine as long as order is preserved during conversion.
//!     * Handles are usize and 0-indexed, converted using the index() method.
//!     * StorageFlags are u32, converted using the bits() method
//!     * Functions only include name, params, and returns. Maybe in the future this can inlude more,
//!       but I don't think inner body info will be of use within gputoy for now.
//!
//! TODO:
//!     * un-omit constants and global_variables
//!

#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::common::{ImageDimension, StorageFormat};

type Handle = usize;
pub type StorageAccess = u32;

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CompileError {
    pub message: String,
    pub span: Option<SourceLocation>,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SourceLocation {
    /// 1-based line number.
    pub line_number: u32,
    /// 1-based column of the start of this span
    pub line_position: u32,
    /// 0-based Offset in code units (in bytes) of the start of the span.
    pub offset: u32,
    /// Length in code units (in bytes) of the span.
    pub length: u32,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Module {
    pub types: Vec<Type>,
    // pub constants: Vec<Constant>,
    // pub global_variables: Vec<GlobalVariable>,
    pub functions: Vec<Function>,
    pub entry_points: Vec<EntryPoint>,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Type {
    pub name: Option<String>,
    pub inner: TypeInner,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TypeInner {
    Scalar {
        kind: ScalarKind,
        width: u8,
    },
    Vector {
        size: VectorSize,
        kind: ScalarKind,
        width: u8,
    },
    Matrix {
        colums: VectorSize,
        rows: VectorSize,
        width: u8,
    },
    Atomic {
        kind: ScalarKind,
        width: u8,
    },
    Pointer {
        base: Handle,
        space: AddressSpace,
    },
    ValuePointer {
        size: Option<VectorSize>,
        kind: ScalarKind,
        width: u8,
        space: AddressSpace,
    },
    Array {
        base: Handle,
        size: ArraySize,
        stride: u32,
    },
    Struct {
        members: Vec<StructMember>,
        span: u32,
    },
    Image {
        dim: ImageDimension,
        arrayed: bool,
        class: ImageClass,
    },
    Sampler {
        comparison: bool,
    },
    BindingArray {
        base: Handle,
        size: ArraySize,
    },
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Function {
    pub name: Option<String>,
    pub arguments: Vec<FunctionArgument>,
    pub result: Option<FunctionResult>,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FunctionArgument {
    pub name: Option<String>,
    pub ty: Handle,
    pub binding: Option<Binding>,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FunctionResult {
    pub ty: Handle,
    pub binding: Option<Binding>,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EntryPoint {
    pub name: String,
    pub stage: ShaderStage,
    // Todo: Find out how to do From for private struct member
    // pub early_depth_test: Option<EarlyDepthTest>,
    pub workgroup_size: [u32; 3],
    pub function: Function,
}

#[allow(dead_code)]
#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EarlyDepthTest {
    pub conservative: Option<ConservativeDepth>,
}

#[allow(dead_code)]
#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConservativeDepth {
    GreaterEqual,
    LessEqual,
    Unchanged,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ShaderStage {
    Vertex,
    Fragment,
    Compute,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ScalarKind {
    Sint,
    Uint,
    Float,
    Bool,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VectorSize {
    Bi = 2,
    Tri = 3,
    Quad = 4,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AddressSpace {
    Function,
    Private,
    WorkGroup,
    Uniform,
    Storage { access: StorageAccess },
    Handle,
    PushConstant,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ArraySize {
    Constant(u32),
    Dynamic,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StructMember {
    pub name: Option<String>,
    pub ty: u32,
    pub binding: Option<Binding>,
    pub offset: u32,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ImageClass {
    Sampled {
        kind: ScalarKind,
        multi: bool,
    },
    Depth {
        multi: bool,
    },
    Storage {
        format: StorageFormat,
        access: StorageAccess,
    },
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Binding {
    Builtin(Builtin),
    Location {
        location: u32,
        interpolation: Option<Interpolation>,
        sampling: Option<Sampling>,
    },
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Builtin {
    Position { invariant: bool },
    ViewIndex,
    BaseInstance,
    BaseVertex,
    ClipDistance,
    CullDistance,
    InstanceIndex,
    PointSize,
    VertexIndex,
    FragDepth,
    FrontFacing,
    PrimitiveIndex,
    SampleIndex,
    SampleMask,
    GlobalInvocationId,
    LocalInvocationId,
    LocalInvocationIndex,
    WorkGroupId,
    WorkGroupSize,
    NumWorkGroups,
    PointCoord,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Interpolation {
    Perspective,
    Linear,
    Flat,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Sampling {
    Center,
    Centroid,
    Sample,
}

#[cfg(feature = "naga")]
mod naga_ext {
    use super::*;

    impl From<(&naga::front::wgsl::ParseError, &crate::file::File)> for CompileError {
        fn from((err, file): (&naga::front::wgsl::ParseError, &crate::file::File)) -> Self {
            Self {
                message: err
                    .emit_to_string(&file.data)
                    .replace("wgsl", &file.canonical_name()),
                span: err.location(&file.data).map(From::from),
            }
        }
    }

    impl From<naga::SourceLocation> for SourceLocation {
        fn from(loc: naga::SourceLocation) -> Self {
            Self {
                line_number: loc.line_number,
                line_position: loc.line_position,
                offset: loc.offset,
                length: loc.length,
            }
        }
    }

    impl From<naga::Module> for Module {
        fn from(module: naga::Module) -> Self {
            let mut types = Vec::new();
            let mut functions = Vec::new();
            for (handle, ty) in module.types.iter() {
                types.insert(handle.index(), ty.into());
            }
            for (handle, function) in module.functions.iter() {
                functions.insert(handle.index(), function.into());
            }
            let entry_points = module.entry_points.into_iter().map(From::from).collect();

            Self {
                types,
                functions,
                entry_points,
            }
        }
    }

    impl From<&naga::Type> for Type {
        fn from(ty: &naga::Type) -> Self {
            Self {
                name: ty.name.clone(),
                inner: (&ty.inner).into(),
            }
        }
    }

    impl From<&naga::TypeInner> for TypeInner {
        fn from(inner: &naga::TypeInner) -> Self {
            match inner {
                naga::TypeInner::Scalar { kind, width } => Self::Scalar {
                    kind: (*kind).into(),
                    width: *width,
                },
                naga::TypeInner::Vector { size, kind, width } => Self::Vector {
                    size: (*size).into(),
                    kind: (*kind).into(),
                    width: *width,
                },
                naga::TypeInner::Matrix {
                    columns,
                    rows,
                    width,
                } => Self::Matrix {
                    colums: (*columns).into(),
                    rows: (*rows).into(),
                    width: *width,
                },
                naga::TypeInner::Atomic { kind, width } => Self::Atomic {
                    kind: (*kind).into(),
                    width: *width,
                },
                naga::TypeInner::Pointer { base, space } => Self::Pointer {
                    base: base.index(),
                    space: (*space).into(),
                },
                naga::TypeInner::ValuePointer {
                    size,
                    kind,
                    width,
                    space,
                } => Self::ValuePointer {
                    size: size.map(From::from),
                    kind: (*kind).into(),
                    width: *width,
                    space: (*space).into(),
                },
                naga::TypeInner::Array { base, size, stride } => Self::Array {
                    base: base.index(),
                    size: (*size).into(),
                    stride: *stride,
                },
                naga::TypeInner::Struct { members, span } => Self::Struct {
                    members: members.iter().map(From::from).collect(),
                    span: *span,
                },
                naga::TypeInner::Image {
                    dim,
                    arrayed,
                    class,
                } => TypeInner::Image {
                    dim: (*dim).into(),
                    arrayed: *arrayed,
                    class: (*class).into(),
                },
                naga::TypeInner::Sampler { comparison } => Self::Sampler {
                    comparison: *comparison,
                },
                naga::TypeInner::BindingArray { base, size } => Self::BindingArray {
                    base: base.index(),
                    size: (*size).into(),
                },
            }
        }
    }

    impl From<&naga::Function> for Function {
        fn from(function: &naga::Function) -> Self {
            Self {
                arguments: function.arguments.iter().map(From::from).collect(),
                name: function.name.clone(),
                result: function.result.as_ref().map(From::from),
            }
        }
    }

    impl From<&naga::FunctionArgument> for FunctionArgument {
        fn from(arg: &naga::FunctionArgument) -> Self {
            Self {
                binding: arg.binding.as_ref().map(From::from),
                name: arg.name.clone(),
                ty: arg.ty.index(),
            }
        }
    }

    impl From<&naga::FunctionResult> for FunctionResult {
        fn from(result: &naga::FunctionResult) -> Self {
            Self {
                binding: result.binding.as_ref().map(From::from),
                ty: result.ty.index(),
            }
        }
    }

    impl From<naga::EntryPoint> for EntryPoint {
        fn from(entry_point: naga::EntryPoint) -> Self {
            Self {
                // early_depth_test: entry_point.early_depth_test.map(From::from),
                function: (&entry_point.function).into(),
                name: entry_point.name,
                stage: entry_point.stage.into(),
                workgroup_size: entry_point.workgroup_size,
            }
        }
    }

    impl From<naga::ConservativeDepth> for ConservativeDepth {
        fn from(conservative_depth: naga::ConservativeDepth) -> Self {
            match conservative_depth {
                naga::ConservativeDepth::GreaterEqual => Self::GreaterEqual,
                naga::ConservativeDepth::LessEqual => Self::LessEqual,
                naga::ConservativeDepth::Unchanged => Self::Unchanged,
            }
        }
    }

    impl From<naga::ShaderStage> for ShaderStage {
        fn from(stage: naga::ShaderStage) -> Self {
            match stage {
                naga::ShaderStage::Vertex => Self::Vertex,
                naga::ShaderStage::Fragment => Self::Fragment,
                naga::ShaderStage::Compute => Self::Compute,
            }
        }
    }

    impl From<naga::ScalarKind> for ScalarKind {
        fn from(kind: naga::ScalarKind) -> Self {
            match kind {
                naga::ScalarKind::Sint => Self::Sint,
                naga::ScalarKind::Uint => Self::Uint,
                naga::ScalarKind::Float => Self::Float,
                naga::ScalarKind::Bool => Self::Bool,
            }
        }
    }

    impl From<naga::VectorSize> for VectorSize {
        fn from(size: naga::VectorSize) -> Self {
            match size {
                naga::VectorSize::Bi => Self::Bi,
                naga::VectorSize::Tri => Self::Tri,
                naga::VectorSize::Quad => Self::Quad,
            }
        }
    }

    impl From<naga::AddressSpace> for AddressSpace {
        fn from(space: naga::AddressSpace) -> Self {
            match space {
                naga::AddressSpace::Function => Self::Function,
                naga::AddressSpace::Private => Self::Private,
                naga::AddressSpace::WorkGroup => Self::WorkGroup,
                naga::AddressSpace::Uniform => Self::Uniform,
                naga::AddressSpace::Storage { access } => Self::Storage {
                    access: access.bits(),
                },
                naga::AddressSpace::Handle => Self::Handle,
                naga::AddressSpace::PushConstant => Self::PushConstant,
            }
        }
    }

    impl From<naga::ArraySize> for ArraySize {
        fn from(size: naga::ArraySize) -> Self {
            match size {
                naga::ArraySize::Constant(handle) => Self::Constant(handle.index() as u32),
                naga::ArraySize::Dynamic => Self::Dynamic,
            }
        }
    }

    impl From<&naga::StructMember> for StructMember {
        fn from(member: &naga::StructMember) -> Self {
            Self {
                binding: member.binding.as_ref().map(From::from),
                name: member.name.clone(),
                offset: member.offset,
                ty: member.ty.index() as u32,
            }
        }
    }

    impl From<&naga::Binding> for Binding {
        fn from(binding: &naga::Binding) -> Self {
            match binding {
                naga::Binding::BuiltIn(builtin) => Self::Builtin((*builtin).into()),
                naga::Binding::Location {
                    location,
                    interpolation,
                    sampling,
                } => Self::Location {
                    location: *location,
                    interpolation: interpolation.map(From::from),
                    sampling: sampling.map(From::from),
                },
            }
        }
    }

    impl From<naga::ImageDimension> for crate::sys::common::ImageDimension {
        fn from(dim: naga::ImageDimension) -> Self {
            match dim {
                naga::ImageDimension::D1 => Self::D1,
                naga::ImageDimension::D2 => Self::D2,
                naga::ImageDimension::D3 => Self::D3,
                naga::ImageDimension::Cube => Self::Cube,
            }
        }
    }

    impl From<naga::ImageClass> for ImageClass {
        fn from(class: naga::ImageClass) -> Self {
            match class {
                naga::ImageClass::Sampled { kind, multi } => Self::Sampled {
                    kind: kind.into(),
                    multi,
                },
                naga::ImageClass::Depth { multi } => Self::Depth { multi },
                naga::ImageClass::Storage { format, access } => Self::Storage {
                    format: format.into(),
                    access: access.bits(),
                },
            }
        }
    }

    impl From<naga::Interpolation> for Interpolation {
        fn from(interpolation: naga::Interpolation) -> Self {
            match interpolation {
                naga::Interpolation::Perspective => Self::Perspective,
                naga::Interpolation::Linear => Self::Linear,
                naga::Interpolation::Flat => Self::Flat,
            }
        }
    }

    impl From<naga::Sampling> for Sampling {
        fn from(sampling: naga::Sampling) -> Self {
            match sampling {
                naga::Sampling::Center => Self::Center,
                naga::Sampling::Centroid => Self::Centroid,
                naga::Sampling::Sample => Self::Sample,
            }
        }
    }

    impl From<naga::BuiltIn> for Builtin {
        fn from(builtin: naga::BuiltIn) -> Self {
            match builtin {
                naga::BuiltIn::Position { invariant } => Self::Position { invariant },
                naga::BuiltIn::ViewIndex => Self::ViewIndex,
                naga::BuiltIn::BaseInstance => Self::BaseInstance,
                naga::BuiltIn::BaseVertex => Self::BaseVertex,
                naga::BuiltIn::ClipDistance => Self::ClipDistance,
                naga::BuiltIn::CullDistance => Self::CullDistance,
                naga::BuiltIn::InstanceIndex => Self::InstanceIndex,
                naga::BuiltIn::PointSize => Self::PointSize,
                naga::BuiltIn::VertexIndex => Self::VertexIndex,
                naga::BuiltIn::FragDepth => Self::FragDepth,
                naga::BuiltIn::FrontFacing => Self::FrontFacing,
                naga::BuiltIn::PrimitiveIndex => Self::PrimitiveIndex,
                naga::BuiltIn::SampleIndex => Self::SampleIndex,
                naga::BuiltIn::SampleMask => Self::SampleMask,
                naga::BuiltIn::GlobalInvocationId => Self::GlobalInvocationId,
                naga::BuiltIn::LocalInvocationId => Self::LocalInvocationId,
                naga::BuiltIn::LocalInvocationIndex => Self::LocalInvocationIndex,
                naga::BuiltIn::WorkGroupId => Self::WorkGroupId,
                naga::BuiltIn::WorkGroupSize => Self::WorkGroupSize,
                naga::BuiltIn::NumWorkGroups => Self::NumWorkGroups,
                naga::BuiltIn::PointCoord => Self::PointCoord,
            }
        }
    }

    impl From<naga::StorageFormat> for crate::sys::common::StorageFormat {
        fn from(format: naga::StorageFormat) -> Self {
            match format {
                naga::StorageFormat::R8Unorm => Self::R8Unorm,
                naga::StorageFormat::R8Snorm => Self::R8Snorm,
                naga::StorageFormat::R8Uint => Self::R8Uint,
                naga::StorageFormat::R8Sint => Self::R8Sint,
                naga::StorageFormat::R16Uint => Self::R16Uint,
                naga::StorageFormat::R16Sint => Self::R16Sint,
                naga::StorageFormat::R16Float => Self::R16Float,
                naga::StorageFormat::Rg8Unorm => Self::Rg8Unorm,
                naga::StorageFormat::Rg8Snorm => Self::Rg8Snorm,
                naga::StorageFormat::Rg8Uint => Self::Rg8Uint,
                naga::StorageFormat::Rg8Sint => Self::Rg8Sint,
                naga::StorageFormat::R32Uint => Self::R32Uint,
                naga::StorageFormat::R32Sint => Self::R32Sint,
                naga::StorageFormat::R32Float => Self::R32Float,
                naga::StorageFormat::Rg16Uint => Self::Rg16Uint,
                naga::StorageFormat::Rg16Sint => Self::Rg16Sint,
                naga::StorageFormat::Rg16Float => Self::Rg16Float,
                naga::StorageFormat::Rgba8Unorm => Self::Rgba8Unorm,
                naga::StorageFormat::Rgba8Snorm => Self::Rgba8Snorm,
                naga::StorageFormat::Rgba8Uint => Self::Rgba8Uint,
                naga::StorageFormat::Rgba8Sint => Self::Rgba8Sint,
                naga::StorageFormat::Rgb10a2Unorm => Self::Rgb10a2Unorm,
                naga::StorageFormat::Rg11b10Float => Self::Rg11b10Float,
                naga::StorageFormat::Rg32Uint => Self::Rg32Uint,
                naga::StorageFormat::Rg32Sint => Self::Rg32Sint,
                naga::StorageFormat::Rg32Float => Self::Rg32Float,
                naga::StorageFormat::Rgba16Uint => Self::Rgba16Uint,
                naga::StorageFormat::Rgba16Sint => Self::Rgba16Sint,
                naga::StorageFormat::Rgba16Float => Self::Rgba16Float,
                naga::StorageFormat::Rgba32Uint => Self::Rgba32Uint,
                naga::StorageFormat::Rgba32Sint => Self::Rgba32Sint,
                naga::StorageFormat::Rgba32Float => Self::Rgba32Float,
                naga::StorageFormat::R16Unorm => todo!(),
                naga::StorageFormat::R16Snorm => todo!(),
                naga::StorageFormat::Rg16Unorm => todo!(),
                naga::StorageFormat::Rg16Snorm => todo!(),
                naga::StorageFormat::Rgba16Unorm => todo!(),
                naga::StorageFormat::Rgba16Snorm => todo!(),
            }
        }
    }
}
