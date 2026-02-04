pub use iced::Element;
pub use iced::Theme;
pub use iced::widget::{Column, button, column, text};

mod tree;
use tree::{Node, tree_view};

fn main() -> iced::Result {
    iced::run(TreeDemo::update, TreeDemo::view)
}

#[derive(Debug, Clone)]
pub enum Message {
    Toggle(Vec<usize>),
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
