#!/bin/bash

function gen_test_config() {
  folder="$1"
  language="$2"

  version_file="${folder}/${language}_versions.txt"
  if [ ! -e "${version_file}" ]; then
    version_file="default_${language}_versions.txt"
  fi

  versions=$(cat "${version_file}")

  for version in ${versions}; do
    echo "{\"folder\": \"${folder}\", \"language\":\"${language}\", \"version\":\"${version}\"}"
  done
}

for folder in "${@}"; do
  folder_clean=$(basename "$(realpath "${folder}")")

  if [ -e "${folder_clean}/__init__.py" ]; then
    gen_test_config "${folder_clean}" "python"
  fi

  if [ -e "${folder_clean}/Cargo.toml" ]; then
    gen_test_config "${folder_clean}" "rust"
  fi

done | jq -s -c
