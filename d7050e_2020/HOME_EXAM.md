
# Home Exam D7050E


## Your repo

- Link to your repo here: https://github.com/CarlOsterberg/D7050E

## Your syntax

- Give an as complete as possible EBNF grammar for your language.

Program
```ebnf
: FunctionDec+
;
```

FunctionDec

```ebnf
: "fn" Var "(" [ "," ], Params ")" "->" Return_type "{" Statement+ "};"
;
```
Params
```ebnf
: Var ":" Type
| "mut" Var ":" Type
| Var ":" Unary Type
| "mut" Var ":" Unary Type
;
```
Statement

```ebnf
: "let" Var ":" Type "=" Statement ";"
| Deref Var "=" Statement ";"
| Var "=" Statement ";"
| "if" Expr "{" Statement "};"
| "if" Expr "{" Statement "}; else "{" Statement "};"
| "while" Expr "{" Statement "};"
| Expr
;
```
Bin

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
;
```

Return_type

```ebnf
: "i32"
| "bool"
| "()"
;
```
Term

```ebnf
: Unary
| Prefix
| FuncCall
| Num
| Bool
| Var
| "(" Expr ")"
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
: { _|Letter },{ Letter|Digit }, White_space
;
```
FuncCall

```ebnf
: Var "(" [ "," ], Expr ")"
;
```

Letter

```ebnf
: ([a-z]|[A-Z])+
;
```
Num
```ebnf
: [0-9]+
;
```
White_space
```ebnf
: [ ] -> skip
;
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

An expression can either be a
- n, number
- b, boolean
- f, function
- Give a (simplified) Structural Operational Semantics (SOS) for your language. You don't need to detail rules that are similar (follow the same pattern). Regarding variable environment (store) you may omit details as long as the presentation is easy to follow.

Sequence

$$\frac{<c0,σ> ⇓ σ' <c1,σ'> ⇓ σ''}{<c0;c1,σ> ⇓ σ''}$$

```rust
let a:i32 = 5;
a();
```
A sequence is chain of commands where the first one is executed then the seconds command and so forth. If this wasnt the case the commands wouldnt infuence the next ones correctly, the state σ' would be gone.

Operations

$$\frac{<e1, σ>⇓n1 <e2, σ>⇓n2}{e1⊗e2,σ⇓n}$$ 

The ⊗- sign represents one of the implemented arithmetic operations

- +, plus
- -, minus
- *, multiplication
- /, division
- ">", greater than
- "<", less than
- "==", equals
```rust
(1+2) * 3 / 2;
1 < 2;
5 == 5;
```

Operations for booleans are implemented in similar fashion and supports
- "!", not equal
- "&&", and
- "||", or 
- "==", equals
```rust
1 > 2 && true;
false || true;
```

If true/false else

$$\frac{<b,σ>⇓ b \;<c1,σ>⇓σ'}{<if\;b\;then\;c1\;else\;c2, σ>⇓σ'}$$

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
 While true/false

$$\frac{<b,σ>⇓ b \;<c1,σ>⇓σ' <while\;b\;do\;c, σ'>⇓σ''}{<while\;b\;do\;c,σ>⇓σ'' }$$

```rust
while a<5 {
	a = a + 1;
};
```
Both if/else and while commands their blocks are evaluted based on the condition given.

Let assignment

$$\frac{<x, σ>⇓i32<let \text{ } x:=e, σ>⇓σ'}{<let \text{ } x := e, σ> ⇓ σ[x := e]}$$

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

$$\frac{}{<x := e, σ> ⇓ σ[x := e]}$$
```rust
a = 5;
b = !false;
c = func();
d = &a;
e = &mut b;
```
The evaluated expression on the right side is moved to the already declared variable on the left. The left hand variable has to be declared mutable for assignments to be possible.

Return

$$\frac{<f, σ>⇓σ}{< e, σ> ⇓ σ[e]}$$

```rust
fn a() -> i32 {
	let a:i32 = 5;
	a //returns 5
};
```
If the last command in a function evaluates to an expression, that is moved to the state of the function.

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.

In Rust everything has a type, even statements, and following that implicit returns become easy to introduce. In the vein of following Rust implementation implict returns are implemented. While the requirements stated explicit returns, implicit ones do the same thing, but with less freedom. 

## Your type checker

  

- Give a simplified set of Type Checking Rules for your language (those rules look very much like the SOS rules, but over types not values). Also here you don't need to detail rules that are similar (follow the same pattern).

  

- Demonstrate each "type rule" by an example. You may use one or several "programs" to showcase where rules successfully apply.

  

- For your implementation, give a set of programs demonstrating that ill-typed programs are rejected, connect back to the Type Checking Rules to argue why these are illegal and thus should be rejected.

  

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.

  

## Your borrrow checker

The borrow checker should check that multiple references can be set and only a single mutable reference. A variable cannot be referenced and mutably referenced at the same time. A referenced variable can only be changed via its mutable reference.

The solution implements rusts borrow checking, references can be made and removed. Variables can be accessed and changed via a mutable reference. Variables passed as argument map back to the original variable. Except for a case where the stack scopes arent handled correctly. To short lifetimes references are also rejected, eq a function would try to return a referenced variable that were the owner of that variable goes out of scope at the end of the function.

## Your LLVM/Crane-Lift backend (optional)

Not implemented.

## Overall course goals and learning outcomes.

  

Comment on the alignment of the concrete course goals (taken from the course description) to the theory presented, work You have done and knowledge You have gained. (I have put some comments in [...]).

  

- Lexical analysis, syntax analysis, and translation into abstract syntax.

  

- Regular expressions and grammars, context-free languages and grammars, lexer and parser generators. [lalr-pop is a classical parser generator, it auto generated the lexer for you based on regular expressions but allows for you to define the lexer yourself for more control]

  

- Identifier handling and symbol table organization. Type-checking, logical inference systems. [SOS is a logical inference system]

  

- Intermediate representations and transformations for different languages. [If you attended, Recall lectures relating LLVM/Crane-lift, discussions on SSA (Single Static Assignment) used in LLVM/Crane-lift, and discussions/examples on high [level optimization](https://gitlab.henriktjader.com/pln/d7050e_2020/-/tree/generics_and_traits/examples)]

  

- Code optimization and register allocation. Machine code generation for common architectures. [Both LLVM/Crane-Lift does the "dirty work" of backend optimization/register allocation leveraging the SSA form of the LLVM-IR]

  

Comment on additional things that you have experienced and learned throughout the course.