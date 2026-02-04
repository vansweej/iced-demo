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
            Message::StartEdit(path) => {
                // Find the node and start editing
                let node = self.get_node(&path);
                if let Some(node) = node {
                    self.edit_value = node.label.clone();
                    self.editing_path = Some(path);
                }
            }
            Message::EditLabel(value) => {
                self.edit_value = value;
            }
            Message::FinishEdit => {
                if let Some(path) = &self.editing_path {
                    let mut node = &mut self.root;
                    for i in path {
                        if let Some(child) = node.children.get_mut(*i) {
                            node = child;
                        } else {
                            self.editing_path = None;
                            return;
                        }
                    }
                    node.label = self.edit_value.clone();
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

    fn view(&self) -> Element<'_, Message> {
        tree_view(&self.root, vec![], &self.editing_path, &self.edit_value).into()
    }
}

impl Default for TreeDemo {
    fn default() -> Self {
        Self::new()
    }
}
