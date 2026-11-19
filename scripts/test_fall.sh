set -euo pipefail

COMPILER="$1"
C_COMPILER="$2"
WORK_DIR="$3"
SOURCE=tests/runtime/fall.a
mkdir -p "${WORK_DIR}"
printf '\n[fall] unfathomable.\n       at %s:8:5\n\n' "${SOURCE}" > "${WORK_DIR}/expected"

"${COMPILER}" b "${SOURCE}" "${WORK_DIR}/fall.c"
if [[ "${C_COMPILER}" == *tcc* ]]; then
    "${C_COMPILER}" -std=c99 -D_POSIX_C_SOURCE=200809L "${WORK_DIR}/fall.c" -o "${WORK_DIR}/fall"
else
    "${C_COMPILER}" -std=gnu99 -D_POSIX_C_SOURCE=200809L "${WORK_DIR}/fall.c" -o "${WORK_DIR}/fall"
fi

expect_failure() {
    local status=0
    timeout 10s "$@" > "${WORK_DIR}/stdout" 2> "${WORK_DIR}/stderr" || status=$?
    if [[ "${status}" != 1 ]]; then
        printf 'Expected exit status 1, got %s\n' "${status}" >&2
        cat "${WORK_DIR}/stderr" >&2
        return 1
    fi
    if [[ -s "${WORK_DIR}/stdout" ]]; then
        cat "${WORK_DIR}/stdout" >&2
        return 1
    fi
    diff --strip-trailing-cr -u "${WORK_DIR}/expected" "${WORK_DIR}/stderr"
}

expect_failure "${WORK_DIR}/fall"
expect_failure "${COMPILER}" r "${SOURCE}"
