# Lowercase key

Detects if a key has lowercase characters:

### ❌ Wrong

```env
FOo_BAR=FOOBAR

foo_bar=FOOBAR
```

### ✅ Correct

```env
FOO_BAR=FOOBAR
```
