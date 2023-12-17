#!/bin/bash

for folder in "${@}"; do
  folder_clean=$(basename "$(realpath "${folder}")")

  if [ -e "${folder_clean}/__init__.py" ]; then
    language="python"
  elif [ -e "${folder_clean}/Cargo.toml" ]; then
    language="rust"
  else
    continue
  fi

  version_file="${folder_clean}/${language}_versions.txt"
  if [ ! -e "${version_file}" ]; then
    version_file="default_${language}_versions.txt"
  fi

  versions=$(cat "${version_file}")

  for version in ${versions}; do
    echo "{\"folder\": \"${folder_clean}\", \"language\":\"${language}\", \"version\":\"${version}\"}"
  done
done | jq -s -c
