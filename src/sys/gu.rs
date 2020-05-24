use core::ffi::c_void;
use core::ptr::null_mut;

use crate::sys::ge::{GeContext, GeListArgs, GeCallbackData, GeBreakParam, GeStack};
use crate::sys::types::{IMatrix4, FMatrix4, FVector3};

pub const PI: f32 = 3.141593;

/// Primitive types
#[repr(u32)]
pub enum Primitive {
    Points = 0,
    Lines = 1,
    LineStrip = 2,
    Triangles = 3,
    TriangleStrip = 4,
    TriangleFan = 5,
    Sprites = 6,
}

/// States
#[repr(u32)]
pub enum State {
    AlphaTest = 0,
    DepthTest = 1,
    ScissorTest = 2,
    StencilTest = 3,
    Blend = 4,
    CullFace = 5,
    Dither = 6,
    Fog = 7,
    ClipPlanes = 8,
    Texture2D = 9,
    Lighting = 10,
    Light0 = 11,
    Light1 = 12,
    Light2 = 13,
    Light3 = 14,
    LineSmooth = 15,
    PatchCullFace = 16,
    ColorTest = 17,
    ColorLogicOp = 18,
    FaceNormalReverse = 19,
    PatchFace = 20,
    Fragment2X = 21,
}

/// Matrix modes
#[repr(u32)]
pub enum MatrixMode {
    Projection = 0,
    View = 1,
    Model = 2,
    Texture = 3,
}

// Vertex Declarations Begin
const fn texture_shift(n: u32) -> u32 {
    n << 0
}

#[repr(u32)]
pub enum Texture {
    Texture8bit = texture_shift(1),
    Texture16bit = texture_shift(2),
    Texture32bitf = texture_shift(3),
}

const fn color_shift(n: u32) -> u32 {
    n << 2
}

#[repr(u32)]
pub enum Color {
    Color5650 = color_shift(4),
    Color5551 = color_shift(5),
    Color4444 = color_shift(6),
    Color8888 = color_shift(7),
}

const fn normal_shift(n: u32) -> u32 {
    n << 5
}

#[repr(u32)]
pub enum Normal {
    Normal8bit = normal_shift(1),
    Normal16bit = normal_shift(2),
    Normal32bitf = normal_shift(3),
}

const fn vertex_shift(n: u32) -> u32 {
    n << 7
}

#[repr(u32)]
pub enum Vertex {
    Vertex8bit = vertex_shift(1),
    Vertex16bit = vertex_shift(2),
    Vertex32bitf = vertex_shift(3),
}

const fn weight_shift(n: u32) -> u32 {
    n << 9
}

#[repr(u32)]
pub enum Weight {
    Weight8bit = weight_shift(1),
    Weight16bit = weight_shift(2),
    Weight32bitf = weight_shift(3),
}

const fn index_shift(n: u32) -> u32 {
    n << 11
}

#[repr(u32)]
pub enum Index {
    Index8bit = index_shift(1),
    Index16bit = index_shift(2),
}

const fn weights(n: u32) -> u32 {
    (((n) - 1) & 7) << 14
}

const fn vertices(n: u32) -> u32 {
    (((n) - 1) & 7) << 18
}

pub const WEIGHTS_BITS: u32 = weights(8);
pub const VERTICES_BITS: u32 = vertices(8);

const fn transform_shift(n: u32) -> u32 {
    n << 23
}

#[repr(u32)]
pub enum Transform {
    Transform3D = transform_shift(0),
    Transform2D = transform_shift(1),
}

// Vertex Declarations End

/// Pixel Formats
#[repr(u32)]
pub enum PixelFormat {
    Psm5650 = 0,
    Psm5551 = 1,
    Psm4444 = 2,
    Psm8888 = 3,
    PsmT4 = 4,
    PsmT8 = 5,
    PsmT16 = 6,
    PsmT32 = 7,
    PsmDxt1 = 8,
    PsmDxt3 = 9,
    PsmDxt5 = 10,
}

/// Spline Mode
#[repr(u32)]
pub enum SplineMode {
    FillFill = 0,
    OpenFill = 1,
    FillOpen = 2,
    OpenOpen = 3,
}

/// Shading Model
#[repr(u32)]
pub enum ShadingModel {
    Flat = 0,
    Smooth = 1,
}

/// Logical operation
#[repr(u32)]
pub enum LogicalOperation {
    Clear = 0,
    And = 1,
    AndReverse = 2,
    Copy_ = 3,
    AndInverted = 4,
    Noop = 5,
    Xor = 6,
    Or = 7,
    Nor = 8,
    Equiv = 9,
    Inverted = 10,
    OrReverse = 11,
    CopyInverted = 12,
    OrInverted = 13,
    Nand = 14,
    Set = 15,
}

/// Texture Filter
#[repr(u32)]
pub enum TextureFilter {
    Nearest = 0,
    Linear = 1,
    NearestMipmapNearest = 4,
    LinearMipmapNearest = 5,
    NearestMipmapLinear = 6,
    LinearMipmapLinear = 7,
}

/// Texture Map Mode
#[repr(u32)]
pub enum TextureMapMode {
    TextureCoords = 0,
    TextureMatrix = 1,
    EnvironmentMap = 2,
}

/// Texture Level Mode
#[repr(u32)]
pub enum TextureLevelMode {
    TextureAuto = 0,
    TextureConst = 1,
    TextureSlope = 2,
}

/// Texture Projection Map Mode
#[repr(u32)]
pub enum TextureProjectionMapMode {
    Position = 0,
    Uv = 1,
    NormalizedNormal = 2,
    Normal = 3,
}

/// Wrap Mode
#[repr(u32)]
pub enum WrapMode {
    Repeat = 0,
    Clamp = 1,
}

/// Front Face Direction
#[repr(u32)]
pub enum FrontFaceDirection {
    CW = 0,
    CCW = 1,
}

/// Test Function
#[repr(u32)]
pub enum TestFunction {
    Never = 0,
    Always = 1,
    Equal = 2,
    Notequal = 3,
    Less = 4,
    Lequal = 5,
    Greater = 6,
    Gequal = 7,
}

/// Clear Buffer Mask
#[repr(u32)]
pub enum ClearBuffer {
    ColorBufferBit = 1,
    StencilBufferBit = 2,
    DepthBufferBit = 4,
    FastClearBit = 16,
}

/// Texture Effect
#[repr(u32)]
pub enum TextureEffect {
    TfxModulate = 0,
    TfxDecal = 1,
    TfxBlend = 2,
    TfxReplace = 3,
    TfxAdd = 4,
}

/// Texture Color Component
#[repr(u32)]
pub enum TextureColorComponent {
    TccRgb = 0,
    TccRgba = 1,
}

/// Blending Op
#[repr(u32)]
pub enum BlendingOperation {
    Add = 0,
    Subtract = 1,
    ReverseSubtract = 2,
    Min = 3,
    Max = 4,
    Abs = 5,
}

/// Blending Factor Source
#[repr(u32)]
pub enum BlendingFactorSrc {
    SrcColor = 0,
    OneMinusSrcColor = 1,
    SrcAlpha = 2,
    OneMinusSrcAlpha = 3,
}

/// Blending Factor Destination
pub enum BlendingFactorDst {
    DstColor = 0,
    OneMinusDstColor = 1,
    DstAlpha = 4,
    OneMinusDstAlpha = 5,
    Fix = 10,
}

/// Stencil Operations
#[repr(u32)]
pub enum StencilOperation {
    Keep = 0,
    Zero = 1,
    Replace = 2,
    Invert = 3,
    Incr = 4,
    Decr = 5,
}

/// Light Components
#[repr(u32)]
pub enum LightComponent {
    Ambient = 1,
    Diffuse = 2,
    Specular = 4,
    AmbientAndDiffuse = 3,
    DiffuseAndSpecular = 6,
    UnknownLightComponent = 8,
}

/// Light modes
#[repr(u32)]
pub enum LightMode {
    SingleColor = 0,
    SeparateSpecularColor = 1,
}

/// Light Type
#[repr(u32)]
pub enum LightType {
    Directional = 0,
    Pointlight = 1,
    Spotlight = 2,
}

/// Contexts
#[repr(u32)]
pub enum Context {
    Direct = 0,
    Call = 1,
    Send_ = 2,
}

/// List Queue
#[repr(u32)]
pub enum ListQueue {
    Tail = 0,
    Head = 1,
}

/// Sync behavior (mode)
#[repr(u32)]
pub enum SyncMode {
    SyncFinish = 0,
    SyncSignal = 1,
    SyncDone = 2,
    SyncList = 3,
    SyncSend = 4,
}

/// behavior (what)
#[repr(u32)]
pub enum SyncModeWhat {
    SyncWait = 0,
    SyncNowait = 1,
}

/// Sync behavior (what)
#[repr(u32)]
pub enum SyncBehaviorWhat {
    SyncWhatDone = 0,
    SyncWhatQueued = 1,
    SyncWhatDraw = 2,
    SyncWhatStall = 3,
    SyncWhatCancel = 4,
}

/// Signals
#[repr(u32)]
pub enum Signal {
    CallbackSignal = 1,
    CallbackFinish = 4,
}

/// Signal behavior
#[repr(u32)]
pub enum SignalBehavior {
    BehaviorSuspend = 1,
    BehaviorContinue = 2,
}

#[inline]
/// Color Macros, maps 8 bit unsigned channels into one 32-bit value */
pub const fn abgr(a: u8, b: u8, g: u8, r: u8) -> u32 {
    let mut res: u32 = 0;
    res += (a as u32) << 24;
    res += (b as u32) << 16;
    res += (g as u32) << 8;
    res += r as u32;
    return res;
}

#[inline]
pub const fn argb(a: u8, r: u8, g: u8, b: u8) -> u32 {
    abgr(a, b, g, r)
}

#[inline]
pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    argb(a, r, g, b)
}

#[inline]
/// Color Macro, maps floating point channels (0..1) into one 32-bit value
pub fn color(r: f32, g: f32, b: f32, a: f32) -> u32 {
    rgba(
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        (a * 255.0) as u8,
    )
}

pub type GuCallback = Option<fn(arg: i32)>;
pub type GuSwapBuffersCallback = Option<fn(display: *mut *mut c_void, render: *mut *mut c_void)>;

//GU INTERNAL
struct GuSettings {
    sig: GuCallback,
    fin: GuCallback,
    signal_history: [i16; 16],
    signal_offset: u32,
    kernel_event_flag: u32,
    ge_callback_id: u32,
    swapBuffersCallback: GuSwapBuffersCallback,
    swapBuffersBehaviour: u32,
}

struct GuDisplayList {
    start: *mut u32,
    current: *mut u32,
    parent_context: i32,
}

struct GuContext {
    list: GuDisplayList,
    scissor_enable: i32,
    scissor_start: [i32; 2],
    scissor_end: [i32; 2],
    near_plane: i32,
    far_plane: i32,
    depth_offset: i32,
    fragment_2x: i32,
    texture_function: i32,
    texture_proj_map_mode: i32,
    texture_map_mode: i32,
    sprite_mode: [i32; 4],
    clear_color: u32,
    clear_stencil: u32,
    clear_depth: u32,
    texture_mode: i32,
}

struct GuDrawBuffer {
    pixel_size: i32,
    frame_width: i32,
    frame_buffer: *mut c_void,
    disp_buffer: *mut c_void,
    depth_buffer: *mut c_void,
    depth_width: i32,
    width: i32,
    height: i32,
}

struct GuLightSettings {
    /// Light enable
    enable: u8,
    /// Light type
    type_: u8,
    /// X position
    xpos: u8,
    /// Y position
    ypos: u8,
    /// Z position
    zpos: u8,
    /// X direction
    xdir: u8,
    /// Y direction
    ydir: u8,
    /// Z direction
    zdir: u8,

    /// Ambient color
    ambient: u8,
    /// Diffuse color
    diffuse: u8,
    /// Specular color
    specular: u8,
    /// Constant attenuation
    constant: u8,
    /// Linear attenuation
    linear: u8,
    /// Quadratic attenuation
    quadratic: u8,
    /// Light exponent
    exponent: u8,
    /// Light cutoff
    cutoff: u8,
}

static mut gu_current_frame: u32 = 0;
static mut gu_contexts: [GuContext; 3] = [
    GuContext {
        list: GuDisplayList {
            start: null_mut(),
            current: null_mut(),
            parent_context: 0,
        },
        scissor_enable: 0,
        scissor_start: [0, 0],
        scissor_end: [0, 0],
        near_plane: 0,
        far_plane: 0,
        depth_offset: 0,
        fragment_2x: 0,
        texture_function: 0,
        texture_proj_map_mode: 0,
        texture_map_mode: 0,
        sprite_mode: [0, 0, 0, 0],
        clear_color: 0,
        clear_stencil: 0,
        clear_depth: 0,
        texture_mode: 0,
    },
    GuContext {
        list: GuDisplayList {
            start: null_mut(),
            current: null_mut(),
            parent_context: 0,
        },
        scissor_enable: 0,
        scissor_start: [0, 0],
        scissor_end: [0, 0],
        near_plane: 0,
        far_plane: 0,
        depth_offset: 0,
        fragment_2x: 0,
        texture_function: 0,
        texture_proj_map_mode: 0,
        texture_map_mode: 0,
        sprite_mode: [0, 0, 0, 0],
        clear_color: 0,
        clear_stencil: 0,
        clear_depth: 0,
        texture_mode: 0,
    },
    GuContext {
        list: GuDisplayList {
            start: null_mut(),
            current: null_mut(),
            parent_context: 0,
        },
        scissor_enable: 0,
        scissor_start: [0, 0],
        scissor_end: [0, 0],
        near_plane: 0,
        far_plane: 0,
        depth_offset: 0,
        fragment_2x: 0,
        texture_function: 0,
        texture_proj_map_mode: 0,
        texture_map_mode: 0,
        sprite_mode: [0, 0, 0, 0],
        clear_color: 0,
        clear_stencil: 0,
        clear_depth: 0,
        texture_mode: 0,
    },
];

static mut ge_list_executed: [i32; 2] = [0, 0];
static mut ge_edram_address: *mut c_void = null_mut();

static mut gu_settings: GuSettings = GuSettings {
    sig: None,
    fin: None,
    signal_history: [0; 16],
    signal_offset: 0,
    kernel_event_flag: 0,
    ge_callback_id: 0,
    swapBuffersBehaviour: 0,
    swapBuffersCallback: None,
};

static mut gu_list: *mut GuDisplayList = null_mut();
static mut gu_curr_context: i32 = 0;
static mut gu_init: i32 = 0;
static mut gu_display_on: i32 = 0;
static mut gu_call_mode: i32 = 0;
static mut gu_states: i32 = 0;

static mut gu_draw_buffer: GuDrawBuffer = GuDrawBuffer {
    depth_buffer: null_mut(),
    frame_buffer: null_mut(),
    disp_buffer: null_mut(),
    width: 0,
    height: 0,
    depth_width: 0,
    frame_width: 0,
    pixel_size: 0,
};

static mut gu_object_stack: *mut *mut u32 = null_mut();
static mut gu_object_stack_depth: i32 = 0;
static mut light_settings: [GuLightSettings; 4] = [
    GuLightSettings {
        enable: 0x18,
        type_: 0x5f,
        xpos: 0x63,
        ypos: 0x64,
        zpos: 0x65,
        xdir: 0x6f,
        ydir: 0x70,
        zdir: 0x71,
        ambient: 0x8f,
        diffuse: 0x90,
        specular: 0x91,
        constant: 0x7b,
        linear: 0x7c,
        quadratic: 0x7d,
        exponent: 0x87,
        cutoff: 0x8b,
    },
    GuLightSettings {
        enable: 0x19,
        type_: 0x60,
        xpos: 0x66,
        ypos: 0x67,
        zpos: 0x68,
        xdir: 0x72,
        ydir: 0x73,
        zdir: 0x74,
        ambient: 0x92,
        diffuse: 0x93,
        specular: 0x94,
        constant: 0x7e,
        linear: 0x7f,
        quadratic: 0x80,
        exponent: 0x88,
        cutoff: 0x8c,
    },
    GuLightSettings {
        enable: 0x1A,
        type_: 0x61,
        xpos: 0x69,
        ypos: 0x6A,
        zpos: 0x6B,
        xdir: 0x75,
        ydir: 0x76,
        zdir: 0x77,
        ambient: 0x95,
        diffuse: 0x99,
        specular: 0x9A,
        constant: 0x84,
        linear: 0x82,
        quadratic: 0x83,
        exponent: 0x89,
        cutoff: 0x8d,
    },
    GuLightSettings {
        enable: 0x1B,
        type_: 0x62,
        xpos: 0x6c,
        ypos: 0x6d,
        zpos: 0x6e,
        xdir: 0x78,
        ydir: 0x79,
        zdir: 0x7A,
        ambient: 0x98,
        diffuse: 0x99,
        specular: 0x9A,
        constant: 0x84,
        linear: 0x85,
        quadratic: 0x86,
        exponent: 0x8a,
        cutoff: 0x8e,
    },
];

/// Get the size of VRAM.
///
/// # Return Value
///
/// The size of VRAM (in bytes).
pub fn sce_ge_edram_get_size() -> u32 {unimplemented!()}

/// Get the eDRAM address.
///
/// # Return Value
///
/// A pointer to the base of the eDRAM.
pub fn sce_ge_edram_get_addr() -> *mut c_void {unimplemented!()}

/// Retrieve the current value of a GE command.
///
/// # Parameters
///
/// - `cmd`: The GE command register to retrieve (0 to 0xFF, both included).
///
/// # Return Value
///
/// The value of the GE command, < 0 on error.
pub fn sce_ge_get_cmd(cmd: i32) -> u32 {unimplemented!()}

/// Retrieve a matrix of the given type.
///
/// # Parameters
///
/// - `type`: One of ::GeMatrixTypes.
/// - `matrix`: Pointer to a variable to store the matrix.
///
/// # Return Value
///
/// < 0 on error.
pub fn sce_ge_get_mtx(type_: i32, matrix: *mut c_void) -> i32 {unimplemented!()}

/// Retrieve the stack of the display list currently being executed.
///
/// # Parameters
///
/// - `stackId`: The ID of the stack to retrieve.
/// - `stack`: Pointer to a structure to store the stack, or NULL to not store it.
///
/// # Return Value
///
/// The number of stacks of the current display list, < 0 on error.
pub fn sce_ge_get_stack(stack_id: i32, stack: *mut GeStack) -> i32 {unimplemented!()}

/// Save the GE's current state.
///
/// # Parameters
///
/// - `context`: Pointer to a ::GeContext.
///
/// # Return Value
///
/// < 0 on error.
pub fn sce_ge_save_context(context: *mut GeContext) -> i32 {unimplemented!()}

/// Restore a previously saved GE context.
///
/// # Parameters
///
/// - `context`: Pointer to a ::GeContext.
///
/// # Return Value
///
/// < 0 on error.
pub fn sce_ge_restore_context(context: *const GeContext) -> i32 {unimplemented!()}

/// Enqueue a display list at the tail of the GE display list queue.
///
/// # Parameters
///
/// - `list`: The head of the list to queue.
/// - `stall`: The stall address.
/// If NULL then no stall address is set and the list is transferred immediately.
/// # Parameters
///
/// - `cbid`: ID of the callback set by calling sceGeSetCallback
/// - `arg`: Structure containing GE context buffer address
///
/// # Return Value
///
/// The ID of the queue, < 0 on error.
pub fn sce_ge_list_en_queue(
    list: *const c_void,
    stall: *mut c_void,
    cbid: i32,
    arg: *mut GeListArgs,
) -> i32 {unimplemented!()}

/// Enqueue a display list at the head of the GE display list queue.
///
/// # Parameters
///
/// - `list`: The head of the list to queue.
/// - `stall`: The stall address.
/// If NULL then no stall address is set and the list is transferred immediately.
/// # Parameters
///
/// - `cbid`: ID of the callback set by calling sceGeSetCallback
/// - `arg`: Structure containing GE context buffer address
///
/// # Return Value
///
/// The ID of the queue, < 0 on error.
pub fn sce_ge_list_en_queue_head(
    list: *const c_void,
    stall: *mut c_void,
    cbid: i32,
    arg: *mut GeListArgs,
) -> i32 {unimplemented!()}

/// Cancel a queued or running list.
///
/// # Parameters
///
/// - `qid`: The ID of the queue.
///
/// # Return Value
///
/// < 0 on error.
pub fn sce_ge_list_de_queue(qid: i32) -> i32 {unimplemented!()}

/// Update the stall address for the specified queue.
///
/// # Parameters
///
/// - `qid`: The ID of the queue.
/// - `stall`: The new stall address.
///
/// # Return Value
///
/// < 0 on error
pub fn sce_ge_list_update_stall_addr(qid: i32, stall: *mut c_void) -> i32 {unimplemented!()}

/// Wait for syncronisation of a list.
///
/// # Parameters
///
/// - `qid`: The queue ID of the list to sync.
/// - `syncType`: 0 if you want to wait for the list to be completed, or 1 if you just want to peek the actual state.
///
/// # Return Value
///
/// The specified queue status, one of ::GeListState.
pub fn sce_ge_list_sync(qid: i32, sync_type: i32) -> i32 {unimplemented!()}

/// Wait for drawing to complete.
///
/// # Parameters
///
/// - `syncType`: 0 if you want to wait for the drawing to be completed, or 1 if you just want to peek the state of the display list currently being executed.
///
/// # Return Value
///
/// The current queue status, one of ::GeListState.
pub fn sce_ge_draw_sync(sync_type: i32) -> i32 {unimplemented!()}

/// Register callback handlers for the the GE.
///
/// # Parameters
///
/// - `cb`: Configured callback data structure.
///
/// # Return Value
///
/// The callback ID, < 0 on error.
pub fn sce_ge_set_callback(cb: *mut GeCallbackData) -> i32 {unimplemented!()}

/// Unregister the callback handlers.
///
/// # Parameters
///
/// - `cbid`: The ID of the callbacks, returned by sceGeSetCallback().
///
/// # Return Value
///
/// < 0 on error
pub fn sce_ge_unset_callback(cbid: i32) -> i32 {unimplemented!()}

/// Interrupt drawing queue.
///
/// # Parameters
///
/// - `mode`: If set to 1, reset all the queues.
/// - `pParam`: Unused (just K1-checked).
///
/// # Return Value
///
/// The stopped queue ID if mode isn't set to 0, otherwise 0, and < 0 on error.
pub fn sce_ge_break(mode: i32, pparam: *mut GeBreakParam) -> i32 {unimplemented!()}

/// Restart drawing queue.
///
/// # Return Value
///
/// < 0 on error.
pub fn sce_ge_continue() -> i32 {unimplemented!()}

/// Set the eDRAM address translation mode.
///
/// # Parameters
///
/// - `width`: 0 to not set the translation width, otherwise 512, 1024, 2048 or 4096.
///
/// # Return Value
///
/// The previous width if it was set, otherwise 0, < 0 on error.
pub fn sce_ge_edram_set_addr_translation(width: i32) -> i32 {unimplemented!()}

/// Set depth buffer parameters
///
/// # Parameters
///
/// - `zbp`: VRAM pointer where the depthbuffer should start
/// - `zbw`: The width of the depth-buffer (block-aligned)
pub fn sce_gu_depth_buffer(zbp: *mut c_void, zbw: i32) {unimplemented!()}

/// Set display buffer parameters
///
/// @par Example: Setup a standard 16-bit display buffer
/// @code
/// sceGuDispBuffer(480,272,(void*)512*272*2,512) { // 480*272, skipping the draw buffer located at address 0
/// @endcode
///
/// # Parameters
///
/// - `width`: Width of the display buffer in pixels
/// - `height`: Width of the display buffer in pixels
/// - `dispbp`: VRAM pointer to where the display-buffer starts
/// - `dispbw`: Display buffer width (block aligned)
pub fn sce_gu_disp_buffer(width: i32, height: i32, dispbp: *mut c_void, dispbw: i32) {unimplemented!()}

/// Set draw buffer parameters (and store in context for buffer-swap)
///
/// Available pixel formats are:
///   - GU_PSM_5650
///   - GU_PSM_5551
///   - GU_PSM_4444
///   - GU_PSM_8888
///
/// @par Example: Setup a standard 16-bit draw buffer
/// @code
/// sceGuDrawBuffer(GU_PSM_5551,(void*)0,512) {
/// @endcode
///
/// # Parameters
///
/// - `psm`: Pixel format to use for rendering (and display)
/// - `fbp`: VRAM pointer to where the draw buffer starts
/// - `fbw`: Frame buffer width (block aligned)
pub fn sce_gu_draw_buffer(psm: i32, fbp: *mut c_void, fbw: i32) {unimplemented!()}

/// Set draw buffer directly, not storing parameters in the context
///
/// # Parameters
///
/// - `psm`: Pixel format to use for rendering
/// - `fbp`: VRAM pointer to where the draw buffer starts
/// - `fbw`: Frame buffer width (block aligned)
pub fn sce_gu_draw_buffer_list(psm: i32, fbp: *mut c_void, fbw: i32) {unimplemented!()}

/// Turn display on or off
///
/// Available states are:
///   - GU_TRUE (1) - Turns display on
///   - GU_FALSE (0) - Turns display off
///
/// # Parameters
///
/// - `state`: Turn display on or off
/// # Return Value
///
/// State of the display prior to this call
pub fn sce_gu_display(state: i32) -> i32 {unimplemented!()}

/// Select which depth-test function to use
///
/// Valid choices for the depth-test are:
///   - GU_NEVER - No pixels pass the depth-test
///   - GU_ALWAYS - All pixels pass the depth-test
///   - GU_EQUAL - Pixels that match the depth-test pass
///   - GU_NOTEQUAL - Pixels that doesn't match the depth-test pass
///   - GU_LESS - Pixels that are less in depth passes
///   - GU_LEQUAL - Pixels that are less or equal in depth passes
///   - GU_GREATER - Pixels that are greater in depth passes
///   - GU_GEQUAL - Pixels that are greater or equal passes
///
/// # Parameters
///
/// - `function`: Depth test function to use
pub fn sce_gu_depth_func(function: i32) {unimplemented!()}

/// Mask depth buffer writes
///
/// # Parameters
///
/// - `mask`: GU_TRUE(1) to disable Z writes, GU_FALSE(0) to enable
pub fn sce_gu_depth_mask(mask: i32) {unimplemented!()}

pub fn sce_gu_depth_offset(offset: u32) {unimplemented!()}

/// Set which range to use for depth calculations.
///
/// @note The depth buffer is inversed, and takes values from 65535 to 0.
///
/// Example: Use the entire depth-range for calculations:
/// @code
/// sceGuDepthRange(65535,0) {
/// @endcode
///
/// # Parameters
///
/// - `near`: Value to use for the near plane
/// - `far`: Value to use for the far plane
pub fn sce_gu_depth_range(near: i32, far: i32) {unimplemented!()}

pub fn sce_gu_fog(near: f32, far: f32, color: u32) {unimplemented!()}

/// Initalize the GU system
/// This function MUST be called as the first function, otherwise state is undetermined.
pub fn sce_gu_init() {unimplemented!()}

/// Shutdown the GU system
/// Called when GU is no longer needed
pub fn sce_gu_term() {unimplemented!()}

pub fn sce_gu_break(a0: i32) {unimplemented!()}

pub fn sce_gu_continue() {unimplemented!()}

/// Setup signal handler
///
/// Available signals are:
///   - GU_CALLBACK_SIGNAL - Called when sceGuSignal is used
///   - GU_CALLBACK_FINISH - Called when display list is finished
///
/// # Parameters
///
/// - `signal`: Signal index to install a handler for
/// - `callback`: Callback to call when signal index is triggered
/// # Return Value
///
/// The old callback handler
pub fn sce_gu_set_callback(
    signal: i32,
    callback: Option<unsafe extern "C" fn(arg1: i32)>,
) -> *mut c_void {
    unimplemented!()
}

/// Trigger signal to call code from the command stream
///
/// Available behaviors are:
///   - GU_BEHAVIOR_SUSPEND - Stops display list execution until callback function finished
///   - GU_BEHAVIOR_CONTINUE - Do not stop display list execution during callback
///
/// # Parameters
///
/// - `signal`: Signal to trigger
/// - `behavior`: Behavior type
pub fn sce_gu_signal(signal: i32, behavior: i32) {unimplemented!()}

/// Send raw float-command to the GE
///
/// The argument is converted into a 24-bit float before transfer.
///
/// # Parameters
///
/// - `cmd`: Which command to send
/// - `argument`: Argument to pass along
pub fn sce_gu_send_commandf(cmd: i32, argument: f32) {unimplemented!()}

/// Send raw command to the GE
///
/// Only the 24 lower bits of the argument is passed along.
///
/// # Parameters
///
/// - `cmd`: Which command to send
/// - `argument`: Argument to pass along
pub fn sce_gu_send_commandi(cmd: i32, argument: i32) {unimplemented!()}

/// Allocate memory on the current display list for temporary storage
///
/// @note This function is NOT for permanent memory allocation, the
/// memory will be invalid as soon as you start filling the same display
/// list again.
///
/// # Parameters
///
/// - `size`: How much memory to allocate
/// # Return Value
///
/// Memory-block ready for use
pub fn sce_gu_get_memory(size: i32) -> *mut c_void {unimplemented!()}

/// Start filling a new display-context
///
/// Contexts available are:
///   - GU_DIRECT - Rendering is performed as list is filled
///   - GU_CALL - List is setup to be called from the main list
///   - GU_SEND - List is buffered for a later call to sceGuSendList()
///
/// The previous context-type is stored so that it can be restored at sceGuFinish().
///
/// # Parameters
///
/// - `cid`: Context Type
/// - `list`: Pointer to display-list (16 byte aligned)
pub fn sce_gu_start(cid: i32, list: *mut c_void) {unimplemented!()}

/// Finish current display list and go back to the parent context
///
/// If the context is GU_DIRECT, the stall-address is updated so that the entire list will
/// execute. Otherwise, only the terminating action is written to the list, depending on
/// context-type.
///
/// The finish-callback will get a zero as argument when using this function.
///
/// This also restores control back to whatever context that was active prior to this call.
///
/// # Return Value
///
/// Size of finished display list
pub fn sce_gu_finish() -> i32 {unimplemented!()}

/// Finish current display list and go back to the parent context, sending argument id for
/// the finish callback.
///
/// If the context is GU_DIRECT, the stall-address is updated so that the entire list will
/// execute. Otherwise, only the terminating action is written to the list, depending on
/// context-type.
///
/// # Parameters
///
/// - `id`: Finish callback id (16-bit)
/// # Return Value
///
/// Size of finished display list
pub fn sce_gu_finish_id(id: u32) -> i32 {unimplemented!()}

/// Call previously generated display-list
///
/// # Parameters
///
/// - `list`: Display list to call
pub fn sce_gu_call_list(list: *const c_void) {unimplemented!()}

/// Set wether to use stack-based calls or signals to handle execution of called lists.
///
/// # Parameters
///
/// - `mode`: GU_TRUE(1) to enable signals, GU_FALSE(0) to disable signals and use
/// normal calls instead.
pub fn sce_gu_call_mode(mode: i32) {unimplemented!()}

/// Check how large the current display-list is
///
/// # Return Value
///
/// The size of the current display list
pub fn sce_gu_check_list() -> i32 {unimplemented!()}

/// Send a list to the GE directly
///
/// Available modes are:
///   - GU_TAIL - Place list last in the queue, so it executes in-order
///   - GU_HEAD - Place list first in queue so that it executes as soon as possible
///
/// # Parameters
///
/// - `mode`: Whether to place the list first or last in queue
/// - `list`: List to send
/// - `context`: Temporary storage for the GE context
pub fn sce_gu_send_list(mode: i32, list: *const c_void, context: *mut GeContext) {unimplemented!()}

/// Swap display and draw buffer
///
/// # Return Value
///
/// Pointer to the new drawbuffer
pub fn sce_gu_swap_buffers() -> *mut c_void {unimplemented!()}

/// Wait until display list has finished executing
///
/// @par Example: Wait for the currently executing display list
/// @code
/// sceGuSync(0,0) {
/// @endcode
///
/// Available what are:
///   - GU_SYNC_WHAT_DONE
///   - GU_SYNC_WHAT_QUEUED
///   - GU_SYNC_WHAT_DRAW
///   - GU_SYNC_WHAT_STALL
///   - GU_SYNC_WHAT_CANCEL
///
/// Available mode are:
///   - GU_SYNC_FINISH - Wait until the last sceGuFinish command is reached
///   - GU_SYNC_SIGNAL - Wait until the last (?) signal is executed
///   - GU_SYNC_DONE - Wait until all commands currently in list are executed
///   - GU_SYNC_LIST - Wait for the currently executed display list (GU_DIRECT)
///   - GU_SYNC_SEND - Wait for the last send list
///
/// # Parameters
///
/// - `mode`: What to wait for
/// - `what`: What to sync to
/// # Return Value
///
/// Unknown at this time
pub fn sce_gu_sync(mode: i32, what: i32) -> i32 {unimplemented!()}

/// Draw array of vertices forming primitives
///
/// Available primitive-types are:
///   - GU_POINTS - Single pixel points (1 vertex per primitive)
///   - GU_LINES - Single pixel lines (2 vertices per primitive)
///   - GU_LINE_STRIP - Single pixel line-strip (2 vertices for the first primitive, 1 for every following)
///   - GU_TRIANGLES - Filled triangles (3 vertices per primitive)
///   - GU_TRIANGLE_STRIP - Filled triangles-strip (3 vertices for the first primitive, 1 for every following)
///   - GU_TRIANGLE_FAN - Filled triangle-fan (3 vertices for the first primitive, 1 for every following)
///   - GU_SPRITES - Filled blocks (2 vertices per primitive)
///
/// The vertex-type decides how the vertices align and what kind of information they contain.
/// The following flags are ORed together to compose the final vertex format:
///   - GU_TEXTURE_8BIT - 8-bit texture coordinates
///   - GU_TEXTURE_16BIT - 16-bit texture coordinates
///   - GU_TEXTURE_32BITF - 32-bit texture coordinates (float)
///
///   - GU_COLOR_5650 - 16-bit color (R5G6B5A0)
///   - GU_COLOR_5551 - 16-bit color (R5G5B5A1)
///   - GU_COLOR_4444 - 16-bit color (R4G4B4A4)
///   - GU_COLOR_8888 - 32-bit color (R8G8B8A8)
///
///   - GU_NORMAL_8BIT - 8-bit normals
///   - GU_NORMAL_16BIT - 16-bit normals
///   - GU_NORMAL_32BITF - 32-bit normals (float)
///
///   - GU_VERTEX_8BIT - 8-bit vertex position
///   - GU_VERTEX_16BIT - 16-bit vertex position
///   - GU_VERTEX_32BITF - 32-bit vertex position (float)
///
///   - GU_WEIGHT_8BIT - 8-bit weights
///   - GU_WEIGHT_16BIT - 16-bit weights
///   - GU_WEIGHT_32BITF - 32-bit weights (float)
///
///   - GU_INDEX_8BIT - 8-bit vertex index
///   - GU_INDEX_16BIT - 16-bit vertex index
///
///   - GU_WEIGHTS(n) - Number of weights (1-8)
///   - GU_VERTICES(n) - Number of vertices (1-8)
///
///   - GU_TRANSFORM_2D - Coordinate is passed directly to the rasterizer
///   - GU_TRANSFORM_3D - Coordinate is transformed before passed to rasterizer
///
/// @note Every vertex must align to 32 bits, which means that you HAVE to pad if it does not add up!
///
/// Vertex order:
/// [for vertices(1-8)]
/// [weights (0-8)]
/// [texture uv]
/// [color]
/// [normal]
/// [vertex]
/// [/for]
///
/// @par Example: Render 400 triangles, with floating-point texture coordinates, and floating-point position, no indices
/// @code
/// sceGuDrawArray(GU_TRIANGLES,GU_TEXTURE_32BITF|GU_VERTEX_32BITF,400*3,0,vertices) {
/// @endcode
///
/// # Parameters
///
/// - `prim`: What kind of primitives to render
/// - `vtype`: Vertex type to process
/// - `count`: How many vertices to process
/// - `indices`: Optional pointer to an index-list
/// - `vertices`: Pointer to a vertex-list
pub fn sce_gu_draw_array(
    prim: i32,
    vtype: i32,
    count: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {
}

/// Begin conditional rendering of object
///
/// If no vertices passed into this function are inside the scissor region, it will skip rendering
/// the object. There can be up to 32 levels of conditional testing, and all levels HAVE to
/// be terminated by sceGuEndObject().
///
/// @par Example: test a boundingbox against the frustum, and if visible, render object
/// @code
/// sceGuBeginObject(GU_VERTEX_32BITF,8,0,boundingBox) {
///   sceGuDrawArray(GU_TRIANGLES,GU_TEXTURE_32BITF|GU_VERTEX_32BITF,vertexCount,0,vertices) {
/// sceGuEndObject() {
/// @endcode
///
/// # Parameters
///
/// - `vtype`: Vertex type to process
/// - `count`: Number of vertices to test
/// - `indices`: Optional list to an index-list
/// - `vertices`: Pointer to a vertex-list
pub fn sce_gu_begin_object(
    vtype: i32,
    count: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {
}

/// End conditional rendering of object
pub fn sce_gu_end_object() {unimplemented!()}

/// Enable or disable GE state
///
/// Look at sceGuEnable() for a list of states
///
/// # Parameters
///
/// - `state`: Which state to change
/// - `status`: Wether to enable or disable the state
pub fn sce_gu_set_status(state: i32, status: i32) {unimplemented!()}

/// Get if state is currently enabled or disabled
///
/// Look at sceGuEnable() for a list of states
///
/// # Parameters
///
/// - `state`: Which state to query about
/// # Return Value
///
/// Wether state is enabled or not
pub fn sce_gu_get_status(state: i32) -> i32 {unimplemented!()}

/// Set the status on all 22 available states
///
/// Look at sceGuEnable() for a list of states
///
/// # Parameters
///
/// - `status`: Bit-mask (0-21) containing the status of all 22 states
pub fn sce_gu_set_all_status(status: i32) {unimplemented!()}

/// Query status on all 22 available states
///
/// Look at sceGuEnable() for a list of states
///
/// # Return Value
///
/// Status of all 22 states as a bitmask (0-21)
pub fn sce_gu_get_all_status() -> i32 {unimplemented!()}

/// Enable GE state
///
/// The currently available states are:
///   - GU_ALPHA_TEST
///   - GU_DEPTH_TEST
///   - GU_SCISSOR_TEST
///   - GU_BLEND
///   - GU_CULL_FACE
///   - GU_DITHER
///   - GU_CLIP_PLANES
///   - GU_TEXTURE_2D
///   - GU_LIGHTING
///   - GU_LIGHT0
///   - GU_LIGHT1
///   - GU_LIGHT2
///   - GU_LIGHT3
///   - GU_COLOR_LOGIC_OP
///
/// # Parameters
///
/// - `state`: Which state to enable
pub fn sce_gu_enable(state: i32) {unimplemented!()}

/// Disable GE state
///
/// Look at sceGuEnable() for a list of states
///
/// # Parameters
///
/// - `state`: Which state to disable
pub fn sce_gu_disable(state: i32) {unimplemented!()}

/// Set light parameters
///
/// Available light types are:
///   - GU_DIRECTIONAL - Directional light
///   - GU_POINTLIGHT - Single point of light
///   - GU_SPOTLIGHT - Point-light with a cone
///
/// Available light components are:
///   - GU_AMBIENT_AND_DIFFUSE
///   - GU_DIFFUSE_AND_SPECULAR
///   - GU_UNKNOWN_LIGHT_COMPONENT
///
/// # Parameters
///
/// - `light`: Light index
/// - `type`: Light type
/// - `components`: Light components
/// - `position`: Light position
pub fn sce_gu_light(light: i32, type_: i32, components: i32, position: *const FVector3) {unimplemented!()}

/// Set light attenuation
///
/// # Parameters
///
/// - `light`: Light index
/// - `atten0`: Constant attenuation factor
/// - `atten1`: Linear attenuation factor
/// - `atten2`: Quadratic attenuation factor
pub fn sce_gu_light_att(light: i32, atten0: f32, atten1: f32, atten2: f32) {unimplemented!()}

/// Set light color
///
/// Available light components are:
///   - GU_AMBIENT
///   - GU_DIFFUSE
///   - GU_SPECULAR
///   - GU_AMBIENT_AND_DIFFUSE
///   - GU_DIFFUSE_AND_SPECULAR
///
/// # Parameters
///
/// - `light`: Light index
/// - `component`: Which component to set
/// - `color`: Which color to use
pub fn sce_gu_light_color(light: i32, component: i32, color: u32) {unimplemented!()}

/// Set light mode
///
/// Available light modes are:
///   - GU_SINGLE_COLOR
///   - GU_SEPARATE_SPECULAR_COLOR
///
/// Separate specular colors are used to interpolate the specular component
/// independently, so that it can be added to the fragment after the texture color.
///
/// # Parameters
///
/// - `mode`: Light mode to use
pub fn sce_gu_light_mode(mode: i32) {unimplemented!()}

/// Set spotlight parameters
///
/// # Parameters
///
/// - `light`: Light index
/// - `direction`: Spotlight direction
/// - `exponent`: Spotlight exponent
/// - `cutoff`: Spotlight cutoff angle (in radians)
pub fn sce_gu_light_spot(light: i32, direction: *const FVector3, exponent: f32, cutoff: f32) {
}

/// Clear current drawbuffer
///
/// Available clear-flags are (OR them together to get final clear-mode):
///   - GU_COLOR_BUFFER_BIT - Clears the color-buffer
///   - GU_STENCIL_BUFFER_BIT - Clears the stencil-buffer
///   - GU_DEPTH_BUFFER_BIT - Clears the depth-buffer
///
/// # Parameters
///
/// - `flags`: Which part of the buffer to clear
pub fn sce_gu_clear(flags: i32) {unimplemented!()}

/// Set the current clear-color
///
/// # Parameters
///
/// - `color`: Color to clear with
pub fn sce_gu_clear_color(color: u32) {unimplemented!()}

/// Set the current clear-depth
///
/// # Parameters
///
/// - `depth`: Set which depth to clear with (0x0000-0xffff)
pub fn sce_gu_clear_depth(depth: u32) {unimplemented!()}

/// Set the current stencil clear value
///
/// # Parameters
///
/// - `stencil`: Set which stencil value to clear with (0-255)
pub fn sce_gu_clear_stencil(stencil: u32) {unimplemented!()}

/// Set mask for which bits of the pixels to write
///
/// # Parameters
///
/// - `mask`: Which bits to filter against writes
pub fn sce_gu_pixel_mask(mask: u32) {unimplemented!()}

/// Set current primitive color
///
/// # Parameters
///
/// - `color`: Which color to use (overriden by vertex-colors)
pub fn sce_gu_color(color: u32) {unimplemented!()}

/// Set the color test function
///
/// The color test is only performed while GU_COLOR_TEST is enabled.
///
/// Available functions are:
///   - GU_NEVER
///   - GU_ALWAYS
///   - GU_EQUAL
///   - GU_NOTEQUAL
///
/// @par Example: Reject any pixel that does not have 0 as the blue channel
/// @code
/// sceGuColorFunc(GU_EQUAL,0,0xff0000) {
/// @endcode
///
/// # Parameters
///
/// - `func`: Color test function
/// - `color`: Color to test against
/// - `mask`: Mask ANDed against both source and destination when testing
pub fn sce_gu_color_func(func: i32, color: u32, mask: u32) {unimplemented!()}

/// Set which color components that the material will receive
///
/// The components are ORed together from the following values:
///   - GU_AMBIENT
///   - GU_DIFFUSE
///   - GU_SPECULAR
///
/// # Parameters
///
/// - `components`: Which components to receive
pub fn sce_gu_color_material(components: i32) {unimplemented!()}

/// Set the alpha test parameters
///
/// Available comparison functions are:
///   - GU_NEVER
///   - GU_ALWAYS
///   - GU_EQUAL
///   - GU_NOTEQUAL
///   - GU_LESS
///   - GU_LEQUAL
///   - GU_GREATER
///   - GU_GEQUAL
///
/// # Parameters
///
/// - `func`: Specifies the alpha comparison function.
/// - `value`: Specifies the reference value that incoming alpha values are compared to.
/// - `mask`: Specifies the mask that both values are ANDed with before comparison.
pub fn sce_gu_alpha_func(func: i32, value: i32, mask: i32) {unimplemented!()}

pub fn sce_gu_ambient(color: u32) {unimplemented!()}

pub fn sce_gu_ambient_color(color: u32) {unimplemented!()}

/// Set the blending-mode
///
/// Keys for the blending operations:
///   - Cs - Source color
///   - Cd - Destination color
///   - Bs - Blend function for source fragment
///   - Bd - Blend function for destination fragment
///
/// Available blending-operations are:
///   - GU_ADD - (Cs*Bs) + (Cd*Bd)
///   - GU_SUBTRACT - (Cs*Bs) - (Cd*Bd)
///   - GU_REVERSE_SUBTRACT - (Cd*Bd) - (Cs*Bs)
///   - GU_MIN - Cs < Cd ? Cs : Cd
///   - GU_MAX - Cs < Cd ? Cd : Cs
///   - GU_ABS - |Cs-Cd|
///
/// Available blending-functions are:
///   - GU_SRC_COLOR
///   - GU_ONE_MINUS_SRC_COLOR
///   - GU_SRC_ALPHA
///   - GU_ONE_MINUS_SRC_ALPHA
///   - GU_DST_ALPHA
///   - GU_ONE_MINUS_DST_ALPHA
///   - GU_DST_COLOR
///   - GU_ONE_MINUS_DST_COLOR
///   - GU_FIX
///
/// # Parameters
///
/// - `op`: Blending Operation
/// - `src`: Blending function for source operand
/// - `dest`: Blending function for dest operand
/// - `srcfix`: Fix value for GU_FIX (source operand)
/// - `destfix`: Fix value for GU_FIX (dest operand)
pub fn sce_gu_blend_func(op: i32, src: i32, dest: i32, srcfix: u32, destfix: u32) {unimplemented!()}

pub fn sce_gu_material(mode: i32, color: i32) {unimplemented!()}

pub fn sce_gu_model_color(emissive: u32, ambient: u32, diffuse: u32, specular: u32) {unimplemented!()}

/// Set stencil function and reference value for stencil testing
///
/// Available functions are:
///   - GU_NEVER
///   - GU_ALWAYS
///   - GU_EQUAL
///   - GU_NOTEQUAL
///   - GU_LESS
///   - GU_LEQUAL
///   - GU_GREATER
///   - GU_GEQUAL
///
/// # Parameters
///
/// - `func`: Test function
/// - `ref`: The reference value for the stencil test
/// - `mask`: Mask that is ANDed with both the reference value and stored stencil value when the test is done
pub fn sce_gu_stencil_func(func: i32, ref_: i32, mask: i32) {unimplemented!()}

/// Set the stencil test actions
///
/// Available actions are:
///   - GU_KEEP - Keeps the current value
///   - GU_ZERO - Sets the stencil buffer value to zero
///   - GU_REPLACE - Sets the stencil buffer value to ref, as specified by sceGuStencilFunc()
///   - GU_INCR - Increments the current stencil buffer value
///   - GU_DECR - Decrease the current stencil buffer value
///   - GU_INVERT - Bitwise invert the current stencil buffer value
///
/// As stencil buffer shares memory with framebuffer alpha, resolution of the buffer
/// is directly in relation.
///
/// # Parameters
///
/// - `fail`: The action to take when the stencil test fails
/// - `zfail`: The action to take when stencil test passes, but the depth test fails
/// - `zpass`: The action to take when both stencil test and depth test passes
pub fn sce_gu_stencil_op(fail: i32, zfail: i32, zpass: i32) {unimplemented!()}

/// Set the specular power for the material
///
/// # Parameters
///
/// - `power`: Specular power
pub fn sce_gu_specular(power: f32) {unimplemented!()}

/// Set the current face-order (for culling)
///
/// This only has effect when culling is enabled (GU_CULL_FACE)
///
/// Culling order can be:
///   - GU_CW - Clockwise primitives are not culled
///   - GU_CCW - Counter-clockwise are not culled
///
/// # Parameters
///
/// - `order`: Which order to use
pub fn sce_gu_front_face(order: i32) {unimplemented!()}

/// Set color logical operation
///
/// Available operations are:
///   - GU_CLEAR
///   - GU_AND
///   - GU_AND_REVERSE
///   - GU_COPY
///   - GU_AND_INVERTED
///   - GU_NOOP
///   - GU_XOR
///   - GU_OR
///   - GU_NOR
///   - GU_EQUIV
///   - GU_INVERTED
///   - GU_OR_REVERSE
///   - GU_COPY_INVERTED
///   - GU_OR_INVERTED
///   - GU_NAND
///   - GU_SET
///
/// This operation only has effect if GU_COLOR_LOGIC_OP is enabled.
///
/// # Parameters
///
/// - `op`: Operation to execute
pub fn sce_gu_logical_op(op: i32) {unimplemented!()}

/// Set ordered pixel dither matrix
///
/// This dither matrix is only applied if GU_DITHER is enabled.
///
/// # Parameters
///
/// - `matrix`: Dither matrix
pub fn sce_gu_set_dither(matrix: *const IMatrix4) {unimplemented!()}

/// Set how primitives are shaded
///
/// The available shading-methods are:
///   - GU_FLAT - Primitives are flatshaded, the last vertex-color takes effet
///   - GU_SMOOTH - Primtives are gouraud-shaded, all vertex-colors take effect
///
/// # Parameters
///
/// - `mode`: Which mode to use
pub fn sce_gu_shade_model(mode: i32) {unimplemented!()}

/// Image transfer using the GE
///
/// @note Data must be aligned to 1 quad word (16 bytes)
///
/// @par Example: Copy a fullscreen 32-bit image from RAM to VRAM
/// @code
/// sceGuCopyImage(GU_PSM_8888,0,0,480,272,512,pixels,0,0,512,(void*)(((unsigned int)framebuffer)+0x4000000)) {
/// @endcode
///
/// # Parameters
///
/// - `psm`: Pixel format for buffer
/// - `sx`: Source X
/// - `sy`: Source Y
/// - `width`: Image width
/// - `height`: Image height
/// - `srcw`: Source buffer width (block aligned)
/// - `src`: Source pointer
/// - `dx`: Destination X
/// - `dy`: Destination Y
/// - `destw`: Destination buffer width (block aligned)
/// - `dest`: Destination pointer
pub fn sce_gu_copy_image(
    psm: i32,
    sx: i32,
    sy: i32,
    width: i32,
    height: i32,
    srcw: i32,
    src: *mut c_void,
    dx: i32,
    dy: i32,
    destw: i32,
    dest: *mut c_void,
) {unimplemented!()}

/// Specify the texture environment color
///
/// This is used in the texture function when a constant color is needed.
///
/// See sceGuTexFunc() for more information.
///
/// # Parameters
///
/// - `color`: Constant color (0x00BBGGRR)
pub fn sce_gu_tex_env_color(color: u32) {unimplemented!()}

/// Set how the texture is filtered
///
/// Available filters are:
///   - GU_NEAREST
///   - GU_LINEAR
///   - GU_NEAREST_MIPMAP_NEAREST
///   - GU_LINEAR_MIPMAP_NEAREST
///   - GU_NEAREST_MIPMAP_LINEAR
///   - GU_LINEAR_MIPMAP_LINEAR
///
/// # Parameters
///
/// - `min`: Minimizing filter
/// - `mag`: Magnifying filter
pub fn sce_gu_tex_filter(min: i32, mag: i32) {unimplemented!()}

/// Flush texture page-cache
///
/// Do this if you have copied/rendered into an area currently in the texture-cache
pub fn sce_gu_tex_flush() {unimplemented!()}

/// Set how textures are applied
///
/// Key for the apply-modes:
///   - Cv - Color value result
///   - Ct - Texture color
///   - Cf - Fragment color
///   - Cc - Constant color (specified by sceGuTexEnvColor())
///
/// Available apply-modes are: (TFX)
///   - GU_TFX_MODULATE - Cv=Ct*Cf TCC_RGB: Av=Af TCC_RGBA: Av=At*Af
///   - GU_TFX_DECAL - TCC_RGB: Cv=Ct,Av=Af TCC_RGBA: Cv=Cf*(1-At)+Ct*At Av=Af
///   - GU_TFX_BLEND - Cv=(Cf*(1-Ct))+(Cc*Ct) TCC_RGB: Av=Af TCC_RGBA: Av=At*Af
///   - GU_TFX_REPLACE - Cv=Ct TCC_RGB: Av=Af TCC_RGBA: Av=At
///   - GU_TFX_ADD - Cv=Cf+Ct TCC_RGB: Av=Af TCC_RGBA: Av=At*Af
///
/// The fields TCC_RGB and TCC_RGBA specify components that differ between
/// the two different component modes.
///
///   - GU_TFX_MODULATE - The texture is multiplied with the current diffuse fragment
///   - GU_TFX_REPLACE - The texture replaces the fragment
///   - GU_TFX_ADD - The texture is added on-top of the diffuse fragment
///
/// Available component-modes are: (TCC)
///   - GU_TCC_RGB - The texture alpha does not have any effect
///   - GU_TCC_RGBA - The texture alpha is taken into account
///
/// # Parameters
///
/// - `tfx`: Which apply-mode to use
/// - `tcc`: Which component-mode to use
pub fn sce_gu_tex_func(tfx: i32, tcc: i32) {unimplemented!()}

/// Set current texturemap
///
/// Textures may reside in main RAM, but it has a huge speed-penalty. Swizzle textures
/// to get maximum speed.
///
/// @note Data must be aligned to 1 quad word (16 bytes)
///
/// # Parameters
///
/// - `mipmap`: Mipmap level
/// - `width`: Width of texture (must be a power of 2)
/// - `height`: Height of texture (must be a power of 2)
/// - `tbw`: Texture Buffer Width (block-aligned)
/// - `tbp`: Texture buffer pointer (16 byte aligned)
pub fn sce_gu_tex_image(mipmap: i32, width: i32, height: i32, tbw: i32, tbp: *const c_void) {unimplemented!()}

/// Set texture-level mode (mipmapping)
///
/// Available modes are:
///   - GU_TEXTURE_AUTO
///   - GU_TEXTURE_CONST
///   - GU_TEXTURE_SLOPE
///
/// # Parameters
///
/// - `mode`: Which mode to use
/// - `bias`: Which mipmap bias to use
pub fn sce_gu_tex_level_mode(mode: u32, bias: f32) {unimplemented!()}

/// Set the texture-mapping mode
///
/// Available modes are:
///   - GU_TEXTURE_COORDS
///   - GU_TEXTURE_MATRIX
///   - GU_ENVIRONMENT_MAP
///
/// # Parameters
///
/// - `mode`: Which mode to use
/// - `a1`: Unknown
/// - `a2`: Unknown
pub fn sce_gu_tex_map_mode(mode: i32, a1: u32, a2: u32) {unimplemented!()}

/// Set texture-mode parameters
///
/// Available texture-formats are:
///   - GU_PSM_5650 - Hicolor, 16-bit
///   - GU_PSM_5551 - Hicolor, 16-bit
///   - GU_PSM_4444 - Hicolor, 16-bit
///   - GU_PSM_8888 - Truecolor, 32-bit
///   - GU_PSM_T4 - Indexed, 4-bit (2 pixels per byte)
///   - GU_PSM_T8 - Indexed, 8-bit
///
/// # Parameters
///
/// - `tpsm`: Which texture format to use
/// - `maxmips`: Number of mipmaps to use (0-8)
/// - `a2`: Unknown, set to 0
/// - `swizzle`: GU_TRUE(1) to swizzle texture-reads
pub fn sce_gu_tex_mode(tpsm: i32, maxmips: i32, a2: i32, swizzle: i32) {unimplemented!()}

/// Set texture offset
///
/// @note Only used by the 3D T&L pipe, renders done with GU_TRANSFORM_2D are
/// not affected by this.
///
/// # Parameters
///
/// - `u`: Offset to add to the U coordinate
/// - `v`: Offset to add to the V coordinate
pub fn sce_gu_tex_offset(u: f32, v: f32) {unimplemented!()}

/// Set texture projection-map mode
///
/// Available modes are:
///   - GU_POSITION
///   - GU_UV
///   - GU_NORMALIZED_NORMAL
///   - GU_NORMAL
///
/// # Parameters
///
/// - `mode`: Which mode to use
pub fn sce_gu_tex_proj_map_mode(mode: i32) {unimplemented!()}

/// Set texture scale
///
/// @note Only used by the 3D T&L pipe, renders ton with GU_TRANSFORM_2D are
/// not affected by this.
///
/// # Parameters
///
/// - `u`: Scalar to multiply U coordinate with
/// - `v`: Scalar to multiply V coordinate with
pub fn sce_gu_tex_scale(u: f32, v: f32) {unimplemented!()}

pub fn sce_gu_tex_slope(slope: f32) {unimplemented!()}

/// Synchronize rendering pipeline with image upload.
///
/// This will stall the rendering pipeline until the current image upload initiated by
/// sceGuCopyImage() has completed.
pub fn sce_gu_tex_sync() {unimplemented!()}

/// Set if the texture should repeat or clamp
///
/// Available modes are:
///   - GU_REPEAT - The texture repeats after crossing the border
///   - GU_CLAMP - Texture clamps at the border
///
/// # Parameters
///
/// - `u`: Wrap-mode for the U direction
/// - `v`: Wrap-mode for the V direction
pub fn sce_gu_tex_wrap(u: i32, v: i32) {unimplemented!()}

/// Upload CLUT (Color Lookup Table)
///
/// @note Data must be aligned to 1 quad word (16 bytes)
///
/// # Parameters
///
/// - `num_blocks`: How many blocks of 8 entries to upload (32*8 is 256 colors)
/// - `cbp`: Pointer to palette (16 byte aligned)
pub fn sce_gu_clut_load(num_blocks: i32, cbp: *const c_void) {unimplemented!()}

/// Set current CLUT mode
///
/// Available pixel formats for palettes are:
///   - GU_PSM_5650
///   - GU_PSM_5551
///   - GU_PSM_4444
///   - GU_PSM_8888
///
/// # Parameters
///
/// - `cpsm`: Which pixel format to use for the palette
/// - `shift`: Shifts color index by that many bits to the right
/// - `mask`: Masks the color index with this bitmask after the shift (0-0xFF)
/// - `a3`: Unknown, set to 0
pub fn sce_gu_clut_mode(cpsm: u32, shift: u32, mask: u32, a3: u32) {unimplemented!()}

/// Set virtual coordinate offset
///
/// The PSP has a virtual coordinate-space of 4096x4096, this controls where rendering is performed
///
/// @par Example: Center the virtual coordinate range
/// @code
/// sceGuOffset(2048-(480/2),2048-(480/2)) {
/// @endcode
///
/// # Parameters
///
/// - `x`: Offset (0-4095)
/// - `y`: Offset (0-4095)
pub fn sce_gu_offset(x: u32, y: u32) {unimplemented!()}

/// Set what to scissor within the current viewport
///
/// Note that scissoring is only performed if the custom scissoring is enabled (GU_SCISSOR_TEST)
///
/// # Parameters
///
/// - `x`: Left of scissor region
/// - `y`: Top of scissor region
/// - `w`: Width of scissor region
/// - `h`: Height of scissor region
pub fn sce_gu_scissor(x: i32, y: i32, w: i32, h: i32) {unimplemented!()}

/// Set current viewport
///
/// @par Example: Setup a viewport of size (480,272) with origo at (2048,2048)
/// @code
/// sceGuViewport(2048,2048,480,272) {
/// @endcode
///
/// # Parameters
///
/// - `cx`: Center for horizontal viewport
/// - `cy`: Center for vertical viewport
/// - `width`: Width of viewport
/// - `height`: Height of viewport
pub fn sce_gu_viewport(cx: i32, cy: i32, width: i32, height: i32) {unimplemented!()}

/// Draw bezier surface
///
/// # Parameters
///
/// - `vtype`: Vertex type, look at sceGuDrawArray() for vertex definition
/// - `ucount`: Number of vertices used in the U direction
/// - `vcount`: Number of vertices used in the V direction
/// - `indices`: Pointer to index buffer
/// - `vertices`: Pointer to vertex buffer
pub fn sce_gu_draw_bezier(
    vtype: i32,
    ucount: i32,
    vcount: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {unimplemented!()}

/// Set dividing for patches (beziers and splines)
///
/// # Parameters
///
/// - `ulevel`: Number of division on u direction
/// - `vlevel`: Number of division on v direction
pub fn sce_gu_patch_divide(ulevel: u32, vlevel: u32) {unimplemented!()}

pub fn sce_gu_patch_front_face(a0: u32) {unimplemented!()}

/// Set primitive for patches (beziers and splines)
///
/// # Parameters
///
/// - `prim`: Desired primitive type (GU_POINTS | GU_LINE_STRIP | GU_TRIANGLE_STRIP)
pub fn sce_gu_patch_prim(prim: i32) {unimplemented!()}

pub fn sce_gu_draw_spline(
    vtype: i32,
    ucount: i32,
    vcount: i32,
    uedge: i32,
    vedge: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {unimplemented!()}

/// Set transform matrices
///
/// Available matrices are:
///   - GU_PROJECTION - View->Projection matrix
///   - GU_VIEW - World->View matrix
///   - GU_MODEL - Model->World matrix
///   - GU_TEXTURE - Texture matrix
///
/// # Parameters
///
/// - `type`: Which matrix-type to set
/// - `matrix`: Matrix to load
pub fn sce_gu_set_matrix(type_: i32, matrix: *const FMatrix4) {unimplemented!()}

/// Specify skinning matrix entry
///
/// To enable vertex skinning, pass GU_WEIGHTS(n), where n is between
/// 1-8, and pass available GU_WEIGHT_??? declaration. This will change
/// the amount of weights passed in the vertex araay, and by setting the skinning,
/// matrices, you will multiply each vertex every weight and vertex passed.
///
/// Please see sceGuDrawArray() for vertex format information.
///
/// # Parameters
///
/// - `index`: Skinning matrix index (0-7)
/// - `matrix`: Matrix to set
pub fn sce_gu_bone_matrix(index: u32, matrix: *const FMatrix4) {unimplemented!()}

/// Specify morph weight entry
///
/// To enable vertex morphing, pass GU_VERTICES(n), where n is between
/// 1-8. This will change the amount of vertices passed in the vertex array,
/// and by setting the morph weights for every vertex entry in the array,
/// you can blend between them.
///
/// Please see sceGuDrawArray() for vertex format information.
///
/// # Parameters
///
/// - `index`: Morph weight index (0-7)
/// - `weight`: Weight to set
pub fn sce_gu_morph_weight(index: i32, weight: f32) {unimplemented!()}

pub fn sce_gu_draw_array_n(
    primitive_type: i32,
    vertex_type: i32,
    count: i32,
    a3: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {unimplemented!()}
