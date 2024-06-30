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
    use metio::client;
    use metio::streams::Stream;

    let cfg = client::Config {
        host: "localhost".to_string(),
        prefix: None,
    };

    let client = client::connect(cfg).await.map_err(|e| e.to_string())?;
    let _ = Stream::new("peter".to_string(), client);

    Ok(())
}
