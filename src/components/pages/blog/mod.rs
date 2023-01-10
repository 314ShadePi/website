pub mod post;
use crate::models::blog_list::BlogList;
use c314_utils::prelude::ToStaticStr;
use dioxus::prelude::*;

use gloo::net::http::Request;

pub fn blog(cx: Scope) -> Element {
    let router = use_router(&cx);
    let blog_list = use_ref(&cx, || BlogList { posts: vec![] });
    let first_run = use_state(&cx, || true);
    cx.spawn({
        let blog_list_c = blog_list.clone();
        let first_run = first_run.clone();
        async move {
            if first_run == false {
                return;
            }
            first_run.set(false);

            let resp = Request::get("https://raw.githubusercontent.com/314ShadePi/314shadepi-website-static/main/blog/index.json")
                .send()
                .await
                .unwrap();

            if resp.ok() {
                let blog_list = resp.json::<BlogList>().await.unwrap();
                blog_list_c.set(blog_list);
            }
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
                        h2 {
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
