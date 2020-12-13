
# Home Exam D7050E


## Your repo

https://github.com/CarlOsterberg/D7050E

## Your syntax

Program
```ebnf
: FunctionDec+
;
```

FunctionDec

```ebnf
: "fn" Var "(" [ "," ], Params* ")" "->" Type "{" Statement* "};"
;
```
Params
```ebnf
: [ "mut" ] Var ":" [Unary] Type
;
```
Statement

```ebnf
: [ "let" ] Var ":" Type "=" Statement ";"
| [Deref] Var "=" Statement ";"
| "if" Expr "{" Statement* "};" ["else" "{" Statement* "};"]
| "while" Expr "{" Statement* "};"
| Expr
;
```

Expr

```ebnf
: Term,(Elemen|Factor|BoolAlg),Term
| (Prefix|Unary|Deref),Term
| Term
| "(" Expr ")"
(* Precende of operations is specified below *)
;
```

Precedence

```ebnf
(* Associativity is left to right, highest precedence at the top *)
"(" Expr ")"
FuncCall
(Prefix, Unary, Deref)
Factor
Elemen
BoolAlg
```

Elemen

```ebnf
: "+"
| "-"
;
```
Factor

```ebnf
: "*"
| "/"
;
```

BoolAlg

```ebnf
: "<"
| ">"
| "||"
| "&&"
| "||"
| "=="
;
```

Prefix

```ebnf
: "-"
| "!"
;
```

Unary

```ebnf
: "&mut"
| "&"
;
```

Deref
```ebnf
: "*"
;
```

Type

```ebnf
: "i32"
| "bool"
| "()"
;
```

Term

```ebnf
: [0-9]+
|Var
|Bool
|FuncCall
;
```
Bool
```ebnf
: "true"
| "false"
;
```
Var
```ebnf
: ( _|[a-z]|[A-Z] ),([a-z]|[A-Z]|[0-9]|_)*
;
```
FuncCall

```ebnf
: Var "(" [ "," ], Expr ")"
; (* Multiple Exprs are executed from left to right *)
```

Showcase

```rust
fn  i32_return(a:&mut  i32, mut  b:i32) ->  i32 {
	while  b<10 {
		b  =  b  +  1;
	};
	*a  =  3;
	-b
};
fn  bool_return(a:&bool, b:bool) ->  bool {
	if  !(*a  &&  b  ||  false) ==  true {
		true
	}; else {
		false
	};
};
fn  main() -> () {
	let mut a:i32 = 5;
	let b:bool = false;
	let d:bool = bool_return(&b, true);
	i32_return(&mut a, 1);
	let mut t:i32 = if d{3}; else {5};
};
```
The EBNF above describe a small subset of Rust with Function declarations with explicit return type being either Unit () or one of the primitive types.
The primitive types are i32 and boolean.
Supported statements are let, assignments, if/else, while, implicit returns all with explicit types.
Expression operands have a few different categories, infix operations which are split into arithmetic and boolean algebraic operations. In retrospect less than and greater than has been wrongly grouped as boolean algebraic operations, but for development it made sense because they all evaluate into a boolean. Supported prefix operations are negation, subtraction, reference, mutable reference and dereference. Parenthesized expression have precedence, prefix operations have precedence over infix operations and arithmetic operations before boolean operations. FactorOp has precedence over BinOp.
Error recovery is not implemented.


## Your semantics

  Symbols:

- σ, state
- σ', derived state
- ⇓, evaluates
- c, command
- x, variable
- e, expression

An expression can either be a
- n, number
- b, boolean
- f, function

For clarity v is used instead of number n or boolean b.

Commands can also evaluate into unit which defined as ().

Sequence

<img src="https://render.githubusercontent.com/render/math?math=\frac{<c0,\sigma> \Downarrow \sigma' <c1,\sigma'> \Downarrow <v,\sigma''>}{<c0%3Bc1,\sigma> \Downarrow <v, \sigma''>}">

```rust
let a:i32 = 5;
a();
```
A sequence is chain of commands where the first one is executed then the seconds command and so forth. If this wasn't the case the commands wouldn't influence the next ones correctly, the state σ' would be lost.

Operations

Infix:

@ = {+,-,*,/}

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e1, \sigma>\Downarrow n1, \sigma' <e2, \sigma'>\Downarrow n2,\sigma''}{e1 %40 e2,\sigma\Downarrow <n, \sigma''>} ">

\# = {<,>,==}

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e1, \sigma>\Downarrow n1, \sigma' <e2, \sigma'>\Downarrow n2,\sigma''}{e1 %23 e2,\sigma\Downarrow <b, \sigma''>} ">

? = {&&,||,==}

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e1, \sigma>\Downarrow b1, \sigma' <e2, \sigma'>\Downarrow b2,\sigma''}{e1 %3F e2,\sigma\Downarrow <b, \sigma''>} ">

Prefix: 

@ = {-}

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e, \sigma>\Downarrow <n, \sigma'>}{%40 e,\sigma\Downarrow <n,\sigma'>} ">

\# = {!}

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e, \sigma>\Downarrow <b, \sigma'>}{%23 e,\sigma\Downarrow <b,\sigma'>} ">


Here the operation * is dereference. Also: ? = {&}

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e, \sigma>\Downarrow <x,\sigma'> \sigma'[x] = ?y}{<*e, \sigma > \Downarrow y,\sigma'}">

For mutable references it looks pretty similar, ¤ = {&mut}

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e, \sigma>\Downarrow <x,\sigma'> \sigma'[x] = \text{¤}y}{<*e, \sigma > \Downarrow \text{mut y},\sigma'}">

And the other way around, ? = {&}

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e, \sigma>\Downarrow <*x,\sigma'> \sigma'[x] = y}{<?e, \sigma > \Downarrow y,\sigma'}">

And the same for mutable references ¤ = {&mut}

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e, \sigma>\Downarrow <*\text{mut x},\sigma'> \sigma'[x] = \text{mut y}}{<\text{¤}e, \sigma > \Downarrow \text{mut y},\sigma'}">

And on and on it goes ...




The signs represents one of the implemented arithmetic operations

- +, plus
- -, minus
- *, multiplication
- /, division
- ">", greater than
- "<", less than
- "==", equals
```rust
(1 + 2) * 3 / 2;
1 < 2;
5 == 5;
-1;
```
Operations for booleans are implemented in similar fashion and supports
- "!", not equal
- "&&", and
- "||", or 
- "==", equals
```rust
1 > 2 && true;
false || true;
!true;
```
For prefix the operations 

- "&"
- "&mut"
- "*" 

are also defined where a & or &mut makes a reference to a variable that the dereference operation * then accesses.

If/Else true

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e,\sigma>\Downarrow true <c1,\sigma>\Downarrow <v, \sigma'>}{<if\%3Btrue\%3Bthen\%3B c1 \%3Belse\%3Bc2, \sigma>\Downarrow <v, \sigma'>} ">

If/Else false 

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e,\sigma>\Downarrow false <c2,\sigma>\Downarrow <v, \sigma'>}{<if\%3Bfalse\%3Bthen\%3B c1 \%3Belse\%3Bc2, \sigma>\Downarrow <v, \sigma'>} ">

If true

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e,\sigma>\Downarrow true <c1,\sigma>\Downarrow <(), \sigma'>}{<if\%3Btrue\%3Bthen\%3B c1, \sigma>\Downarrow <(), \sigma'>} ">

If false

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e,\sigma>\Downarrow false <c1,\sigma>\Downarrow <(), \sigma>}{<if\%3Bfalse\%3Bthen\%3B c1, \sigma>\Downarrow <(), \sigma>} ">


```rust
if true {
	a = 1 + 1;
}; else {
	a=5+5;
};
if false {
	a();
};
```
 While true

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e,\sigma >\Downarrow true \%3B<c1,\sigma>\Downarrow (), \sigma' <while\%3B e \%3B do\%3B c, \sigma'>\Downarrow (), \sigma''}{<while\%3B true \%3B do \%3B c,\sigma>\Downarrow (), \sigma'' } ">

While false

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e,\sigma >\Downarrow false \%3B<c1,\sigma>\Downarrow (), \sigma' <while\%3B e\%3B do\%3B c, \sigma'>\Downarrow (), \sigma''}{<while\%3B false \%3B do \%3B c,\sigma>\Downarrow (), \sigma' } ">

```rust
while a<5 {
	a = a + 1;
};
```
Both if/else and while commands have their blocks evaluated or skipped based on the condition given.
If/else statements can all evaluate into a expression, but while cannot.

Let assignment

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e, \sigma>\Downarrow v}{<let \text{ } x \text{ %3A= } e, \sigma> \Downarrow <v, \sigma[x \text{%3A= v}]>} ">

```rust
let a:i32 = 5 * 3;
let mut b:bool = !(false || true); 
let c:i32 = true; //Error in type-checking
```
A let command is a unique declaration of a variable, if a new variable would be declared with the same name as another declared variable, the old one would be overwritten. Declarations for both i32 and bool follow the same pattern. References and mutable references work in the same way but mutable references to a declaration can only be made once at a time. 
```rust
let a:i32 = 5;
let mut b:&mut i32 = &mut a; 
let d:bool = true;
let e:&bool = &d;
```
Assignment

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e,\sigma> \Downarrow v }{<x\text{%3A=} e, \sigma> \Downarrow (),\sigma[x\text{%3A=} v]} ">

```rust
a = 5;
b = !false;
c = func();
d = &a;
e = &mut b;
```
The evaluated expression on the right side is moved to the already declared variable on the left. The left hand variable has to be declared mutable for assignments to be possible.

Function

The syntax for function declarations must be done as follows.

<img src="https://render.githubusercontent.com/render/math?math=f(p_0%3Apt_0,...,p_n%3Apt_n)\text{-> }t">

Then functions are evaluated as follows.

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e_0,\sigma> \Downarrow <v_0, \sigma'>  ... <e_n,\sigma^n> \Downarrow <n_n, \sigma^{n}><f.c,\sigma^{n}[p_0=v_0,...,p_n=v_n]>\Downarrow<v,\sigma^R>}{<f(e_0,e_1,...,e_n),\sigma>\Downarrow<v,\sigma^R>}">

```rust
5,7,true,&mut 5,a()
1+3+4+5,h,!false
```
Parameters are evaluated from left to right, this is important because some expressions may change the state.
A parameter may be of any type, except unit type, which is reserved for when a function shouldn't return anything.


```rust
a(5,1,true,!false);
b(c(),h*3,&g, &mut j);
```
A function call looks the same for each other type it can evaluate into. The function f takes the parameters p and evaluates into a value of some type. A function call can alter the state of the program.

```rust
fn a() -> i32 {
	let a:i32 = 5;
	a //returns 5
};
```

If the last command in a function evaluates to an expression, that is moved to the state of the function.

In Rust everything has a type, even statements, and following that implicit returns become easy to introduce. In the vein of following Rust implementation implicit returns are implemented. While the requirements stated explicit returns, implicit ones do the same thing, but with less freedom.  Other then that the implementation follows the requirements.

## Your type checker

The types implemented are

- i32
- bool
- unit
- Reference
- Mutable Reference

These are abstracted away to t.

Operations
Infix:

@ = {+,-,/,*}

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e1, \sigma>\Downarrow i32< e2, \sigma> \Downarrow i32}{<e1 %40 e2,\sigma>\Downarrow i32} ">

\# = {<,>,==}

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e1, \sigma>\Downarrow i32 < e2,\sigma > \Downarrow i32}{<e1 %23 e2,\sigma>\Downarrow bool} ">

? = {&&,||,==}

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e1, \sigma>\Downarrow bool <e2, \sigma>\Downarrow bool}{<e1 %3F e2,\sigma>\Downarrow bool} ">

Prefix: 

@ = {-}

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e1, \sigma>\Downarrow i32}{%40 e1,\sigma\Downarrow i32} ">

\# = {!}

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e1, \sigma>\Downarrow bool}{%23 e1,\sigma\Downarrow bool} ">

Here the operation * is dereference. Also: ? = {&}

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e, \sigma>\Downarrow <x> \sigma[x] = ?t}{<*e, \sigma > \Downarrow t}">

For mutable references it looks pretty similar, ¤ = {&mut}

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e, \sigma>\Downarrow <x> \sigma[x] = \text{¤}t}{<*e, \sigma > \Downarrow \text{mut t},}">

And the other way around, ? = {&}

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e, \sigma>\Downarrow <*x> \sigma[x] = t}{<?e, \sigma > \Downarrow t}">

And the same for mutable references ¤ = {&mut}

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e, \sigma>\Downarrow <*\text{mut x}> \sigma[x] = \text{mut t}}{<\text{¤}e, \sigma > \Downarrow \text{mut t}}">

And on and on it goes ...

```rust
1+1;
1 + true; //Error
```

The sign @ represents all the supported operations which are given in the section above. For bool type the operations look the same.


If/Else

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e,\sigma>\Downarrow bool <c1,\sigma>\Downarrow t, \sigma'<c2,\sigma'>\Downarrow t,\sigma''}{<if\%3Bbool\%3Bthen\%3B c1 \%3Belse\%3Bc2, \sigma>\Downarrow t, \sigma''} ">

```rust
if 5 {//Error not bool
	let a:i32 = 5;
};
if true {
	3
}; else {
	false
}; //Error both blocks must return the same
if true {
	1
}; else {
	5
}; //Works
```
To note is that aslong as both of c1 and c2 evaluate into the same type everything will work fine.

 if

 <img src="https://render.githubusercontent.com/render/math?math=\frac{<e,\sigma>\Downarrow <bool,\sigma'> <c,\sigma'>\Downarrow (),\sigma''}{<if\%3Bbool\%3Bthen\%3B c \%3B, \sigma>\Downarrow (),\sigma''} ">

 ```rust
if 5 {//Error not bool type
	let a:i32 = 5;
};
if true {};//works
```

while 

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e,\sigma>\Downarrow bool,\sigma' <c,\sigma'>\Downarrow (),\sigma''}{<while\%3Bbool\%3Bthen\%3B c \%3B, \sigma>\Downarrow (),\sigma''} ">

```rust
while 5 {//Error not bool type
	let a:i32 = 5;
};
while true {};//works
```
In the type-checker there is no big difference to a while loop or if/else. It checks both the condition and the block as it doesn't care about the actual value only the type that is evaluated into. To note is that if the if/else statement doesn't return anything its type becomes unit, otherwise it gets the type of the return, so it acts like a function without parameters. While is always handled as type unit.

Let assignment

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e, \sigma>\Downarrow t}{<let \text{ } x \text{%3A=} e, \sigma> \Downarrow (), \sigma[x \text{%3A=} t]} ">

```rust
let a:i32 = false;//error
let b:bool = 5;//error
let mut c:i32 = 3;//works
let t:&mut i32 = &mut c;//works
```

All types are implemented in a similar way to above. To note is that the type-checker doesn't implement any borrow checking as it doesn't keep track of what variable it was assigned to, only that variable type.

Assignment

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e,\sigma> \Downarrow t ,\sigma[x\text{=} t]}{<x \text{%3A=} e, \sigma>\Downarrow ()}">

```rust
let a:i32 = 1;//works
a = 5;//Error not mutable
let mut b:i32 = 5;
b = true;//Error type mismatch
b = 6;//works
let c:&i32 = &a;//works
let d:i32 = *c;//works
let e:&mut i32 = &mut b;
*e = 3;//works
```

The last example shows how references can be used with assignments, when left hand side is dereferenced it gets the variable b instead and assigns the left hand side to it.

Function

A function is defined as  follows

<img src="https://render.githubusercontent.com/render/math?math=f(p_0%3Apt_0,...,p_n%3Apt_n)\text{-> }f.t">

Then functions are evaluated as follows, to note is that f.c are the commands in the body of the function and f.t is the declared return type as seen above in the declaration.

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e_0,\sigma> \Downarrow pt_1  ... <e_n,\sigma> \Downarrow pt_n<f.c,\sigma>\Downarrow f.t}{<f(e_0,e_1,...,e_n),\sigma>\Downarrow<f.t,\sigma>}">


```rust
a(5,1,true,!false);
b(c(),h*3,&g, &mut j);
```
A function call looks the same for each other type it can evaluate into. In the typechecker the parameters are checked versus the arguments so that the types match, and then the return type is set as the function call. No evaluation of the function is needed here.


```rust
fn a() -> i32 {
	let a:i32 = 5;
	a //returns 5
};
```

The implicit return must match the declared return type of the function.

Of note about the typechecker is that it scans from top to bottom and checks each type. This means that function calls only do lookups in a hashmap for the function name and verifies the types. This hashmap is made at the start off the typechecker before any commands have been evaluated.

 To implement the underlying functionality for borrow checking assignments have to support dereference on the left side. This made me have to redo the parser to support operations on the left side.

## Your borrow checker

The borrow-checker checks operations surrounding references. References should not allow for data races, therefor if a variable is referenced, the referenced variable shouldn't be able to be changed. 
```rust
let mut a:i32 = 5;
let b:&i32 = &a;
a = 3; //Error
```
Mutable references can only be created to mutable variables. For references either is possible.
```rust
let a:i32 = 5;
let b:&mut i32 = &mut a; //Error
```
Multiple references are allowed to the same variable. 
```rust
let a:i32 = 5;
let b:&i32 = &a;
let c:&i32 = &a; //Works
```
Multiple mutable references aren't allowed to the same variable. 
```rust
let mut a:i32 = 5;
let b:&mut i32 = &a;
let c:&mut i32 = &a; //Error
```
Mutable references and references to the same variable cannot exist at the same time.
```rust
let mut a:i32 = 5;
let b:&i32 = &a;
let c:&mut i32 = &mut a; //Error
------------
let mut d:i32 = 11;
let e:&mut i32 = &mut d;
let f:&i32 = &d; //Error
```
Also because of ownership in rust, whenever a variables owner goes out of scope, the variable goes out of scope. This has implications for references because they don't own the variable they reference. Therefor references shouldn't have longer lifetimes then the variable they reference.
```rust
fn a() -> i32 {
	*b(5);
}
fn b(c:i32) -> &i32 {
	&c //Error
}
```



## Your LLVM/Crane-Lift backend (optional)

Not implemented.

## Overall course goals and learning outcomes.

  

Comment on the alignment of the concrete course goals (taken from the course description) to the theory presented, work You have done and knowledge You have gained. (I have put some comments in [...]).

  

- Lexical analysis, syntax analysis, and translation into abstract syntax.

I have learned a great deal about lexical analasys thanks to lalrpop. It made it digestible and easy to get of the ground. The only big issue i ran into was with how statements and expressions where to be handled. As i tried to treat them as the same i ran in to alot of ambiguity problems. When it comes to the AST that was built finding the correct structure to support everything took alot of iterations. Also realizing how things should be structured required that i understood how they should be used, which led to some backtracking when type-checking was started.

- Regular expressions and grammars, context-free languages and grammars, lexer and parser generators. [lalr-pop is a classical parser generator, it auto generated the lexer for you based on regular expressions but allows for you to define the lexer yourself for more control]

I know know what a lexer and context free grammar is and how it is used to implement powerful parsers around.
 
- Identifier handling and symbol table organization. Type-checking, logical inference systems. [SOS is a logical inference system]
  
 The type-checker gave me some insight into my own generated AST which made me able to rework it.
How SOS explains the compiler and catches these properties, the SOS given above is probably rather poor actual representation as i would need a some more iterations with more understanding of my own semantics before i encapsulate it correctly.

- Intermediate representations and transformations for different languages. [If you attended, Recall lectures relating LLVM/Crane-lift, discussions on SSA (Single Static Assignment) used in LLVM/Crane-lift, and discussions/examples on high [level optimization](https://gitlab.henriktjader.com/pln/d7050e_2020/-/tree/generics_and_traits/examples)]

An intermediate representation for code is made inbetween the parser and the actual machine code, often a VM of some sort that tries to execute the code. In other words some form of interpreter. During implementation this is where i handled both Rusts borrow-checking and the ownership model.

- Code optimization and register allocation. Machine code generation for common architectures. [Both LLVM/Crane-Lift does the "dirty work" of backend optimization/register allocation leveraging the SSA form of the LLVM-IR]

For code optimization i know have some knowledge regarding to how the compiler does it via the SSA. Also that depending on optimization levels the generated machine code program may differ from the written program. A method to verify correctness was made by CompCert, where they give proof of correctness aswell as optimised code.

- Comment on additional things that you have experienced and learned throughout the course.

This course is great for a more indepth look into Rust, I have a very good understanding of how Rust is implemented and why it was implemented that way (linear types, avoiding data races...). 