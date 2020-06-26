# Key without value

Detects if a line has a key without a value:

### ❌ Wrong

```env
FOO
```

### ✅ Correct

```
FOO=

FOO=BAR
```
