# Extra Blank Line

Detects if a file contains more than one blank line in a row:

```env
❌ Wrong
A=B


FOO=BAR
```

```env
❌ Wrong
A=B
FOO=BAR


```

```env
✅ Correct
A=B

FOO=BAR

```

```env
✅ Correct
A=B
FOO=BAR

```
