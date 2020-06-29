# Space character

Detects lines with a whitespace around equal sign character `=`:

```env
❌ Wrong
FOO =BAR

❌ Wrong
FOO= BAR

❌ Wrong
FOO = BAR

✅ Correct
FOO=BAR
```
