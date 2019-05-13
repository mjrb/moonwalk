## Warmup / Approach
In order to familiarize ourselves with rust we wrote some smaller
programs, and eventually writing a BrainF*** interpreter (attached in `src/bf.rs`)

the BF interpreter gave a some indication as to the architecture to work with.
we wound up using the following fairly standard pipeline:
1. lex into tokens
2. parse into lines
3. check labels and link
4. interpret

## modifications after the proposal
dereferencing is now `*A` instead of `(A)`. This resolved ambiguity
relating to parentheses in if expressions. In typical fashion the original
language had no way to do IO and I forgot to specify some PC instructions,
so we added `io`, `forwards`, `backwards`, and `reverse`

## What we learned
### Mickey J
Parsing turned out to be harder than I anticipated. This was mostly
due to the complexity of parsing expressions. I had 1 idea for a
parser that used a DFA and a stack to parse the whole line, but
eventually settled on this incremental one that uses 2 stacks to
parse if expressions. It's basically an adaptation of the 112 expression
evaluator, but the evaluation code builds the AST for expressions.

I also got to used the knowledge regex and context free grammars I gained
from this class to plan and implement this language. I also Heavily used
and abused Rust's algebraic datatypes, which I first encountered with
Haskell. I even used some category theory by realizing I could map `Option`s
and `Result`s like Haskell's `Maybe`s for cleaner code.

One of the most interesting features of rust was the borrow checker and the
friendliness of compiler. I expected it to be a huge struggle to deal with
its strictness but I think the rust compiler overcompensates and is really
nice on purpose. This created a pleasant experience. However I did run into
a super weird error where it was complaining that the type of a break statement
was not void and was expr. This didn't make any sense but wound up being caused
by type inferencing. I also even used adavnced rust features like traits (like
interfaces) and a lifetime parameter (`struct Thing<'l>`) to make sure a struct
didn't outlive an embedded reference.

### Alex

## Grammar
```
HEX = 0x[0-9A-F]+
DEC = [0-9]+
LABEL = [a-zA-Z0-9\-]+

REG = A | B | C | D
ADDR = <HEX> | <DEC>
LITERAL = \$(<HEX> | <DEC>)

SOURCE = <REG> | <ADDR> | <LITERAL> | \*+<SOURCE>
DEST = <REG> | <ADDR> | \*+<DEST>

INCREMENT = inc <DEST> <LITERAL>

JUMP = jump <LABEL>?
jump to label.
if label isn't present, jump to the last from that got us here
ignored when reversed.

FROM = from <LABEL>?
similar to jump but ignored moving forward. an interesting consequence of how
from and jmp are defined is that every jmp or from line has its own call stack.

IO = io <SOURCE>
going forward it reads 1 character into source
going backwads it outputs source as 1 character

HALT = halt
stops the program

FORWARDS = forwards
makes the program execute forwards

BACKWARDS = backwards
makes the program execute backwards

REVERSE = reverse
switches execution direction

INSTRUCTION = <INCREMENT> | <JUMP> | <FROM> | <HALT> | <IO> | <FORWARDS> | <BACKWARDS> | <REVERSE>
COMMENT = ;.*
CONDITION-EXP = backwards
	      | forwards
	      | \(<CONDITION-EXP>\)
	      | <CONDITION-EXP> (and | or) <CONDITION-EXP>
	      | <SOURCE> (<|>)=?|= <SOURCE>

LINE = (<LABEL>:)? <INSTRUCTION> (if <CONDITION-EXP>)
PROGRAM = (<LINE> | <COMMENT>)*
```