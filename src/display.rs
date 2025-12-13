pub fn size_display(size: u64) -> String {
    let mut size = size as f64;
    let display = ["B", "KB", "MB", "GB", "TB"];
    let mut display_index = 0;

    while size >= 1024_f64 {
        size /= 1024_f64;
        display_index += 1;
    }
    format!("{:.2}{}", size, display[display_index])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_display() {
        assert_eq!(size_display(1024), "1.00KB");
        assert_eq!(size_display(1024 * 1024), "1.00MB");
        assert_eq!(size_display(1024 * 1024 * 1024), "1.00GB");
        assert_eq!(size_display(1024 * 1024 * 1024 * 1024), "1.00TB");
    }
}
