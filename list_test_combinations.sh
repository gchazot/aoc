#!/bin/bash

for folder in "${@}"; do
  folder_clean=$(basename "$(realpath "${folder}")")
  version_file="${folder_clean}/python_versions.txt"
  if [ -e "${version_file}" ]; then
    python_versions=$(cat "${version_file}")
  else
    python_versions="3.10"
  fi

  for python_version in ${python_versions}; do
    echo "{\"folder\": \"${folder_clean}\", \"python-version\":\"${python_version}\"}"
  done
done | jq -s -c
