# Space character

Detects lines with a whitespace around equal sign character `=`:

### ❌ Wrong

```env
FOO =BAR

FOO= BAR

FOO = BAR
```

### ✅ Correct

```env
FOO=BAR
```
