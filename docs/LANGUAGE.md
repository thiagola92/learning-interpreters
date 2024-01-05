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

a()
```

Functions can include parameters.    
```
func a(b: int, c: float):
    pass

a(10, 5.0)
```

And return type.  
```
func a(b: int, c: float) -> int:
    pass

print(a(10, 5.0))
```

Declare coroutine.  
```
coroutine a():
    pass

var co = a()
resume co()
```

Coroutines can include parameters to be used in the initialization.  
```
coroutine a(b: int, c: float):
    pass

var co = a(10, 5.0)
resume co()
```

And return type.  
```
coroutine a(b: int, c: float) -> int:
    pass

var co = a(10, 5.0)
print(resume co())
```

Coroutines can be suspended and resume, but it's not possible to know the types in this cases.  

Declare struct.  
```
struct a:
    b: int
    c: float

var z = a(10, 5.0)
```

Note that structs values are initialized on creation, you can't intialize later.  

Declare class, this class can't be initialized because doesn't contains a construct.  
```
class a:
    pass
```

Classes can have properties, in this case they are not acessible because they are private by default.  
```
class a:
    var b: int
    var c: float
```

Having an constructor let others initialize your class, but they are private by default so this can't be initialized.  
```
class a:
    var b: int
    var c: float

    construct(b: int, c: float):
        self.b = b
        self.c = c
```

Public will let you create instances with this class.  
```
class a:
    public var b: int
    public var c: float

    public construct(b: int, c: float):
        self.b = b
        self.c = c

var x = a(10, 5.0)
print(x.b)
print(x.c)
```

Declare singleton, they are classes that can only be initialized once.  
```
singleton a:
    public var b: int
    public var c: float

    public construct(b: int, c: float):
        self.b = b
        self.c = c

var x = a(10, 5.0)
var z = a(5, 10.0)

# Constructor can still be called multiple times, but always return the same instance.
print(x.b, x.c) # 10 5.0
print(z.b. z.c) # 10 5.0
```

Declare an empty interface (any class satisfy this condition).  
```
interface a:
    pass
```

This make classes that implement this interface have this 3 methods.  
```
interface a:
    a()
    b(a: int)
    c(a: int, b: float)
```


Constructors allow your class to be initialized.  
```
class a:
    var b: int
    var c: float

    public construct(b: int, c: float):
        self.b = b
        self.c = c

var x = a(10, 5.0)
```


You can have multiple constructors. They use the signature to know which to use.  
```
class a:
    var b: int
    var c: float

    public construct(b: int, c: float):
        self.b = b
        self.c = c

    public construct(b: int):
        self.b = b
        self.c = 25.0

var x = a(10, 5.0)
var z = a(10)
```

Destructor is called when the class is freed.  
```
class a:
    var b: int
    var c: float

    public construct(b: int, c: float):
        self.b = b
        self.c = c

    destructor:
        print("bye")

var x = a(10, 5.0)
# Will leave scope and call destructor
```