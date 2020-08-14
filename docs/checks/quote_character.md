# Quote character

Detects if a value contains quote characters (`'` / `"`):

```env
❌ Wrong
FOO="BAR"

❌ Wrong
FOO='BAR'

❌ Wrong
FOO='B"AR'

✅ Correct
FOO=BAR
```
