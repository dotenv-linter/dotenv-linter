# Leading character

Detects if a line starts with an unallowed character (characters from `A` to `Z` and `_` (underscore) are allowed):

### ❌ Wrong

```env
 FOO=BAR

.FOO=BAR

*FOO=BAR

1FOO=BAR
```

### ✅ Correct

```env
FOO=BAR

_FOO=BAR
```
