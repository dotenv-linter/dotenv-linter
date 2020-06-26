# Unordered Key

Detects if a key is not alphabetically ordered:

### ❌ Wrong

```env
FOO=BAR
BAR=FOO

```

### ✅ Correct

```env
BAR=FOO
FOO=BAR
```

You can use blank lines to split lines into groups (will be available in [v2.1.0](https://github.com/dotenv-linter/dotenv-linter/issues/217)):

### ❌ Wrong

```env
FOO=BAR
BAR=FOO
```

### ✅ Correct

```env
FOO=BAR

BAR=FOO
```