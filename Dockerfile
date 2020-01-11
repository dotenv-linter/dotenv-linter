FROM alpine:latest

COPY dotenv-linter /usr/local/bin/

ENTRYPOINT ["dotenv-linter"]
