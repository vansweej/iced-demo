use crate::{Column, Message, button, column, text};

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
pub fn tree_view(node: &Node, path: Vec<usize>) -> Column<'_, Message> {
    let indent = path.len() as f32 * 24.0;
    
    let icon = if node.children.is_empty() {
        "  "
    } else if node.open {
        "▼ "
    } else {
        "▶ "
    };
    
    let label = format!("{}{}", icon, node.label);
    
    // Create a button for the node that toggles on click
    let node_button = button(text(label))
        .on_press(Message::Toggle(path.clone()))
        .style(iced::widget::button::text)
        .padding(0);
    
    let node_row = column![node_button]
        .padding(iced::Padding::new(0.0).left(indent));
    
    let mut col = column![node_row];
    
    // Recursively render children if the node is open
    if node.open {
        for (i, child) in node.children.iter().enumerate() {
            let mut child_path = path.clone();
            child_path.push(i);
            col = col.push(tree_view(child, child_path));
        }
    }
    
    col
}
