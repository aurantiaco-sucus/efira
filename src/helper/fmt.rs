use alloc::format;
use alloc::string::String;

pub fn format_size(bytes: usize) -> String {
    let size = bytes;
    if size < 1024 {
        return format!("{size}B");
    }
    let size = size as f64 / 1024.0;
    if size < 1024.0 {
        return format!("{size:.1}KB");
    }
    let size = size / 1024.0;
    if size < 1024.0 {
        return format!("{size:.2}MB");
    }
    let size = size / 1024.0;
    if size < 1024.0 {
        return format!("{size:.3}GB");
    }
    let size = size / 1024.0;
    format!("{size:.4}TB")
}