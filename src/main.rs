pub use iced::Element;
pub use iced::Theme;
pub use iced::widget::{Column, button, column, text, text_input};

mod tree;
use tree::{Node, tree_view};

fn main() -> iced::Result {
    iced::run(TreeDemo::update, TreeDemo::view)
}

#[derive(Debug, Clone)]
pub enum Message {
    Toggle(Vec<usize>),
    StartEdit(Vec<usize>),
    EditLabel(String),
    FinishEdit,
}

pub struct TreeDemo {
    root: Node,
    editing_path: Option<Vec<usize>>,
    edit_value: String,
}

impl TreeDemo {
    fn new() -> Self {
        Self {
            root: Node::new(
                "Root",
                true,
                vec![
                    Node::new(
                        "Branch 1",
                        false,
                        vec![
                            Node::new("Leaf 1.1", false, vec![]),
                            Node::new("Leaf 1.2", false, vec![]),
                        ],
                    ),
                    Node::new(
                        "Branch 2",
                        false,
                        vec![Node::new("Leaf 2.1", false, vec![])],
                    ),
                ],
            ),
            editing_path: None,
            edit_value: String::new(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Toggle(path) => {
                if let Some(node) = self.get_node_mut(&path) {
                    node.open = !node.open;
                }
            }
            Message::StartEdit(path) => {
                // Find the node and start editing
                if let Some(node) = self.get_node(&path) {
                    self.edit_value = node.label.clone();
                    self.editing_path = Some(path);
                }
            }
            Message::EditLabel(value) => {
                self.edit_value = value;
            }
            Message::FinishEdit => {
                if let Some(path) = self.editing_path.clone() {
                    let new_label = self.edit_value.clone();
                    if let Some(node) = self.get_node_mut(&path) {
                        node.label = new_label;
                    }
                }
                self.editing_path = None;
                self.edit_value.clear();
            }
        }
    }
    
    fn get_node(&self, path: &[usize]) -> Option<&Node> {
        let mut node = &self.root;
        for i in path {
            node = node.children.get(*i)?;
        }
        Some(node)
    }
    
    fn get_node_mut(&mut self, path: &[usize]) -> Option<&mut Node> {
        let mut node = &mut self.root;
        for i in path {
            node = node.children.get_mut(*i)?;
        }
        Some(node)
    }

    fn view(&self) -> Element<'_, Message> {
        tree_view(&self.root, vec![], &self.editing_path, &self.edit_value).into()
    }
}

impl Default for TreeDemo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_demo_new() {
        let demo = TreeDemo::new();
        
        assert_eq!(demo.root.label, "Root");
        assert_eq!(demo.root.open, true);
        assert_eq!(demo.root.children.len(), 2);
        assert_eq!(demo.editing_path, None);
        assert_eq!(demo.edit_value, "");
    }

    #[test]
    fn test_tree_demo_default() {
        let demo = TreeDemo::default();
        
        assert_eq!(demo.root.label, "Root");
        assert_eq!(demo.editing_path, None);
    }

    #[test]
    fn test_toggle_node() {
        let mut demo = TreeDemo::new();
        let path = vec![0]; // First child
        
        // Get initial state
        let initial_state = demo.get_node(&path).unwrap().open;
        
        // Toggle the node
        demo.update(Message::Toggle(path.clone()));
        
        // Verify the state changed
        let new_state = demo.get_node(&path).unwrap().open;
        assert_eq!(new_state, !initial_state);
    }

    #[test]
    fn test_toggle_root() {
        let mut demo = TreeDemo::new();
        let path = vec![]; // Root node
        
        assert_eq!(demo.root.open, true);
        
        demo.update(Message::Toggle(path.clone()));
        assert_eq!(demo.root.open, false);
        
        demo.update(Message::Toggle(path));
        assert_eq!(demo.root.open, true);
    }

    #[test]
    fn test_start_edit() {
        let mut demo = TreeDemo::new();
        let path = vec![0];
        
        demo.update(Message::StartEdit(path.clone()));
        
        assert_eq!(demo.editing_path, Some(path));
        assert_eq!(demo.edit_value, "Branch 1");
    }

    #[test]
    fn test_edit_label() {
        let mut demo = TreeDemo::new();
        
        demo.update(Message::EditLabel("New Value".to_string()));
        
        assert_eq!(demo.edit_value, "New Value");
    }

    #[test]
    fn test_finish_edit() {
        let mut demo = TreeDemo::new();
        let path = vec![0];
        
        // Start editing
        demo.update(Message::StartEdit(path.clone()));
        assert_eq!(demo.edit_value, "Branch 1");
        
        // Change the value
        demo.update(Message::EditLabel("Updated Branch".to_string()));
        
        // Finish editing
        demo.update(Message::FinishEdit);
        
        // Verify the label was updated
        assert_eq!(demo.get_node(&path).unwrap().label, "Updated Branch");
        assert_eq!(demo.editing_path, None);
        assert_eq!(demo.edit_value, "");
    }

    #[test]
    fn test_finish_edit_without_start() {
        let mut demo = TreeDemo::new();
        
        // Try to finish edit without starting
        demo.update(Message::FinishEdit);
        
        // Should not panic and state should remain unchanged
        assert_eq!(demo.editing_path, None);
        assert_eq!(demo.edit_value, "");
    }

    #[test]
    fn test_get_node_root() {
        let demo = TreeDemo::new();
        
        let node = demo.get_node(&[]);
        assert!(node.is_some());
        assert_eq!(node.unwrap().label, "Root");
    }

    #[test]
    fn test_get_node_first_child() {
        let demo = TreeDemo::new();
        
        let node = demo.get_node(&[0]);
        assert!(node.is_some());
        assert_eq!(node.unwrap().label, "Branch 1");
    }

    #[test]
    fn test_get_node_nested() {
        let demo = TreeDemo::new();
        
        let node = demo.get_node(&[0, 0]);
        assert!(node.is_some());
        assert_eq!(node.unwrap().label, "Leaf 1.1");
    }

    #[test]
    fn test_get_node_invalid_path() {
        let demo = TreeDemo::new();
        
        // Path that doesn't exist
        let node = demo.get_node(&[99]);
        assert!(node.is_none());
    }

    #[test]
    fn test_get_node_mut() {
        let mut demo = TreeDemo::new();
        
        // Modify a node directly
        if let Some(node) = demo.get_node_mut(&[0]) {
            node.label = "Modified".to_string();
        }
        
        assert_eq!(demo.get_node(&[0]).unwrap().label, "Modified");
    }

    #[test]
    fn test_multiple_edits() {
        let mut demo = TreeDemo::new();
        
        // Edit first branch
        demo.update(Message::StartEdit(vec![0]));
        demo.update(Message::EditLabel("Branch A".to_string()));
        demo.update(Message::FinishEdit);
        
        // Edit second branch
        demo.update(Message::StartEdit(vec![1]));
        demo.update(Message::EditLabel("Branch B".to_string()));
        demo.update(Message::FinishEdit);
        
        assert_eq!(demo.get_node(&[0]).unwrap().label, "Branch A");
        assert_eq!(demo.get_node(&[1]).unwrap().label, "Branch B");
    }

    #[test]
    fn test_edit_deep_node() {
        let mut demo = TreeDemo::new();
        let path = vec![0, 1]; // Leaf 1.2
        
        demo.update(Message::StartEdit(path.clone()));
        demo.update(Message::EditLabel("Deep Leaf".to_string()));
        demo.update(Message::FinishEdit);
        
        assert_eq!(demo.get_node(&path).unwrap().label, "Deep Leaf");
    }
}
