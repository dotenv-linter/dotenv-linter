# Extra Blank Line

Detects if a file contains more than one blank line in a row:

### ❌ Wrong
```env
A=B


FOO=BAR
```

```env
A=B
FOO=BAR


```

### ✅ Correct
```env
A=B

FOO=BAR

```

```env
A=B
FOO=BAR

```
