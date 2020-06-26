# Quote character

Detects if a value is wrapped in quotes:

### ❌ Wrong

```env
FOO="BAR"

FOO='BAR'
```

### ✅ Correct

```env
FOO=BAR
```
