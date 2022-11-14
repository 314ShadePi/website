use dioxus::prelude::*;

pub struct CLink(pub String, pub String);

impl CLink {
    pub fn render(self, cx: Scope) -> Element {
        let binding = self.0;
        let to = binding.as_str();
        let binding = self.1;
        let text = binding.as_str();

        cx.render(rsx! {
            Link {
                to: "{to}",
                "{text}"
            }
        })
    }
}
