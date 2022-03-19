list defined by paranthesis: ( )

list items seperated by spaces: (atom turkey or)

all atoms are expressions
all lists are expressions


// primitives:
*car* => first item a list >>-> expression
*cdr* => a list without *car* >>-> list
*cons* -> appends any expression to the front of a list >>-> list
*null?* => if the given list is empty >>-> bool
*quote ()* => empty list >>-> list // or '()
*atom?* => if the given expression is an atom >>-> bool
*eq?* => if the two given "non-numeric" atoms are the same >>-> bool

// keywords:
*cond* ... <=> asks question // pairs of (bool? , val if true) // else keyword thats just true
*lambda* ... <=> creates function
*define* ... <=> names function
*or* ... <=> or operator over 2 questions // bools

#t <=> true
#f <=> false


Subset of Scheme used:
car, cdr, cons, eq? , null?, zero? , addl, subl, number?, and, or, quote, lambda, define, and cond.


cons a list
cons([a , list])
