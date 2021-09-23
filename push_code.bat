@echo off

cargo fmt
git add .
git commit --amend --no-edit
git push origin master --force