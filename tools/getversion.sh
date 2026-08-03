# This gets the MPack version string out of the source code. It
# is meant to be sourced by other scripts.

MAJOR=`grep '^#define MPACK_VERSION_MAJOR' reference-c/mpack/mpack-common.h|sed 's/.* \([0-9][0-9]*\) .*/\1/'`
MINOR=`grep '^#define MPACK_VERSION_MINOR' reference-c/mpack/mpack-common.h|sed 's/.* \([0-9][0-9]*\) .*/\1/'`
PATCH=`grep '^#define MPACK_VERSION_PATCH' reference-c/mpack/mpack-common.h|sed 's/.* \([0-9][0-9]*\) .*/\1/'`

VERSION="$MAJOR.$MINOR"
if [[ "$PATCH" -gt 0 ]]; then
    VERSION="$VERSION.$PATCH"
fi
