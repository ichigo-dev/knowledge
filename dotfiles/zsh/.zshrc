################################################################################
# Keybind
################################################################################

alias n="nvim"
alias ls="ls -F --color=auto"
alias la="ls -lAFhX --color=auto"

# git
alias g="git"
alias gs="git status"
alias ga="git add"
alias gaa="git add ."
alias gc="git commit"
alias gp="git push"
alias gpu="git pull"
alias gb="git branch"
alias gch="git checkout"
alias gl="git log"
alias glgg="git log --graph --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset' --abbrev-commit"
alias gf="git fetch"
alias gm="git merge"

# docker
alias dc="sudo docker-compose"
alias dcb="sudo docker-compose build"
alias dcu="sudo docker-compose up"
alias dcud="sudo docker-compose up -d"
alias dcd="sudo docker-compose down"
alias dce="sudo docker-compose exec"
alias dcp="sudo docker-compose ps"

# tmux
alias ta="tmux attach -t"
alias td="tmux detach"
alias tk="tmux kill-server"
alias tls="tmux list-sessions"
alias tns="tmux new-session -s"
alias trs="tmux rename-session"
alias tks="tmux kill-session -t"
alias tlw="tmux list-window"
alias tnw="tmux new-window"
alias trw="tmux rename-window"
alias tsw="(){tmux swap-window -s $1 -t $2}"

# rm
alias rm="trash-put"
alias rmf="rm"

# pacman
alias pclean="sudo pacman -Rs $(pacman -Qdtq)"
alias pup="sudo pacman -Syu"
alias pi="sudo pacman -S"
alias pr="sudo pacman -Rs"
alias pls="sudo pacman -Qe"
alias pfind="sudo pacman -Qi"


################################################################################
# man
################################################################################
man()
{
	env \
		LESS_TERMCAP_mb=$'\E[01;31m' \
		LESS_TERMCAP_md=$'\E[01;38;5;74m' \
		LESS_TERMCAP_me=$'\E[0m' \
		LESS_TERMCAP_se=$'\E[0m' \
		LESS_TERMCAP_so=$'\E[38;5;246m' \
		LESS_TERMCAP_ue=$'\E[0m' \
		LESS_TERMCAP_us=$'\E[04;38;5;146m' \
		man "$@"
}


################################################################################
# fzf
################################################################################

function fzf-cd()
{
	local dir
	dir=$(find ${1:-.} -path "*/\.*" -prune \
		-o -type d -print 2> /dev/null | fzf-tmux +m) &&
	cd "$dir"
}
zle -N fzf-cd
bindkey "^F" fzf-cd

function fzf-history()
{
	BUFFER=$(history -n -r 1 | fzf --no-sort +m --query "$LBUFFER" --prompt="History > ")
	CURSOR=$#BUFFER
}
zle -N fzf-history
bindkey "^R" fzf-history

function fzf-git-switch()
{
	local branches branch

	branches=$(git branch --all | grep -v HEAD) &&
	branch=$(echo "$branches" |
		fzf -d $(( 2 + $(wc -l <<< "$branches") )) +m) &&
	git switch $(echo "$branch" | sed "s/.* //" | sed "s#remotes/[^/]*/##")
}

function fzf-git-show()
{
	git log --graph --color=always \
		--format="%C(auto)%h%d %s %C(black)%C(bold)%cr" "$@" |
	fzf --ansi --no-sort --reverse --tiebreak=index --bind=ctrl-s:toggle-sort \
		--bind "ctrl-m:execute:
			(grep -o "[a-f0-9]\{7\}" | head -1 |
			xargs -I % sh -c "git show --color=always % | less -R") << "FZF-EOF"
			{}
			FZF-EOF"
}

function fzf-git-add()
{
	local out q n addfiles
	while out=$(
		git status --short |
		awk "{if (substr($0,2,1) !~ / /) print $2}" |
		fzf --multi --exit-0 --expect=ctrl-d); do

		q=$(head -1 <<< "$out")

		n=$[$(wc -l <<< "$out") - 1]
		addfiles=(`echo $(tail "-$n" <<< "$out")`)

		[[ -z "$addfiles" ]] && continue
		if [ "$q" = ctrl-d ]; then
			git diff --color=always $addfiles | less -R
		else
			git add $addfiles
		fi
	done
}

function fzf-kill()
{
	local pid
	pid=$(ps -ef | sed 1d | fzf -m | awk "{print $2}")

	if [ "x$pid" != "x" ]
	then
		echo $pid | xargs kill -${1:-9}
	fi
}


################################################################################
# Zsh Config
################################################################################

# environment variable
export LANG=en_US.UTF-8

# sheldon plugin manager
eval "$(sheldon source)"


################################################################################
# History
################################################################################

HISTFILE=$HOME/.zsh_history
HISTSIZE=100000
SAVEHIST=1000000

setopt inc_append_history
setopt share_history

setopt extended_history
setopt hist_save_no_dups
setopt hist_expire_dups_first
setopt hist_ignore_all_dups
setopt hist_ignore_space
setopt hist_reduce_blanks


################################################################################
# Prompt
################################################################################

function prompt-git-current-branch
{
	local branch_name st branch_status
 
	if [ ! -e  ".git" ]; then
		return
	fi

	branch_name=`git rev-parse --abbrev-ref HEAD 2> /dev/null`
	st=`git status 2> /dev/null`

	if [[ -n `echo "$st" | grep "^nothing to"` ]]; then
		prompt_color="%F{green}"
		branch_status=""
	elif [[ -n `echo "$st" | grep "^Untracked files"` ]]; then
		prompt_color="%F{red}"
		branch_status="*"
	elif [[ -n `echo "$st" | grep "^Changes not staged for commit"` ]]; then
		prompt_color="%F{red}"
		branch_status="+"
	elif [[ -n `echo "$st" | grep "^Changes to be committed"` ]]; then
		prompt_color="%F{yellow}"
		branch_status="!"
	elif [[ -n `echo "$st" | grep "^rebase in progress"` ]]; then
		echo "%F{red}!(no branch)"
		return
	else
		prompt_color="%F{blue}"
		branch_status=""
	fi

	echo "${prompt_color}[$branch_name]${branch_status}%f"
}
 
setopt prompt_subst

PROMPT='
%F{green}[%D %T]%f %F{cyan}%n%f@%m %F{blue}[%d]%f `prompt-git-current-branch` 
$ '


################################################################################
# Set option
################################################################################

setopt correct
setopt print_eight_bit
setopt ignore_eof
setopt interactive_comments
setopt extended_glob

autoload -U compinit && compinit -u

# Set default editor
export EDITOR="nvim"

# Shell bindkey mode
setopt vi
bindkey "^S" history-incremental-search-forward
bindkey "^P" history-beginning-search-backward
bindkey "^N" history-beginning-search-forward
