use dioxus::prelude::*;

#[test]
fn app_drops() {
    fn App() -> Element {
        render! { div {} }
    }

    let mut dom = VirtualDom::new(App);

    _ = dom.rebuild(&mut dioxus_core::NoOpMutations);
    dom.mark_dirty(ScopeId::ROOT);
    _ = dom.render_immediate_to_vec();
}

#[test]
fn hooks_drop() {
    fn App() -> Element {
        cx.use_hook(|| String::from("asd"));
        cx.use_hook(|| String::from("asd"));
        cx.use_hook(|| String::from("asd"));
        cx.use_hook(|| String::from("asd"));

        render! { div {} }
    }

    let mut dom = VirtualDom::new(App);

    _ = dom.rebuild(&mut dioxus_core::NoOpMutations);
    dom.mark_dirty(ScopeId::ROOT);
    _ = dom.render_immediate_to_vec();
}

#[test]
fn contexts_drop() {
    fn App() -> Element {
        cx.provide_context(String::from("asd"));

        render! {
            div { ChildComp {} }
        }
    }

    fn ChildComp() -> Element {
        let el = cx.consume_context::<String>().unwrap();

        render! { div { "hello {el}" } }
    }

    let mut dom = VirtualDom::new(App);

    _ = dom.rebuild(&mut dioxus_core::NoOpMutations);
    dom.mark_dirty(ScopeId::ROOT);
    _ = dom.render_immediate_to_vec();
}

#[test]
fn tasks_drop() {
    fn App() -> Element {
        cx.spawn(async {
            // tokio::time::sleep(std::time::Duration::from_millis(100000)).await;
        });

        render! { div {} }
    }

    let mut dom = VirtualDom::new(App);

    _ = dom.rebuild(&mut dioxus_core::NoOpMutations);
    dom.mark_dirty(ScopeId::ROOT);
    _ = dom.render_immediate_to_vec();
}

#[test]
fn root_props_drop() {
    #[derive(Clone)]
    struct RootProps(String);

    let mut dom = VirtualDom::new_with_props(
        |cx| render!( div { "{cx.0}" } ),
        RootProps("asdasd".to_string()),
    );

    _ = dom.rebuild(&mut dioxus_core::NoOpMutations);
    dom.mark_dirty(ScopeId::ROOT);
    _ = dom.render_immediate_to_vec();
}

#[test]
fn diffing_drops_old() {
    fn App() -> Element {
        render! {
            div {
                match generation() % 2 {
                    0 => render!( ChildComp1 { name: "asdasd".to_string() }),
                    1 => render!( ChildComp2 { name: "asdasd".to_string() }),
                    _ => todo!()
                }
            }
        }
    }

    #[component]
    fn ChildComp1(name: String) -> Element {
        render! {"Hello {name}"}
    }

    #[component]
    fn ChildComp2(name: String) -> Element {
        render! {"Goodbye {name}"}
    }

    let mut dom = VirtualDom::new(App);
    _ = dom.rebuild(&mut dioxus_core::NoOpMutations);
    dom.mark_dirty(ScopeId::ROOT);

    _ = dom.render_immediate_to_vec();
}
