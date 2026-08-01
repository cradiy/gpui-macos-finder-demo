use std::time::Duration;

use gpui::{
    App, AppContext, Bounds, Div, FontWeight, MouseButton, Render, Stateful, Window, WindowBounds,
    WindowOptions, div, linear_color_stop, linear_gradient, point, prelude::*, px, rgb, rgba, size,
    svg,
};
use gpui_effects::{GlassMaterial, GlassPanel};
use gpui_platform::application;
use uic::assets::{LucideAssets, LucideIcons};

#[derive(Clone, Copy, PartialEq, Eq)]
enum ViewMode {
    Grid,
    List,
}

struct Macos26Sidebar {
    view_mode: ViewMode,
    context_menu_position: Option<gpui::Point<gpui::Pixels>>,
}

impl Macos26Sidebar {
    fn traffic_light(color: u32) -> impl IntoElement {
        div()
            .size(px(13.))
            .rounded_full()
            .bg(rgb(color))
            .border_1()
            .border_color(rgba(0x00000033))
    }

    fn section(label: &'static str) -> impl IntoElement {
        div()
            .h(px(32.))
            .px(px(22.))
            .pb(px(5.))
            .flex()
            .items_end()
            .text_sm()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(rgba(0xa39dab9c))
            .child(label)
    }

    fn item(
        id: &'static str,
        label: &'static str,
        icon: LucideIcons,
        selected: bool,
    ) -> impl IntoElement {
        div()
            .id(id)
            .h(px(29.))
            .mx(px(12.))
            .px(px(10.))
            .flex()
            .items_center()
            .gap(px(9.))
            .rounded_lg()
            .cursor_pointer()
            .text_sm()
            .font_weight(if selected {
                FontWeight::SEMIBOLD
            } else {
                FontWeight::MEDIUM
            })
            .text_color(rgba(0xf3f1f7ed))
            .when(selected, |row| row.bg(rgba(0x3a39488f)))
            .when(!selected, |row| {
                row.hover(|style| style.bg(rgba(0xffffff14)))
            })
            .child(svg().path(icon).size(px(17.)).text_color(rgba(0xf5f3f9e8)))
            .child(label)
    }

    fn tag(id: &'static str, label: &'static str, color: u32, outlined: bool) -> impl IntoElement {
        div()
            .id(id)
            .h(px(29.))
            .mx(px(12.))
            .px(px(12.))
            .flex()
            .items_center()
            .gap(px(11.))
            .rounded_lg()
            .cursor_pointer()
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .text_color(rgba(0xf3f1f7e8))
            .hover(|style| style.bg(rgba(0xffffff14)))
            .child(
                div()
                    .size(px(12.))
                    .rounded_full()
                    .when(outlined, |dot| {
                        dot.border_2().border_color(rgba(0xf5f3f9db))
                    })
                    .when(!outlined, |dot| dot.bg(rgb(color))),
            )
            .child(label)
    }

    fn toolbar_button(id: &'static str, icon: LucideIcons) -> impl IntoElement {
        div()
            .id(id)
            .size(px(36.))
            .flex()
            .items_center()
            .justify_center()
            .rounded_3xl()
            .cursor_pointer()
            .text_color(rgba(0xe6e5eacf))
            .hover(|style| style.bg(rgba(0xffffff14)))
            .child(svg().path(icon).size_5().text_color(rgba(0xe6e5eacf)))
    }

    fn view_toolbar_button(
        id: &'static str,
        icon: LucideIcons,
        mode: ViewMode,
        active: bool,
        cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        div()
            .id(id)
            .size(px(36.))
            .flex()
            .items_center()
            .justify_center()
            .rounded_2xl()
            .cursor_pointer()
            .when(active, |button| {
                button
                    .bg(rgba(0x55566675))
                    .border_1()
                    .border_color(rgba(0xffffff12))
            })
            .when(!active, |button| {
                button.hover(|style| style.bg(rgba(0xffffff14)))
            })
            .on_click(cx.listener(move |this, _, _, cx| {
                this.view_mode = mode;
                cx.notify();
            }))
            .child(svg().path(icon).size_5().text_color(rgba(0xf1f0f4e8)))
    }

    fn toolbar_group(id: &'static str, buttons: Vec<gpui::AnyElement>) -> Stateful<Div> {
        div()
            .id(id)
            .h(px(40.))
            .px(px(3.))
            .flex()
            .items_center()
            .rounded_3xl()
            .bg(rgba(0x12131a15))
            .border_1()
            .border_color(rgba(0xffffff16))
            .shadow_sm()
            .children(buttons)
    }

    fn folder_icon(tint: u32, glyph: Option<LucideIcons>) -> impl IntoElement {
        let red = tint == 0xef334c;
        let tab_color = if red { 0xff435c } else { 0x55caed };
        let back_color = if red { 0xc91e37 } else { 0x168bbf };
        let face_top = if red { 0xff425a } else { 0x62d5f2 };
        let face_bottom = if red { 0xe51f3c } else { 0x269fd5 };
        let edge_color = if red {
            rgba(0xff8796a8)
        } else {
            rgba(0xa1e9f8b8)
        };

        div()
            .relative()
            .w(px(76.))
            .h(px(60.))
            .child(
                div()
                    .absolute()
                    .left(px(5.))
                    .top_0()
                    .w(px(32.))
                    .h(px(15.))
                    .rounded_tl_lg()
                    .rounded_tr_md()
                    .bg(rgb(tab_color)),
            )
            .child(
                div()
                    .absolute()
                    .left_0()
                    .right_0()
                    .top(px(9.))
                    .bottom(px(3.))
                    .rounded_lg()
                    .bg(rgb(back_color))
                    .border_1()
                    .border_color(rgba(0x00000028)),
            )
            .child(
                div()
                    .absolute()
                    .left_0()
                    .right_0()
                    .top(px(14.))
                    .bottom_0()
                    .rounded(px(9.))
                    .bg(linear_gradient(
                        180.,
                        linear_color_stop(rgb(face_top), 0.),
                        linear_color_stop(rgb(face_bottom), 1.),
                    ))
                    .border_1()
                    .border_color(edge_color)
                    .shadow(vec![
                        gpui::BoxShadow::new(px(0.), px(4.), rgba(0x00000052).into())
                            .blur_radius(px(7.))
                            .spread_radius(px(-3.)),
                    ]),
            )
            .child(
                div()
                    .absolute()
                    .left(px(6.))
                    .right(px(6.))
                    .top(px(15.))
                    .h(px(1.))
                    .bg(rgba(0xffffff61)),
            )
            .when_some(glyph, |folder, glyph| {
                folder.child(
                    svg()
                        .absolute()
                        .left(px(27.))
                        .top(px(27.))
                        .size(px(21.))
                        .path(glyph)
                        .text_color(if red {
                            rgba(0x8e10286e)
                        } else {
                            rgba(0x0c719c82)
                        }),
                )
            })
    }

    fn folder_tile(
        label: &'static str,
        count: &'static str,
        tint: u32,
        glyph: Option<LucideIcons>,
    ) -> impl IntoElement {
        div()
            .w(px(122.))
            .pt(px(5.))
            .flex()
            .flex_col()
            .items_center()
            .gap(px(6.))
            .rounded_xl()
            .child(Self::folder_icon(tint, glyph))
            .child(
                div()
                    .w_full()
                    .px(px(3.))
                    .overflow_hidden()
                    .text_center()
                    .text_size(px(13.))
                    .line_clamp(2)
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(0xf2f1f5e8))
                    .child(label),
            )
            .child(
                div()
                    .h(px(18.))
                    .flex()
                    .items_center()
                    .text_xs()
                    .text_color(rgba(0x30a9e6c2))
                    .child(count),
            )
    }

    fn grid_content() -> impl IntoElement {
        let folders = [
            ("AndroidStudioProjects", "5 items", 0x52c8ed, None),
            (
                "Applications",
                "1 item",
                0x52c8ed,
                Some(LucideIcons::AppWindow),
            ),
            ("CloudDrive", "5 items", 0x52c8ed, None),
            ("Code", "6 items", 0x52c8ed, None),
            ("Desktop", "No items", 0x52c8ed, Some(LucideIcons::Monitor)),
            (
                "Documents",
                "9 items",
                0x52c8ed,
                Some(LucideIcons::FileText),
            ),
            (
                "Downloads",
                "14 items",
                0x52c8ed,
                Some(LucideIcons::CircleArrowDown),
            ),
            ("Dropbox", "4 items", 0x52c8ed, Some(LucideIcons::Cloud)),
            ("Github", "4 items", 0x52c8ed, None),
            ("Google Drive", "5 items", 0x52c8ed, None),
            ("Modal", "5 items", 0x52c8ed, None),
            ("Movies", "7 items", 0x52c8ed, Some(LucideIcons::Film)),
            ("Music", "5 items", 0x52c8ed, Some(LucideIcons::Music)),
            ("Pictures", "10 items", 0x52c8ed, Some(LucideIcons::Image)),
            ("Public", "1 item", 0x52c8ed, None),
            ("Rust", "7 items", 0xef334c, None),
        ];

        div()
            .flex_1()
            .min_h_0()
            .overflow_hidden()
            .px(px(22.))
            .py(px(15.))
            .flex()
            .flex_wrap()
            .content_start()
            .items_start()
            .gap_x(px(15.))
            .gap_y(px(30.))
            .children(
                folders.into_iter().map(|(label, count, tint, glyph)| {
                    Self::folder_tile(label, count, tint, glyph)
                }),
            )
    }

    fn mini_folder(red: bool, alias: bool) -> impl IntoElement {
        let tab = if red { 0xff425a } else { 0x58cef0 };
        let back = if red { 0xc91e37 } else { 0x168dbf };
        let face = if red { 0xee2945 } else { 0x35addb };

        div()
            .relative()
            .w(px(16.))
            .h(px(14.))
            .flex_none()
            .child(
                div()
                    .absolute()
                    .left(px(1.))
                    .top_0()
                    .w(px(7.))
                    .h(px(4.))
                    .rounded_tl_sm()
                    .rounded_tr_sm()
                    .bg(rgb(tab)),
            )
            .child(
                div()
                    .absolute()
                    .left_0()
                    .right_0()
                    .top(px(3.))
                    .bottom(px(1.))
                    .rounded_sm()
                    .bg(rgb(back)),
            )
            .child(
                div()
                    .absolute()
                    .left_0()
                    .right_0()
                    .top(px(5.))
                    .bottom_0()
                    .rounded_sm()
                    .bg(rgb(face))
                    .border_1()
                    .border_color(rgba(0xb8edf885)),
            )
            .when(alias, |folder| {
                folder.child(
                    div()
                        .absolute()
                        .left(px(-2.))
                        .bottom(px(-2.))
                        .size(px(7.))
                        .rounded_full()
                        .bg(rgba(0xf2f2f2f2))
                        .child(
                            svg()
                                .absolute()
                                .inset(px(1.))
                                .path(LucideIcons::ArrowUpRight)
                                .text_color(rgba(0x252630e8)),
                        ),
                )
            })
    }

    fn list_header() -> impl IntoElement {
        div()
            .h(px(27.))
            .px(px(13.))
            .flex()
            .items_center()
            .border_b_1()
            .border_color(rgba(0xffffff17))
            .text_size(px(11.))
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(rgba(0xd6d4dbbf))
            .child(div().flex_1().min_w_0().child("Name"))
            .child(div().w(px(215.)).child("Date Modified"))
            .child(div().w(px(85.)).child("Size"))
            .child(div().w(px(90.)).child("Kind"))
    }

    #[allow(clippy::too_many_arguments)]
    fn list_row(
        label: &'static str,
        modified: &'static str,
        size: &'static str,
        kind: &'static str,
        index: usize,
        red: bool,
        alias: bool,
        tagged: bool,
        selected: bool,
    ) -> impl IntoElement {
        div()
            .h(px(26.))
            .px(px(13.))
            .flex()
            .items_center()
            .text_size(px(11.5))
            .text_color(rgba(0xe8e6ebdf))
            .when(index % 2 == 1, |row| row.bg(rgba(0xffffff08)))
            .when(selected, |row| row.bg(rgba(0x4a4b586e)))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .pr(px(8.))
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .min_w_0()
                            .flex()
                            .items_center()
                            .gap(px(7.))
                            .child(Self::mini_folder(red, alias))
                            .child(div().truncate().child(label)),
                    )
                    .when(tagged, |cell| {
                        cell.child(div().size(px(8.)).rounded_full().bg(rgb(0xff453a)))
                    }),
            )
            .child(
                div()
                    .w(px(215.))
                    .text_color(rgba(0xd5d3dac2))
                    .child(modified),
            )
            .child(div().w(px(85.)).text_color(rgba(0xd5d3daa6)).child(size))
            .child(div().w(px(90.)).text_color(rgba(0xd5d3daa6)).child(kind))
    }

    fn empty_list_row(index: usize) -> impl IntoElement {
        div()
            .h(px(26.))
            .when(index % 2 == 1, |row| row.bg(rgba(0xffffff08)))
    }

    fn list_content() -> impl IntoElement {
        let rows = [
            (
                "AndroidStudioProjects",
                "Feb 13, 2026 at 3:41 PM",
                "--",
                "Folder",
                false,
                false,
                false,
                true,
            ),
            (
                "Applications",
                "Apr 16, 2026 at 11:49 PM",
                "--",
                "Folder",
                false,
                false,
                false,
                false,
            ),
            (
                "CloudDrive",
                "Mar 21, 2026 at 11:38 AM",
                "--",
                "Folder",
                false,
                false,
                false,
                false,
            ),
            (
                "Code",
                "Jan 14, 2026 at 12:08 AM",
                "--",
                "Folder",
                false,
                false,
                false,
                false,
            ),
            (
                "Desktop",
                "Jul 27, 2026 at 11:21 PM",
                "--",
                "Folder",
                false,
                false,
                false,
                false,
            ),
            (
                "Documents",
                "Jul 27, 2026 at 11:20 PM",
                "--",
                "Folder",
                false,
                false,
                false,
                false,
            ),
            (
                "Downloads",
                "Yesterday at 10:18 PM",
                "--",
                "Folder",
                false,
                false,
                false,
                false,
            ),
            (
                "Dropbox",
                "Jan 11, 2026 at 12:22 AM",
                "42 bytes",
                "Alias",
                false,
                true,
                false,
                false,
            ),
            (
                "Github",
                "Feb 26, 2026 at 12:34 AM",
                "--",
                "Folder",
                false,
                false,
                false,
                false,
            ),
            (
                "Google Drive",
                "Yesterday at 10:11 AM",
                "66 bytes",
                "Alias",
                false,
                true,
                false,
                false,
            ),
            (
                "Modal",
                "Apr 21, 2025 at 12:36 AM",
                "--",
                "Folder",
                false,
                false,
                false,
                false,
            ),
            (
                "Movies",
                "Jun 8, 2025 at 8:47 PM",
                "--",
                "Folder",
                false,
                false,
                false,
                false,
            ),
            (
                "Music",
                "Jan 11, 2026 at 12:17 PM",
                "--",
                "Folder",
                false,
                false,
                false,
                false,
            ),
            (
                "Pictures",
                "Jan 12, 2026 at 11:53 PM",
                "--",
                "Folder",
                false,
                false,
                false,
                false,
            ),
            (
                "Public",
                "Mar 1, 2025 at 2:12 PM",
                "--",
                "Folder",
                false,
                false,
                false,
                false,
            ),
            (
                "Rust",
                "Jun 4, 2025 at 7:05 PM",
                "--",
                "Folder",
                true,
                false,
                true,
                false,
            ),
        ];
        let row_count = rows.len();

        div()
            .flex_1()
            .min_h_0()
            .overflow_hidden()
            .flex()
            .flex_col()
            .child(Self::list_header())
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .overflow_hidden()
                    .children(rows.into_iter().enumerate().map(
                        |(index, (label, modified, size, kind, red, alias, tagged, selected))| {
                            Self::list_row(
                                label, modified, size, kind, index, red, alias, tagged, selected,
                            )
                        },
                    ))
                    .children((0..18).map(|index| Self::empty_list_row(row_count + index))),
            )
    }

    fn context_menu_item(
        id: &'static str,
        label: &'static str,
        icon: LucideIcons,
        shortcut: &'static str,
        danger: bool,
        cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        div()
            .id(id)
            .h(px(32.))
            .px(px(10.))
            .flex()
            .items_center()
            .justify_between()
            .rounded_lg()
            .cursor_pointer()
            .text_size(px(12.5))
            .font_weight(FontWeight::MEDIUM)
            .text_color(if danger {
                rgba(0xff6b73ef)
            } else {
                rgba(0xf5f2f8ed)
            })
            .hover(|style| {
                style
                    .bg(rgba(0x0a84ffd6))
                    .border_1()
                    .border_color(rgba(0x8dcbff52))
            })
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.context_menu_position = None;
                    cx.notify();
                }),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(9.))
                    .child(svg().path(icon).size(px(15.)).text_color(if danger {
                        rgba(0xff7078ef)
                    } else {
                        rgba(0xece8f2d6)
                    }))
                    .child(label),
            )
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::NORMAL)
                    .text_color(rgba(0xc8c2cf85))
                    .child(shortcut),
            )
    }

    fn context_menu_separator() -> impl IntoElement {
        div()
            .h(px(9.))
            .px(px(7.))
            .flex()
            .items_center()
            .child(div().h(px(1.)).w_full().bg(rgba(0xffffff16)))
    }

    fn context_menu(
        position: gpui::Point<gpui::Pixels>,
        cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        GlassPanel::new()
            .material(GlassMaterial::Thin)
            .animation("finder-context-menu-glass", Duration::from_secs(4))
            .radius(px(18.))
            .tint(rgba(0x10162470))
            .border_color(rgba(0xc6d6f06b))
            .glass_opacity(1.0)
            .shader_tint([0.035, 0.055, 0.1, 0.80])
            .optics([12.0, 1.35, 0.40, 1.1])
            .surface([0.01, 0.58, 0.38, 1.08])
            .deformation(1.08)
            .wave_strength(1.15)
            .absolute()
            .left(position.x)
            .top(position.y)
            .w(px(226.))
            .p(px(6.))
            .shadow(vec![
                gpui::BoxShadow::new(px(0.), px(13.), rgba(0x00000080).into())
                    .blur_radius(px(34.))
                    .spread_radius(px(-8.)),
                gpui::BoxShadow::new(px(0.), px(1.), rgba(0xffffff1f).into()).blur_radius(px(1.)),
            ])
            .child(
                div()
                    .relative()
                    .flex()
                    .flex_col()
                    .child(Self::context_menu_item(
                        "menu-open",
                        "Open",
                        LucideIcons::FolderOpen,
                        "⌘O",
                        false,
                        cx,
                    ))
                    .child(Self::context_menu_item(
                        "menu-new-window",
                        "Open in New Window",
                        LucideIcons::PanelsTopLeft,
                        "",
                        false,
                        cx,
                    ))
                    .child(Self::context_menu_separator())
                    .child(Self::context_menu_item(
                        "menu-info",
                        "Get Info",
                        LucideIcons::Info,
                        "⌘I",
                        false,
                        cx,
                    ))
                    .child(Self::context_menu_item(
                        "menu-rename",
                        "Rename",
                        LucideIcons::Pencil,
                        "↩",
                        false,
                        cx,
                    ))
                    .child(Self::context_menu_item(
                        "menu-quick-look",
                        "Quick Look",
                        LucideIcons::Eye,
                        "Space",
                        false,
                        cx,
                    ))
                    .child(Self::context_menu_separator())
                    .child(Self::context_menu_item(
                        "menu-copy",
                        "Copy",
                        LucideIcons::Copy,
                        "⌘C",
                        false,
                        cx,
                    ))
                    .child(Self::context_menu_item(
                        "menu-trash",
                        "Move to Trash",
                        LucideIcons::Trash2,
                        "⌘⌫",
                        true,
                        cx,
                    )),
            )
    }

    fn content(&mut self, window: &Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let compact_toolbar = window.bounds().size.width < px(1050.);

        div()
            .flex_1()
            .flex()
            .flex_col()
            .child(
                div()
                    .h(px(72.))
                    .px(px(19.))
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(rgba(0xffffff08))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(15.))
                            .child(Self::toolbar_group(
                                "history-controls",
                                vec![
                                    Self::toolbar_button("back", LucideIcons::ChevronLeft)
                                        .into_any_element(),
                                    Self::toolbar_button("forward", LucideIcons::ChevronRight)
                                        .into_any_element(),
                                ],
                            ))
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgba(0xf4f3f6ee))
                                    .child("cradiy"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(12.))
                            .child(
                                Self::toolbar_group(
                                    "view-controls",
                                    vec![
                                        Self::view_toolbar_button(
                                            "grid",
                                            LucideIcons::Grid2x2,
                                            ViewMode::Grid,
                                            self.view_mode == ViewMode::Grid,
                                            cx,
                                        )
                                        .into_any_element(),
                                        Self::view_toolbar_button(
                                            "list",
                                            LucideIcons::List,
                                            ViewMode::List,
                                            self.view_mode == ViewMode::List,
                                            cx,
                                        )
                                        .into_any_element(),
                                        Self::toolbar_button("columns", LucideIcons::Columns3)
                                            .into_any_element(),
                                    ],
                                )
                                .pl_3()
                                .pr_3(),
                            )
                            .when(!compact_toolbar, |toolbar| {
                                toolbar
                                    .child(Self::toolbar_group(
                                        "sort-controls",
                                        vec![
                                            Self::toolbar_button("sort", LucideIcons::ListFilter)
                                                .into_any_element(),
                                        ],
                                    ))
                                    .child(Self::toolbar_group(
                                        "file-controls",
                                        vec![
                                            Self::toolbar_button(
                                                "new-folder",
                                                LucideIcons::FolderPlus,
                                            )
                                            .into_any_element(),
                                            Self::toolbar_button("delete", LucideIcons::Trash2)
                                                .into_any_element(),
                                            Self::toolbar_button("more", LucideIcons::Ellipsis)
                                                .into_any_element(),
                                        ],
                                    ))
                                    .child(
                                        div()
                                            .w(px(190.))
                                            .h(px(40.))
                                            .px(px(13.))
                                            .flex()
                                            .items_center()
                                            .gap(px(7.))
                                            .rounded_2xl()
                                            .bg(rgba(0x12131a75))
                                            .border_1()
                                            .border_color(rgba(0xffffff0f))
                                            .child(
                                                svg()
                                                    .path(LucideIcons::Search)
                                                    .size(px(16.))
                                                    .text_color(rgba(0xb9b7c1a3)),
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgba(0xb9b7c18c))
                                                    .child("Search"),
                                            ),
                                    )
                            })
                            .when(compact_toolbar, |toolbar| {
                                toolbar.child(Self::toolbar_button(
                                    "compact-more",
                                    LucideIcons::Ellipsis,
                                ))
                            }),
                    ),
            )
            .child(
                div()
                    .relative()
                    .flex_1()
                    .min_h_0()
                    .overflow_hidden()
                    .on_mouse_down(
                        MouseButton::Right,
                        cx.listener(|this, event: &gpui::MouseDownEvent, window, cx| {
                            let max_x = (window.bounds().size.width - px(577.)).max(px(8.));
                            let max_y = (window.bounds().size.height - px(359.)).max(px(8.));
                            this.context_menu_position = Some(point(
                                (event.position.x - px(343.)).clamp(px(8.), max_x),
                                (event.position.y - px(72.)).clamp(px(8.), max_y),
                            ));
                            cx.stop_propagation();
                            cx.notify();
                        }),
                    )
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            if this.context_menu_position.take().is_some() {
                                cx.notify();
                            }
                        }),
                    )
                    .child(match self.view_mode {
                        ViewMode::Grid => Self::grid_content().into_any_element(),
                        ViewMode::List => Self::list_content().into_any_element(),
                    })
                    .when_some(self.context_menu_position, |body, position| {
                        body.child(Self::context_menu(position, cx))
                    }),
            )
            .child(
                div()
                    .h(px(27.))
                    .px(px(12.))
                    .flex()
                    .items_center()
                    .gap(px(6.))
                    .border_t_1()
                    .border_color(rgba(0xffffff10))
                    .text_xs()
                    .text_color(rgba(0xc3c1c99c))
                    .child("▣  Macintosh HD")
                    .child("›")
                    .child("▣  Users")
                    .child("›")
                    .child("▣  cradiy"),
            )
    }

    fn sidebar() -> impl IntoElement {
        GlassPanel::new()
            .material(GlassMaterial::Regular)
            .animation("finder-sidebar-glass", Duration::from_secs(8))
            .radius(px(25.))
            .flex_1()
            .tint(rgba(0x17102f78))
            .border_color(rgba(0x7656d48f))
            .optics([15.0, 1.8, 0.54, 1.12])
            .surface([0.2, 0.54, 0.78, 0.82])
            .shader_tint([0.055, 0.035, 0.16, 0.46])
            .deformation(0.32)
            .wave_strength(0.72)
            .w(px(330.))
            .shadow(vec![
                gpui::BoxShadow::new(px(0.), px(8.), rgba(0x00000070).into())
                    .blur_radius(px(30.))
                    .spread_radius(px(-9.)),
            ])
            .child(
                div()
                    .relative()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .h(px(25.))
                            .px(px(20.))
                            .flex()
                            .items_center()
                            .gap(px(10.))
                            .on_mouse_down(MouseButton::Left, |_, window, _| {
                                window.start_window_move();
                            })
                            .child(Self::traffic_light(0xff5f57))
                            .child(Self::traffic_light(0xffbd2e))
                            .child(Self::traffic_light(0x28c840)),
                    )
                    .child(Self::section("Favorites"))
                    .child(Self::item(
                        "applications",
                        "Applications",
                        LucideIcons::AppWindow,
                        false,
                    ))
                    .child(Self::item(
                        "desktop",
                        "Desktop",
                        LucideIcons::Monitor,
                        false,
                    ))
                    .child(Self::item(
                        "documents",
                        "Documents",
                        LucideIcons::FileText,
                        false,
                    ))
                    .child(Self::item(
                        "downloads",
                        "Downloads",
                        LucideIcons::CircleArrowDown,
                        false,
                    ))
                    .child(Self::item("movies", "Movies", LucideIcons::Film, false))
                    .child(Self::item("music", "Music", LucideIcons::Music, false))
                    .child(Self::item(
                        "pictures",
                        "Pictures",
                        LucideIcons::Image,
                        false,
                    ))
                    .child(Self::section("Locations"))
                    .child(Self::item(
                        "icloud",
                        "iCloud Drive",
                        LucideIcons::Cloud,
                        false,
                    ))
                    .child(Self::item("dropbox", "Dropbox", LucideIcons::Box, false))
                    .child(Self::item("home", "cradiy", LucideIcons::House, true))
                    .child(Self::item(
                        "macbook",
                        "cradiy's MacBook Air",
                        LucideIcons::Laptop,
                        false,
                    ))
                    .child(Self::item(
                        "airdrop",
                        "AirDrop",
                        LucideIcons::Airplay,
                        false,
                    ))
                    .child(Self::item("trash", "Trash", LucideIcons::Trash2, false))
                    .child(Self::section("Tags"))
                    .child(Self::tag("tag-code", "Code", 0x0a84ff, false))
                    .child(Self::tag("tag-work", "Work", 0, true))
                    .child(Self::tag("tag-important", "Important", 0x00d85a, false))
                    .child(Self::tag("tag-all", "All Tags...", 0, true)),
            )
    }
}

impl Render for Macos26Sidebar {
    fn render(&mut self, window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .overflow_hidden()
            .flex()
            .bg(rgb(0x20222e))
            .child(div().flex_col().m_5().flex().child(Self::sidebar()))
            .child(self.content(window, cx))
    }
}

fn main() {
    application()
        .with_assets(LucideAssets::new())
        .run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(1280.), px(820.)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: None,
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|_| Macos26Sidebar {
                        view_mode: ViewMode::Grid,
                        context_menu_position: None,
                    })
                },
            )
            .expect("failed to open macOS 26 sidebar demo");
            cx.activate(true);
        });
}
