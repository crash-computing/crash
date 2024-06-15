# The Crash Programming Language

It's fairly easy. There's only one thing you have to know to keep going: There are functions, variables and blueprints.

## Functions

Functions are types. Now what does that mean?
Take a look at this code: 

```crash
player ( uuid[uid], name[string] ) = {

    println("Created new player {name}");

    get_uuid[uid] = uuid;

    get_name[string] = name;
}

player1[player]: player(rand_uid(), "Player1");

player1.get_uuid();
```

So what does that mean? - 
We create a function called player with the parameters `uuid` of type `uid`
and `name` of type `string`. If the function is called, everything inside
the curly braces is executed. It begins with printing the message "Created new player xy".
After this call, two more sub-functions `get_uuid` and `get_name` get declared.
So after we declared the player function, we create a variable which is
holding an instance of a created player 1 and so we're also able to call the sub-functions
of the player instance.

So remember:

- Functions can be procedures or just values
- Functions can be (re-)declared in procedures
- Functions can be used like types
- Variables hold references to instances of types
- Parameters work like Variables
- Variables cannot be used outside the procedure

## Blueprints

Blueprints are like traits from Rust or interfaces from Java.
They don't change the program itself. They're just there
to make sure that we can make use of polymorphism.

```crash

// Declaration of the blueprint Player
Player := {

    // You can think of ~ as being the parent functions name
    clone[~];

    get_name[string];

    /* 
        Sets the players name and 
        returns the parent function.
    */
    set_name[~] ( new_name[string] );
}


// Implementation of a real player
real_player[Player] (uuid[uid], name[string] ) = {

    clone[~] = real_player(uuid, name);

    get_name[string] = name;

    set_name[~] (new_name[string]) = {
        name = new_name;
        // @ is usually known as a return from the current scope
        // ^ means current instance of parent function
        // Here it's actually unnecessary, since
        // the current instance is returned automatically.
        @ ^
    }

    get_uuid[uid] = uuid;

    // Returns nothing (void)
    disconnect[] (message[string]) = {
        // some odd code
    }
}

fake_player[Player] = ( name[string] ) = {

    clone = ~(name);

    get_name[string] = name;

    set_name ( new_name[string] ) = {
        name = new_name;
    }
}

// Assign a real-player to some_player
some_player[Player]: real_player(rand_uid(), "ProGamer187");

// Reassign some_player
some_player : fake_player("Goofy");
```

## Procedural parameters

*I haven't seen any language implement this, but
I think it could be quite helpful.*

Beside giving arguments to a function's parameters,
you can give procedures as scopes to a function's parameters.
You could build your own if-statement with something like this.

```crash
foo (condition[bool]) {body} = {
    // call body procedure
    body();
}

foo(true) {
    // some odd code
}

/* 
    Here we declare an array with "^" of length n
    and n bodies as parameters.
    "..." makes it possible to write
    all array elements as function parameters.
    Only works, if the array is the last parameter
    of some function.
    "$n" is a numeric variable for the compiler, that assures,
    everything is written correctly.
*/
foofoo (conditions[bool^$n]...) {bodies^$n} = {
    // some odd code
}

foofoo(true, false) {
    // Code of body 1
} : {
    // Code og body 2
}
```

## Macros

Macros don't have that many features, but are extremly powerful.
They just tell the compiler to replace x with y.

```crash
someFunction | `example/code/someFunction`;

someFunction = {
   // some odd code 
}

someFunction();

// is similar to

example/code/someFunction();
```

You can give a macro parameters. We call this a macro-function.

```crash
import(name, path) | `name | path`;

import(hello, test);
```

The macro-system is dynamic and has sort of it's own language.

```crash

test(x) | {
    if (x) {
        `#[test]`
        @@
    }
    `#[no_test]`
};

test(true)
some_function = {

}
```

## Attributes

Attributes are applied on functions.


There are different styles of attributes, like tags,
which look like this:

```crash
#[some_tag]
foo = {

}

has_some_tag[bool] : foo#some_tag != null;
```

We can check the attributes of some caller function.
This is used for permissions.

```crash
#[perm.idk]
foo = {
    some_odd_function();
}

some_odd_function §#[perm.idk] = {

}
```

You can give attributes literal values.
It is not possible to call functions from attributes.
Only the function itself is able to change it's attributes.
You can seperate attributes by a "," instead of writing
them under each other.

```crash
#[version = "0.0.0", test]
test_something = {
    #version = "0.2.2"
}
```


