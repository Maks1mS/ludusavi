//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::Metal::*;
use crate::MetalKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetalKit_MTKView")]
    pub struct MTKView;

    #[cfg(feature = "MetalKit_MTKView")]
    unsafe impl ClassType for MTKView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MetalKit_MTKView")]
unsafe impl NSAccessibility for MTKView {}

#[cfg(feature = "MetalKit_MTKView")]
unsafe impl NSAccessibilityElementProtocol for MTKView {}

#[cfg(feature = "MetalKit_MTKView")]
unsafe impl NSAnimatablePropertyContainer for MTKView {}

#[cfg(feature = "MetalKit_MTKView")]
unsafe impl NSAppearanceCustomization for MTKView {}

#[cfg(feature = "MetalKit_MTKView")]
unsafe impl NSCoding for MTKView {}

#[cfg(feature = "MetalKit_MTKView")]
unsafe impl NSDraggingDestination for MTKView {}

#[cfg(feature = "MetalKit_MTKView")]
unsafe impl NSObjectProtocol for MTKView {}

#[cfg(feature = "MetalKit_MTKView")]
unsafe impl NSUserInterfaceItemIdentification for MTKView {}

extern_methods!(
    #[cfg(feature = "MetalKit_MTKView")]
    unsafe impl MTKView {
        #[method_id(@__retain_semantics Init initWithFrame:device:)]
        pub unsafe fn initWithFrame_device(
            this: Option<Allocated<Self>>,
            frame_rect: CGRect,
            device: Option<&ProtocolObject<dyn MTLDevice>>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn MTKViewDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn MTKViewDelegate>>);

        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Option<Id<ProtocolObject<dyn MTLDevice>>>;

        #[method(setDevice:)]
        pub unsafe fn setDevice(&self, device: Option<&ProtocolObject<dyn MTLDevice>>);

        #[method(framebufferOnly)]
        pub unsafe fn framebufferOnly(&self) -> bool;

        #[method(setFramebufferOnly:)]
        pub unsafe fn setFramebufferOnly(&self, framebuffer_only: bool);

        #[method(depthStencilAttachmentTextureUsage)]
        pub unsafe fn depthStencilAttachmentTextureUsage(&self) -> MTLTextureUsage;

        #[method(setDepthStencilAttachmentTextureUsage:)]
        pub unsafe fn setDepthStencilAttachmentTextureUsage(
            &self,
            depth_stencil_attachment_texture_usage: MTLTextureUsage,
        );

        #[method(multisampleColorAttachmentTextureUsage)]
        pub unsafe fn multisampleColorAttachmentTextureUsage(&self) -> MTLTextureUsage;

        #[method(setMultisampleColorAttachmentTextureUsage:)]
        pub unsafe fn setMultisampleColorAttachmentTextureUsage(
            &self,
            multisample_color_attachment_texture_usage: MTLTextureUsage,
        );

        #[method(presentsWithTransaction)]
        pub unsafe fn presentsWithTransaction(&self) -> bool;

        #[method(setPresentsWithTransaction:)]
        pub unsafe fn setPresentsWithTransaction(&self, presents_with_transaction: bool);

        #[method(colorPixelFormat)]
        pub unsafe fn colorPixelFormat(&self) -> MTLPixelFormat;

        #[method(setColorPixelFormat:)]
        pub unsafe fn setColorPixelFormat(&self, color_pixel_format: MTLPixelFormat);

        #[method(depthStencilPixelFormat)]
        pub unsafe fn depthStencilPixelFormat(&self) -> MTLPixelFormat;

        #[method(setDepthStencilPixelFormat:)]
        pub unsafe fn setDepthStencilPixelFormat(&self, depth_stencil_pixel_format: MTLPixelFormat);

        #[method(depthStencilStorageMode)]
        pub unsafe fn depthStencilStorageMode(&self) -> MTLStorageMode;

        #[method(setDepthStencilStorageMode:)]
        pub unsafe fn setDepthStencilStorageMode(&self, depth_stencil_storage_mode: MTLStorageMode);

        #[method(sampleCount)]
        pub unsafe fn sampleCount(&self) -> NSUInteger;

        #[method(setSampleCount:)]
        pub unsafe fn setSampleCount(&self, sample_count: NSUInteger);

        #[method(clearColor)]
        pub unsafe fn clearColor(&self) -> MTLClearColor;

        #[method(setClearColor:)]
        pub unsafe fn setClearColor(&self, clear_color: MTLClearColor);

        #[method(clearDepth)]
        pub unsafe fn clearDepth(&self) -> c_double;

        #[method(setClearDepth:)]
        pub unsafe fn setClearDepth(&self, clear_depth: c_double);

        #[method(clearStencil)]
        pub unsafe fn clearStencil(&self) -> u32;

        #[method(setClearStencil:)]
        pub unsafe fn setClearStencil(&self, clear_stencil: u32);

        #[method_id(@__retain_semantics Other depthStencilTexture)]
        pub unsafe fn depthStencilTexture(&self) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;

        #[method_id(@__retain_semantics Other multisampleColorTexture)]
        pub unsafe fn multisampleColorTexture(&self) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;

        #[method(releaseDrawables)]
        pub unsafe fn releaseDrawables(&self);

        #[cfg(feature = "Metal_MTLRenderPassDescriptor")]
        #[method_id(@__retain_semantics Other currentRenderPassDescriptor)]
        pub unsafe fn currentRenderPassDescriptor(&self) -> Option<Id<MTLRenderPassDescriptor>>;

        #[method(preferredFramesPerSecond)]
        pub unsafe fn preferredFramesPerSecond(&self) -> NSInteger;

        #[method(setPreferredFramesPerSecond:)]
        pub unsafe fn setPreferredFramesPerSecond(&self, preferred_frames_per_second: NSInteger);

        #[method(enableSetNeedsDisplay)]
        pub unsafe fn enableSetNeedsDisplay(&self) -> bool;

        #[method(setEnableSetNeedsDisplay:)]
        pub unsafe fn setEnableSetNeedsDisplay(&self, enable_set_needs_display: bool);

        #[method(autoResizeDrawable)]
        pub unsafe fn autoResizeDrawable(&self) -> bool;

        #[method(setAutoResizeDrawable:)]
        pub unsafe fn setAutoResizeDrawable(&self, auto_resize_drawable: bool);

        #[method(drawableSize)]
        pub unsafe fn drawableSize(&self) -> CGSize;

        #[method(setDrawableSize:)]
        pub unsafe fn setDrawableSize(&self, drawable_size: CGSize);

        #[method(preferredDrawableSize)]
        pub unsafe fn preferredDrawableSize(&self) -> CGSize;

        #[method_id(@__retain_semantics Other preferredDevice)]
        pub unsafe fn preferredDevice(&self) -> Option<Id<ProtocolObject<dyn MTLDevice>>>;

        #[method(isPaused)]
        pub unsafe fn isPaused(&self) -> bool;

        #[method(setPaused:)]
        pub unsafe fn setPaused(&self, paused: bool);

        #[method(draw)]
        pub unsafe fn draw(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "MetalKit_MTKView")]
    unsafe impl MTKView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "MetalKit_MTKView")]
    unsafe impl MTKView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MetalKit_MTKView")]
    unsafe impl MTKView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait MTKViewDelegate: NSObjectProtocol {
        #[cfg(feature = "MetalKit_MTKView")]
        #[method(mtkView:drawableSizeWillChange:)]
        unsafe fn mtkView_drawableSizeWillChange(&self, view: &MTKView, size: CGSize);

        #[cfg(feature = "MetalKit_MTKView")]
        #[method(drawInMTKView:)]
        unsafe fn drawInMTKView(&self, view: &MTKView);
    }

    unsafe impl ProtocolType for dyn MTKViewDelegate {}
);