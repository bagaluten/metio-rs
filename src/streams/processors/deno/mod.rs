use deno_core::*;

use super::Processor;
use crate::types::Event;

pub struct DenoProcessor {
    js_runtime: JsRuntime,
}

impl DenoProcessor {
    pub fn new() -> Self {
        const DECL: OpDecl = get_events();
        let ext = Extension {
            name: "op_get_events",
            ops: std::borrow::Cow::Borrowed(&[DECL]),
            ..Default::default()
        };
        let runtime = JsRuntime::new(RuntimeOptions {
            extensions: vec![ext],
            ..Default::default()
        });
        Self {
            js_runtime: runtime,
        }
    }
}

#[op2]
#[serde]
fn get_events(state: &mut OpState) -> Vec<Event> {
    let a = state.take::<Vec<Event>>();
    a
}

impl Processor for DenoProcessor {
    fn process<I>(&mut self, events: I) -> impl IntoIterator<Item = Event>
    where
        I: IntoIterator<Item = Event>,
    {
        self.js_runtime
            .op_state()
            .borrow_mut()
            .put(events.into_iter().collect::<Vec<Event>>());
        let res = self
            .js_runtime
            .execute_script("get_events()", include_str!("filter.js"));
        let res = res.unwrap();
        let scope = &mut self.js_runtime.handle_scope();
        let local = v8::Local::new(scope, res);
        let events = serde_v8::from_v8::<Vec<Event>>(scope, local).unwrap();

        events
    }
}
