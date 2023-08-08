"rpginit" - A table-top RPG initiative tracker Discord bot

This is a practice program that I turned into a Discord bot. The original version had a CLI interface, but when development finished, I adapted it so that it could be run as a bot in Discord.

The program works by adding entities to a list, using the following commands:
"!add0 [name]" --- adds a permanent entity to the list (typically for players)
"!add1 [name]" --- adds a temporary entity to the list (typically for enemies)

Initiative scores are set using:
"!set [name] [score]"
Secondary initiative scores are supported, in the event of ties:
"!set2 [name] [score]"

To start running initative, enter the following command (which first sorts the list in initiative order):
"!start"

To advance to the next turn, use:
"!next"

Rounds are also tracked.

When finished, the "!clear" command performs the following actions:
1. Deletes any temporary entities from the list
2. Clears all permanent entity score values to zero
3. Resets the Round counter

Other useful commands include:
"!sort" --- sorts the list in initiative order
"!print" --- prints the current list and initiative scores
"!help" --- prints all available commands, and some notes
"!example" --- prints an example session, with explanatory notations

The "!help" command prints:

```
Command    Arg1    Arg2     Description
-------    ----    ----     -----------
add0       [name]  -        Add a permanent entity to list
add1       [name]  -        Add a temporary entity to list
set        [name]  [score]  Set primary initiative score for [name]
set2       [name]  [score]  Set secondary initiative score for [name] (use for ties)
start      -       -        Sort list by initiatives and print list, indicate first turn
next       -       -        Print list, indicating next turn
remove     [name]  -        Remove entity from list
clear      -       -        Remove temporary entities, reset scores for permanent entities
reset      -       -        Resets entire list (makes it blank)
print      -       -        Print all entities on list
printname  [name]  -        Print entity with [name]
sort       -       -        Sort entities in initiative order
show       -       -        Show initiative scores when printing lists
hide       -       -        Hide initiative scores when printing lists
example    -       -        Prints an example of a session
version    -       -        Print program version
help       -       -        Print this help menu

Notes:
> Spaces in names with be replaced by dashes (-), and will be converted to lowercase
> If [name] argument is not explicity provided, the bot will attempt to use the user name
```