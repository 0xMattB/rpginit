/***************************************************************************************************
* RPG Initiative Tracker
* Version 0.3.0 (08/04/23)
* messages.rs
* 0xMattB
***************************************************************************************************/
pub fn help() -> String {
"
| Command    Arg1    Arg2     Description
| -------    ----    ----     -----------
| add0       [name]  -        Add a permanent entity to list
| add1       [name]  -        Add a temporary entity to list
| set        [name]  [score]  Set primary initiative score for [name]
| set2       [name]  [score]  Set secondary initiative score for [name] (use for ties)
| start      -       -        Sort list by initiatives and print list, indicate first turn
| next       -       -        Print list, indicating next turn
| remove     [name]  -        Remove entity from list
| clear      -       -        Remove temporary entities, reset scores for permanent entities
| reset      -       -        Resets entire list (makes it blank)
| print      -       -        Print all entities on list
| printname  [name]  -        Print entity with [name]
| sort       -       -        Sort entities in initiative order
| show       -       -        Show initiative scores when printing lists
| hide       -       -        Hide initiative scores when printing lists
| example    -       -        Prints an example of a session
| version    -       -        Print program version
| help       -       -        Print this help menu
|
| Notes:
| > Spaces in names with be replaced by dashes (-), and will be converted to lowercase
| > If [name] argument is not explicity provided, the bot will attempt to use the user name
".to_string()
}

pub fn example() -> String {
"
| >>> First we create entities:
|
| > add0 william       << create a permanent entity named 'william'
| > add0 betty         << create a permanent entity named 'betty'
| > add1 goblin        << create a temporary entity named 'goblin'
|
| >>> Then we set initiative scores:
|
| > setall             << runs loop that sets primary initiative for all entities
|   william: 16        << ... william rolls a 16 
|   betty: 9           << ... betty rolls a 9
|   goblin: 16         << ... goblin rolls a 16
|                      << uh oh, william and goblin tied!
|                      << they each roll again, and we enter these as secondary scores
| > set2 william 12    << ... william rolls a 12
| > set2 goblin 14     << ... goblin rolls a 14
|
| >>> List tasks (optional):
|
| > sort              << we can sort the list, but this not required to be done manually
| > print             << we can also print the list to the screen
|   goblin  ( 16 / 14 )
|   william ( 16 / 12 )
|   betty   (  9 /  0 )
|
| >>> We are now ready to run initiative
|
| > run                   << this sorts the entities in initiative order and starts the tracker
| (*** initiative tracker not shown here ***)
|
| >>> Cleaning up the list:
|
| > clear                << this command: (1) removes any temporary entities from the list ...
|                        << ... and (2) resets all initiative scores back to zero
| > print                << we can print the list to verify this
|   william (  0 /  0 )
|   betty   (  0 /  0 )
|
| > exit                 << when we are done, we can exit the program
".to_string()
}