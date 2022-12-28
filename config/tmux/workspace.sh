#!/bin/bash

################################################################################
# Sample to build tmux workspace
################################################################################

REPODIR="~/repo/knowledge"
WORKDIR="~/repo/knowledge"

# nvim
tmux rename-window nvim
tmux send-keys -t 1 "cd $WORKDIR" C-m
tmux send-keys -t 1 "nvim" C-m

# git
tmux new-window
tmux rename-window git
tmux send-keys -t 1 "cd $REPODIR" C-m
tmux send-keys -t 1 "git status" C-m

# cmd
tmux new-window
tmux rename-window cmd

# select window
tmux select-window -t nvim
