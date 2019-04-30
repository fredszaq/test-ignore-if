# `test-ignore-if`

A rust proc macro to conditionnaly ignore tests

## Supported conditions
 - environement variables set at build time
 
 
## Use it

First, you need to add this line to your `build.rs`

```rust
test_ignore_if_utils::enable_ignore_if_env_set_for("CI")
```
This ensures your code is recompiled if the `CI` environment variable 
changes, and tels the proc macro you've done enabled this

Then you can annotate your tests
```rust
use test_ignore_if::ignore_if;

#[ignore_if(env_set="CI")]
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
} 
```
The test will be ignored if the `CI` env var was set at compile time

## Licence

```
Copyright 2019 Thibaut Lorrain

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
