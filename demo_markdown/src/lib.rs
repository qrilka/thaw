mod markdown;

use crate::markdown::parse_markdown;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::ItemFn;

#[proc_macro]
pub fn include_md(_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let file_list = vec![
        (
            "InstallationMdPage",
            include_str!("../docs/_guide/installation.md"),
        ),
        (
            "ServerSiderRenderingMdPage",
            include_str!("../docs/_guide/server_sider_rendering.md"),
        ),
        ("UsageMdPage", include_str!("../docs/_guide/usage.md")),
        (
            "NavBarMdPage",
            include_str!("../docs/_mobile/nav_bar/mod.md"),
        ),
        (
            "TabbarMdPage",
            include_str!("../docs/_mobile/tabbar/mod.md"),
        ),
        ("ToastMdPage", include_str!("../docs/_mobile/toast/mod.md")),
        ("AlertMdPage", include_str!("../docs/alert/mod.md")),
        (
            "AutoCompleteMdPage",
            include_str!("../docs/auto_complete/mod.md"),
        ),
        ("AvatarMdPage", include_str!("../docs/avatar/mod.md")),
        ("BadgeMdPage", include_str!("../docs/badge/mod.md")),
        (
            "BreadcrumbMdPage",
            include_str!("../docs/breadcrumb/mod.md"),
        ),
        ("ButtonMdPage", include_str!("../docs/button/mod.md")),
        ("CalendarMdPage", include_str!("../docs/calendar/mod.md")),
        ("CardMdPage", include_str!("../docs/card/mod.md")),
        ("CheckboxMdPage", include_str!("../docs/checkbox/mod.md")),
        ("CollapseMdPage", include_str!("../docs/collapse/mod.md")),
        (
            "ColorPickerMdPage",
            include_str!("../docs/color_picker/mod.md"),
        ),
        (
            "DatePickerMdPage",
            include_str!("../docs/date_picker/mod.md"),
        ),
        ("DividerMdPage", include_str!("../docs/divider/mod.md")),
        ("DrawerMdPage", include_str!("../docs/drawer/mod.md")),
        ("GridMdPage", include_str!("../docs/grid/mod.md")),
        ("IconMdPage", include_str!("../docs/icon/mod.md")),
        ("ImageMdPage", include_str!("../docs/image/mod.md")),
        ("InputMdPage", include_str!("../docs/input/mod.md")),
        (
            "InputNumberMdPage",
            include_str!("../docs/input_number/mod.md"),
        ),
        ("LayoutMdPage", include_str!("../docs/layout/mod.md")),
        (
            "LoadingBarMdPage",
            include_str!("../docs/loading_bar/mod.md"),
        ),
        ("MenuMdPage", include_str!("../docs/menu/mod.md")),
        ("MessageMdPage", include_str!("../docs/message/mod.md")),
        ("ModalMdPage", include_str!("../docs/modal/mod.md")),
        ("PopoverMdPage", include_str!("../docs/popover/mod.md")),
        ("ProgressMdPage", include_str!("../docs/progress/mod.md")),
        ("RadioMdPage", include_str!("../docs/radio/mod.md")),
        ("SelectMdPage", include_str!("../docs/select/mod.md")),
        ("SkeletonMdPage", include_str!("../docs/skeleton/mod.md")),
        ("SliderMdPage", include_str!("../docs/slider/mod.md")),
        ("SpaceMdPage", include_str!("../docs/space/mod.md")),
        ("SpinnerMdPage", include_str!("../docs/spinner/mod.md")),
        ("SwitchMdPage", include_str!("../docs/switch/mod.md")),
        ("TableMdPage", include_str!("../docs/table/mod.md")),
        ("TabsMdPage", include_str!("../docs/tabs/mod.md")),
        ("TagMdPage", include_str!("../docs/tag/mod.md")),
        ("ThemeMdPage", include_str!("../docs/theme/mod.md")),
        (
            "TimePickerMdPage",
            include_str!("../docs/time_picker/mod.md"),
        ),
        (
            "TypographyMdPage",
            include_str!("../docs/typography/mod.md"),
        ),
        ("UploadMdPage", include_str!("../docs/upload/mod.md")),
    ];

    let mut fn_list = vec![];

    for (fn_name, file_str) in file_list {
        let fn_name = Ident::new(fn_name, Span::call_site());

        let (body, demos) = match parse_markdown(file_str) {
            Ok(body) => body,
            Err(err) => {
                return quote!(compile_error!(#err)).into();
            }
        };

        let demos: Vec<ItemFn> = demos
            .into_iter()
            .enumerate()
            .map(|(index, demo)| {
                format!(
                    "#[component] fn Demo{}() -> impl IntoView {{ {} }}",
                    index + 1,
                    demo
                )
            })
            .map(|demo| {
                syn::parse_str::<ItemFn>(&demo)
                    .expect(&format!("Cannot be resolved as a function: \n {demo}"))
            })
            .collect();

        fn_list.push(quote! {
            #[component]
            pub fn #fn_name() -> impl IntoView {
                #(#demos)*

                view! {
                    <div class="demo-components__component">
                        #body
                    </div>
                }
            }
        });
    }

    quote! {
        #(#fn_list)*
    }
    .into()
}
