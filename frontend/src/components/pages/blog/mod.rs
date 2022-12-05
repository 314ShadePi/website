pub mod post;
use crate::models::blog_list::BlogList;
use c314_utils::prelude::ToStaticStr;
use dioxus::prelude::*;
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;

pub fn blog(cx: Scope) -> Element {
    let router = use_router(&cx);
    let blog_list = use_ref(&cx, || BlogList { posts: vec![] });

    let blog_list_clone = blog_list.clone();
    spawn_local(async move {
        let resp = Request::get("https://raw.githubusercontent.com/314ShadePi/314shadepi-website-static/main/blog/index.json")
            .send()
            .await
            .unwrap();

        if resp.ok() {
            let blog_list = resp.json::<BlogList>().await.unwrap();
            blog_list_clone.set(blog_list);
        }
    });
    let onclick = move |filename: String| {
        router.push_route(format!("/blog/post/{}", filename).as_str(), None, None)
    };
    cx.render(rsx! {
        div {
            class: "container",
            id: "blog-list",
            blog_list.read().posts.iter().map(|post| {
                let binding: String = post.filename.clone();
                let filename: &'static str = binding.to_static_str();
                rsx! {
                    article {
                        class: "post",
                        id: "{post.id}",
                        onclick: move |_| onclick(filename.to_string()),
                        h1 {
                            class: "title",
                            "{post.title}"
                        }
                        div {
                            class: "info",
                            p {
                                class: "short",
                                "{post.short}",
                            }
                            p {
                                class: "author",
                                "By {post.author}"
                            }
                        }
                    }
                }
            })
        }
    })
}
