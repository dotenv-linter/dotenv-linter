# Incorrect delimiter

Detects if a key does not use an underscore to separate words:

### ❌ Wrong

```env
FOO-BAR=FOOBAR
```

### ✅ Correct

```env
FOO_BAR=FOOBAR
```
