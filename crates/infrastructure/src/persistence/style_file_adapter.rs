//! Style file adapter for writing style definitions to CLAUDE.md.

use domain::value_objects::TokenMap;
use std::path::Path;
use tokio::fs;

use crate::persistence::{PersistenceError, Result};

/// Adapter for writing style definitions to CLAUDE.md.
///
/// This adapter manages style sections in CLAUDE.md using special markers
/// to delineate the style content.
pub struct StyleFileAdapter {
    file_path: std::path::PathBuf,
}

impl StyleFileAdapter {
    /// Marker for the beginning of a style section.
    pub const BEGIN_MARKER: &'static str = "<!-- AAD_STYLE:BEGIN -->";
    /// Marker for the end of a style section.
    pub const END_MARKER: &'static str = "<!-- AAD_STYLE:END -->";

    /// Creates a new StyleFileAdapter.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the CLAUDE.md file
    pub fn new<P: AsRef<Path>>(file_path: P) -> Self {
        Self {
            file_path: file_path.as_ref().to_path_buf(),
        }
    }

    /// Writes a style section to the CLAUDE.md file.
    ///
    /// If markers exist, replaces the content between them.
    /// If markers don't exist, appends the style section to the end.
    ///
    /// # Arguments
    ///
    /// * `style_name` - Name of the style being applied
    /// * `tokens` - Token map for replacing tokens in the style content
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or written.
    pub async fn write_style(&self, style_name: &str, tokens: &TokenMap) -> Result<()> {
        // Read existing content or create new
        let existing_content = if self.file_path.exists() {
            fs::read_to_string(&self.file_path).await?
        } else {
            String::new()
        };

        // Generate style content with token replacement
        let template = self.generate_style_template(style_name);
        let style_content = tokens
            .replace_tokens(&template)
            .map_err(|e| PersistenceError::TokenReplacementError(e.to_string()))?;

        // Build new content
        let new_content = self.update_content(&existing_content, &style_content)?;

        // Write to file
        fs::write(&self.file_path, new_content).await?;

        Ok(())
    }

    /// Generates a style template for a given style name.
    fn generate_style_template(&self, style_name: &str) -> String {
        format!(
            "\n{}\n## Style: {}\n\nApplied on: {{{{date}}}}\nBy: {{{{author}}}}\n{}\n",
            Self::BEGIN_MARKER,
            style_name,
            Self::END_MARKER
        )
    }

    /// Updates content by replacing or appending style section.
    fn update_content(&self, existing: &str, style_content: &str) -> Result<String> {
        // Check if markers exist
        if let Some(begin_pos) = existing.find(Self::BEGIN_MARKER) {
            if let Some(end_pos) = existing.find(Self::END_MARKER) {
                // Markers exist - replace content between them
                let before = &existing[..begin_pos];
                let after = &existing[end_pos + Self::END_MARKER.len()..];
                return Ok(format!("{}{}{}", before, style_content, after));
            }
        }

        // No markers - append to end
        let separator = if existing.is_empty() || existing.ends_with('\n') {
            ""
        } else {
            "\n"
        };
        Ok(format!("{}{}{}", existing, separator, style_content))
    }

    /// Reads the current style section from the file.
    ///
    /// Returns `None` if no style section exists.
    pub async fn read_style(&self) -> Result<Option<String>> {
        if !self.file_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&self.file_path).await?;

        if let Some(begin_pos) = content.find(Self::BEGIN_MARKER) {
            if let Some(end_pos) = content.find(Self::END_MARKER) {
                let style_section = &content[begin_pos..end_pos + Self::END_MARKER.len()];
                return Ok(Some(style_section.to_string()));
            }
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_write_style_to_new_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("CLAUDE.md");
        let adapter = StyleFileAdapter::new(&file_path);

        let mut tokens = TokenMap::new();
        tokens.insert("date", "2026-01-18");
        tokens.insert("author", "Test Author");

        adapter.write_style("expert-mode", &tokens).await.unwrap();

        let content = fs::read_to_string(&file_path).await.unwrap();
        assert!(content.contains(StyleFileAdapter::BEGIN_MARKER));
        assert!(content.contains(StyleFileAdapter::END_MARKER));
        assert!(content.contains("## Style: expert-mode"));
        assert!(content.contains("Applied on: 2026-01-18"));
        assert!(content.contains("By: Test Author"));
    }

    #[tokio::test]
    async fn test_write_style_replaces_existing() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("CLAUDE.md");
        let adapter = StyleFileAdapter::new(&file_path);

        let mut tokens = TokenMap::new();
        tokens.insert("date", "2026-01-18");
        tokens.insert("author", "Test Author");

        // Write first style
        adapter.write_style("style-1", &tokens).await.unwrap();

        // Write second style - should replace
        tokens.insert("date", "2026-01-19");
        adapter.write_style("style-2", &tokens).await.unwrap();

        let content = fs::read_to_string(&file_path).await.unwrap();
        assert!(content.contains("## Style: style-2"));
        assert!(!content.contains("## Style: style-1"));
        assert!(content.contains("Applied on: 2026-01-19"));
    }

    #[tokio::test]
    async fn test_write_style_preserves_content() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("CLAUDE.md");

        // Write initial content
        let initial_content = "# Project Documentation\n\nSome important content.\n";
        fs::write(&file_path, initial_content).await.unwrap();

        let adapter = StyleFileAdapter::new(&file_path);
        let mut tokens = TokenMap::new();
        tokens.insert("date", "2026-01-18");
        tokens.insert("author", "Test Author");

        adapter.write_style("minimal", &tokens).await.unwrap();

        let content = fs::read_to_string(&file_path).await.unwrap();
        assert!(content.contains("# Project Documentation"));
        assert!(content.contains("Some important content."));
        assert!(content.contains(StyleFileAdapter::BEGIN_MARKER));
    }

    #[tokio::test]
    async fn test_read_style() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("CLAUDE.md");
        let adapter = StyleFileAdapter::new(&file_path);

        // Initially no style
        assert!(adapter.read_style().await.unwrap().is_none());

        // Write style
        let mut tokens = TokenMap::new();
        tokens.insert("date", "2026-01-18");
        tokens.insert("author", "Test Author");
        adapter.write_style("test-style", &tokens).await.unwrap();

        // Read style
        let style = adapter.read_style().await.unwrap();
        assert!(style.is_some());
        let style_content = style.unwrap();
        assert!(style_content.contains("## Style: test-style"));
    }

    #[tokio::test]
    async fn test_update_content_with_markers() {
        let adapter = StyleFileAdapter::new("dummy.md");
        let existing = format!(
            "Before\n{}Old Style{}\nAfter",
            StyleFileAdapter::BEGIN_MARKER,
            StyleFileAdapter::END_MARKER
        );
        let new_style = format!(
            "{}New Style{}",
            StyleFileAdapter::BEGIN_MARKER,
            StyleFileAdapter::END_MARKER
        );

        let result = adapter.update_content(&existing, &new_style).unwrap();
        assert!(result.contains("Before"));
        assert!(result.contains("New Style"));
        assert!(result.contains("After"));
        assert!(!result.contains("Old Style"));
    }

    #[tokio::test]
    async fn test_update_content_without_markers() {
        let adapter = StyleFileAdapter::new("dummy.md");
        let existing = "Existing content";
        let new_style = format!(
            "{}New Style{}",
            StyleFileAdapter::BEGIN_MARKER,
            StyleFileAdapter::END_MARKER
        );

        let result = adapter.update_content(existing, &new_style).unwrap();
        assert!(result.contains("Existing content"));
        assert!(result.contains("New Style"));
    }
}
