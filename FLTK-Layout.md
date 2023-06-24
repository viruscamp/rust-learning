- 无
    类似 Group
- 相对定位 rust 才有，需要有大小 with_size
    above_of(&widget, padding): places the widget above the passed widget
    below_of(&widget, padding): places the widget below the passed widget
    right_of(&widget, padding): places the widget right of the passed widget
    left_of(&widget, padding): places the widget left of the passed widget
    center_of(&widget): places the widget at the center (both x and y axes) of the passed widget.
    center_of_parent(): places the widget at the center (both x and y axes) of the parent.
    center_x(&widget): places the widget at the center (x-axis) of the passed widget.
    center_y(&widget): places the widget at the center (y-axis) of the passed widget.
    size_of(&widget): constructs the widget with the same size of the passed widget.
    size_of_parent(): constructs the widget with the same size of its parent.
- Group
    子项依赖初始大小位置
    * group.make_resizable(false); 所有子项固定左上
    * group.resizable(group); 或 group.make_resizable(true); 所有子项按比例变化
    * group.resizable(child1); 确定的子项会变化 同行或同列会被带着变化
- Pack
    PackType::Vertical 子项宽度相同 高度不变
    PackType::Horizontal 子项高度相同 宽度不变
    子项可超出父项,侵占其他容器
    flex.set_spacing(8); 子项间隙
- Flex
    FlexType::Column 相当于 PackType::Vertical 子项宽度相同
    FlexType::Row 相当于 PackType::Horizontal 子项高度相同
    flex.set_size(&b2, 40); 的子项固定，其他子项平分剩余大小
    子项可超出父项,侵占其他容器
    flex.set_margin(4); 外围间隙
    flex.set_pad(8); 子项间隙
    应该可以完全替代 Pack
- Tile
    可拖动修改大小
    要保证子项紧贴，所以要用 right_of below_of 来设定位置
    如果子项间有空隙，导致左边/上边元素可调整大小，右边/下边元素只会被挤占缩小无法扩大
- VGrid HGrid rust才有
    内部为 横向Pack + 纵向Pack
    没有 span 同一子项占据多个格子
    VGrid 3x3
        5子项 3第一行 2第二行平分 第三行空
        7子项 3第一行 3第二行 2第三行平分
- fltk_grid::Grid
    // 5 rows, 5 columns
    grid.set_layout(5, 5); 
    // widget, row, col
    grid.insert(&mut button::Button::default().with_label("Click"), 0, 1); 
    // widget, row, col, row_span, col_span
    grid.insert_ext(&mut button::Button::default().with_label("Button 2"), 2, 1, 3, 1);
- fltk_archor::Anchor
    anchor 只针对顶层窗口，不是父容器
    最好保证同一行或列上，最多只有一个可变大小的子项，可以有零或多个固定大小子项
    * 两边 Left+Right 为跟随容器变化的元素
    * 左 Left 固定大小 靠左
    * 右 Right 固定大小 靠右
    * 无 目前相当于 Right