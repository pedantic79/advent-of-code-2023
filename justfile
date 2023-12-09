commit-input:
    cd input && [[ -n $(git status -s) ]] && git add . && git commit -m "$(git status --porcelain | sed 's/A /Add/')" && git push

