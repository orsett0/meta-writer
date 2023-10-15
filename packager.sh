#!/usr/bin/env bash

# meta-writerCopyright 2023 Alessio Orsini <alessiorsini.ao@proton.me>
# SPDX-License-Identifier: Apache-2.0
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

function join {
  local d=${1-} f=${2-}
  if shift 2; then
    printf %s "$f" "${@/#/$d}"
  fi
}

echo "Generating package.json"

sections=($(grep -E '^\[.*\]$' Cargo.toml | grep -Fv '[package]'))
in_right_section=false

while IFS="" read -r line || [ -n "$p" ]
do
    if ! $in_right_section && [[ "$line" != "[package]" ]]; then
        continue
    elif [[ "$line" == "[package]" ]]; then
        in_right_section=true
        continue
    elif [[ " ${sections[*]} " == *" $line "* ]]; then
        break
    fi

    line=$(echo $line | sed -E 's/^authors = \[(.*)\]/author = \1/')
    line=$(echo $line | sed -e 's/^publish = false/private = true/')
    line=$(echo $line | sed -e 's/^name = "meta-writer"$/name = "@orsetto\/meta-writer"/')

    val=$(echo $line | sed -E 's/^( )+(.*)( )+$/\2/' | sed -E 's/^(.*) = (.*)$/\"\1\": \2/')
    [ ! -z "$val" ] && values+=("$val")
done < Cargo.toml

values+=('"type": "module"')
values+=('"main": "loader.js"')
values+=('"dependencies": { "suppress-experimental-warnings": "^1.1.17" }')
values+=('"engines": { "node": ">=14" }')

printf "{\n  " { > pkg/package.json
join $',\n  ' "${values[@]}" >> pkg/package.json
printf "\n}\n" >> pkg/package.json
