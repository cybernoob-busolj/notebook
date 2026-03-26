gpush() {
    DEST="/home/busolj/Documents/to-github"

    SOURCES=(
        "/home/busolj/Desktop/Notes"
        "/home/busolj/Desktop/Scripts"
    )

    for SRC in "${SOURCES[@]}"; do
        rsync -av --update "$SRC/" "$DEST/$(basename "$SRC")/"
    done

    cd "$DEST" || return

    git add .

    if ! git diff --cached --quiet; then
        MSG="Commit automatico $(date '+%Y-%m-%d %H:%M')"
        git commit -m "$MSG"
        git push origin main
        echo "[+] Commit realizado: $MSG"
    else
        echo "[*] Nenhuma alteração para commit"
    fi
}