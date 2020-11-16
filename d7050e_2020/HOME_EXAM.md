
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
Supported statements are let, assigments, if/else, while, implicit returns all with explicit types.
Expression operands have a few different categories, infix operations which are split into arithmetic and boolean algebraic operations. In retrospect less than and greater than has been wrongly grouped as boolean algebraic operations, but for development it made sense beacause they all evaluate into a boolean. Supported prefix operations are negation, subtraction, reference, mutable reference and dereference. Parenthesized expression have precedence, prefix operations have precedence over infix operations and arithmetic operations before boolean operations. FactorOp has precedence over BinOp.
Error recovery is not implemented.


## Your semantics

  Symbols:

- σ, state
- σ', derived state
- ⇓, evaluates
- c, command
- x, variable
- e, expression
- p, parameters

An expression can either be a
- n, number
- b, boolean
- f, function

Sequence

<img src="https://render.githubusercontent.com/render/math?math=\frac{<c0,\sigma> \Downarrow \sigma' <c1,\sigma'> \Downarrow \sigma''}{<c0%3Bc1,\sigma> \Downarrow \sigma''}">

```rust
let a:i32 = 5;
a();
```
A sequence is chain of commands where the first one is executed then the seconds command and so forth. If this wasnt the case the commands wouldnt infuence the next ones correctly, the state σ' would be lost.

Operations

Infix:

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e1, \sigma>\Downarrow n1 <e2, \sigma>\Downarrow n2}{e1 %40 e2,\sigma\Downarrow n} ">

Prefix: 

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e1, \sigma>\Downarrow n1}{%40 e1,\sigma\Downarrow n} ">

The @ sign represents one of the implemented arithmetic operations

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

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e,\sigma>\Downarrow true <c1,\sigma>\Downarrow\sigma'}{<if\%3Btrue\%3Bthen\%3B c1 \%3Belse\%3Bc2, \sigma>\Downarrow\sigma'} ">

If/Else false 

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e,\sigma>\Downarrow false <c2,\sigma>\Downarrow\sigma'}{<if\%3Bfalse\%3Bthen\%3B c1 \%3Belse\%3Bc2, \sigma>\Downarrow\sigma'} ">

If true

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e,\sigma>\Downarrow true <c1,\sigma>\Downarrow\sigma'}{<if\%3Btrue\%3Bthen\%3B c1, \sigma>\Downarrow\sigma'} ">

If false

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e,\sigma>\Downarrow false <c1,\sigma>\Downarrow\sigma}{<if\%3Bfalse\%3Bthen\%3B c1, \sigma>\Downarrow\sigma} ">


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

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e,\sigma >\Downarrow true \%3B<c1,\sigma>\Downarrow\sigma' <while\%3B e \%3B do\%3B c, \sigma'>\Downarrow\sigma''}{<while\%3B true \%3B do \%3B c,\sigma>\Downarrow\sigma'' } ">

While false

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e,\sigma >\Downarrow false \%3B<c1,\sigma>\Downarrow\sigma' <while\%3B e\%3B do\%3B c, \sigma'>\Downarrow\sigma}{<while\%3B false \%3B do \%3B c,\sigma>\Downarrow\sigma } ">

```rust
while a<5 {
	a = a + 1;
};
```
Both if/else and while commands have their blocks evaluted or skipped based on the condition given.

Let assignment

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e, \sigma>\Downarrow n <let \text{ } x%3A=e, \sigma>\Downarrow\sigma'}{<let \text{ } x %3A= e, \sigma> \Downarrow \sigma[x %3A= n]} ">

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

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e,\sigma> \Downarrow n}{<x%3A= n, \sigma> \Downarrow \sigma[x%3A= n]} ">

```rust
a = 5;
b = !false;
c = func();
d = &a;
e = &mut b;
```
The evaluated expression on the right side is moved to the already declared variable on the left. The left hand variable has to be declared mutable for assignments to be possible.

Parameter

<img src="https://render.githubusercontent.com/render/math?math=\frac{<e_1,\sigma> \Downarrow n_1, \sigma' <e_2,\sigma'> \Downarrow n_2, \sigma'' ... <e_n,\sigma^{n %2D 1}> \Downarrow n_n, \sigma^{n}}{<e_1,e_2,...,e_n, \sigma>\Downarrow n_1, n_2,...,n_n,\sigma^n}">

```rust
5,7,true,&mut 5,a()
1+3+4+5,h,!false
```
Parameters are evaluated from left to right, this is important beacause some expressions may change the state.

Function call

<img src="https://render.githubusercontent.com/render/math?math=\frac{<p, f, \sigma>\Downarrow n}{<f(p),\sigma>\Downarrow n} ">

```rust
a(5,1,true,!false);
b(c(),h*3,&g, &mut j);
```

Return

$$\frac{<f, σ>⇓σ}{< e, σ> ⇓ σ[e]}$$

```rust
fn a() -> i32 {
	let a:i32 = 5;
	a //returns 5
};
```
If the last command in a function evaluates to an expression, that is moved to the state of the function.

In Rust everything has a type, even statements, and following that implicit returns become easy to introduce. In the vein of following Rust implementation implict returns are implemented. While the requirements stated explicit returns, implicit ones do the same thing, but with less freedom.  Other then that the implementation follows the requirements.

## Your type checker

Three types are implemented

- i32
- bool
- unit
- Reference
- Mutable Reference

Operations
Prefix $$\frac{<e1, σ>⇓i32 <e2, σ>⇓i32}{e1⊗e2,σ⇓i32}$$ 
Infix $$\frac{<e1, σ>⇓i32}{⊗e1,σ⇓i32}$$ 

```rust
1+1;
1 + true; //Error
```

The sign ⊗ represents all the supported operations which are given in the section above. For bool type the operations look the same. The unit type is reserved for function return types.

If true/false else

$$\frac{<bool,σ>⇓ bool \;<c1,σ>⇓σ'}{<if\;bool\;then\;c1\;else\;c2, σ>⇓σ'}$$

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

 While true/false

$$\frac{<bool,σ>⇓ bool \;<c1,σ>⇓σ' <while\;bool\;do\;c, σ'>⇓σ''}{<while\;bool\;do\;c,σ>⇓σ'' }$$

```rust
while 5 {//Error not bool type
	let a:i32 = 5;
};
while true {};//works
```
In the type-checker there is no big difference to a while loop or if/else. It check both the condition and the block as it doesnt care about the actual value only the type that is evaluated into. To note is that if the if/else statement doesnt return anything its type becomes unit, otherwise it get the type of the return, so it acts like a function without parameters. The is always handled as typ unit.

Let assignment

$$\frac{<x, σ>⇓i32<let\;x:=i32, σ>⇓σ'}{<let \; x := i32, σ> ⇓ σ[x := i32]}$$

```rust
let a:i32 = false;//error
let b:bool = 5;//error
let mut c:i32 = 3;//works
let t:&mut i32 = &mut c;//works
```

All types are implmented in a similar way to above. To note is that the type-checker doesnt implement any borrow checking as it doesnt keep track of what variable it was assigned to, only that variable type.

Assignment

$$\frac{}{<x := i32, σ> ⇓ σ[x := i32]}$$

```rust
let a:i32 = 1;//works
a = 5;//Error not mutable
let mut b:i32 = 5;
b = true;//Error type missmatch
b = 6;//works
let c:&i32 = &a;//works
let d:i32 = *c;//works
let e:&mut i32 = &mut b;
*e = 3;//works
```

The last example shows how references can be used with assignments, when left hand side is dereferenced it gets the variable b instead and assigns the left hand side to it.


Return

$$\frac{<f, σ>⇓σ}{< i32, σ> ⇓ σ[i32]}$$

```rust
fn a() -> () {};//works
fn b() -> i32 {5};//works
fn c() -> bool {false};//works
fn d() -> () {true};//type error
fn e() -> &i32 {let a:i32 = 5;&a};//works in type-checking not interpreter
```

The implicit return must match the declared return type of the function.

 To implement the underlying functionality for borrow checking assignments have to support dereference on the left side. This made me have to redo the parser to support operations on the left side.

## Your borrrow checker

The borrow checker should check that multiple references can be set and only a single mutable reference. A variable cannot be referenced and mutably referenced at the same time. A referenced variable can only be changed via its mutable reference.

The interpreter implements rusts borrow checking, references can be made and removed. Variables can be accessed and changed via a mutable reference. Variables passed as argument map back to the original variable. Except for a case where the scopes stack isnt handled correctly. To short lifetimes references are also rejected, eq a function that would try to return a referenced variable were the owner of that variable goes out of scope at the end of the function is an error.

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