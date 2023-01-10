mod components;
mod models;
mod traits;

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
                    oncreate: page_oncreate,
                    div {
                        class: "page page-home",
                        p { "Home" }
                        Link { to: "/about", "About" }
                        p { "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" }
                    }
                }
                page::page {
                    to: "/about",
                    name: "About",
                    should_be_on_navbar: true,
                    oncreate: page_oncreate,
                    div {
                        class: "page page-about",
                        p { "About" }
                        Link { to: "/", "Home" }
                    }
                }
                page::page {
                    to: "/projects/:id",
                    name: "Project",
                    should_be_on_navbar: false,
                    oncreate: page_oncreate,
                    div {
                        class: "page page-project",
                        project::project {}
                    }
                }
                page::page {
                    to: "/projects",
                    name: "Projects",
                    should_be_on_navbar: true,
                    oncreate: page_oncreate,
                    div {
                        class: "page page-projects",
                        projects::projects {}
                    }
                }
                page::page {
                    to: "/blog/post/:year/:month/:day/:filename",
                    name: "Post",
                    should_be_on_navbar: false,
                    oncreate: page_oncreate,
                    div {
                        class: "page page-post",
                        blog::post::post {}
                    }
                }
                page::page {
                    to: "/blog",
                    name: "Blog",
                    should_be_on_navbar: true,
                    oncreate: page_oncreate,
                    div {
                        class: "page page-blog",
                        h1 {
                            class: "title",
                            "314ShadePi's Blog:"
                        }
                        blog::blog {}
                    }
                }
                page::page {
                    to: "/project",
                    name: "Project",
                    should_be_on_navbar: false,
                    oncreate: page_oncreate,
                    Redirect {
                        to: "/projects"
                    }
                }
                page::page {
                    to: "/project/:id",
                    name: "Project",
                    should_be_on_navbar: false,
                    oncreate: page_oncreate,
                    Redirect {
                        to: "/projects"
                    }
                }
                page::page {
                    to: "",
                    name: "404",
                    should_be_on_navbar: false,
                    oncreate: page_oncreate,
                    div {
                        class: "page page-not-found",
                        h1 {
                            class: "title",
                            "404 - Not Found"
                        }
                    }
                }
                div {
                    id: "space-bottom"
                }
                footer::footer {}
            }
    })
}
