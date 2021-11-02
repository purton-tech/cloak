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

# Cargo watch
alias cw='cargo watch --no-gitignore -i *.scss -i *.ts -i "package*" -x fmt -x clippy -x run'

# Zola
#alias zola-serve='docker run -u "$(id -u):$(id -g)" -v $HOST_PROJECT_PATH:/app --workdir /app -p 8080:8080 balthek/zola:0.14.0 serve --interface 0.0.0.0 --port 8080 --base-url development'
#alias zola='docker run -u "$(id -u):$(id -g)" -v $HOST_PROJECT_PATH:/app --workdir /app -p 8080:8080 balthek/zola:0.14.0'

# npm
alias nrs='npm run start'

# Database migrations
alias mr='diesel migration run'
alias mre='diesel migration redo'
alias ml='diesel migration list'
alias db='psql $DATABASE_URL'

alias p='sudo chmod 777 /var/run/docker.sock'
# Leave a line below or the files will cat together

