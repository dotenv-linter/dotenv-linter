# Duplicated Key

Detects if a key is not unique:

### ❌ Wrong

```env
FOO=BAR
FOO=BAR
```
### ✅ Correct

```env
FOO=BAR
BAR=FOO
```