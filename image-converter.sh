#!/usr/bin/env bash

set -Eeuo pipefail
trap cleanup SIGINT SIGTERM ERR EXIT

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)

usage() {
  cat << EOF
Usage: $(basename "${BASH_SOURCE[0]}") [-h] file

Converts a given image into a image able to be parsed by a Kindle Paperwhite.

Available options:
-b, --background  Changes background used when image doesn't fit the resolution
-o, --overwrite   Allows overwriting of already existing image file
-h, --help        Print this help and exit
EOF
  exit
}

cleanup() {
  trap - SIGINT SIGTERM ERR EXIT
  # script cleanup here
}

setup_colors() {
  if [[ -t 2 ]] && [[ -z "${NO_COLOR-}" ]] && [[ "${TERM-}" != "dumb" ]]; then
    NOFORMAT='\033[0m' RED='\033[0;31m' GREEN='\033[0;32m' ORANGE='\033[0;33m' BLUE='\033[0;34m' PURPLE='\033[0;35m' CYAN='\033[0;36m' YELLOW='\033[1;33m'
  else
    NOFORMAT='' RED='' GREEN='' ORANGE='' BLUE='' PURPLE='' CYAN='' YELLOW=''
  fi
}

msg() {
  echo >&2 -e "${1-}"
}

die() {
  local msg=$1
  local code=${2-1} # default exit status 1
  msg "$msg"
  exit "$code"
}

parse_params() {
  # Default values for params
  background='white'

  while :; do
    case "${1-}" in
    -h | --help) usage ;;
    --no-color) NO_COLOR=1 ;;
    -o | --overwrite) OVERWRITE=1 ;;
    -b | --background)
      background="${2-}"
      shift
      ;;
    -?*) die "Unknown option: $1" ;;
    *) break ;;
    esac
    shift
  done

  args=("$@")

  # check required arguments
  [[ ${#args[@]} -eq 0 ]] && die "Missing file name"

  return 0
}

###
# Main
###

parse_params "$@"
setup_colors

file=${args[0]}
filename=$(basename "${file}")

# Check if necessary file for conversion exists
if ! [[ -f kindle_colors.gif ]]; then
  die "${RED}Error:${NOFORMAT} Missing kindle_colors.gif file"
fi

# Create folder for converted files
mkdir -p converted

# Check if target file name is already taken
if [[ -f converted/${filename} ]] && [[ -z "${OVERWRITE-}" ]]; then
  die "${RED}Error:${NOFORMAT} Command would overwrite file ${BLUE}converted/${filename}${NOFORMAT}\n   use --overwrite to proceed anyways"
fi

magick "${file}" -filter LanczosSharp -resize 758x1024 -background ${background-} \
         -gravity center -extent 758x1024 -colorspace Gray -dither FloydSteinberg \
         -remap kindle_colors.gif -quality 75 -define png:color-type=0 \
         -define png:bit-depth=8 "converted/${filename%.*}.png"

msg "Image converted!"
msg "   ${BLUE}converted/${filename}${NOFORMAT}"
