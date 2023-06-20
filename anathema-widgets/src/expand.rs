use anathema_render::{Size, Style};

use super::{Axis, LocalPos, NodeId, PaintCtx, PositionCtx, Widget, WithSize};
use crate::contexts::LayoutCtx;
use crate::error::Result;
use crate::layout::single::Single;
use crate::layout::Layouts;
use crate::lookup::WidgetFactory;
use crate::values::ValuesAttributes;
use crate::{fields, AnyWidget, TextPath, WidgetContainer};

const DEFAULT_FACTOR: usize = 1;

/// The `Expand` widget will fill up all remaining space inside a widget in both horizontal and
/// vertical direction.
///
/// To only expand in one direction, set the `direction` of the `Expand` widget.
///
/// A [`Direction`] can be set when creating a new widget
/// ```
/// use anathema_widgets::{Expand, Axis};
/// let horizontal = Expand::new(2, Axis::Horizontal, None);
/// let vertical = Expand::new(5, Axis::Vertical, None);
/// ```
///
/// The total available space is divided between the `Expand` widgets and multiplied by the
/// widgets `factor`.
///
/// ```ignore
/// # use anathema_widgets::{NodeId, HStack, Constraints, Widget};
/// use anathema_widgets::Expand;
/// let left = Expand::new(2, None, None);
/// let right = Expand::new(3, None, None);
/// # let left = left.into_container(NodeId::anon());
/// # let right = right.into_container(NodeId::anon());
/// # let left_id = left.id();
/// # let right_id = right.id();
///
/// // ... layout
///
/// # let mut root = HStack::new(10, 5);
/// # root.children.push(left);
/// # root.children.push(right);
/// # let mut root = root.into_container(NodeId::anon());
/// # root.layout(Constraints::new(10, 5), false);
/// # {
/// // The left `Expand` widget has a factor of two.
/// // The right `Expand` widget has a factor of three.
/// // Given the total width of ten, and a total factor count of five,
/// // This means the left widget has a width of four: `10 / 5 * 2`
/// // and the right widget has a width of six: `10 / 5 * 3`
///
/// let left = root.by_id(&left_id).unwrap();
/// assert_eq!(left.size().width, 4);
/// # }
///
/// let right = root.by_id(&right_id).unwrap();
/// assert_eq!(right.size().width, 6);
/// ```
#[derive(Debug)]
pub struct Expand {
    /// The direction to expand in.
    pub axis: Option<Axis>,
    /// Fill the space by repeating the characters.
    pub fill: String,
    /// The style of the expansion.
    pub style: Style,
    pub(crate) factor: usize,
}

impl Expand {
    /// Widget name.
    pub const KIND: &'static str = "Expand";

    /// Create a new instance of an `Expand` widget.
    pub fn new(factor: impl Into<Option<usize>>, direction: impl Into<Option<Axis>>, fill: impl Into<Option<String>>) -> Self {
        let factor = factor.into();
        let axis = direction.into();

        Self {
            factor: factor.unwrap_or(DEFAULT_FACTOR),
            axis,
            fill: fill.into().unwrap_or(String::new()),
            style: Style::new(),
        }
    }
}

impl Widget for Expand {
    fn kind(&self) -> &'static str {
        Self::KIND
    }

    fn layout(&mut self, mut ctx: LayoutCtx<'_, '_, '_>) -> Result<Size> {
        let mut size = Layouts::new(Single, &mut ctx).layout()?.size()?;

        match self.axis {
            Some(Axis::Horizontal) => size.width = ctx.constraints.max_width,
            Some(Axis::Vertical) => size.height = ctx.constraints.max_height,
            None => {
                size.width = ctx.constraints.max_width;
                size.height = ctx.constraints.max_height;
            }
        }

        Ok(size)
    }

    fn position<'gen, 'ctx>(
        &mut self,
        mut ctx: PositionCtx,
        children: &mut [WidgetContainer<'gen>],
    ) {
        if let Some(c) = children.first_mut() {
            c.position(ctx.pos)
        }
    }

    fn paint<'gen, 'ctx>(
        &mut self,
        mut ctx: PaintCtx<'_, WithSize>,
        children: &mut [WidgetContainer<'gen>],
    ) {
        if !self.fill.is_empty() {
            for y in 0..ctx.local_size.height {
                let mut used_width = 0;
                loop {
                    let pos = LocalPos::new(used_width, y);
                    let Some(p) = ctx.print(&self.fill, self.style, pos) else { break };
                    used_width += p.x - used_width;
                }
            }
        }

        if let Some(child) = children.first_mut() {
            let ctx = ctx.sub_context(None);
            child.paint(ctx);
        }
    }

    //     // fn update(&mut self, ctx: UpdateCtx) {
    //     //     ctx.attributes.update_style(&mut self.style);
    //     //     for (k, _) in &ctx.attributes {
    //     //         match k.as_str() {
    //     //             fields::DIRECTION => self.direction = ctx.attributes.direction(),
    //     //             fields::FACTOR => self.factor = ctx.attributes.factor().unwrap_or(DEFAULT_FACTOR),
    //     //             _ => {}
    //     //         }
    //     //     }
    //     // }
}

pub(crate) struct ExpandFactory;

impl WidgetFactory for ExpandFactory {
    fn make(
        &self,
        values: ValuesAttributes<'_, '_>,
        text: Option<&TextPath>,
    ) -> Result<Box<dyn AnyWidget>> {
        let axis = values.axis();
        let factor = values.factor();
        let fill = values.fill().map(|s| s.to_string());
        Ok(Box::new(Expand::new(factor, axis, fill)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::template::{template, template_text, Template};
    use crate::testing::{test_widget, FakeTerm};

    // fn expand_border(dir: Option<Direction>) -> WidgetContainer {
    //     let mut parent = Border::thick(None, None).into_container(NodeId::anon());
    //     let expand = Expand::new(None, dir).into_container(NodeId::anon());
    //     parent.add_child(expand);
    //     parent.layout(Constraints::new(10, 10), false);
    //     parent
    // }

    fn test_expand(widget: impl Widget + 'static, children: &[Template], expected: FakeTerm) {
        let widget = WidgetContainer::new(Box::new(widget), children);
        test_widget(widget, expected);
    }

    // #[test]
    // fn expand_border() {
    //     let border = Border::thin(None, None);
    //     let expand = vec![template("expand", vec![])];
    //     test_hstack(
    //         border,
    //         &expand,
    //         FakeTerm::from_str(
    //         r#"
    //         ╔═] Fake term [═╗
    //         ║┌─┐┌─┐┌─┐      ║
    //         ║│0││1││2│      ║
    //         ║└─┘└─┘└─┘      ║
    //         ║               ║
    //         ║               ║
    //         ╚═══════════════╝
    //         "#,
    //         )
    //     );
    // }

    // #[test]
    // fn expand_inner() {
    //     let parent = expand_border(None);
    //     assert_eq!(Size::new(10, 10), parent.size);
    // }

    // #[test]
    // fn expand_inner_horz() {
    //     let parent = expand_border(Some(Direction::Horizontal));
    //     assert_eq!(Size::new(10, 2), parent.size);
    // }

    // #[test]
    // fn expand_inner_vert() {
    //     let parent = expand_border(Some(Direction::Vertical));
    //     assert_eq!(Size::new(2, 10), parent.size);
    // }

    // #[test]
    // fn style_changes_via_attributes() {
    //     let mut expand = Expand::new(None, None).into_container(NodeId::anon());
    //     expand.update(Attributes::new("italic", true));
    //     assert!(expand.to_mut::<Expand>().style.attributes.contains(anathema_render::Attributes::ITALIC));
    // }

    // #[test]
    // fn fill() {
    //     let mut expand = Expand::new(None, None);
    //     expand.fill = "hello".into();
    //     let expand = expand.into_container(NodeId::anon());

    //     let expected = r#"
    //         ┌───────┐
    //         │hellohe│
    //         │hellohe│
    //         └───────┘
    //     "#;
    //     test_expand(expand, expected);
    // }

    // #[test]
    // fn padding() {
    //     let expand = Expand::new(None, None);
    //     let mut expand = expand.into_container(NodeId::anon());
    //     expand.padding = Padding::new(1);
    //     expand.add_child(Text::with_text("xyz").into_container(NodeId::anon()));
    //     let expected = r#"
    //         ┌───────┐
    //         │       │
    //         │ xyz   │
    //         │       │
    //         └───────┘
    //     "#;
    //     test_expand(expand, expected);
    // }
}
