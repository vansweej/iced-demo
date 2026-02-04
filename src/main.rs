pub use iced::Element;
pub use iced::Theme;
pub use iced::widget::{Column, button, column, text};

fn main() -> iced::Result {
    iced::run(TreeDemo::update, TreeDemo::view)
}

#[derive(Debug, Clone)]
pub enum Message {
    Toggle(Vec<usize>),
}

// Node state: open/closed, label, children
#[derive(Debug, Clone)]
pub struct Node {
    pub label: String,
    pub open: bool,
    pub children: Vec<Node>,
}

impl Node {
    fn new(label: &str, open: bool, children: Vec<Node>) -> Self {
        Self {
            label: label.into(),
            open,
            children,
        }
    }
}

pub struct TreeDemo {
    root: Node,
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
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Toggle(path) => {
                let mut node = &mut self.root;
                for i in path {
                    if let Some(child) = node.children.get_mut(i) {
                        node = child;
                    } else {
                        return;
                    }
                }
                node.open = !node.open;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        tree_view(&self.root, vec![]).into()
    }
}

impl Default for TreeDemo {
    fn default() -> Self {
        Self::new()
    }
}

/// Render a tree node and its children using standard widgets
fn tree_view(node: &Node, path: Vec<usize>) -> Column<'_, Message> {
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
