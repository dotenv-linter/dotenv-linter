# GitHub Actions

Dotenv-linter can also be used within our official [dotenv-linter action](https://github.com/dotenv-linter/action-dotenv-linter) through [GitHub Actions](https://github.com/features/actions)

### Usage

Below is a simple snippet to use this action in your workflow:

<summary>Example: <code>.github/workflows/dotenv_linter.yml</code></summary>

```yaml
name: dotenv-linter
on: [pull_request]
jobs:
  dotenv-linter:
    name: runner / dotenv-linter
    runs-on: ubuntu-latest
    steps:
      - name: Check out code
        uses: actions/checkout@v1
      - name: dotenv-linter
        uses: dotenv-linter/action-dotenv-linter@v2
        with:
          github_token: ${{ secrets.github_token }}
```
