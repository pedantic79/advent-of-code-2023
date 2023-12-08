commit-input:
    cd input && git add . && git commit -m "$(git status --porcelain | sed 's/A /Add/')" && git push

