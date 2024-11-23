
# Git submodule update
git submodule update --init --recursive
git submodule foreach git pull origin main --rebase
