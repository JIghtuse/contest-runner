# contest-runner

Looks for files in current directory with names like:

    input01.txt
    input02.txt
    output01.txt
    output02.txt

And executes binary file (`a.out` or specified as a first argument) on each of
the input file contents, comparing output with corresponding output file
contents. Reports number of failures/successes.

## Usage example

    $ contest_runner
    6 testcases found
    ...E..
    Failed testcases:
    Testcase { input: "input03.txt", output: "output03.txt" }:
      Expected: 'Name1
    Not Found!
    HelloWorld
    Fsa
    '
      Actual:   'Name1
    Not Found!
    HelloWorld
    Fsad
    '
