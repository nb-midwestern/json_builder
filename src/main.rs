use dioxus::prelude::*;
use manganis::{self};
use serde::Serialize;
use serde_json::json;
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
}

fn app() -> Element {
    let mut operation = use_signal(|| "".to_string());
    let mut lefthandside = use_signal(|| vec![]);
    let mut righthandside = use_signal(|| vec![]);
    let mut json_output = use_signal(|| "".to_string());
    let mut copy_state = use_signal(|| 0);

    let on_submit = {
        let operation = operation.clone();
        let lefthandside = lefthandside.clone();
        let righthandside = righthandside.clone();
        move |_| {
            let lhs = format_properties(lefthandside.read().clone());
            let rhs = format_properties(righthandside.read().clone());

            let json = json!({
                operation.read().clone(): lhs.into_iter().chain(rhs.into_iter()).collect::<Vec<_>>()
            });
            let json_pretty = serde_json::to_string_pretty(&json).unwrap();
            json_output.set(json_pretty);
        }
    };

    let add_value = || {
        let mut values = lefthandside.read().clone();
        values.push(Property::Value("".to_string()));
        lefthandside.set(values);
        copy_state.set(1);
    };

    let add_object = |side: &mut Signal<Vec<Property>>| {
        let mut values = side.read().clone();
        values.push(Property::Object {
            name: "var".to_string(),
            value: "".to_string(),
        });
        side.set(values);
    };

    let add_date_object = |side: &mut Signal<Vec<Property>>| {
        let mut values = side.read().clone();
        values.push(Property::Object {
            name: "Date.parse".to_string(),
            value: "Date.now".to_string(),
        });
        side.set(values);
    };

    rsx! {
        div {
            "hello world"
        }
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
                        // lefthandside.read().iter().map(|property| render_property( property, lefthandside)).collect::<Vec<_>>()

                    }
                    {
                        if lefthandside.read().is_empty() {
                            rsx!{
                                button {
                                    class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600",
                                    // onclick: move |_| add_value(lefthandside),
                                    "Value"
                                }
                                button {
                                    class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600",
                                    // onclick: move |_| add_object(lefthandside),
                                    "Variable"
                                }
                                button {
                                    class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600",
                                    // onclick: move |_| add_date_object(lefthandside),
                                    "Date"
                                }
                        }
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
                        oninput: move |e| operation.set(e.value().clone()),
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
                        // righthandside.get().iter().map(|property| render_property(cx, property, righthandside)).collect::<Vec<_>>()
                    }
                    {
                        if righthandside.read().is_empty() {
                            rsx!(
                                button {
                                    class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600",
                                    // onclick: move |_| add_value(righthandside),
                                    "Value"
                                }
                                button {
                                    class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600",
                                    // onclick: move |_| add_object(righthandside),
                                    "Variable"
                                }
                                button {
                                    class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600",
                                    // onclick: move |_| add_date_object(righthandside),
                                    "Date"
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
                    // onclick: reset_page,
                    "Reset"
                }
            }
        }
        div {
            class: "mt-6",
            h3 { class: "text-xl font-semibold", "JSON Output:" }
            pre { class: "p-4 bg-gray-100 border rounded", "{json_output}" }
        }
    }
}

fn render_property(property: &Property, side: &mut Signal<Vec<Property>>) -> VNode {
    match property {
        Property::Value(_) => todo!(),
        Property::Object { name, value } => todo!(),
        Property::Value(val) => rsx! {
            div {
                class: "flex flex-col",
                label { class: "mb-1 text-lg", "Value: " }
                input {
                    class: "p-2 border rounded",
                    r#type: "text",
                    value: "{val}",
                    // oninput: move |e| {
                    //     let mut values = side.read().clone();
                    //     if let Some(index) = values.iter().position(|p| p == property) {
                    //         values[index] = Property::Value(e.value().clone());
                    //         side.set(values);
                    //     }
                    // },
                }
            }
        }
        .unwrap(),
        Property::Object { name, value } => rsx! {
            div {
                class: "flex flex-row space-x-4",
                div {
                    class: "flex flex-col",
                    label { class: "mb-1 text-lg", "Name: " }
                    input {
                        class: "p-2 border rounded",
                        r#type: "text",
                        value: "{name}",
                        // oninput: move |e| {
                        //     let mut values = side.read().clone();
                        //     if let Some(index) = values.iter().position(|p| p == property) {
                        //         if let Property::Object { name, value: _ } = &mut values[index] {
                        //             *name = e.value.clone();
                        //         }
                        //         side.set(values);
                        //     }
                        // },
                    }
                }
                div {
                    class: "flex flex-col",
                    label { class: "mb-1 text-lg", "Value: " }
                    input {
                        class: "p-2 border rounded",
                        r#type: "text",
                        value: "{value}",
                        // oninput: move |e| {
                        //     let mut values = side.read().clone();
                        //     if let Some(index) = values.iter().position(|p| p == property) {
                        //         if let Property::Object { name: _, value } = &mut values[index] {
                        //             *value = e.value.clone();
                        //         }
                        //         side.set(values);
                        //     }
                        // },
                    }
                }
            }
        }
        .unwrap(),
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
