pub mod bar;
pub mod line;
pub mod scatter;

pub trait Renderer {
    fn render(&self, output_path: &str, size: u32) -> anyhow::Result<()>;
}

pub fn ensure_output_dir(path: &str) -> anyhow::Result<()> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(|e| {
                anyhow::anyhow!(
                    "Failed to create output directory '{}': {}",
                    parent.display(),
                    e
                )
            })?;
        }
    }
    Ok(())
}
