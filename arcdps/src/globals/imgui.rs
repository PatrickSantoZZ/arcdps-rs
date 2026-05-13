#[cfg(all(feature = "imgui", feature = "codegen-globals"))]
use crate::{
    imgui::{self, Context, Ui},
    util::Share,
};
#[cfg(all(feature = "imgui", feature = "codegen-globals"))]
use std::{mem::ManuallyDrop, ptr, sync::OnceLock};
use std::ffi::c_void;

#[allow(dead_code)]
pub type MallocFn = unsafe extern "C" fn(size: usize, user_data: *mut c_void) -> *mut c_void;

#[allow(dead_code)]
pub type FreeFn = unsafe extern "C" fn(ptr: *mut c_void, user_data: *mut c_void);

/// ImGui context.
#[cfg(all(feature = "imgui", feature = "codegen-globals"))]
pub static IG_CONTEXT: OnceLock<Share<ManuallyDrop<Context>>> = OnceLock::new();

#[cfg(all(feature = "imgui", feature = "codegen-globals"))]
thread_local! {
    /// ImGui UI.
    pub static IG_UI: Ui = unsafe {
        let _ = imgui_context();
        Ui::new_unchecked(Ui::DEFAULT_BUFFER_SIZE)
    };
}

/// Initializes ImGui information.
#[cfg(all(feature = "imgui", feature = "codegen-globals"))]
pub unsafe fn init_imgui(
    ctx: *mut imgui::sys::ImGuiContext,
    malloc: Option<MallocFn>,
    free: Option<FreeFn>,
) {
    unsafe {
        imgui::sys::igSetCurrentContext(ctx);
        imgui::sys::igSetAllocatorFunctions(malloc, free, ptr::null_mut());
        IG_CONTEXT.get_or_init(|| {
            let context = ManuallyDrop::new(Context::get_current()
                .expect("imgui context unset")
            );
            Share::new(context)
        });
    }
}

/// Retrieves the [`imgui::Context`].
#[inline]
#[cfg(all(feature = "imgui", feature = "codegen-globals"))]
pub unsafe fn imgui_context() -> &'static Context {
    unsafe {
        IG_CONTEXT
            .get()
            .expect("imgui context not initialized")
            .get()
    }
}

/// Retrieves the [`imgui::Ui`] for rendering.
#[inline]
#[cfg(all(feature = "imgui", feature = "codegen-globals"))]
pub unsafe fn with_ui<R>(body: impl FnOnce(&Ui) -> R) -> R {
    IG_UI.with(body)
}
