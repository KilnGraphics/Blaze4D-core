use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

use ash::vk;

#[derive(Eq, Copy, Clone, Debug)]
pub struct CompatibilityClass {
    name: &'static str,
}

macro_rules! define_compatibility_class {
    ($name: ident) => {
        pub const $name: CompatibilityClass = CompatibilityClass::new(stringify!($name));
    }
}

impl CompatibilityClass {
    pub const fn new(name: &'static str) -> Self {
        CompatibilityClass { name }
    }

    pub const fn get_name(&self) -> &'static str {
        self.name
    }

    define_compatibility_class!(BIT8);
    define_compatibility_class!(BIT16);
    define_compatibility_class!(BIT24);
    define_compatibility_class!(BIT32);
    define_compatibility_class!(BIT32_G8B8G8R8);
    define_compatibility_class!(BIT32_B8G8R8G8);
    define_compatibility_class!(BIT48);
    define_compatibility_class!(BIT64);
    define_compatibility_class!(BIT64_R10G10B10A10);
    define_compatibility_class!(BIT64_G10B10G10R10);
    define_compatibility_class!(BIT64_B10G10R10G10);
    define_compatibility_class!(BIT64_R12G12B12A12);
    define_compatibility_class!(BIT64_G12B12G12R12);
    define_compatibility_class!(BIT64_B12G12R12G12);
    define_compatibility_class!(BIT64_G16B16G16R16);
    define_compatibility_class!(BIT64_B16G16R16G16);
    define_compatibility_class!(BIT96);
    define_compatibility_class!(BIT128);
    define_compatibility_class!(BIT192);
    define_compatibility_class!(BIT256);
    define_compatibility_class!(BC1_RGB);
    define_compatibility_class!(BC1_RGBA);
    define_compatibility_class!(BC2);
    define_compatibility_class!(BC3);
    define_compatibility_class!(BC4);
    define_compatibility_class!(BC5);
    define_compatibility_class!(BC6H);
    define_compatibility_class!(BC7);
    define_compatibility_class!(ETC2_RGB);
    define_compatibility_class!(ETC2_RGBA);
    define_compatibility_class!(ETC2_EAC_RGBA);
    define_compatibility_class!(EAC_R);
    define_compatibility_class!(EAC_RG);
    define_compatibility_class!(ASTC_4X4);
    define_compatibility_class!(ASTC_5X4);
    define_compatibility_class!(ASTC_5X5);
    define_compatibility_class!(ASTC_6X5);
    define_compatibility_class!(ASTC_6X6);
    define_compatibility_class!(ASTC_8X5);
    define_compatibility_class!(ASTC_8X6);
    define_compatibility_class!(ASTC_8X8);
    define_compatibility_class!(ASTC_10X5);
    define_compatibility_class!(ASTC_10X6);
    define_compatibility_class!(ASTC_10X8);
    define_compatibility_class!(ASTC_10X10);
    define_compatibility_class!(ASTC_12X10);
    define_compatibility_class!(ASTC_12X12);
    define_compatibility_class!(D16);
    define_compatibility_class!(D24);
    define_compatibility_class!(D32);
    define_compatibility_class!(S8);
    define_compatibility_class!(D16S8);
    define_compatibility_class!(D24S8);
    define_compatibility_class!(D32S8);
    define_compatibility_class!(PLANE3_8BIT_420);
    define_compatibility_class!(PLANE2_8BIT_420);
    define_compatibility_class!(PLANE3_8BIT_422);
    define_compatibility_class!(PLANE2_8BIT_422);
    define_compatibility_class!(PLANE3_8BIT_444);
    define_compatibility_class!(PLANE3_10BIT_420);
    define_compatibility_class!(PLANE2_10BIT_420);
    define_compatibility_class!(PLANE3_10BIT_422);
    define_compatibility_class!(PLANE2_10BIT_422);
    define_compatibility_class!(PLANE3_10BIT_444);
    define_compatibility_class!(PLANE3_12BIT_420);
    define_compatibility_class!(PLANE2_12BIT_420);
    define_compatibility_class!(PLANE3_12BIT_422);
    define_compatibility_class!(PLANE2_12BIT_422);
    define_compatibility_class!(PLANE3_12BIT_444);
    define_compatibility_class!(PLANE3_16BIT_420);
    define_compatibility_class!(PLANE2_16BIT_420);
    define_compatibility_class!(PLANE3_16BIT_422);
    define_compatibility_class!(PLANE2_16BIT_422);
    define_compatibility_class!(PLANE3_16BIT_444);
}

impl PartialEq for CompatibilityClass {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.name, other.name)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum ClearColorType {
    Float,
    Int32,
    Uint32,
}

impl ClearColorType {
    pub const fn make_zero_clear(&self) -> vk::ClearColorValue {
        match self {
            Self::Float => {
                vk::ClearColorValue {
                    float32: [0f32; 4]
                }
            }
            Self::Int32 => {
                vk::ClearColorValue {
                    int32: [0i32; 4]
                }
            }
            Self::Uint32 => {
                vk::ClearColorValue {
                    uint32: [0u32; 4]
                }
            }
        }
    }
}

#[derive(Copy, Clone, Eq)]
pub struct Format {
    format: vk::Format,
    compatibility_class: CompatibilityClass,
    clear_color_type: Option<ClearColorType>,
}

macro_rules! define_formats {
    ($($name:ident, $compatibility_class:expr, $channel_count:expr, $clear_color_type:expr);+) => {
        pub const fn format_for(format: vk::Format) -> &'static Format {
            match format {
                $(
                ash::vk::Format::$name => &Self::$name,
                )+
                _ => { panic!("Unknown format!") }
            }
        }

        $(pub const $name : Format = Format::new(ash::vk::Format::$name, $compatibility_class, $channel_count, $clear_color_type);)+
    }
}

impl Format {
    pub const fn new(format: vk::Format, compatibility_class: CompatibilityClass, _channel_count: u32, clear_color_type: Option<ClearColorType>) -> Self {
        Format { format, compatibility_class, clear_color_type }
    }

    pub const fn get_format(&self) -> vk::Format {
        self.format
    }

    pub const fn get_compatibility_class(&self) -> CompatibilityClass {
        self.compatibility_class
    }

    pub const fn get_clear_color_type(&self) -> Option<ClearColorType> {
        self.clear_color_type
    }

    pub fn is_compatible_with(&self, other: &Format) -> bool {
        self.compatibility_class == other.compatibility_class
    }

    define_formats!(
    R4G4_UNORM_PACK8, CompatibilityClass::BIT8, 2, Some(ClearColorType::Float);
    R4G4B4A4_UNORM_PACK16, CompatibilityClass::BIT16, 4, Some(ClearColorType::Float);
    B4G4R4A4_UNORM_PACK16, CompatibilityClass::BIT16, 4, Some(ClearColorType::Float);
    R5G6B5_UNORM_PACK16, CompatibilityClass::BIT16, 3, Some(ClearColorType::Float);
    B5G6R5_UNORM_PACK16, CompatibilityClass::BIT16, 3, Some(ClearColorType::Float);
    R5G5B5A1_UNORM_PACK16, CompatibilityClass::BIT16, 4, Some(ClearColorType::Float);
    B5G5R5A1_UNORM_PACK16, CompatibilityClass::BIT16, 4, Some(ClearColorType::Float);
    A1R5G5B5_UNORM_PACK16, CompatibilityClass::BIT16, 4, Some(ClearColorType::Float);
    R8_UNORM, CompatibilityClass::BIT8, 1, Some(ClearColorType::Float);
    R8_SNORM, CompatibilityClass::BIT8, 1, Some(ClearColorType::Float);
    R8_USCALED, CompatibilityClass::BIT8, 1, Some(ClearColorType::Float);
    R8_SSCALED, CompatibilityClass::BIT8, 1, Some(ClearColorType::Float);
    R8_UINT, CompatibilityClass::BIT8, 1, Some(ClearColorType::Uint32);
    R8_SINT, CompatibilityClass::BIT8, 1, Some(ClearColorType::Int32);
    R8_SRGB, CompatibilityClass::BIT8, 1, Some(ClearColorType::Float);
    R8G8_UNORM, CompatibilityClass::BIT16, 2, Some(ClearColorType::Float);
    R8G8_SNORM, CompatibilityClass::BIT16, 2, Some(ClearColorType::Float);
    R8G8_USCALED, CompatibilityClass::BIT16, 2, Some(ClearColorType::Float);
    R8G8_SSCALED, CompatibilityClass::BIT16, 2, Some(ClearColorType::Float);
    R8G8_UINT, CompatibilityClass::BIT16, 2, Some(ClearColorType::Uint32);
    R8G8_SINT, CompatibilityClass::BIT16, 2, Some(ClearColorType::Int32);
    R8G8_SRGB, CompatibilityClass::BIT16, 2, Some(ClearColorType::Float);
    R8G8B8_UNORM, CompatibilityClass::BIT24, 3, Some(ClearColorType::Float);
    R8G8B8_SNORM, CompatibilityClass::BIT24, 3, Some(ClearColorType::Float);
    R8G8B8_USCALED, CompatibilityClass::BIT24, 3, Some(ClearColorType::Float);
    R8G8B8_SSCALED, CompatibilityClass::BIT24, 3, Some(ClearColorType::Float);
    R8G8B8_UINT, CompatibilityClass::BIT24, 3, Some(ClearColorType::Uint32);
    R8G8B8_SINT, CompatibilityClass::BIT24, 3, Some(ClearColorType::Int32);
    R8G8B8_SRGB, CompatibilityClass::BIT24, 3, Some(ClearColorType::Float);
    B8G8R8_UNORM, CompatibilityClass::BIT24, 3, Some(ClearColorType::Float);
    B8G8R8_SNORM, CompatibilityClass::BIT24, 3, Some(ClearColorType::Float);
    B8G8R8_USCALED, CompatibilityClass::BIT24, 3, Some(ClearColorType::Float);
    B8G8R8_SSCALED, CompatibilityClass::BIT24, 3, Some(ClearColorType::Float);
    B8G8R8_UINT, CompatibilityClass::BIT24, 3, Some(ClearColorType::Uint32);
    B8G8R8_SINT, CompatibilityClass::BIT24, 3, Some(ClearColorType::Int32);
    B8G8R8_SRGB, CompatibilityClass::BIT24, 3, Some(ClearColorType::Float);
    R8G8B8A8_UNORM, CompatibilityClass::BIT32, 4, Some(ClearColorType::Float);
    R8G8B8A8_SNORM, CompatibilityClass::BIT32, 4, Some(ClearColorType::Float);
    R8G8B8A8_USCALED, CompatibilityClass::BIT32, 4, Some(ClearColorType::Float);
    R8G8B8A8_SSCALED, CompatibilityClass::BIT32, 4, Some(ClearColorType::Float);
    R8G8B8A8_UINT, CompatibilityClass::BIT32, 4, Some(ClearColorType::Uint32);
    R8G8B8A8_SINT, CompatibilityClass::BIT32, 4, Some(ClearColorType::Int32);
    R8G8B8A8_SRGB, CompatibilityClass::BIT32, 4, Some(ClearColorType::Float);
    B8G8R8A8_UNORM, CompatibilityClass::BIT32, 4, Some(ClearColorType::Float);
    B8G8R8A8_SNORM, CompatibilityClass::BIT32, 4, Some(ClearColorType::Float);
    B8G8R8A8_USCALED, CompatibilityClass::BIT32, 4, Some(ClearColorType::Float);
    B8G8R8A8_SSCALED, CompatibilityClass::BIT32, 4, Some(ClearColorType::Float);
    B8G8R8A8_UINT, CompatibilityClass::BIT32, 4, Some(ClearColorType::Uint32);
    B8G8R8A8_SINT, CompatibilityClass::BIT32, 4, Some(ClearColorType::Int32);
    B8G8R8A8_SRGB, CompatibilityClass::BIT32, 4, Some(ClearColorType::Float);
    A8B8G8R8_UNORM_PACK32, CompatibilityClass::BIT32, 4, Some(ClearColorType::Float);
    A8B8G8R8_SNORM_PACK32, CompatibilityClass::BIT32, 4, Some(ClearColorType::Float);
    A8B8G8R8_USCALED_PACK32, CompatibilityClass::BIT32, 4, Some(ClearColorType::Float);
    A8B8G8R8_SSCALED_PACK32, CompatibilityClass::BIT32, 4, Some(ClearColorType::Float);
    A8B8G8R8_UINT_PACK32, CompatibilityClass::BIT32, 4, Some(ClearColorType::Uint32);
    A8B8G8R8_SINT_PACK32, CompatibilityClass::BIT32, 4, Some(ClearColorType::Int32);
    A8B8G8R8_SRGB_PACK32, CompatibilityClass::BIT32, 4, Some(ClearColorType::Float);
    A2R10G10B10_UNORM_PACK32, CompatibilityClass::BIT32, 4, Some(ClearColorType::Float);
    A2R10G10B10_SNORM_PACK32, CompatibilityClass::BIT32, 4, Some(ClearColorType::Float);
    A2R10G10B10_USCALED_PACK32, CompatibilityClass::BIT32, 4, Some(ClearColorType::Float);
    A2R10G10B10_SSCALED_PACK32, CompatibilityClass::BIT32, 4, Some(ClearColorType::Float);
    A2R10G10B10_UINT_PACK32, CompatibilityClass::BIT32, 4, Some(ClearColorType::Uint32);
    A2R10G10B10_SINT_PACK32, CompatibilityClass::BIT32, 4, Some(ClearColorType::Int32);
    A2B10G10R10_UNORM_PACK32, CompatibilityClass::BIT32, 4, Some(ClearColorType::Float);
    A2B10G10R10_SNORM_PACK32, CompatibilityClass::BIT32, 4, Some(ClearColorType::Float);
    A2B10G10R10_USCALED_PACK32, CompatibilityClass::BIT32, 4, Some(ClearColorType::Float);
    A2B10G10R10_SSCALED_PACK32, CompatibilityClass::BIT32, 4, Some(ClearColorType::Float);
    A2B10G10R10_UINT_PACK32, CompatibilityClass::BIT32, 4, Some(ClearColorType::Uint32);
    A2B10G10R10_SINT_PACK32, CompatibilityClass::BIT32, 4, Some(ClearColorType::Int32);
    R16_UNORM, CompatibilityClass::BIT16, 1, Some(ClearColorType::Float);
    R16_SNORM, CompatibilityClass::BIT16, 1, Some(ClearColorType::Float);
    R16_USCALED, CompatibilityClass::BIT16, 1, Some(ClearColorType::Float);
    R16_SSCALED, CompatibilityClass::BIT16, 1, Some(ClearColorType::Float);
    R16_UINT, CompatibilityClass::BIT16, 1, Some(ClearColorType::Uint32);
    R16_SINT, CompatibilityClass::BIT16, 1, Some(ClearColorType::Int32);
    R16_SFLOAT, CompatibilityClass::BIT16, 1, Some(ClearColorType::Float);
    R16G16_UNORM, CompatibilityClass::BIT32, 2, Some(ClearColorType::Float);
    R16G16_SNORM, CompatibilityClass::BIT32, 2, Some(ClearColorType::Float);
    R16G16_USCALED, CompatibilityClass::BIT32, 2, Some(ClearColorType::Float);
    R16G16_SSCALED, CompatibilityClass::BIT32, 2, Some(ClearColorType::Float);
    R16G16_UINT, CompatibilityClass::BIT32, 2, Some(ClearColorType::Uint32);
    R16G16_SINT, CompatibilityClass::BIT32, 2, Some(ClearColorType::Int32);
    R16G16_SFLOAT, CompatibilityClass::BIT32, 2, Some(ClearColorType::Float);
    R16G16B16_UNORM, CompatibilityClass::BIT48, 3, Some(ClearColorType::Float);
    R16G16B16_SNORM, CompatibilityClass::BIT48, 3, Some(ClearColorType::Float);
    R16G16B16_USCALED, CompatibilityClass::BIT48, 3, Some(ClearColorType::Float);
    R16G16B16_SSCALED, CompatibilityClass::BIT48, 3, Some(ClearColorType::Float);
    R16G16B16_UINT, CompatibilityClass::BIT48, 3, Some(ClearColorType::Uint32);
    R16G16B16_SINT, CompatibilityClass::BIT48, 3, Some(ClearColorType::Int32);
    R16G16B16_SFLOAT, CompatibilityClass::BIT48, 3, Some(ClearColorType::Float);
    R16G16B16A16_UNORM, CompatibilityClass::BIT64, 4, Some(ClearColorType::Float);
    R16G16B16A16_SNORM, CompatibilityClass::BIT64, 4, Some(ClearColorType::Float);
    R16G16B16A16_USCALED, CompatibilityClass::BIT64, 4, Some(ClearColorType::Float);
    R16G16B16A16_SSCALED, CompatibilityClass::BIT64, 4, Some(ClearColorType::Float);
    R16G16B16A16_UINT, CompatibilityClass::BIT64, 4, Some(ClearColorType::Uint32);
    R16G16B16A16_SINT, CompatibilityClass::BIT64, 4, Some(ClearColorType::Int32);
    R16G16B16A16_SFLOAT, CompatibilityClass::BIT64, 4, Some(ClearColorType::Float);
    R32_UINT, CompatibilityClass::BIT32, 1, Some(ClearColorType::Uint32);
    R32_SINT, CompatibilityClass::BIT32, 1, Some(ClearColorType::Int32);
    R32_SFLOAT, CompatibilityClass::BIT32, 1, Some(ClearColorType::Float);
    R32G32_UINT, CompatibilityClass::BIT64, 2, Some(ClearColorType::Uint32);
    R32G32_SINT, CompatibilityClass::BIT64, 2, Some(ClearColorType::Int32);
    R32G32_SFLOAT, CompatibilityClass::BIT64, 2, Some(ClearColorType::Float);
    R32G32B32_UINT, CompatibilityClass::BIT96, 3, Some(ClearColorType::Uint32);
    R32G32B32_SINT, CompatibilityClass::BIT96, 3, Some(ClearColorType::Int32);
    R32G32B32_SFLOAT, CompatibilityClass::BIT96, 3, Some(ClearColorType::Float);
    R32G32B32A32_UINT, CompatibilityClass::BIT128, 4, Some(ClearColorType::Uint32);
    R32G32B32A32_SINT, CompatibilityClass::BIT128, 4, Some(ClearColorType::Int32);
    R32G32B32A32_SFLOAT, CompatibilityClass::BIT128, 4, Some(ClearColorType::Float);
    R64_UINT, CompatibilityClass::BIT64, 1, Some(ClearColorType::Uint32);
    R64_SINT, CompatibilityClass::BIT64, 1, Some(ClearColorType::Int32);
    R64_SFLOAT, CompatibilityClass::BIT64, 1, Some(ClearColorType::Float);
    R64G64_UINT, CompatibilityClass::BIT128, 2, Some(ClearColorType::Uint32);
    R64G64_SINT, CompatibilityClass::BIT128, 2, Some(ClearColorType::Int32);
    R64G64_SFLOAT, CompatibilityClass::BIT128, 2, Some(ClearColorType::Float);
    R64G64B64_UINT, CompatibilityClass::BIT192, 3, Some(ClearColorType::Uint32);
    R64G64B64_SINT, CompatibilityClass::BIT192, 3, Some(ClearColorType::Int32);
    R64G64B64_SFLOAT, CompatibilityClass::BIT192, 3, Some(ClearColorType::Float);
    R64G64B64A64_UINT, CompatibilityClass::BIT256, 4, Some(ClearColorType::Uint32);
    R64G64B64A64_SINT, CompatibilityClass::BIT256, 4, Some(ClearColorType::Int32);
    R64G64B64A64_SFLOAT, CompatibilityClass::BIT256, 4, Some(ClearColorType::Float);
    B10G11R11_UFLOAT_PACK32, CompatibilityClass::BIT32, 3, Some(ClearColorType::Float);
    E5B9G9R9_UFLOAT_PACK32, CompatibilityClass::BIT32, 3, Some(ClearColorType::Float);
    D16_UNORM, CompatibilityClass::D16, 1, None;
    X8_D24_UNORM_PACK32, CompatibilityClass::D24, 1, None;
    D32_SFLOAT, CompatibilityClass::D32, 1, None;
    S8_UINT, CompatibilityClass::S8, 1, None;
    D16_UNORM_S8_UINT, CompatibilityClass::D16S8, 2, None;
    D24_UNORM_S8_UINT, CompatibilityClass::D24S8, 2, None;
    D32_SFLOAT_S8_UINT, CompatibilityClass::D32S8, 2, None;
    BC1_RGB_UNORM_BLOCK, CompatibilityClass::BC1_RGB, 3, Some(ClearColorType::Float);
    BC1_RGB_SRGB_BLOCK, CompatibilityClass::BC1_RGB, 3, Some(ClearColorType::Float);
    BC1_RGBA_UNORM_BLOCK, CompatibilityClass::BC1_RGBA, 4, Some(ClearColorType::Float);
    BC1_RGBA_SRGB_BLOCK, CompatibilityClass::BC1_RGBA, 4, Some(ClearColorType::Float);
    BC2_UNORM_BLOCK, CompatibilityClass::BC2, 4, Some(ClearColorType::Float);
    BC2_SRGB_BLOCK, CompatibilityClass::BC2, 4, Some(ClearColorType::Float);
    BC3_UNORM_BLOCK, CompatibilityClass::BC3, 4, Some(ClearColorType::Float);
    BC3_SRGB_BLOCK, CompatibilityClass::BC3, 4, Some(ClearColorType::Float);
    BC4_UNORM_BLOCK, CompatibilityClass::BC4, 1, Some(ClearColorType::Float);
    BC4_SNORM_BLOCK, CompatibilityClass::BC4, 1, Some(ClearColorType::Float);
    BC5_UNORM_BLOCK, CompatibilityClass::BC5, 2, Some(ClearColorType::Float);
    BC5_SNORM_BLOCK, CompatibilityClass::BC5, 2, Some(ClearColorType::Float);
    BC6H_UFLOAT_BLOCK, CompatibilityClass::BC6H, 3, Some(ClearColorType::Float);
    BC6H_SFLOAT_BLOCK, CompatibilityClass::BC6H, 3, Some(ClearColorType::Float);
    BC7_UNORM_BLOCK, CompatibilityClass::BC7, 4, Some(ClearColorType::Float);
    BC7_SRGB_BLOCK, CompatibilityClass::BC7, 4, Some(ClearColorType::Float);
    ETC2_R8G8B8_UNORM_BLOCK, CompatibilityClass::ETC2_RGB, 3, Some(ClearColorType::Float);
    ETC2_R8G8B8_SRGB_BLOCK, CompatibilityClass::ETC2_RGB, 3, Some(ClearColorType::Float);
    ETC2_R8G8B8A1_UNORM_BLOCK, CompatibilityClass::ETC2_RGBA, 4, Some(ClearColorType::Float);
    ETC2_R8G8B8A1_SRGB_BLOCK, CompatibilityClass::ETC2_RGBA, 4, Some(ClearColorType::Float);
    ETC2_R8G8B8A8_UNORM_BLOCK, CompatibilityClass::ETC2_EAC_RGBA, 4, Some(ClearColorType::Float);
    ETC2_R8G8B8A8_SRGB_BLOCK, CompatibilityClass::ETC2_EAC_RGBA, 4, Some(ClearColorType::Float);
    EAC_R11_UNORM_BLOCK, CompatibilityClass::EAC_R, 1, Some(ClearColorType::Float);
    EAC_R11_SNORM_BLOCK, CompatibilityClass::EAC_R, 1, Some(ClearColorType::Float);
    EAC_R11G11_UNORM_BLOCK, CompatibilityClass::EAC_RG, 2, Some(ClearColorType::Float);
    EAC_R11G11_SNORM_BLOCK, CompatibilityClass::EAC_RG, 2, Some(ClearColorType::Float);
    ASTC_4X4_UNORM_BLOCK, CompatibilityClass::ASTC_4X4, 4, Some(ClearColorType::Float);
    ASTC_4X4_SRGB_BLOCK, CompatibilityClass::ASTC_4X4, 4, Some(ClearColorType::Float);
    ASTC_5X4_UNORM_BLOCK, CompatibilityClass::ASTC_5X4, 4, Some(ClearColorType::Float);
    ASTC_5X4_SRGB_BLOCK, CompatibilityClass::ASTC_5X4, 4, Some(ClearColorType::Float);
    ASTC_5X5_UNORM_BLOCK, CompatibilityClass::ASTC_5X5, 4, Some(ClearColorType::Float);
    ASTC_5X5_SRGB_BLOCK, CompatibilityClass::ASTC_5X5, 4, Some(ClearColorType::Float);
    ASTC_6X5_UNORM_BLOCK, CompatibilityClass::ASTC_6X5, 4, Some(ClearColorType::Float);
    ASTC_6X5_SRGB_BLOCK, CompatibilityClass::ASTC_6X5, 4, Some(ClearColorType::Float);
    ASTC_6X6_UNORM_BLOCK, CompatibilityClass::ASTC_6X6, 4, Some(ClearColorType::Float);
    ASTC_6X6_SRGB_BLOCK, CompatibilityClass::ASTC_6X6, 4, Some(ClearColorType::Float);
    ASTC_8X5_UNORM_BLOCK, CompatibilityClass::ASTC_8X5, 4, Some(ClearColorType::Float);
    ASTC_8X5_SRGB_BLOCK, CompatibilityClass::ASTC_8X5, 4, Some(ClearColorType::Float);
    ASTC_8X6_UNORM_BLOCK, CompatibilityClass::ASTC_8X6, 4, Some(ClearColorType::Float);
    ASTC_8X6_SRGB_BLOCK, CompatibilityClass::ASTC_8X6, 4, Some(ClearColorType::Float);
    ASTC_8X8_UNORM_BLOCK, CompatibilityClass::ASTC_8X8, 4, Some(ClearColorType::Float);
    ASTC_8X8_SRGB_BLOCK, CompatibilityClass::ASTC_8X8, 4, Some(ClearColorType::Float);
    ASTC_10X5_UNORM_BLOCK, CompatibilityClass::ASTC_10X5, 4, Some(ClearColorType::Float);
    ASTC_10X5_SRGB_BLOCK, CompatibilityClass::ASTC_10X5, 4, Some(ClearColorType::Float);
    ASTC_10X6_UNORM_BLOCK, CompatibilityClass::ASTC_10X6, 4, Some(ClearColorType::Float);
    ASTC_10X6_SRGB_BLOCK, CompatibilityClass::ASTC_10X6, 4, Some(ClearColorType::Float);
    ASTC_10X8_UNORM_BLOCK, CompatibilityClass::ASTC_10X8, 4, Some(ClearColorType::Float);
    ASTC_10X8_SRGB_BLOCK, CompatibilityClass::ASTC_10X8, 4, Some(ClearColorType::Float);
    ASTC_10X10_UNORM_BLOCK, CompatibilityClass::ASTC_10X10, 4, Some(ClearColorType::Float);
    ASTC_10X10_SRGB_BLOCK, CompatibilityClass::ASTC_10X10, 4, Some(ClearColorType::Float);
    ASTC_12X10_UNORM_BLOCK, CompatibilityClass::ASTC_12X10, 4, Some(ClearColorType::Float);
    ASTC_12X10_SRGB_BLOCK, CompatibilityClass::ASTC_12X10, 4, Some(ClearColorType::Float);
    ASTC_12X12_UNORM_BLOCK, CompatibilityClass::ASTC_12X12, 4, Some(ClearColorType::Float);
    ASTC_12X12_SRGB_BLOCK, CompatibilityClass::ASTC_12X12, 4, Some(ClearColorType::Float);
    G8B8G8R8_422_UNORM, CompatibilityClass::BIT32_G8B8G8R8, 4, Some(ClearColorType::Float);
    B8G8R8G8_422_UNORM, CompatibilityClass::BIT32_B8G8R8G8, 4, Some(ClearColorType::Float);
    G8_B8_R8_3PLANE_420_UNORM, CompatibilityClass::PLANE3_8BIT_420, 3, Some(ClearColorType::Float);
    G8_B8R8_2PLANE_420_UNORM, CompatibilityClass::PLANE2_8BIT_420, 3, Some(ClearColorType::Float);
    G8_B8_R8_3PLANE_422_UNORM, CompatibilityClass::PLANE3_8BIT_422, 3, Some(ClearColorType::Float);
    G8_B8R8_2PLANE_422_UNORM, CompatibilityClass::PLANE2_8BIT_422, 3, Some(ClearColorType::Float);
    G8_B8_R8_3PLANE_444_UNORM, CompatibilityClass::PLANE3_8BIT_444, 3, Some(ClearColorType::Float);
    R10X6_UNORM_PACK16, CompatibilityClass::BIT16, 1, Some(ClearColorType::Float);
    R10X6G10X6_UNORM_2PACK16, CompatibilityClass::BIT32, 2, Some(ClearColorType::Float);
    R10X6G10X6B10X6A10X6_UNORM_4PACK16, CompatibilityClass::BIT64_R10G10B10A10, 4, Some(ClearColorType::Float);
    G10X6B10X6G10X6R10X6_422_UNORM_4PACK16, CompatibilityClass::BIT64_G10B10G10R10, 4, Some(ClearColorType::Float);
    B10X6G10X6R10X6G10X6_422_UNORM_4PACK16, CompatibilityClass::BIT64_B10G10R10G10, 4, Some(ClearColorType::Float);
    G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16, CompatibilityClass::PLANE3_10BIT_420, 3, Some(ClearColorType::Float);
    G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16, CompatibilityClass::PLANE2_10BIT_420, 3, Some(ClearColorType::Float);
    G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16, CompatibilityClass::PLANE3_10BIT_422, 3, Some(ClearColorType::Float);
    G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16, CompatibilityClass::PLANE2_10BIT_422, 3, Some(ClearColorType::Float);
    G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16, CompatibilityClass::PLANE3_10BIT_444, 3, Some(ClearColorType::Float);
    R12X4_UNORM_PACK16, CompatibilityClass::BIT16, 1, Some(ClearColorType::Float);
    R12X4G12X4_UNORM_2PACK16, CompatibilityClass::BIT32, 2, Some(ClearColorType::Float);
    R12X4G12X4B12X4A12X4_UNORM_4PACK16, CompatibilityClass::BIT64_R12G12B12A12, 4, Some(ClearColorType::Float);
    G12X4B12X4G12X4R12X4_422_UNORM_4PACK16, CompatibilityClass::BIT64_G12B12G12R12, 4, Some(ClearColorType::Float);
    B12X4G12X4R12X4G12X4_422_UNORM_4PACK16, CompatibilityClass::BIT64_B12G12R12G12, 4, Some(ClearColorType::Float);
    G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16, CompatibilityClass::PLANE3_12BIT_420, 3, Some(ClearColorType::Float);
    G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16, CompatibilityClass::PLANE2_12BIT_420, 3, Some(ClearColorType::Float);
    G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16, CompatibilityClass::PLANE3_12BIT_422, 3, Some(ClearColorType::Float);
    G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16, CompatibilityClass::PLANE2_12BIT_422, 3, Some(ClearColorType::Float);
    G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16, CompatibilityClass::PLANE3_12BIT_444, 3, Some(ClearColorType::Float);
    G16B16G16R16_422_UNORM, CompatibilityClass::BIT64_G16B16G16R16, 3, Some(ClearColorType::Float);
    B16G16R16G16_422_UNORM, CompatibilityClass::BIT64_B16G16R16G16, 3, Some(ClearColorType::Float);
    G16_B16_R16_3PLANE_420_UNORM, CompatibilityClass::PLANE3_16BIT_420, 3, Some(ClearColorType::Float);
    G16_B16R16_2PLANE_420_UNORM, CompatibilityClass::PLANE2_16BIT_420, 3, Some(ClearColorType::Float);
    G16_B16_R16_3PLANE_422_UNORM, CompatibilityClass::PLANE3_16BIT_422, 3, Some(ClearColorType::Float);
    G16_B16R16_2PLANE_422_UNORM, CompatibilityClass::PLANE2_16BIT_422, 3, Some(ClearColorType::Float);
    G16_B16_R16_3PLANE_444_UNORM, CompatibilityClass::PLANE3_16BIT_444, 3, Some(ClearColorType::Float)
    );
}

impl PartialEq for Format {
    fn eq(&self, other: &Self) -> bool {
        self.format == other.format
    }
}

impl PartialOrd for Format {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.format.partial_cmp(&other.format)
    }
}

impl Ord for Format {
    fn cmp(&self, other: &Self) -> Ordering {
        self.format.cmp(&other.format)
    }
}

impl Hash for Format {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.format.hash(state)
    }
}

impl Debug for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Format").field(&self.format).finish()
    }
}

impl Into<vk::Format> for &Format {
    fn into(self) -> vk::Format {
        self.format
    }
}