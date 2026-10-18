pub fn safe_state_path(path: &std::path::Path) -> bool {
    !path.as_os_str().is_empty()
        && !path.is_absolute()
        && !path
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn traversal() {
        assert!(!safe_state_path(std::path::Path::new("../x")));
        assert!(safe_state_path(std::path::Path::new("target/demo")));
    }
}
