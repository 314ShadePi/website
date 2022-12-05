use dioxus::prelude::*;
use gloo::net::http::Request;
use pulldown_cmark::{html, Options, Parser};
use wasm_bindgen_futures::spawn_local;

pub fn post(cx: Scope) -> Element {
    let route = use_route(&cx);

    let filename = match route.segment("filename") {
        Some(val) => val.to_owned(),
        None => "An unknown error occurred".to_string(),
    };

    let post = use_ref(&cx, || String::from(""));

    let post_clone = post.clone();
    spawn_local(async move {
        let resp = Request::get(format!("https://raw.githubusercontent.com/314ShadePi/314shadepi-website-static/main/blog/{}", filename).as_str())
            .send()
            .await
            .unwrap();

        if resp.ok() {
            let post = resp.text().await.unwrap();
            post_clone.set(post);
        }
    });

    let post = post.read();
    let markdown_input = post.as_str();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_FOOTNOTES);
    let parser = Parser::new_ext(markdown_input, options);
    let mut output = String::new();
    html::push_html(&mut output, parser);

    cx.render(rsx! {
        article {
            class: "post",
            dangerous_inner_html: "{output}"
        }
    })
}
