// DO NOT EDIT THIS FILE!
//
// This module part of a subset of web-sys that is used by wgpu's webgpu backend.
//
// If you want to improve the generated code, please submit a PR to the https://github.com/rustwasm/wasm-bindgen repository.
//
// This file was generated by the `cargo xtask vendor-web-sys --version 0.2.90` command.
#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[doc = "The `GpuTextureFormat` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuTextureFormat`*"]
#[doc = ""]
#[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
#[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuTextureFormat {
    R8unorm = "r8unorm",
    R8snorm = "r8snorm",
    R8uint = "r8uint",
    R8sint = "r8sint",
    R16uint = "r16uint",
    R16sint = "r16sint",
    R16float = "r16float",
    Rg8unorm = "rg8unorm",
    Rg8snorm = "rg8snorm",
    Rg8uint = "rg8uint",
    Rg8sint = "rg8sint",
    R32uint = "r32uint",
    R32sint = "r32sint",
    R32float = "r32float",
    Rg16uint = "rg16uint",
    Rg16sint = "rg16sint",
    Rg16float = "rg16float",
    Rgba8unorm = "rgba8unorm",
    Rgba8unormSrgb = "rgba8unorm-srgb",
    Rgba8snorm = "rgba8snorm",
    Rgba8uint = "rgba8uint",
    Rgba8sint = "rgba8sint",
    Bgra8unorm = "bgra8unorm",
    Bgra8unormSrgb = "bgra8unorm-srgb",
    Rgb9e5ufloat = "rgb9e5ufloat",
    Rgb10a2unorm = "rgb10a2unorm",
    Rg11b10ufloat = "rg11b10ufloat",
    Rg32uint = "rg32uint",
    Rg32sint = "rg32sint",
    Rg32float = "rg32float",
    Rgba16uint = "rgba16uint",
    Rgba16sint = "rgba16sint",
    Rgba16float = "rgba16float",
    Rgba32uint = "rgba32uint",
    Rgba32sint = "rgba32sint",
    Rgba32float = "rgba32float",
    Stencil8 = "stencil8",
    Depth16unorm = "depth16unorm",
    Depth24plus = "depth24plus",
    Depth24plusStencil8 = "depth24plus-stencil8",
    Depth32float = "depth32float",
    Depth32floatStencil8 = "depth32float-stencil8",
    Bc1RgbaUnorm = "bc1-rgba-unorm",
    Bc1RgbaUnormSrgb = "bc1-rgba-unorm-srgb",
    Bc2RgbaUnorm = "bc2-rgba-unorm",
    Bc2RgbaUnormSrgb = "bc2-rgba-unorm-srgb",
    Bc3RgbaUnorm = "bc3-rgba-unorm",
    Bc3RgbaUnormSrgb = "bc3-rgba-unorm-srgb",
    Bc4RUnorm = "bc4-r-unorm",
    Bc4RSnorm = "bc4-r-snorm",
    Bc5RgUnorm = "bc5-rg-unorm",
    Bc5RgSnorm = "bc5-rg-snorm",
    Bc6hRgbUfloat = "bc6h-rgb-ufloat",
    Bc6hRgbFloat = "bc6h-rgb-float",
    Bc7RgbaUnorm = "bc7-rgba-unorm",
    Bc7RgbaUnormSrgb = "bc7-rgba-unorm-srgb",
    Etc2Rgb8unorm = "etc2-rgb8unorm",
    Etc2Rgb8unormSrgb = "etc2-rgb8unorm-srgb",
    Etc2Rgb8a1unorm = "etc2-rgb8a1unorm",
    Etc2Rgb8a1unormSrgb = "etc2-rgb8a1unorm-srgb",
    Etc2Rgba8unorm = "etc2-rgba8unorm",
    Etc2Rgba8unormSrgb = "etc2-rgba8unorm-srgb",
    EacR11unorm = "eac-r11unorm",
    EacR11snorm = "eac-r11snorm",
    EacRg11unorm = "eac-rg11unorm",
    EacRg11snorm = "eac-rg11snorm",
    Astc4x4Unorm = "astc-4x4-unorm",
    Astc4x4UnormSrgb = "astc-4x4-unorm-srgb",
    Astc5x4Unorm = "astc-5x4-unorm",
    Astc5x4UnormSrgb = "astc-5x4-unorm-srgb",
    Astc5x5Unorm = "astc-5x5-unorm",
    Astc5x5UnormSrgb = "astc-5x5-unorm-srgb",
    Astc6x5Unorm = "astc-6x5-unorm",
    Astc6x5UnormSrgb = "astc-6x5-unorm-srgb",
    Astc6x6Unorm = "astc-6x6-unorm",
    Astc6x6UnormSrgb = "astc-6x6-unorm-srgb",
    Astc8x5Unorm = "astc-8x5-unorm",
    Astc8x5UnormSrgb = "astc-8x5-unorm-srgb",
    Astc8x6Unorm = "astc-8x6-unorm",
    Astc8x6UnormSrgb = "astc-8x6-unorm-srgb",
    Astc8x8Unorm = "astc-8x8-unorm",
    Astc8x8UnormSrgb = "astc-8x8-unorm-srgb",
    Astc10x5Unorm = "astc-10x5-unorm",
    Astc10x5UnormSrgb = "astc-10x5-unorm-srgb",
    Astc10x6Unorm = "astc-10x6-unorm",
    Astc10x6UnormSrgb = "astc-10x6-unorm-srgb",
    Astc10x8Unorm = "astc-10x8-unorm",
    Astc10x8UnormSrgb = "astc-10x8-unorm-srgb",
    Astc10x10Unorm = "astc-10x10-unorm",
    Astc10x10UnormSrgb = "astc-10x10-unorm-srgb",
    Astc12x10Unorm = "astc-12x10-unorm",
    Astc12x10UnormSrgb = "astc-12x10-unorm-srgb",
    Astc12x12Unorm = "astc-12x12-unorm",
    Astc12x12UnormSrgb = "astc-12x12-unorm-srgb",
}
