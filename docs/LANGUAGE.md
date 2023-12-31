# Docs
This give us a final vision of the language.

# Control Flow

Check if a condition is true and in case it is, enter next scope.
```
if a:
    pass
```

Check if any value match "a" and enter the scope in case is true.
```
match a:
    b:
        pass
    c:
        pass
    d, e, f:
        pass
    g if a > 10:
        pass
    _:
        pass
```

A same scope can be used to many values.
```
match a:
    b:
        pass
    c:
        pass
    d, e, f:
        pass
```

If doesn't match any scope, it can go to default scope.
```
match a:
    b:
        pass
    c:
        pass
    _:
        pass
```

Condition can be applied to decide if should enter the scope.
```
match a:
    b:
        pass
    c if d == true:
        pass
```

Loop next scope until other control flow command exit.
```
loop:
    pass
```

Loop next scope while condition is true.
```
while a:
    pass
```

Iterate over "b", saving the current value in temporary variable.
```
for a in b:
    pass
```

If a number is provided for the iteration, it will iterate from 0 to the number.
```
for a in 10:
    pass
```

Exit function.
```
func a():
    return
```

Return a value from function.
```
func a():
    return b
```

Return a value and error from function.
```
func a():
    return b, c
```

An statement that does nothing.
```
pass
```

Emit a signal to anyone listening the signal.
```
emit a
```

Arguments can be passed to listening signals.
```
emit a(b, c, d)
```

Suspend the funciton until signal is emitted.
```
await a
```

Can receive a return value from the signal.
```
b = await a
```

Suspend a coroutine.
```
coroutine a():
    yield
```

Coroutines can return value when suspended.
```
coroutine a():
    yield b
```

Resume a coroutine.
```
b = a()
resume b
```

Arguments can be passed to resumed coroutine.
```
b = a()
resume b(c, d)
```

Execute "else" scope in case "if" scope is not executed.
```
if a:
    pass
else:
    pass
```

Can be chained if others "if".
```
if a:
    pass
else if b:
    pass
else:
    pass
```

Exit loop/while/for.
```
loop:
    break

while a:
    break

for a in b:
    break
```

Exit loops in chain.
```
loop:
    loop:
        loop:
            break 3
```

Start next loop iteration.
```
loop:
    if a:
        continue
    else:
        pass
```