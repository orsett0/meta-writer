/*
meta-writerCopyright 2023 Alessio Orsini <alessiorsini.ao@proton.me>
SPDX-License-Identifier: Apache-2.0

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use jzon::JsonValue;

fn main() {
    let val = &jzon::JsonValue::from(1);

    println!("{:?}", val.as_i8().unwrap_or(10).to_be_bytes());
}
