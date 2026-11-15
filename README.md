# Kerf Language

## Variables

### Defining Variables

Variable Types:

 - num (its an i64)
 - dec (its an f64)
 - bol (its a boolean)

Convention:

?x = value;
?x \[type] = value;
(Type annotations are optional)

Example:

```kerf
?x = 10;
```

Forbidden:

```kerf
?x [num];
```

### Operations

You can do all the basic operations (+ - * / %), no others so far

### General Idea

All variables are dropped when they go out of scope (like Rust)

Example:

```kerf
?x = 10;
if x==10 {
  ?y = 3;
} -- var y is no longer availible for use after this "}"
```

## Functions

### Defining Functions

Convention:

?x(args\[args type]) \[return type] { do something }
(Type annotations not optional)

Example:

```kerf
?add_numbers(x[num], y[num]) [num] {
  return x + y;
}
```

## Control Flow

### If / Elif / Else

Convention:

if condition boolean { do something }

Example:

```kerf
if x==16 {
  print("x is 16");
} elif x==8 {
  print("x is 8");
} else {
  print("x isn't 8 or 16");
}
```

### While Loop

Convention:

while condition boolean { do something }

Example:

```kerf
while x!=10 {
  print(x);
  x += 1;
}
```
