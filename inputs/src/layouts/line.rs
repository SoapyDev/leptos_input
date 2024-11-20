use leptos::{component, view, Children, IntoView};

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Gap {
    None,
    Small,
    Medium,
    Large,
}

#[derive(Copy, Clone, PartialEq)]
pub enum DisplayStrategy {
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    Start,
    End,
}
#[component]
pub fn Line(
    #[prop(default = Direction::Row)] direction: Direction,
    #[prop(default = true)] wrap: bool,
    #[prop(default = Gap::Small)] gap: Gap,
    #[prop(default = DisplayStrategy::Start)] justify: DisplayStrategy,
    #[prop(default = DisplayStrategy::Start)] align: DisplayStrategy,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class:row = direction == Direction::Row
            class:column = direction == Direction::Column
            class:row-reverse = direction == Direction::RowReverse
            class:col-reverse = direction == Direction::ColumnReverse
            class:no-wrap = !wrap
            class:gap-small = gap == Gap::Small
            class:gap-medium = gap == Gap::Medium
            class:gap-large = gap == Gap::Large
            class:justify-center = justify == DisplayStrategy::Center
            class:justify-between = justify == DisplayStrategy::SpaceBetween
            class:justify-around = justify == DisplayStrategy::SpaceAround
            class:justify-evenly = justify == DisplayStrategy::SpaceEvenly
            class:justify-start = justify == DisplayStrategy::Start
            class:justify-end = justify == DisplayStrategy::End
            class:align-center = align == DisplayStrategy::Center
            class:align-between = align == DisplayStrategy::SpaceBetween
            class:align-around = align == DisplayStrategy::SpaceAround
            class:align-evenly = align == DisplayStrategy::SpaceEvenly
            class:align-start = align == DisplayStrategy::Start
            class:align-end = align == DisplayStrategy::End
        >
            {children()}
        </div>
    }
}
