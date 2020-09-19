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

You can use blank lines to split lines into groups:

```env
❌ Wrong
FOO=BAR
BAR=FOO

✅ Correct
FOO=BAR

BAR=FOO
```

Control comments also split lines (this is done to make the linter logic more predictable, will be available in [v2.2.0](https://github.com/dotenv-linter/dotenv-linter/issues/238)):

```env
❌ Wrong
FOO=BAR
BAR=FOO

✅ Correct 
FOO=BAR
# dotenv-linter:off LowercaseKey
bar=FOO
```
