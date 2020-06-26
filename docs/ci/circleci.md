# CircleCI

Here is how to do it with [CircleCI](https://circleci.com):

<summary>Example: <code>.circleci/config.yml</code></summary>

```yaml
version: 2.1
jobs:
  dotenv-linter:
    docker:
      - image: circleci/rust:latest
    steps:
      - checkout
      - run:
          name: Run dotenv-linter
          command: |
            wget https://github.com/dotenv-linter/dotenv-linter/releases/latest/download/dotenv-linter-alpine-x86_64.tar.gz \
            -O - -q | tar -xzf -
            ./dotenv-linter
```