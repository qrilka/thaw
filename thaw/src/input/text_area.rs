use crate::{
    theme::{use_theme, Theme},
    utils::{class_list::class_list, mount_style, ComponentRef, Model, OptionalProp},
};
use leptos::*;

#[component]
pub fn TextArea(
    #[prop(optional, into)] value: Model<String>,
    #[prop(optional, into)] allow_value: Option<Callback<String, bool>>,
    #[prop(optional, into)] placeholder: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(optional, into)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(optional, into)] invalid: MaybeSignal<bool>,
    #[prop(optional)] comp_ref: ComponentRef<TextAreaRef>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let theme = use_theme(Theme::light);
    mount_style("text-area", include_str!("./text-area.css"));

    let value_trigger = create_trigger();
    let on_input = move |ev| {
        let input_value = event_target_value(&ev);
        if let Some(allow_value) = allow_value.as_ref() {
            if !allow_value.call(input_value.clone()) {
                value_trigger.notify();
                return;
            }
        }
        value.set(input_value);
    };
    let is_focus = create_rw_signal(false);
    let on_internal_focus = move |ev| {
        is_focus.set(true);
        if let Some(on_focus) = on_focus.as_ref() {
            on_focus.call(ev);
        }
    };
    let on_internal_blur = move |ev| {
        is_focus.set(false);
        if let Some(on_blur) = on_blur.as_ref() {
            on_blur.call(ev);
        }
    };

    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            let border_color_hover = theme.common.color_primary.clone();
            css_vars.push_str(&format!("--thaw-border-color-hover: {border_color_hover};"));
            css_vars.push_str(&format!("--thaw-box-shadow-color: {border_color_hover}33;"));
            let border_radius = theme.common.border_radius.clone();
            css_vars.push_str(&format!("--thaw-border-radius: {border_radius};"));
            css_vars.push_str(&format!(
                "--thaw-background-color: {};",
                theme.input.background_color
            ));
            css_vars.push_str(&format!("--thaw-font-color: {};", theme.input.font_color));
            css_vars.push_str(&format!(
                "--thaw-border-color: {};",
                theme.input.border_color
            ));
            css_vars.push_str(&format!(
                "--thaw-border-color-error: {};",
                theme.common.color_error
            ));
            css_vars.push_str(&format!(
                "--thaw-placeholder-color: {};",
                theme.input.placeholder_color
            ));
            css_vars.push_str(&format!(
                "--thaw-background-color-disabled: {};",
                theme.input.background_color_disabled
            ));
            css_vars.push_str(&format!(
                "--thaw-font-color-disabled: {};",
                theme.input.font_color_disabled
            ));
            css_vars.push_str(&format!(
                "--thaw-box-shadow-color-invalid: {}33;",
                theme.common.color_error
            ));
        });
        css_vars
    });
    let textarea_ref = create_node_ref::<html::Textarea>();
    textarea_ref.on_load(move |_| {
        comp_ref.load(TextAreaRef { textarea_ref });
    });

    #[cfg(debug_assertions)]
    {
        const INNER_ATTRS: [&str; 3] = ["class", "disabled", "placeholder"];
        attrs.iter().for_each(|attr| {
            if INNER_ATTRS.contains(&attr.0) {
                logging::warn!(
                    "Thaw: The '{}' attribute already exists on elements inside the TextArea component, which may cause conflicts.",
                    attr.0
                );
            }
        });
    }

    view! {
        <div
            class=class_list![
                "thaw-textarea", ("thaw-textarea--focus", move || is_focus.get()),
                ("thaw-textarea--disabled", move || disabled.get()), ("thaw-textarea--invalid", move
                || invalid.get()), class.map(| c | move || c.get())
            ]

            style=move || css_vars.get()
        >
            <textarea
                {..attrs}
                prop:value=move || {
                    value_trigger.track();
                    value.get()
                }

                on:input=on_input
                on:focus=on_internal_focus
                on:blur=on_internal_blur
                class="thaw-textarea__textarea-el"
                disabled=move || disabled.get()
                placeholder=placeholder.map(|p| move || p.get())
                ref=textarea_ref
            ></textarea>
        </div>
    }
}

#[derive(Clone)]
pub struct TextAreaRef {
    textarea_ref: NodeRef<html::Textarea>,
}

impl TextAreaRef {
    pub fn focus(&self) {
        if let Some(textarea_el) = self.textarea_ref.get_untracked() {
            _ = textarea_el.focus();
        }
    }

    pub fn blur(&self) {
        if let Some(textarea_el) = self.textarea_ref.get_untracked() {
            _ = textarea_el.blur();
        }
    }
}
