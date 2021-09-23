@echo off

cargo fmt
git add .
git commit --ammend --force
echo :wq
git push origin master