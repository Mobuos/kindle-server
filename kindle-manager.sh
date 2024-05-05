#!/usr/bin/env bash

set -Eeuo pipefail
trap cleanup SIGINT SIGTERM ERR EXIT

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)

usage() {
  cat <<EOF
Usage: $(basename "${BASH_SOURCE[0]}") [-h] -l location -a address -d -p -g filename

Utility script for managing files in the kindle

Available options:
-l, --location  Defines working location, default is /mnt/us/images
-a, --address   Defines address of kindle device, to be used in ssh
-d, --delete    Deletes a given file
-p, --push      Pushes a given file
-s, --set       Clears and sets an image on the kindle to display
-g, --get-all   Gets all file names on kindle
-b, --battery   Get current battery
--prep          Prepare device to act as display
-h, --help      Print this help and exit
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
  aux=0

  while :; do
    case "${1-}" in
    -h | --help) usage ;;
    --no-color) NO_COLOR=1 ;;
    -d | --delete) DELETE=1 ;;
    -p | --push) PUSH=1 ;;
    -g | --get-all) GET_ALL=1 ;;
    -s | --set) SET=1 ;;
    -b | --battery) BATTERY=1 ;;
    --prep) PREP=1 ;;
    -l | --location)
      location="${2-}"
      shift
      ;;
    -a | --address)
      address="${2-}"
      shift
      ;;
    -?*) die "Unknown option: $1" ;;
    *) break ;;
    esac
    shift
  done

  args=("$@")

  # check required params and arguments
  [[ -z "${location-}" ]] && location="/mnt/us/images"
  [[ -z "${address-}" ]] && die "Missing required parameter: address"

  if [[ -n ${DELETE-} || -n ${PUSH-} || -n ${SET-} ]] ; then
    [[ ${#args[@]} -eq 0 ]] && die "Missing file name - Required for delete, push and set"
    file=${args[0]}
    filename=$(basename "${file}")
  fi

  return 0
}

###
# Main
###

parse_params "$@"
setup_colors

if [[ -n ${GET_ALL-} ]]; then
  msg "${BLUE}> Printing all files found${NOFORMAT}"
  ssh ${address} "ls ${location}"

elif [[ -n ${PUSH-} ]]; then
  ssh ${address} "[ -f ${location}/${filename} ]" && die "${RED}Error: ${NOFORMAT}${filename} already exists on kindle"
  msg "${BLUE}> Pushing ${filename}${NOFORMAT}"
  scp ${file} ${address}:${location}/${filename} && msg "Success"

elif [[ -n ${DELETE-} ]]; then
  ssh ${address} "! [ -f ${location}/${filename} ]" && die "${RED}Error: ${NOFORMAT}Could not find ${filename} on kindle"
  msg "${BLUE}> Deleting ${filename}${NOFORMAT}"
  ssh ${address} "rm ${location}/${filename}" && msg "Success"

elif [[ -n ${SET-} ]]; then
  ssh ${address} "! [ -f ${location}/${filename} ]" && die "${RED}Error: ${NOFORMAT}Could not find ${filename} on kindle"
  msg "${BLUE}> Setting ${filename}${NOFORMAT}"
  ssh ${address} "eips -c; eips -f; eips -g ${location}/${filename}"

elif [[ -n ${BATTERY-} ]]; then
  ssh ${address} "gasgauge-info -c"

elif [[ -n ${PREP-} ]]; then
  ssh ${address} "lipc-set-prop -i com.lab126.powerd preventScreenSaver 1 && stop framework && stop powerd" && msg "Finished"
fi
