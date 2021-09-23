@echo off

cargo fmt
git add .
git commit --amend --force
echo :wq
git push origin master