use dioxus::prelude::*;
use manganis::{self, classes};
use serde::Serialize;
use serde_json::json;
use web_sys;

#[derive(Serialize, PartialEq)]
struct FormData {
    operation: String,
    lefthandside: Vec<Property>,
    righthandside: Vec<Property>,
}

#[derive(Serialize, Clone, PartialEq)]
#[serde(untagged)]
enum Property {
    Value(String),
    Object { name: String, value: String },
}

fn main() {
    const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let operation = use_state(&cx, || "".to_string());
    let lefthandside = use_state(&cx, || vec![]);
    let righthandside = use_state(&cx, || vec![]);
    let json_output = use_state(&cx, || "".to_string());

    let on_submit = {
        let operation = operation.clone();
        let lefthandside = lefthandside.clone();
        let righthandside = righthandside.clone();
        move |_| {
            let lhs = format_properties(lefthandside.get().clone());
            let rhs = format_properties(righthandside.get().clone());

            let json = json!({
                operation.get().clone(): lhs.into_iter().chain(rhs.into_iter()).collect::<Vec<_>>()
            });
            let json_pretty = serde_json::to_string_pretty(&json).unwrap();
            json_output.set(json_pretty);
        }
    };

    let add_value = |side: &UseState<Vec<Property>>| {
        let mut values = side.get().clone();
        values.push(Property::Value("".to_string()));
        side.set(values);
    };

    let add_object = |side: &UseState<Vec<Property>>| {
        let mut values = side.get().clone();
        values.push(Property::Object {
            name: "var".to_string(),
            value: "".to_string(),
        });
        side.set(values);
    };

    let add_date_object = |side: &UseState<Vec<Property>>| {
        let mut values = side.get().clone();
        values.push(Property::Object {
            name: "Date.parse".to_string(),
            value: "Date.now".to_string(),
        });
        side.set(values);
    };

    let reset_page = |_| {
        web_sys::window().unwrap().location().reload().unwrap();
    };

    cx.render(rsx!(
        form {
            class: "space-y-4",
            prevent_default: "onsubmit",
            onsubmit: on_submit,
            div {
                class: "flex flex-row space-x-4",
                div {
                    class: "flex flex-col space-y-2",
                    h3 { class: "text-lg", "Left Hand Side" }
                    { 
                        lefthandside.get().into_iter().map(|property| render_property(cx, property, lefthandside))
                    }
                    {
                        if lefthandside.get().is_empty() {
                            rsx!(
                                button {
                                    class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600",
                                    onclick: move |_| add_value(lefthandside),
                                    " Value"
                                }
                                button {
                                    class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600",
                                    onclick: move |_| add_object(lefthandside),
                                    " Variable"
                                }
                                button {
                                    class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600",
                                    onclick: move |_| add_date_object(lefthandside),
                                    " Date"
                                }
                            )
                        } else {
                            rsx!(
                                div {}
                            )
                        }
                    }
                }
                div {
                    class: "flex flex-col mx-4 space-x-2",
                    label { class: "mb-1 text-lg", "Operation: " }
                    select {
                        class: "p-2 border rounded",
                        value: "{operation}",
                        oninput: move |e| operation.set(e.value.clone()),
                        option { value: "==", "==" }
                        option { value: ">", ">" }
                        option { value: "<", "<" }
                        option { value: ">=", ">=" }
                        option { value: "<=", "<=" }
                        option { value: "!=", "!=" }
                        option { value: "===", "===" }
                    }
                }
                div {
                    class: "flex flex-col space-y-2",
                    h3 { class: "text-lg", "Right Hand Side" }
                    { 
                        righthandside.get()
                        .iter()
                        .map(|property| render_property(cx, property, righthandside))

                    }
                    {
                        if righthandside.get().is_empty() {
                            rsx!(
                                button {
                                    class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600",
                                    onclick: move |_| add_value(righthandside),
                                    " Value"
                                }
                                button {
                                    class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600",
                                    onclick: move |_| add_object(righthandside),
                                    " Variable"
                                }
                                button {
                                    class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600",
                                    onclick: move |_| add_date_object(righthandside),
                                    " Date"
                                }
                            )
                        } else {
                            rsx!(
                                div {}
                            )
                        }
                    }
                }
            }
            div {
                class: "flex flex-row space-x-2",
                button {
                    class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                    "Submit"
                }
                button {
                    class: "px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600",
                    onclick: reset_page,
                    "Reset"
                }
            }
        }
        div {
            class: "mt-6",
            h3 { class: "text-xl font-semibold", "JSON Output:" }
            pre { class: "p-4 bg-gray-100 border rounded", "{json_output}" }
        }
    ))
}

fn render_property<'a>(
    cx: Scope<'a>,
    property: &'a Property,
    side: &'a UseState<Vec<Property>>,
) -> VNode<'a> {
    match property {
        Property::Value(val) => cx.render(rsx!(
            div {
                class: "flex flex-col",
                label { class: "mb-1 text-lg", "Value: " }
                input {
                    class: "p-2 border rounded",
                    r#type: "text",
                    value: "{val}",
                    oninput: move |e| {
                        let mut values = side.get().clone();
                        if let Some(index) = values.iter().position(|p| p == property) {
                            values[index] = Property::Value(e.value.clone());
                            side.set(values);
                        }
                    },
                }
            }
        )).expect("msg"),
        Property::Object { name, value } => cx.render(rsx!(
            div {
                class: "flex flex-row space-x-4",
                div {
                    class: "flex flex-col",
                    label { class: "mb-1 text-lg", "Name: " }
                    input {
                        class: "p-2 border rounded",
                        r#type: "text",
                        value: "{name}",
                        oninput: move |e| {
                            let mut values = side.get().clone();
                            if let Some(index) = values.iter().position(|p| p == property) {
                                if let Property::Object { name, value: _ } = &mut values[index] {
                                    *name = e.value.clone();
                                }
                                side.set(values);
                            }
                        },
                    }
                }
                div {
                    class: "flex flex-col",
                    label { class: "mb-1 text-lg", "Value: " }
                    input {
                        class: "p-2 border rounded",
                        r#type: "text",
                        value: "{value}",
                        oninput: move |e| {
                            let mut values = side.get().clone();
                            if let Some(index) = values.iter().position(|p| p == property) {
                                if let Property::Object { name: _, value } = &mut values[index] {
                                    *value = e.value.clone();
                                }
                                side.set(values);
                            }
                        },
                    }
                }
            }
        )).expect("msg"),
    }
}

fn format_properties(properties: Vec<Property>) -> Vec<serde_json::Value> {
    properties
        .into_iter()
        .map(|property| match property {
            Property::Value(val) => serde_json::Value::String(val),
            Property::Object { name, value } => json!({ name: value }),
        })
        .collect()
}
