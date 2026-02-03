pub use iced::Element;
pub use iced::Settings;
pub use iced::Theme;
pub use iced::advanced::Widget;
pub use iced::advanced::widget::Tree;
pub use iced::advanced::widget::tree::State;
pub use iced::advanced::widget::tree::Tag;
pub use iced::application::Application;
pub use iced::command::Command;
pub use iced::executor;
pub use iced::widget::{Column, button, column, text};

fn main() -> iced::Result {
    TreeDemo::run(Settings::default())
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

impl Application for TreeDemo {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
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
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "iced::advanced::widget::Tree Demo".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Toggle(path) => {
                let mut node = &mut self.root;
                for i in path {
                    if let Some(child) = node.children.get_mut(i) {
                        node = child;
                    } else {
                        return Command::none();
                    }
                }
                node.open = !node.open;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        TreeWidget::new(&self.root, vec![]).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

/// Widget state for recursive tree nodes (required for iced's Tree pattern)
pub struct TreeWidget<'a> {
    node: &'a Node,
    path: Vec<usize>,
}

impl<'a> TreeWidget<'a> {
    pub fn new(node: &'a Node, path: Vec<usize>) -> Self {
        Self { node, path }
    }
}

impl<'a, R> Widget<Message, Theme, R> for TreeWidget<'a>
where
    R: iced::advanced::Renderer + iced::advanced::text::Renderer,
{
    fn tag(&self) -> Tag {
        Tag::of::<()>()
    }

    fn state(&self) -> State {
        State::new(())
    }

    fn children(&self) -> Vec<Tree> {
        vec![]
    }

    fn diff(&self, _tree: &mut Tree) {
        // No-op for stateless widget
    }

    fn size(&self) -> iced::Size<iced::Length> {
        iced::Size {
            width: iced::Length::Shrink,
            height: iced::Length::Shrink,
        }
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &R,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        let _indent = self.path.len() as f32 * 24.0;
        let row_height = 24.0;

        let mut total_height = row_height;

        if self.node.open {
            for child in &self.node.children {
                // Recursively calculate height
                total_height += calculate_height(child) * row_height;
            }
        }

        iced::advanced::layout::Node::new(iced::Size::new(limits.max().width, total_height))
    }

    fn on_event(
        &mut self,
        _state: &mut Tree,
        event: iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        _renderer: &R,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &iced::Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        use iced::Event::Mouse;
        use iced::advanced::graphics::core::event::Status;
        use iced::mouse::{Button, Event as MouseEvent};

        let bounds = layout.bounds();
        let row_bounds = iced::Rectangle {
            x: bounds.x,
            y: bounds.y,
            width: bounds.width,
            height: 24.0,
        };

        if let Mouse(MouseEvent::ButtonPressed(Button::Left)) = event
            && let Some(pos) = cursor.position()
            && row_bounds.contains(pos)
        {
            shell.publish(Message::Toggle(self.path.clone()));
            return Status::Captured;
        }
        Status::Ignored
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut R,
        _theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        use iced::{Color, Pixels, alignment};

        let bounds = layout.bounds();
        let indent = self.path.len() as f32 * 24.0;

        let icon = if self.node.children.is_empty() {
            "  "
        } else if self.node.open {
            "▼ "
        } else {
            "▶ "
        };

        let text = format!("{}{}", icon, self.node.label);

        renderer.fill_text(
            iced::advanced::text::Text {
                content: &text,
                bounds: iced::Size::new(bounds.width - indent, 24.0),
                size: Pixels(16.0),
                line_height: iced::widget::text::LineHeight::default(),
                font: renderer.default_font(),
                horizontal_alignment: alignment::Horizontal::Left,
                vertical_alignment: alignment::Vertical::Top,
                shaping: iced::advanced::text::Shaping::Basic,
                //                wrapping: iced::advanced::text::Wrapping::default(),
            },
            iced::Point::new(bounds.x + indent, bounds.y),
            Color::WHITE,
            bounds,
        );

        // Recursively draw children
        if self.node.open {
            let mut y_offset = 24.0;
            for (i, child) in self.node.children.iter().enumerate() {
                let mut child_path = self.path.clone();
                child_path.push(i);
                let child_widget = TreeWidget::new(child, child_path);

                let child_bounds = iced::Rectangle {
                    x: bounds.x,
                    y: bounds.y + y_offset,
                    width: bounds.width,
                    height: bounds.height - y_offset,
                };

                let child_layout_node = iced::advanced::layout::Node::new(
                    child_bounds.size(),
                );
                let child_layout = iced::advanced::Layout::new(&child_layout_node);

                child_widget.draw(
                    _tree,
                    renderer,
                    _theme,
                    _style,
                    child_layout,
                    _cursor,
                    _viewport,
                );

                y_offset += 24.0 * (1 + count_open_children(child)) as f32;
            }
        }
    }
}

fn count_open_children(node: &Node) -> usize {
    if !node.open {
        0
    } else {
        node.children
            .iter()
            .map(|c| 1 + count_open_children(c))
            .sum()
    }
}

fn calculate_height(node: &Node) -> f32 {
    if node.open {
        1.0 + node.children.iter().map(calculate_height).sum::<f32>()
    } else {
        1.0
    }
}

impl<'a> From<TreeWidget<'a>> for Element<'a, Message> {
    fn from(widget: TreeWidget<'a>) -> Self {
        Element::new(widget)
    }
}
