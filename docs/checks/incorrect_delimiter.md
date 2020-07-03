# Incorrect delimiter

Detects if a key does not use an underscore to separate words:

```env
❌ Wrong
FOO-BAR=FOOBAR

✅ Correct
FOO_BAR=FOOBAR
```
