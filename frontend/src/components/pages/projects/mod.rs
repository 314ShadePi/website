use common::database::project::{description_part::*, p_type::*, single::Project, tags::*};
use dioxus::prelude::*;

pub fn projects(cx: Scope) -> Element {
    let test_data = vec![
        Project {
            id: "test_id".to_string(),
            name: "TestProj".to_string(),
            tags: vec![Tags::Cli, Tags::Game],
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
        },
        Project {
            id: "test_id".to_string(),
            name: "TestProj".to_string(),
            tags: vec![Tags::Cli, Tags::Game],
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
        },
        Project {
            id: "test_id".to_string(),
            name: "TestProj".to_string(),
            tags: vec![Tags::Cli, Tags::Game],
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
        },
    ];

    cx.render(rsx! {
        div {
            class: "container",
            id: "project-list",
            test_data.iter().map(|project| {

            })
        }
    })
}
