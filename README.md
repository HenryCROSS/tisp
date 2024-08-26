# Tisp Syntax Guide

## Introduction
Tisp is a lisp dialect

## CFG
```
<program>       ::= <expression>* 
                 | <macro-definition>*

<macro-definition>  ::= '(' 'macro' <symbol> <arg-list> <expression>* ')' // 宏定义

<expression>    ::= <atom> 
                 | <list>

<list>          ::= '(' <list-content> ')'

<list-content>  ::= 'def' <symbol> <definition>
                 | <special-form>
                 | <expression>*

<definition>    ::= <atom>
                 | <function-definition>

<function-definition> ::= '(' 'fn' <arg-list> <expression>* ')' // Lambda 表达式

<arg-list>  ::= '(' <symbol>* ')'

<special-form>  ::= 'quote' <expression>  // 引用特殊形式

<atom>          ::= <symbol>
                 | <keyword>
                 | <number>
                 | <string>
                 | <boolean>
                 | <character>
                 | <quote>

<symbol>        ::= [a-zA-Z_+\-*/><=!?][a-zA-Z0-9_+\-*/><=!?]*

<keyword>       ::= ':'[a-zA-Z_+\-*/><=!?'][a-zA-Z0-9_+\-*/><=!?']*

<number>        ::= <integer> 
                 | <float>

<integer>       ::= ['-']?[0-9]+

<float>         ::= ['-']?[0-9]+'.'[0-9]+

<string>        ::= '"' [^"]* '"'

<boolean>       ::= '#t'
                 | '#f'

<character>     ::= '#\' [a-zA-Z]

<quote>         ::= '\'' <expression>
```

## Basic Syntax

```lisp
42          ; Number literal (atom)
"hello"     ; String literal (atom)
x           ; Symbol (atom)

(+ 1 2)     ; A list representing the addition of 1 and 2
(if (> x 0)
    (print "positive")
    (print "non-positive"))
```

## Functions

```lisp
(defun square (x)
  (* x x))

(square 5)  ; Calls the function 'square' with argument 5
```

## Control Structures
```lisp
(if (> x 0)
    (print "positive")
    (print "non-positive"))

```

## Macros
```lisp
(defmacro unless (condition &rest body)
  `(if (not ,condition)
       (progn ,@body)))
```