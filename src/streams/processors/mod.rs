/*
 * Copyright 2024 Bagaluten GmbH <contact@bagaluten.email>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::types::Event;

#[cfg(feature = "processor_deno")]
pub mod deno;

pub trait Processor {
    fn process<I>(&mut self, events: I) -> impl IntoIterator<Item = Event>
    where
        I: IntoIterator<Item = Event>;
}

/// function Proccesor
pub struct FunctionProcessor {
    func: Box<dyn Fn(Event) -> Option<Event> + Send + Sync>,
}

impl FunctionProcessor {
    pub fn new<F>(func: F) -> Self
    where
        F: Fn(Event) -> Option<Event> + Send + Sync + 'static,
    {
        Self {
            func: Box::new(func),
        }
    }
}

impl Processor for FunctionProcessor {
    fn process<I>(&mut self, events: I) -> impl IntoIterator<Item = Event>
    where
        I: IntoIterator<Item = Event>,
    {
        events
            .into_iter()
            .filter_map(|event| (self.func)(event))
            .collect::<Vec<Event>>()
    }
}
