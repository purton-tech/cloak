# Git aliases.
alias gst='git status'
alias gcm='git checkout main'
alias c=clear
alias gp='git push'
alias gcam='git commit -a -m'
alias gpsup="git push --set-upstream origin $(git symbolic-ref -q HEAD | sed -e 's|^refs/heads/||')"
#alias gpsup='git push --set-upstream origin $(git_current_branch)'
alias gcb='git checkout -b'
alias gitsetup='git config --global user.name \$NAME && git config --global user.email \$EMAIL && mkdir -p ~/.ssh && cp -u /home/host-ssh/id_rsa ~/.ssh && chmod 600 ~/.ssh/id_rsa && ssh-keygen -y -f ~/.ssh/id_rsa > ~/.ssh/id_rsa.pub'
alias gcr='f() { git checkout -b $1 origin/$1; }; f'
alias gdb='git branch | grep -v "main" | xargs git branch -D'

# Cargo watch
alias cw='mold -run cargo watch --no-gitignore -i "*.scss" -i "*.ts" -i node_modules -x run'
alias zs='zola serve --interface 0.0.0.0 --port 7104'

alias spell='docker run --rm -ti -v $HOST_PROJECT_PATH/www/content:/workdir tmaier/markdown-spellcheck:latest "**/*.md"'

# Zola
#alias zola-serve='docker run -u "$(id -u):$(id -g)" -v $HOST_PROJECT_PATH:/app --workdir /app -p 8080:8080 balthek/zola:0.14.0 serve --interface 0.0.0.0 --port 8080 --base-url development'
#alias zola='docker run -u "$(id -u):$(id -g)" -v $HOST_PROJECT_PATH:/app --workdir /app -p 8080:8080 balthek/zola:0.14.0'

# npm
alias nrs='npm run start'


alias db='psql $DATABASE_URL'

# Leave a line below or the files will cat together

