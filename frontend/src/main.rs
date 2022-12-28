mod components;
mod models;

use dioxus::prelude::*;

use crate::components::pages::{
    blog,
    core::{footer, header, page},
    project, projects,
};

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let pages = use_state(&cx, || {
        vec![page::Page {
            to: "/non-existing-page",
            name: "test",
            display: false,
        }]
    });

    let page_oncreate = move |page: page::Page| {
        if !pages.contains(&page) {
            pages.with_mut(|pages| pages.append(&mut vec![page]));
        }
    };

    gloo::console::info!("Secrets may be found by getting a heading and navigating.");

    cx.render(rsx! {
            Router {
                header::header { pages: pages.current().clone() }
                div {
                    id: "space-top"
                }
                page::page {
                    to: "/",
                    name: "Home",
                    should_be_on_navbar: true,
                    content: cx.render(rsx! {
                        div {
                            class: "page page-home",
                            p { "Home" }
                            Link { to: "/about", "About" }
                            p { "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" }
                        }
                    }),
                    oncreate: page_oncreate
                }
                page::page {
                    to: "/about",
                    name: "About",
                    should_be_on_navbar: true,
                    content: cx.render(rsx! {
                        div {
                            class: "page page-about",
                            p { "About" }
                            Link { to: "/", "Home" }
                        }
                    }),
                    oncreate: page_oncreate
                }
                page::page {
                    to: "/1ryqe-3aakenrScHyF4T6A9LTg7rw4Sk2LmpUlCtrjWASMBKvmtMTkW62up198TtDwPxQr5U5Ew0CfcONSQz2JnAr7cK_5MtZkGGjF3xVFS-RQuOWyxuBDI0y2-YSb6Kc4BQuaWHsW_IOk6RgXl3iqS1jQ_3-W4kcH6EmDn5uY488k3QWoOKs3eg-E20ByHJXiA2VQJqpU_qCrQEKioBaD0bKzFw",
                    name: "Secret",
                    should_be_on_navbar: false,
                    content: cx.render(rsx! {
                        div {
                            class: "page page-secret",
                            h1 { class: "title", "U+1F44D" }
                        }
                    }),
                    oncreate: page_oncreate
                }
                page::page {
                    to: "/projects/:id",
                    name: "Project",
                    should_be_on_navbar: false,
                    content: cx.render(rsx! {
                        div {
                            class: "page page-project",
                            project::project {}
                        }
                    }),
                    oncreate: page_oncreate
                }
                page::page {
                    to: "/projects",
                    name: "Projects",
                    should_be_on_navbar: true,
                    content: cx.render(rsx! {
                        div {
                            class: "page page-projects",
                            projects::projects {}
                        }
                    }),
                    oncreate: page_oncreate
                }
                page::page {
                    to: "/blog/post/:year/:month/:day/:filename",
                    name: "Post",
                    should_be_on_navbar: false,
                    content: cx.render(rsx! {
                        div {
                            class: "page page-post",
                            blog::post::post {}
                        }
                    }),
                    oncreate: page_oncreate
                }
                page::page {
                    to: "/blog",
                    name: "Blog",
                    should_be_on_navbar: true,
                    content: cx.render(rsx! {
                        div {
                            class: "page page-blog",
                            blog::blog {}
                        }
                    }),
                    oncreate: page_oncreate
                }
                page::page {
                    to: "/project",
                    name: "Project",
                    should_be_on_navbar: false,
                    content: cx.render(rsx! {
                        Redirect {
                            to: "/projects"
                        }
                    }),
                    oncreate: page_oncreate
                }
                page::page {
                    to: "/project/:id",
                    name: "Project",
                    should_be_on_navbar: false,
                    content: cx.render(rsx! {
                        Redirect {
                            to: "/projects"
                        }
                    }),
                    oncreate: page_oncreate
                }
                div {
                    id: "space-bottom"
                }
                footer::footer {}
            }
    })
}
