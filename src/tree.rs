use crate::{Column, Message, button, column, text, text_input};

/// Node state: open/closed, label, children
#[derive(Debug, Clone)]
pub struct Node {
    pub label: String,
    pub open: bool,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(label: &str, open: bool, children: Vec<Node>) -> Self {
        Self {
            label: label.into(),
            open,
            children,
        }
    }
}

/// Render a tree node and its children using standard widgets
pub fn tree_view<'a>(
    node: &'a Node,
    path: Vec<usize>,
    editing_path: &'a Option<Vec<usize>>,
    edit_value: &'a str,
) -> Column<'a, Message> {
    let indent = path.len() as f32 * 24.0;
    
    let icon = if node.children.is_empty() {
        "  "
    } else if node.open {
        "▼ "
    } else {
        "▶ "
    };
    
    let is_editing = editing_path.as_ref() == Some(&path);
    
    let node_widget = if is_editing {
        // Show text input when editing
        column![
            text_input(&format!("Edit {}", node.label), edit_value)
                .on_input(Message::EditLabel)
                .on_submit(Message::FinishEdit)
                .padding(2)
        ]
    } else {
        // Show clickable label when not editing        
        let toggle_button = button(text(format!("{}{}", icon, node.label)))
            .on_press(Message::Toggle(path.clone()))
            .style(iced::widget::button::text)
            .padding(0);
        
        const EDIT_BUTTON_LEFT_PADDING: f32 = 8.0;
        let edit_button = button(text("✏"))
            .on_press(Message::StartEdit(path.clone()))
            .style(iced::widget::button::text)
            .padding(iced::Padding::new(0.0).left(EDIT_BUTTON_LEFT_PADDING));
        
        column![
            iced::widget::row![toggle_button, edit_button]
                .spacing(4)
        ]
    };
    
    let node_row = node_widget
        .padding(iced::Padding::new(0.0).left(indent));
    
    let mut col = column![node_row];
    
    // Recursively render children if the node is open
    if node.open {
        for (i, child) in node.children.iter().enumerate() {
            let mut child_path = path.clone();
            child_path.push(i);
            col = col.push(tree_view(child, child_path, editing_path, edit_value));
        }
    }
    
    col
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_new() {
        let node = Node::new("Test Label", true, vec![]);
        
        assert_eq!(node.label, "Test Label");
        assert_eq!(node.open, true);
        assert_eq!(node.children.len(), 0);
    }

    #[test]
    fn test_node_new_with_children() {
        let child1 = Node::new("Child 1", false, vec![]);
        let child2 = Node::new("Child 2", false, vec![]);
        let parent = Node::new("Parent", true, vec![child1, child2]);
        
        assert_eq!(parent.label, "Parent");
        assert_eq!(parent.open, true);
        assert_eq!(parent.children.len(), 2);
        assert_eq!(parent.children[0].label, "Child 1");
        assert_eq!(parent.children[1].label, "Child 2");
    }

    #[test]
    fn test_node_nested_structure() {
        let leaf = Node::new("Leaf", false, vec![]);
        let branch = Node::new("Branch", false, vec![leaf]);
        let root = Node::new("Root", true, vec![branch]);
        
        assert_eq!(root.children[0].children[0].label, "Leaf");
    }
}
