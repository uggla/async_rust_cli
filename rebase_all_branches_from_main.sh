#!/bin/bash

# Rebase all code branches from main change.
# It basically update documentation in all code branches.

set -euo pipefail
arg=${1:-}
arg=$(echo "$arg" | tr '[:upper:]' '[:lower:]')

branches=$(git branch -a | grep -P '/\d\d-' | sed 's/^.\+\///' | sort -u)

cd "$(dirname "$0")"

if [[ "${arg[0]}" == "-f" ]]; then
  for branch in $branches; do
    git push --force-with-lease origin "$branch"
  done
  exit 0
fi

solution_branches=$(echo "$branches" | grep -E -- '-solution$' || true)
latest_solution_branch=$(echo "$solution_branches" | sort | tail -n 1)

if [[ -z "$latest_solution_branch" ]]; then
  echo "No solution branches found."
  exit 1
fi

echo "Rebasing from $latest_solution_branch onto main. If conflicts occur, resolve them and continue the rebase."
git checkout "$latest_solution_branch"
git rebase -i main --update-ref

non_solution_branches=$(echo "$branches" | grep -Ev -- '-solution$' || true)

for branch in $non_solution_branches; do
  commit_id=$(git rev-parse "$branch")
  solution_branch="${branch}-solution"

  echo "Recreating $branch from $solution_branch. If cherry-pick fails, resolve conflicts then continue manually."
  git checkout "$solution_branch"
  git branch -D "$branch"
  git checkout -b "$branch"
  git cherry-pick "$commit_id"
done

git checkout main
exit 0
