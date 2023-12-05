#!/bin/bash

for folder in "${@}"; do
  folder_clean=$(basename "$(realpath "${folder}")")
  version_file="${folder_clean}/python_versions.txt"
  if [ ! -e "${version_file}" ]; then
    version_file="default_python_versions.txt"
  fi

  python_versions=$(cat "${version_file}")

  for python_version in ${python_versions}; do
    echo "{\"folder\": \"${folder_clean}\", \"python-version\":\"${python_version}\"}"
  done
done | jq -s -c
