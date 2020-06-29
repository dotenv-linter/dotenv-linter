# Leading character

Detects if a line starts with an unallowed character (characters from `A` to `Z` and `_` (underscore) are allowed):

```env
❌ Wrong
 FOO=BAR

❌ Wrong
.FOO=BAR

❌ Wrong
*FOO=BAR

❌ Wrong
1FOO=BAR

✅ Correct
FOO=BAR

✅ Correct
_FOO=BAR
```
