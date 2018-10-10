In this chapter I learned some stuff about variables.

- veriables are created using the let key-woord
- creating a veriable with just the let key-word will make them immutable. (immutable variables should be written in all caps)
- The mut key-word will make the variable mutable
- by recreating a variable (with let) you overwrite/shadow the variable.
- by recreating/shadowing a variable you can change its type.
- a let mut variable cannot change type.

Datatypes
- a signed number is a number with a minus or positive sign before it. if a number is unsigned it will alsays be apositvie number. this is only valid for integers
- there are 4 scalar types:integers,.floating-points, Booleans and characters.
- numbers can have underscores as seperators. example: 99_222 = 99222
- floating points have two forms, f32, f64. default version is f64.
- characters use single quotes. charactars represent a unicose scalar value. Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.

Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

- Tuples can be accesed by using a (.) operator. 
- use brackets to make a tubple. ()

- arrays are made using square braces []
- arrays are accesed using the square brackets [0]

Functions:
- All functions can be called appon anywhere in the file if the are defined in that file. The order in which the functions are defined do not matter.( the scope still does )
- input variables are defined at the start of the function. When the are defined so is there type. 
- Return statements are defined at the begin of a function ( after the input is defined ). 

Statements do not return values. Therefore, you can’t assign a let statement to another variable. So you cannot assaign another variable to another variable when they are just created.
A scope always returns something. You can use a state to return a variable as in:

let y = {
        let x = 3;
        x + 1
    };

Note that the line x + 1 has no semi-colon therefore this value is returned to the newly created variable y. 

Control flow:
if statements:
- if loops are used by using the if expression.
- unlike match explessions if statements stop checking other clauses en run the code behind the first statement that yield true.
- if statements need an expression that returns a boolean.
- An if statement can be used to assign a variable to a let statment. Still only one variable type can be cast to the let variable from the if statement.

loop loops:
- loop will continuesly execute the code inside the loop until the code breaks, is broken out of or is forced to stop manually.

While loops:
- While loops are used as normal while loops in other languages.

for loops:
- forl loops take an iterator as argument. This can be given to the loop in two ways.
- for element in a.iter() {} will run the code for every element in the vector a.
- for number in (1..4) {} will run the code for 3 times changing the variable number from 1,2 to 3. 
