use crate::{
    components::linenumber::LineNumber,
    external::{self, HighlightResult},
    state::{Action, GameState},
};
use wasm_bindgen::JsValue;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;
use yewdux::prelude::{use_selector, Dispatch};

#[function_component]
pub fn Window() -> Html {
    let dispatch = Dispatch::<GameState>::new();
    let code = use_selector(|state: &GameState| state.code.clone());
    let language = use_selector(|state: &GameState| state.language.clone());

    let input_ref = use_node_ref();
    let correct_ref = use_node_ref();

    use_effect_with_deps(
        {
            let input_ref = input_ref.clone();
            move |_| {
                let input = input_ref.cast::<HtmlInputElement>().unwrap();
                input.focus().unwrap();
                || ()
            }
        },
        (),
    );

    use_effect_with_deps(
        {
            let correct = code.correct.clone();
            let correct_ref = correct_ref.clone();
            move |_| {
                if !correct.is_empty() {
                    let options = external::HighlightOptions {
                        language: "rust".to_string(),
                        ignore_illegals: true,
                    };
                    let highlighted: HighlightResult = external::Hljs::highlight(
                        &correct,
                        &JsValue::from_serde(&options).unwrap(),
                    );

                    let correct_code = correct_ref.cast::<HtmlElement>().unwrap();
                    correct_code.set_inner_html(&highlighted.value());
                }
                || ()
            }
        },
        code.correct.clone(),
    );

    let onclick = {
        let input_ref = input_ref.clone();

        Callback::from(move |_| {
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            input.focus().unwrap();
        })
    };

    let onkeydown = {
        let wrong = code.wrong.clone();
        let input_ref = input_ref.clone();

        Callback::from(move |e: KeyboardEvent| {
            let key: Option<char> = match e.key().as_str() {
                "Escape" => {
                    let input = input_ref.cast::<HtmlInputElement>().unwrap();
                    input.blur().unwrap();
                    None
                }
                "Backspace" if !wrong.is_empty() => {
                    dispatch.apply(Action::BackSpace);
                    None
                }
                "Enter" => Some('\n'),
                "Tab" => {
                    e.prevent_default();
                    Some('\t')
                }
                k if k.len() == 1 => k.chars().next(),
                _ => None,
            };

            if let Some(k) = key {
                if k.is_alphanumeric() || k.is_whitespace() || k.is_ascii_punctuation() {
                    dispatch.apply(Action::KeyPress(k));
                }
            }
        })
    };

    let wrong = &code.wrong.replace('\n', "↵\n");

    let cursor = {
        if let Some(cursor) = code.cursor {
            match cursor {
                '\n' => "↵\n".to_string(),
                _ => cursor.to_string(),
            }
        } else {
            "".to_string()
        }
    };

    let hljs_classes = classes!(
        "hljs",
        "text-white",
        format!("language-{}", language.to_lowercase())
    );

    html! {
        <div>
            <div class="flex flex-row px-6 pt-6 gap-2">
                <LineNumber lines={code.lines}/>
                <pre {onclick} class="relative display-inline w-full break-all" style="tab-size: 4;">
                    <code ref={correct_ref} class={hljs_classes} />
                    <code class="text-red">{wrong}</code>
                    <code class="bg-white-light text-black-light">{cursor}</code>
                    <code class="text-white">{&code.remaining}</code>
                    <input
                        ref={input_ref}
                        {onkeydown}
                        autocomplete="off"
                        type="text"
                        style="position: absolute; width: 1px; left: -10000px;"
                    />
                </pre>
            </div>
        </div>
    }
}
