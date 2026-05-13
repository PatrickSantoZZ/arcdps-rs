//! Global instances with information.

#[cfg(all(feature = "windows", feature = "codegen-globals"))]
pub mod arc;
#[cfg(all(feature = "windows", feature = "codegen-globals"))]
pub mod dxgi;
pub mod imgui;
