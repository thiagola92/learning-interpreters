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

# Definition

Declare a variable without value.  
```
var a
```

Declare a variable with a value.
```
var a = 10
```

Declare a constant with a value, constants are read only.
```
const a = 10
```

Declare an enum.
```
enum a:
    b
    c
    d
    e
    f
    g
    h
```

Enums can be used as tagged union, so each tag holds data.
```
enum a:
    b(int)
    c(float)
    d(string)
    e(int, int)
    f(int, float)
    g

var z

z = a.b(10)
print(z[0])

z = a.c(5.0)
print(z[0])

z = a.d("string")
print(z[0])

z = a.e(10, 20)
print(z[0], z[1])

z = a.f(20, 5.0)
print(z[0], z[1])

z = a.g
```

Passing signal/func/coroutine signatures is also possible, this will enforce the types.  
```
enum a:
    b(signal(int, float)),
    c(func(int, int) -> float),
    d(coroutine(float, float) -> int),

var z

z = a.b(mysignal)
emit z[0](10, 5.0)

z = a.c(myfunc)
z[0](10, 10)

z = a.d(mycoro)
y = z[0](5.0, 5.0)
resume y
```

Passing struct/class signature is also possible, this will enforce being an instance of it.  
```
enum a:
    b(StructFoo),
    c(ClassFoo),

var z

z = a.b(StructFoo())
print(z[0].struct_field)

z = a.c(ClassFoo())
print(z[0].class_field)
```

Identifiers can be used.  
```
enum a:
    b(c: int, d: float),
    c(e: ClassFoo),

var z

z = a.b(10, 5.0)
print(z.c, z.d)

z = a.c(ClassFoo())
print(z.e.class_field)
```

Declare signal.  
```
signal a
```

Signals can send data too.  
```
signal a(b: int, c: float)
```

Declare function.  
```
func a():
    pass
```

Functions can include parameters.    
```
func a(b: int, c: float):
    pass
```

And return type.  
```
func a(b: int, c: float) -> int:
    pass
```