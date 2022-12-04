pub mod post;
use crate::models::blog_list::BlogList;
use dioxus::prelude::*;
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;

pub fn blog(cx: Scope) -> Element {
    let blog_list = use_ref(&cx, || BlogList { posts: vec![] });

    let blog_list_clone = blog_list.clone();
    spawn_local(async move {
        let resp = Request::get("https://raw.githubusercontent.com/314ShadePi/314shadepi-website-static/main/blog/index.json")
            .send()
            .await
            .unwrap();

        if resp.status() == 200 {
            let blog_list = resp.json::<BlogList>().await.unwrap();
            blog_list_clone.set(blog_list);
        }
    });
    cx.render(rsx! {
        div {
            class: "container",
            id: "blog-list",
            blog_list.read().posts.iter().map(|post| {
                rsx! {
                    article {
                        class: "post",
                        id: "{post.id}",
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
