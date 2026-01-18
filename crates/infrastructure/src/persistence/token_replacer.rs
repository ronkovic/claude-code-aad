//! Token replacement utility for template processing.

use domain::value_objects::TokenMap;

use crate::persistence::{PersistenceError, Result};

/// Utility for replacing tokens in template strings.
///
/// This is a convenience wrapper around `TokenMap::replace_tokens()`
/// with additional validation and error handling.
pub struct TokenReplacer;

impl TokenReplacer {
    /// Replaces tokens in a template string.
    ///
    /// Tokens are in the format `{{token_name}}`.
    ///
    /// # Arguments
    ///
    /// * `template` - The template string containing tokens
    /// * `token_map` - Map of token names to their replacement values
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Circular references are detected
    /// - Maximum replacement depth is exceeded
    /// - Other token replacement errors occur
    ///
    /// # Examples
    ///
    /// ```
    /// use infrastructure::persistence::TokenReplacer;
    /// use domain::value_objects::TokenMap;
    ///
    /// let mut tokens = TokenMap::new();
    /// tokens.insert("name", "Alice");
    /// tokens.insert("greeting", "Hello");
    ///
    /// let result = TokenReplacer::replace(
    ///     "{{greeting}}, {{name}}!",
    ///     &tokens
    /// ).unwrap();
    /// assert_eq!(result, "Hello, Alice!");
    /// ```
    pub fn replace(template: &str, token_map: &TokenMap) -> Result<String> {
        token_map
            .replace_tokens(template)
            .map_err(|e| PersistenceError::TokenReplacementError(e.to_string()))
    }

    /// Validates that all tokens in the template can be resolved.
    ///
    /// This performs a dry-run replacement and reports any undefined tokens
    /// that were found.
    ///
    /// Note: Due to TokenMap's design, undefined tokens are left as-is
    /// rather than causing errors. This method checks for their presence.
    ///
    /// # Arguments
    ///
    /// * `template` - The template string to validate
    /// * `token_map` - Map of token names to check against
    ///
    /// # Returns
    ///
    /// A vector of undefined token names found in the template.
    pub fn find_undefined_tokens(template: &str, token_map: &TokenMap) -> Vec<String> {
        let mut undefined = Vec::new();
        let mut current_pos = 0;

        while let Some(start) = template[current_pos..].find("{{") {
            let abs_start = current_pos + start;
            if let Some(end) = template[abs_start..].find("}}") {
                let abs_end = abs_start + end;
                let token_name = &template[abs_start + 2..abs_end];

                // Check if token is defined
                if token_map.get(token_name).is_none() {
                    undefined.push(token_name.to_string());
                }

                current_pos = abs_end + 2;
            } else {
                break;
            }
        }

        // Remove duplicates
        undefined.sort();
        undefined.dedup();
        undefined
    }

    /// Replaces tokens and ensures no undefined tokens remain.
    ///
    /// Unlike `replace()`, this method returns an error if any tokens
    /// in the template are not defined in the token map.
    ///
    /// # Arguments
    ///
    /// * `template` - The template string containing tokens
    /// * `token_map` - Map of token names to their replacement values
    ///
    /// # Errors
    ///
    /// Returns an error if any tokens are undefined.
    pub fn replace_strict(template: &str, token_map: &TokenMap) -> Result<String> {
        let undefined = Self::find_undefined_tokens(template, token_map);
        if !undefined.is_empty() {
            return Err(PersistenceError::TokenReplacementError(format!(
                "Undefined tokens: {}",
                undefined.join(", ")
            )));
        }

        Self::replace(template, token_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::value_objects::TokenMap;

    #[test]
    fn test_replace_single_token() {
        let mut tokens = TokenMap::new();
        tokens.insert("role", "賢者");

        let result = TokenReplacer::replace("You are a {{role}}", &tokens).unwrap();
        assert_eq!(result, "You are a 賢者");
    }

    #[test]
    fn test_replace_multiple_tokens() {
        let mut tokens = TokenMap::new();
        tokens.insert("feature", "authentication");
        tokens.insert("version", "1.0");

        let result =
            TokenReplacer::replace("Implementing {{feature}} v{{version}}", &tokens).unwrap();
        assert_eq!(result, "Implementing authentication v1.0");
    }

    #[test]
    fn test_replace_no_tokens() {
        let tokens = TokenMap::new();
        let result = TokenReplacer::replace("No tokens here", &tokens).unwrap();
        assert_eq!(result, "No tokens here");
    }

    #[test]
    fn test_replace_undefined_token_is_left_as_is() {
        let tokens = TokenMap::new();
        let result = TokenReplacer::replace("Hello {{unknown}}", &tokens).unwrap();
        assert_eq!(result, "Hello {{unknown}}");
    }

    #[test]
    fn test_replace_nested_tokens() {
        let mut tokens = TokenMap::new();
        tokens.insert("inner", "world");
        tokens.insert("outer", "Hello {{inner}}");

        let result = TokenReplacer::replace("{{outer}}!", &tokens).unwrap();
        assert_eq!(result, "Hello world!");
    }

    #[test]
    fn test_replace_circular_reference_error() {
        let mut tokens = TokenMap::new();
        tokens.insert("a", "{{b}}");
        tokens.insert("b", "{{a}}");

        let result = TokenReplacer::replace("{{a}}", &tokens);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_undefined_tokens() {
        let mut tokens = TokenMap::new();
        tokens.insert("defined", "value");

        let template = "{{defined}} {{undefined1}} {{undefined2}}";
        let undefined = TokenReplacer::find_undefined_tokens(template, &tokens);

        assert_eq!(undefined.len(), 2);
        assert!(undefined.contains(&"undefined1".to_string()));
        assert!(undefined.contains(&"undefined2".to_string()));
    }

    #[test]
    fn test_find_undefined_tokens_no_undefined() {
        let mut tokens = TokenMap::new();
        tokens.insert("token1", "value1");
        tokens.insert("token2", "value2");

        let template = "{{token1}} and {{token2}}";
        let undefined = TokenReplacer::find_undefined_tokens(template, &tokens);

        assert!(undefined.is_empty());
    }

    #[test]
    fn test_find_undefined_tokens_duplicate_handling() {
        let tokens = TokenMap::new();
        let template = "{{unknown}} {{unknown}} {{unknown}}";
        let undefined = TokenReplacer::find_undefined_tokens(template, &tokens);

        assert_eq!(undefined.len(), 1);
        assert_eq!(undefined[0], "unknown");
    }

    #[test]
    fn test_replace_strict_success() {
        let mut tokens = TokenMap::new();
        tokens.insert("name", "Alice");

        let result = TokenReplacer::replace_strict("Hello {{name}}", &tokens).unwrap();
        assert_eq!(result, "Hello Alice");
    }

    #[test]
    fn test_replace_strict_failure() {
        let mut tokens = TokenMap::new();
        tokens.insert("defined", "value");

        let result = TokenReplacer::replace_strict("{{defined}} {{undefined}}", &tokens);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.to_string().contains("Undefined tokens"));
        assert!(err.to_string().contains("undefined"));
    }
}
