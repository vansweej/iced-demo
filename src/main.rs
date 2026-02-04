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
    AddChild(Vec<usize>),
    RemoveChild(Vec<usize>),
}

pub struct TreeDemo {
    roots: Vec<Node>,
    editing_path: Option<Vec<usize>>,
    edit_value: String,
}

impl TreeDemo {
    fn new() -> Self {
        Self {
            roots: vec![
                Node::new(
                    "Root 1",
                    true,
                    vec![
                        Node::new(
                            "Branch 1.1",
                            false,
                            vec![
                                Node::new("Leaf 1.1.1", false, vec![]),
                                Node::new("Leaf 1.1.2", false, vec![]),
                            ],
                        ),
                        Node::new(
                            "Branch 1.2",
                            false,
                            vec![Node::new("Leaf 1.2.1", false, vec![])],
                        ),
                    ],
                ),
                Node::new(
                    "Root 2",
                    true,
                    vec![
                        Node::new("Branch 2.1", false, vec![]),
                        Node::new("Branch 2.2", false, vec![]),
                    ],
                ),
            ],
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
            Message::AddChild(path) => {
                if let Some(node) = self.get_node_mut(&path) {
                    let new_child = Node::new("New Node", false, vec![]);
                    node.add_child(new_child);
                    // Open the parent to show the new child
                    node.open = true;
                }
            }
            Message::RemoveChild(path) => {
                // Path should have at least 2 elements: parent and child index
                if path.len() < 2 {
                    return;
                }
                
                let parent_path = &path[..path.len() - 1];
                let child_index = path[path.len() - 1];
                
                if let Some(parent) = self.get_node_mut(parent_path) {
                    parent.remove_child(child_index);
                }
            }
        }
    }
    
    /// Gets a reference to a node at the specified path.
    /// The first element of the path is the root index, subsequent elements navigate through children.
    /// Returns None if the path is empty or if any index is out of bounds.
    fn get_node(&self, path: &[usize]) -> Option<&Node> {
        if path.is_empty() {
            return None;
        }
        let mut node = self.roots.get(path[0])?;
        for i in &path[1..] {
            node = node.children.get(*i)?;
        }
        Some(node)
    }
    
    /// Gets a mutable reference to a node at the specified path.
    /// The first element of the path is the root index, subsequent elements navigate through children.
    /// Returns None if the path is empty or if any index is out of bounds.
    fn get_node_mut(&mut self, path: &[usize]) -> Option<&mut Node> {
        if path.is_empty() {
            return None;
        }
        let mut node = self.roots.get_mut(path[0])?;
        for i in &path[1..] {
            node = node.children.get_mut(*i)?;
        }
        Some(node)
    }

    fn view(&self) -> Element<'_, Message> {
        let mut col = column![];
        for (i, root) in self.roots.iter().enumerate() {
            col = col.push(tree_view(root, vec![i], &self.editing_path, &self.edit_value));
        }
        col.into()
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
        
        assert_eq!(demo.roots.len(), 2);
        assert_eq!(demo.roots[0].label, "Root 1");
        assert_eq!(demo.roots[0].open, true);
        assert_eq!(demo.roots[0].children.len(), 2);
        assert_eq!(demo.roots[1].label, "Root 2");
        assert_eq!(demo.editing_path, None);
        assert_eq!(demo.edit_value, "");
    }

    #[test]
    fn test_tree_demo_default() {
        let demo = TreeDemo::default();
        
        assert_eq!(demo.roots.len(), 2);
        assert_eq!(demo.roots[0].label, "Root 1");
        assert_eq!(demo.editing_path, None);
    }

    #[test]
    fn test_toggle_node() {
        let mut demo = TreeDemo::new();
        let path = vec![0, 0]; // First root's first child
        
        // Get initial state
        let initially_open = demo.get_node(&path).unwrap().open;
        
        // Toggle the node
        demo.update(Message::Toggle(path.clone()));
        
        // Verify the state changed
        let is_open_after_toggle = demo.get_node(&path).unwrap().open;
        assert_eq!(is_open_after_toggle, !initially_open);
    }

    #[test]
    fn test_toggle_root() {
        let mut demo = TreeDemo::new();
        let path = vec![0]; // First root node
        
        assert!(demo.roots[0].open);
        
        demo.update(Message::Toggle(path.clone()));
        assert!(!demo.roots[0].open);
        
        demo.update(Message::Toggle(path));
        assert!(demo.roots[0].open);
    }

    #[test]
    fn test_start_edit() {
        let mut demo = TreeDemo::new();
        let path = vec![0, 0]; // First root's first child
        
        demo.update(Message::StartEdit(path.clone()));
        
        assert_eq!(demo.editing_path, Some(path));
        assert_eq!(demo.edit_value, "Branch 1.1");
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
        let path = vec![0, 0]; // First root's first child
        
        // Start editing
        demo.update(Message::StartEdit(path.clone()));
        assert_eq!(demo.edit_value, "Branch 1.1");
        
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
        
        let node = demo.get_node(&[0]);
        assert!(node.is_some());
        assert_eq!(node.unwrap().label, "Root 1");
    }

    #[test]
    fn test_get_node_first_child() {
        let demo = TreeDemo::new();
        
        let node = demo.get_node(&[0, 0]);
        assert!(node.is_some());
        assert_eq!(node.unwrap().label, "Branch 1.1");
    }

    #[test]
    fn test_get_node_nested() {
        let demo = TreeDemo::new();
        
        let node = demo.get_node(&[0, 0, 0]);
        assert!(node.is_some());
        assert_eq!(node.unwrap().label, "Leaf 1.1.1");
    }

    #[test]
    fn test_get_node_invalid_path() {
        let demo = TreeDemo::new();
        
        // Path that doesn't exist
        let node = demo.get_node(&[99]);
        assert!(node.is_none());
        
        // Empty path should also return None
        let node = demo.get_node(&[]);
        assert!(node.is_none());
    }

    #[test]
    fn test_get_node_mut() {
        let mut demo = TreeDemo::new();
        
        // Modify a node directly
        if let Some(node) = demo.get_node_mut(&[0, 0]) {
            node.label = "Modified".to_string();
        }
        
        assert_eq!(demo.get_node(&[0, 0]).unwrap().label, "Modified");
    }

    #[test]
    fn test_multiple_edits() {
        let mut demo = TreeDemo::new();
        
        // Edit first root's first child
        demo.update(Message::StartEdit(vec![0, 0]));
        demo.update(Message::EditLabel("Branch A".to_string()));
        demo.update(Message::FinishEdit);
        
        // Edit second root's first child
        demo.update(Message::StartEdit(vec![1, 0]));
        demo.update(Message::EditLabel("Branch B".to_string()));
        demo.update(Message::FinishEdit);
        
        assert_eq!(demo.get_node(&[0, 0]).unwrap().label, "Branch A");
        assert_eq!(demo.get_node(&[1, 0]).unwrap().label, "Branch B");
    }

    #[test]
    fn test_edit_deep_node() {
        let mut demo = TreeDemo::new();
        let path = vec![0, 0, 1]; // First root, first child, second leaf
        
        demo.update(Message::StartEdit(path.clone()));
        demo.update(Message::EditLabel("Deep Leaf".to_string()));
        demo.update(Message::FinishEdit);
        
        assert_eq!(demo.get_node(&path).unwrap().label, "Deep Leaf");
    }

    #[test]
    fn test_multiple_roots() {
        let demo = TreeDemo::new();
        
        // Verify we have multiple roots
        assert_eq!(demo.roots.len(), 2);
        
        // Verify first root
        let root1 = demo.get_node(&[0]);
        assert!(root1.is_some());
        assert_eq!(root1.unwrap().label, "Root 1");
        
        // Verify second root
        let root2 = demo.get_node(&[1]);
        assert!(root2.is_some());
        assert_eq!(root2.unwrap().label, "Root 2");
    }

    #[test]
    fn test_toggle_different_roots() {
        let mut demo = TreeDemo::new();
        
        // Toggle first root
        let path1 = vec![0];
        assert!(demo.roots[0].open);
        demo.update(Message::Toggle(path1.clone()));
        assert!(!demo.roots[0].open);
        
        // Toggle second root
        let path2 = vec![1];
        assert!(demo.roots[1].open);
        demo.update(Message::Toggle(path2.clone()));
        assert!(!demo.roots[1].open);
        
        // Verify first root is still closed
        assert!(!demo.roots[0].open);
    }

    #[test]
    fn test_edit_nodes_in_different_roots() {
        let mut demo = TreeDemo::new();
        
        // Edit a node in the first root
        demo.update(Message::StartEdit(vec![0, 0]));
        demo.update(Message::EditLabel("Modified Root 1 Branch".to_string()));
        demo.update(Message::FinishEdit);
        
        // Edit a node in the second root
        demo.update(Message::StartEdit(vec![1, 1]));
        demo.update(Message::EditLabel("Modified Root 2 Branch".to_string()));
        demo.update(Message::FinishEdit);
        
        // Verify both edits were applied
        assert_eq!(demo.get_node(&[0, 0]).unwrap().label, "Modified Root 1 Branch");
        assert_eq!(demo.get_node(&[1, 1]).unwrap().label, "Modified Root 2 Branch");
    }

    #[test]
    fn test_get_node_second_root() {
        let demo = TreeDemo::new();
        
        // Get second root
        let root2 = demo.get_node(&[1]);
        assert!(root2.is_some());
        assert_eq!(root2.unwrap().label, "Root 2");
        
        // Get child of second root
        let child = demo.get_node(&[1, 0]);
        assert!(child.is_some());
        assert_eq!(child.unwrap().label, "Branch 2.1");
    }

    #[test]
    fn test_add_child_to_root() {
        let mut demo = TreeDemo::new();
        let path = vec![0]; // First root
        
        let initial_children = demo.get_node(&path).unwrap().children.len();
        
        demo.update(Message::AddChild(path.clone()));
        
        let node = demo.get_node(&path).unwrap();
        assert_eq!(node.children.len(), initial_children + 1);
        assert_eq!(node.children[initial_children].label, "New Node");
        // Verify the parent was opened
        assert!(node.open);
    }

    #[test]
    fn test_add_child_to_branch() {
        let mut demo = TreeDemo::new();
        let path = vec![0, 0]; // First root's first child
        
        let initial_children = demo.get_node(&path).unwrap().children.len();
        
        demo.update(Message::AddChild(path.clone()));
        
        let node = demo.get_node(&path).unwrap();
        assert_eq!(node.children.len(), initial_children + 1);
        assert_eq!(node.children[initial_children].label, "New Node");
        assert!(node.open);
    }

    #[test]
    fn test_add_multiple_children() {
        let mut demo = TreeDemo::new();
        let path = vec![1, 0]; // Second root's first child
        
        let initial_children = demo.get_node(&path).unwrap().children.len();
        
        demo.update(Message::AddChild(path.clone()));
        demo.update(Message::AddChild(path.clone()));
        demo.update(Message::AddChild(path.clone()));
        
        let node = demo.get_node(&path).unwrap();
        assert_eq!(node.children.len(), initial_children + 3);
    }

    #[test]
    fn test_add_child_to_leaf() {
        let mut demo = TreeDemo::new();
        let path = vec![0, 0, 0]; // A leaf node
        
        // Verify it starts with no children
        assert_eq!(demo.get_node(&path).unwrap().children.len(), 0);
        
        demo.update(Message::AddChild(path.clone()));
        
        let node = demo.get_node(&path).unwrap();
        assert_eq!(node.children.len(), 1);
        assert_eq!(node.children[0].label, "New Node");
    }

    #[test]
    fn test_remove_child() {
        let mut demo = TreeDemo::new();
        // First root has 2 children initially
        let parent_path = vec![0];
        let initial_count = demo.get_node(&parent_path).unwrap().children.len();
        
        // Remove the first child
        demo.update(Message::RemoveChild(vec![0, 0]));
        
        let node = demo.get_node(&parent_path).unwrap();
        assert_eq!(node.children.len(), initial_count - 1);
    }

    #[test]
    fn test_remove_child_from_branch() {
        let mut demo = TreeDemo::new();
        let path = vec![0, 0]; // First root's first child
        let initial_count = demo.get_node(&path).unwrap().children.len();
        
        // Remove its first child
        demo.update(Message::RemoveChild(vec![0, 0, 0]));
        
        let node = demo.get_node(&path).unwrap();
        assert_eq!(node.children.len(), initial_count - 1);
    }

    #[test]
    fn test_remove_nonexistent_child() {
        let mut demo = TreeDemo::new();
        let parent_path = vec![0];
        let initial_count = demo.get_node(&parent_path).unwrap().children.len();
        
        // Try to remove a child with an invalid index
        demo.update(Message::RemoveChild(vec![0, 99]));
        
        // Count should remain the same
        let node = demo.get_node(&parent_path).unwrap();
        assert_eq!(node.children.len(), initial_count);
    }

    #[test]
    fn test_remove_child_with_short_path() {
        let mut demo = TreeDemo::new();
        let initial_roots = demo.roots.len();
        
        // Try to remove with a path that's too short (just root)
        demo.update(Message::RemoveChild(vec![0]));
        
        // Should not remove root nodes
        assert_eq!(demo.roots.len(), initial_roots);
    }

    #[test]
    fn test_add_and_remove_child_sequence() {
        let mut demo = TreeDemo::new();
        let path = vec![1, 0];
        
        let initial_count = demo.get_node(&path).unwrap().children.len();
        
        // Add a child
        demo.update(Message::AddChild(path.clone()));
        assert_eq!(demo.get_node(&path).unwrap().children.len(), initial_count + 1);
        
        // Remove the newly added child
        demo.update(Message::RemoveChild(vec![1, 0, initial_count]));
        assert_eq!(demo.get_node(&path).unwrap().children.len(), initial_count);
    }

    #[test]
    fn test_remove_all_children() {
        let mut demo = TreeDemo::new();
        let path = vec![0, 0]; // First root's first child (has 2 children initially)
        
        let child_count = demo.get_node(&path).unwrap().children.len();
        
        // Remove all children
        for _ in 0..child_count {
            demo.update(Message::RemoveChild(vec![0, 0, 0]));
        }
        
        let node = demo.get_node(&path).unwrap();
        assert_eq!(node.children.len(), 0);
    }

    #[test]
    fn test_add_child_updates_tree_structure() {
        let mut demo = TreeDemo::new();
        let path = vec![0, 1]; // First root's second child
        
        let initial_count = demo.get_node(&path).unwrap().children.len();
        
        demo.update(Message::AddChild(path.clone()));
        
        // Verify we can access the new child (it's added at the end)
        let new_child_path = vec![0, 1, initial_count];
        let new_child = demo.get_node(&new_child_path);
        assert!(new_child.is_some());
        assert_eq!(new_child.unwrap().label, "New Node");
    }

    #[test]
    fn test_add_child_to_closed_node_opens_it() {
        let mut demo = TreeDemo::new();
        let path = vec![0, 0]; // First root's first child
        
        // Close the node first
        if demo.get_node(&path).unwrap().open {
            demo.update(Message::Toggle(path.clone()));
        }
        assert!(!demo.get_node(&path).unwrap().open);
        
        // Add a child
        demo.update(Message::AddChild(path.clone()));
        
        // Verify the node was opened
        assert!(demo.get_node(&path).unwrap().open);
    }

    #[test]
    fn test_edit_newly_added_child() {
        let mut demo = TreeDemo::new();
        let parent_path = vec![0, 0];
        
        // Add a new child
        demo.update(Message::AddChild(parent_path.clone()));
        
        let child_count = demo.get_node(&parent_path).unwrap().children.len();
        let new_child_path = vec![0, 0, child_count - 1];
        
        // Edit the new child
        demo.update(Message::StartEdit(new_child_path.clone()));
        demo.update(Message::EditLabel("Edited New Node".to_string()));
        demo.update(Message::FinishEdit);
        
        assert_eq!(demo.get_node(&new_child_path).unwrap().label, "Edited New Node");
    }
}
