################################################################################
# Keybind
################################################################################

alias n='nvim'
alias ls='ls -F --color=auto'
alias la='ls -lAFhX --color=auto'

# git
alias g='git'
alias gs='git status'
alias ga='git add'
alias gaa='git add .'
alias gc='git commit'
alias gp='git push'
alias gpu='git pull'
alias gb='git branch'
alias gch='git checkout'
alias gl='git log'
alias glgg='git log --graph --pretty=format:"%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset" --abbrev-commit'
alias gf='git fetch'
alias gm='git merge'

# docker
alias dc='sudo docker-compose'
alias dcb='sudo docker-compose build'
alias dcu='sudo docker-compose up'
alias dcud='sudo docker-compose up -d'
alias dcd='sudo docker-compose down'
alias dce='sudo docker-compose exec'
alias dcp='sudo docker-compose ps'

# tmux
alias ta='tmux attach -t'
alias td='tmux detach'
alias tk='tmux kill-server'
alias tls='tmux list-sessions'
alias tns='tmux new-session -s'
alias trs='tmux rename-session'
alias tks='tmux kill-session -t'
alias tlw='tmux list-window'
alias tnw='tmux new-window'
alias trw='tmux rename-window'
alias tsw='(){tmux swap-window -s $1 -t $2}'


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
%F{green}[%D %T]%f %F{cyan}%n%f@%m %F{blue}[%d]%f`prompt-git-current-branch` 
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
bindkey "^R" history-incremental-search-backward
bindkey "^S" history-incremental-search-forward
bindkey "^P" history-beginning-search-backward
bindkey "^N" history-beginning-search-forward
