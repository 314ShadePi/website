mod components;

use common::database::project::{description_part::*, p_type::*, single::Project, tags::*};
use dioxus::prelude::*;

use crate::components::pages::core::page;

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

    let test_data = Project {
        id: "test_id".to_string(),
        name: "TestProj".to_string(),
        tags: vec![Tags::Cli, Tags::Game, Tags::Gui],
        p_type: Type::Game {
            os: vec![OS::Windows {
                version: vec![WinVer::Eleven],
            }],
            engine: Engine::Unreal {
                version: UnrealVer::Five,
            },
        },
        description: vec![
            DescriptionPart {
                desc_type: DescType::Usage,
                is_html: false,
                content: "cargo run".to_string(),
            },
            DescriptionPart {
                desc_type: DescType::Installation,
                is_html: false,
                content: "cargo install test-proj".to_string(),
            },
            DescriptionPart {
                desc_type: DescType::Faq,
                is_html: true,
                content: "<h3>Is good?</h3><p>Yes.</p>".to_string(),
            },
            DescriptionPart {
                desc_type: DescType::Other {
                    title: "Test section 1".to_string(),
                },
                is_html: false,
                content: "Test 1".to_string(),
            },
            DescriptionPart {
                desc_type: DescType::Other {
                    title: "Test section 2".to_string(),
                },
                is_html: false,
                content: "Test 2".to_string(),
            },
            DescriptionPart {
                desc_type: DescType::Other {
                    title: "Test section 3".to_string(),
                },
                is_html: false,
                content: "Test 3".to_string(),
            },
        ],
    };

    cx.render(rsx! {
            Router {
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
                    oncreate: page_oncreate,
                    pages: pages.current()
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
                    pages: pages.current()
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
                    pages: pages.current()
                }
                page::page {
                    to: "/project",
                    name: "Project",
                    should_be_on_navbar: true,
                    content: cx.render(rsx! {
                        div {
                            class: "page page-project",
                            test_data.render_single(cx)
                        }
                    }),
                    oncreate: page_oncreate
                    pages: pages.current()
                }
            }
    })
}
