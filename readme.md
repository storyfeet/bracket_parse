Bracket Parse
============

This is intended to convert a bracketd lisp/prolog like string into a tree where each bracket is considered a parent of some kind

Changes v0.1.2

Added tail_n for tail chain skipping as tail().tail() drops the borrow.
Added tail_h for tail(n).head(), again to avoid borrow drops().

Impl Display for Bracket //TODO Escape strings safely 

Coming in v01.3

Iterator on &Bracket 
