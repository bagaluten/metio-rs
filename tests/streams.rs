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

#[tokio::test()]
#[cfg(feature = "streams")]
async fn test_writing_to_streams() -> Result<(), String> {
    use metio::streams::Stream;
    use metio::types::{Event, EventType};
    use std::collections::HashMap;
    use std::str::FromStr;

    let client = metio::connect!("localhost")
        .await
        .map_err(|e| e.to_string())?;

    let s = Stream::new("peter".to_string(), client);

    let event_type = EventType::from_str("core/echo/1").map_err(|e| e.to_string())?;
    let events = vec![
        Event {
            event_id: "test_event".to_string(),
            object_id: None,
            event_type: event_type.clone(),
            timestamp: chrono::Utc::now(),
            payload: HashMap::new(),
        },
        Event {
            event_id: "test_event2".to_string(),
            object_id: None,
            event_type: event_type.clone(),
            timestamp: chrono::Utc::now(),
            payload: HashMap::new(),
        },
    ];

    s.publish(events).await.map_err(|e| e.to_string())?;

    Ok(())
}
