#!/usr/bin/env bash

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

    val=$(echo $line | sed -E 's/^( )+(.*)( )+$/\2/' | sed -E 's/^(.*) = (.*)$/\"\1\": \2/')
    [ ! -z "$val" ] && values+=("$val")
done < Cargo.toml

values+=('"type": "module"')
values+=('"main": "index.js"')

printf "{\n  " { > pkg/package.json
join $',\n  ' "${values[@]}" >> pkg/package.json
printf "\n}\n" >> pkg/package.json
