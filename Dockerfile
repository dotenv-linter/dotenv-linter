FROM alpine:latest

COPY dotenv-linter /usr/local/bin/

CMD ["dotenv-linter"]
