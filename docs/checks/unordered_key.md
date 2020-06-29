# Unordered Key

Detects if a key is not alphabetically ordered:

```env
❌ Wrong
FOO=BAR
BAR=FOO

✅ Correct
BAR=FOO
FOO=BAR
```

You can use blank lines to split lines into groups (will be available in [v2.1.0](https://github.com/dotenv-linter/dotenv-linter/issues/217)):

```env
❌ Wrong
FOO=BAR
BAR=FOO

✅ Correct
FOO=BAR

BAR=FOO
```
