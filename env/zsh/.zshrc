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
alias gm="git merge"

# docker
alias dc="sudo docker-compose"
alias dcb="sudo docker-compose build"
alias dcu="sudo docker-compose up"
alias dcud="sudo docker-compose up -d"
alias dcd="sudo docker-compose down"
alias dce="sudo docker-compose exec"
alias dcp="sudo docker-compose ps"


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

function rprompt-git-current-branch {
	local branch_name st branch_status
 
	if [ ! -e  ".git" ]; then
		return
	fi

	branch_name=`git rev-parse --abbrev-ref HEAD 2> /dev/null`
	st=`git status 2> /dev/null`

	if [[ -n `echo "$st" | grep "^nothing to"` ]]; then
		branch_status="%F{green}"
	elif [[ -n `echo "$st" | grep "^Untracked files"` ]]; then
		branch_status="%F{red}?"
	elif [[ -n `echo "$st" | grep "^Changes not staged for commit"` ]]; then
		branch_status="%F{red}+"
	elif [[ -n `echo "$st" | grep "^Changes to be committed"` ]]; then
		branch_status="%F{yellow}!"
	elif [[ -n `echo "$st" | grep "^rebase in progress"` ]]; then
		echo "%F{red}!(no branch)"
		return
	else
		branch_status="%F{blue}"
	fi

	echo "${branch_status}[$branch_name]"
}
 
setopt prompt_subst

PROMPT='%F{green}[%D %T]%f %n%F{magenta}@%f%~ $ '
RPROMPT='`rprompt-git-current-branch`'

################################################################################
# Set option
################################################################################

setopt correct
setopt print_eight_bit
setopt ignore_eof
setopt interactive_comments
setopt extended_glob
