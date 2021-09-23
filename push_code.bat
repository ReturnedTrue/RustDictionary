@echo off

cargo fmt
git add .
git commit --amend
echo :wq
git push origin master --force