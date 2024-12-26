# mindustry-plassembly

PLAssembly is a interpreter and a programming language based on assembly language of logic progessors from [Mindustry](https://mindustrygame.github.io/). It adds macros that can be defined and used with any "words" of original Mindustry language (variable, number, string, etc.) and with blocks of code.

## Syntax

### Macro definiton

```
!<macro_name> <params>
<body>
!!
```

where `<params>` is a list of parameters separated with spacases. A parameter can be a simple word (`param1`) or a block-parameter (`&block1`); the `<body>` consists of lines of Mindustry assemby code with references to the parameters (`param1!`) and block-parameters (`&block1`). Also it is possible convert a parameter into a label (parameter: `label1`; reference: `label1!:`; argument: `myLabel`; result: `myLabel!`). Also you can use macros inside others, they will be expanded too.

Examples:

```
!printl msg
print msg!
print "\n"
!!

!dec var
op sub var! 1
!!

!for loopname var &body
loopname!:
dec! var!
&body
jump loopname! greaterThan var! 0
!!
```

### Macro usage

To use a macro, type its name ending with `!` and list arguments. To provide a body-argument, use keywords `$begin` and `$end`.

Example:

```
set var 10
print "var="
printl! var

for! for1 var
$begin
print var
printl! "..."
printflush message1
wait 1
$end

printl! "Bang!"
printflush message1
```

The code above will be compiled into:

```
set var 10
print "var="
print var
print "\n"
for1:
op sub var 1
print var
print "..."
print "\n"
printflush message1
wait 1
jump for1 greaterThan var 0
print "Bang!"
print "\n"
printflush message1
```

## How to use the compiler

1. Download prebuilded executable file from the last release.
2. Prepare a file with your plassembly code.
3. Open terminal and run command `plassembly your_code.plas`; it will create a file with name like `your_code.plas.txt`.
4. Copy code from the created file and paste it in processtor in the game.
5. Enjoy your working processor! :)
