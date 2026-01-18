use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, List, ListItem, Widget},
};

/// Spec ツリーを表示するWidget
pub struct SpecTree<'a> {
    specs: Vec<&'a str>,
}

impl<'a> SpecTree<'a> {
    pub fn new(specs: Vec<&'a str>) -> Self {
        Self { specs }
    }
}

impl<'a> Widget for SpecTree<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let items: Vec<ListItem> = self.specs.iter().map(|s| ListItem::new(*s)).collect();

        let list =
            List::new(items).block(Block::default().title("Spec Tree").borders(Borders::ALL));

        list.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spec_tree_creation() {
        let specs = vec!["SPEC-001", "SPEC-002"];
        let widget = SpecTree::new(specs);
        assert_eq!(widget.specs.len(), 2);
    }

    #[test]
    fn test_spec_tree_empty() {
        let widget = SpecTree::new(vec![]);
        assert_eq!(widget.specs.len(), 0);
    }
}
