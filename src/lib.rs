// This crate owns the placeholder guest until the mirrored WIT worlds land.

/// Keeps the template buildable before generated host bindings are available.
#[unsafe(no_mangle)]
pub extern "C" fn pito_plugin_placeholder() {}
