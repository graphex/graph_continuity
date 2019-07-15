Graph Continuity
================

A simple command line program written in Rust which asks for a set of terms that are
equal to each other first, then will tell you if any provided terms are unequal, given the
relationships provided previously.

Example Compile:
```
   Compiling graph_continuity v0.1.0 (~/graph_continuity)
    Finished dev [unoptimized + debuginfo] target(s) in 0.84s
     Running `target\debug\graph_continuity`
```

Example run:
```
Enter your space-separated graph links in the following form a=b:

a=b b=c c=d x=y

First-Degree Relationships:
{"d": {"c"}, "y": {"x"}, "a": {"b"}, "c": {"d", "b"}, "b": {"a", "c"}, "x": {"y"}}

Enter a discontinuity to test for in the form a!=b or 'q' to quit:

a!=b
Searching for a connection between a and b
looking for b in a
a had a connection to b through the path ["a"]

Enter a discontinuity to test for in the form a!=b or 'q' to quit:

a!=d
Searching for a connection between a and d
looking for d in a
looking for d in b
looking for d in c
c had a connection to d through the path ["a", "b", "c"]

Enter a discontinuity to test for in the form a!=b or 'q' to quit:

a!=x
Searching for a connection between a and x
looking for x in a
looking for x in b
looking for x in c
looking for x in d
No connection found

Enter a discontinuity to test for in the form a!=b or 'q' to quit:

q

Process finished with exit code 0
```