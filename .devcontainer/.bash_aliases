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

# Watch
alias watch-app='mold -run cargo watch --workdir /workspace/ -w crates/primer-rsx -w crates/ui-components -w crates/grpc-api -w crates/axum-server -w crates/db -w crates/asset-pipeline/dist -w crates/asset-pipeline/images --no-gitignore -x "run --bin cloak"'
alias wa=watch-app
alias watch-es='mold -run cargo watch --workdir /workspace/ -w crates/external-secrets -w crates/grpc-api --no-gitignore -x "run --bin external-secrets"'
alias wes=watch-es
alias watch-pipeline='npm install --prefix /workspace/crates/asset-pipeline && npm run start --prefix /workspace/crates/asset-pipeline'
alias wp=watch-pipeline
alias watch-zola='cd /workspace/www && zola serve --drafts --interface 0.0.0.0 --port 7104 --base-url localhost'
alias wz=watch-zola

alias spell='docker run --rm -ti -v $HOST_PROJECT_PATH/www/content:/workdir tmaier/markdown-spellcheck:latest "**/*.md"'

# npm
alias nrs='npm run start'

# Database
alias dbmate='dbmate --no-dump-schema --migrations-dir /workspace/crates/db/migrations'
alias db='psql $DATABASE_URL'
alias adb='psql $APP_DATABASE_URL'
alias rdb='psql $RO_DATABASE_URL'

# Leave a line below or the files will cat together

