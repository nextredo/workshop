#!/usr/bin/env bash
# Old version of https://github.com/nextredo/nextredo.github.io/blob/master/hooks/pre-commit
# That script is much better at the intended purpose than this one
# But, this one's good to demonstrate complex shell usage
# (and maybe parallelise stuff that's harder to paralellise)
#
# A small script to remove sensitive EXIF data from images
# Modelled off https://dunkirk.sh/blog/remove-exif-git-hook/

# Check if exiftool is installed (using BASH builtin)
if ! command -v exiftool &> /dev/null; then
    echo "Error: exiftool is not installed.  Please install it." >&2
    exit 1
fi

# TODO modify so this works for arbitrary commands, rather than specifically exiftool
# The bones of the script are here, but it probably needs some work

# ------------------------------------------------------------------------------
# | Debugging / dev |
# --------------------
# Randomly return a failure
# rand_failure() {
#     # Adjust failure chance w/ number
#     return (( RANDOM % 20 )) && true || false
# }
# export -f rand_failure
# ------------------------------------------------------------------------------

# ------------------------------------------------------------------------------
# | Subshell goodies |
# --------------------

# ANSI Colour Codes
GREEN='\033[1;32m'
RED='\033[1;31m'
NC='\033[0m' # No colour

check_ret() {
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}SUCCESS${NC}" >&2
    else
        echo -e "${RED}FAILURE${NC}" >&2

        # So xargs exits immediately w/ failure
        exit 255
    fi
}

strip_exif() {
    # Remove most metadata on all supported files
    # Overwrites stripped files
    # exiftool -all= --icc_profile:all \
    #     -tagsfromfile @ -orientation \
    #     -ignoreMinorErrors \
    #     -overwrite_original "$1"
    echo "$1"
    check_ret
}

# Export so subshells can use various items
export -f strip_exif
export -f check_ret
export GREEN
export RED
export NC

# ------------------------------------------------------------------------------

# Shell is spawned in the repo root, so '.' works fine as the directory
# List git files (either in the index, or not in it)
# Pipe into xargs (parallelised, substituted as "{}" in command)
# Spawn in subshell so it has access to the function
# Run "strip_exif" in the subshell
    # "$@" expands all args (excl. $0 - i.e. $1, $2, $3...) given to the full subshell cmd 'strip_exif ...'
        # Quoted and split w spaces
    # The "-c" used for bash invocation provides "_" and "{}" as args $0 and $1 to the subshell cmd respectively
        # _ is the last arg of the prev command
        # {} is the positional substitution of whatever xargs is outputting (a line from the "git ls-files ..." cmd)

# NOTE: Add `-P $(nproc)` to xargs to parallelise the operation
#       May mean multiple failures happen before xargs fully exits
git ls-files --cached | \
    xargs -P $(nproc) -I {} bash -x -c 'strip_exif "$@"' _ {}

git ls-files --other | \
    xargs -P $(nproc) -I {} bash -x -c 'strip_exif "$@"' _ {}


# Verify it exited successfully
if [ $? -ne 0 ]; then
    echo "Error: exiftool failed to process a file." >&2
    exit 1
fi

# Exit successfully
exit 0
