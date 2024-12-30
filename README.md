# mindustry-plassembly

PLAssembly is a interpreter and a programming language based on assembly language of logic progessors from [Mindustry](https://mindustrygame.github.io/). It adds macros that can be defined and used with any "words" of original Mindustry language (variable, number, string, etc.) and with blocks of code.

## Syntax

### Macro definiton

```
!<macro_name> <params>
<body>
!!
```

where `<params>` is a list of parameters separated with spacases. A parameter can be a simple word (`param1`) or a block-parameter (`&block1`).

The `<body>` consists of lines of Mindustry assembly code with additional kinds tokens:

- reference to parameter, like `param1!`;
- reference to block-parameter, like `&block1`;
- label-reference to parameter, like `param1!:` (parameter: `label1`; reference: `label1!:`; argument: `myLabel`; result: `myLabel!`);
- generic identifiers and labels, like `#ident` and `#label:`; after expanding a macro, such identifier of label will be converted into a unique name; it is useful with labels in general.

You can use macros inside others, they will be expanded too.

Examples:

```
!printl msg
print msg!
print "\n"
!!

!dec var
op sub var! 1
!!

!for var &body
#loop:
dec! var!
&body
jump #loop greaterThan var! 0
!!
```

### Macro usage

To use a macro, type its name ending with `!` and list arguments. To provide a body-argument, use keywords `$begin` and `$end`.

Example:

```
set var 10
print "var="
printl! var

for! var
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
__GI_0_loop:
op sub var 1
print var
print "..."
print "\n"
printflush message1
wait 1
jump __GI_0_loop greaterThan var 0
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
