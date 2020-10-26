
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
: { _|Letter },{ Letter|Digit }*, White_space
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
	while  b<10 && b>-5 {
		b  =  b  +  1;
	};
	*a  =  3;
	b
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

- Give an example that showcases all rules of your EBNF. The program should "do" something as used in the next exercise.

  

- For your implementation, show that your compiler successfully accepts the input program.

  

- Give a set of examples that are syntactically illegal, and rejected by your compiler.

  

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.

  

## Your semantics

  

- Give a (simplified) Structural Operational Semantics (SOS) for your language. You don't need to detail rules that are similar (follow the same pattern). Regarding variable environment (store) you may omit details as long as the presentation is easy to follow.

  

- Explain (in text) what an interpretation of your example should produce, do that by dry running your given example step by step. Relate back to the SOS rules. You may skip repetitions to avoid cluttering.

  

- For your implementation, give a program (or set of test programs) that cover all the semantics of your language that you have successfully implemented. (Maybe a subset of the input language accepted by the grammar.)

  

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.


## Your type checker

  

- Give a simplified set of Type Checking Rules for your language (those rules look very much like the SOS rules, but over types not values). Also here you don't need to detail rules that are similar (follow the same pattern).

  

- Demonstrate each "type rule" by an example. You may use one or several "programs" to showcase where rules successfully apply.

  

- For your implementation, give a set of programs demonstrating that ill-typed programs are rejected, connect back to the Type Checking Rules to argue why these are illegal and thus should be rejected.

  

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.

  

## Your borrrow checker

  

- Give a specification for well versus ill formed borrows. (What are the rules the borrow checker should check).

  

- Demonstrate the cases of ill formed borrows that your borrow checker is able to detect and reject.

  

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.

  

## Your LLVM/Crane-Lift backend (optional)

  

- Let your backend produce LLVM-IR/Crane Lift IR for an example program (covering the translations implemented).

  

- Describe the translation process, and connect back to the generated IR.

  

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.

  

## Overall course goals and learning outcomes.

  

Comment on the alignment of the concrete course goals (taken from the course description) to the theory presented, work You have done and knowledge You have gained. (I have put some comments in [...]).

  

- Lexical analysis, syntax analysis, and translation into abstract syntax.

  

- Regular expressions and grammars, context-free languages and grammars, lexer and parser generators. [lalr-pop is a classical parser generator, it auto generated the lexer for you based on regular expressions but allows for you to define the lexer yourself for more control]

  

- Identifier handling and symbol table organization. Type-checking, logical inference systems. [SOS is a logical inference system]

  

- Intermediate representations and transformations for different languages. [If you attended, Recall lectures relating LLVM/Crane-lift, discussions on SSA (Single Static Assignment) used in LLVM/Crane-lift, and discussions/examples on high [level optimization](https://gitlab.henriktjader.com/pln/d7050e_2020/-/tree/generics_and_traits/examples)]

  

- Code optimization and register allocation. Machine code generation for common architectures. [Both LLVM/Crane-Lift does the "dirty work" of backend optimization/register allocation leveraging the SSA form of the LLVM-IR]

  

Comment on additional things that you have experienced and learned throughout the course.