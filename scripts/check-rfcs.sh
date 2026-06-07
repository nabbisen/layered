#!/usr/bin/env bash
# RFC directory hygiene checks (RFC 000 "Optional CI invariants").
# Run from the repository root: scripts/check-rfcs.sh
set -u

RFCS_DIR="rfcs"
fail=0

err() {
    echo "ERROR: $1" >&2
    fail=1
}

# 1. Filenames match NNN-slug.md.
for f in "$RFCS_DIR"/proposed/*.md "$RFCS_DIR"/done/*.md "$RFCS_DIR"/archive/*.md; do
    [ -e "$f" ] || continue
    base=$(basename "$f")
    echo "$base" | grep -Eq '^[0-9]{3,4}-[a-z0-9-]+\.md$' \
        || err "bad filename pattern: $f"
done

# 2. Status field matches folder (folder is the source of truth).
for f in "$RFCS_DIR"/proposed/*.md; do
    [ -e "$f" ] || continue
    head -20 "$f" | grep -q '^\*\*Status\.\*\* Proposed' \
        || err "status field does not say Proposed: $f"
done
for f in "$RFCS_DIR"/done/*.md; do
    [ -e "$f" ] || continue
    head -20 "$f" | grep -q '^\*\*Status\.\*\* Implemented' \
        || err "status field does not say Implemented: $f"
done
for f in "$RFCS_DIR"/archive/*.md; do
    [ -e "$f" ] || continue
    head -20 "$f" | grep -Eq '^\*\*Status\.\*\* (Withdrawn|Superseded)' \
        || err "status field does not say Withdrawn/Superseded: $f"
done

# 3. No RFC number duplicated across folders.
numbers=$(find "$RFCS_DIR" -name '[0-9]*.md' -printf '%f\n' | cut -d- -f1 | sort)
dupes=$(echo "$numbers" | uniq -d)
[ -z "$dupes" ] || err "duplicated RFC numbers: $(echo "$dupes" | tr '\n' ' ')"

# 4. Every RFC file is listed in the index, and every index link resolves.
for f in $(find "$RFCS_DIR" -name '[0-9]*.md' -printf '%P\n'); do
    grep -q "(\./$f)" "$RFCS_DIR/README.md" \
        || err "not listed in rfcs/README.md: $f"
done
while read -r link; do
    [ -e "$RFCS_DIR/${link#./}" ] || err "broken index link: $link"
done < <(grep -oE '\(\./(proposed|done|archive)/[^)]+\)' "$RFCS_DIR/README.md" \
    | tr -d '()')

# 5. Relative cross-links inside RFCs resolve. Fenced code blocks are
#    skipped: example links in the policy text are not real references.
strip_fences() {
    awk '/^[[:space:]]*```/ { fence = !fence; next } !fence' "$1"
}
for f in $(find "$RFCS_DIR" -name '*.md'); do
    dir=$(dirname "$f")
    while read -r link; do
        [ -e "$dir/$link" ] || err "broken link $link in $f"
    done < <(strip_fences "$f" \
        | grep -oE '\]\((\.\./)?(proposed|done|archive)/[0-9][^)]+\.md\)' \
        | sed 's/^](//; s/)$//')
done

if [ "$fail" -eq 0 ]; then
    echo "rfcs/: all checks passed"
else
    exit 1
fi
