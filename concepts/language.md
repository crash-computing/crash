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

- Functions are just procedures
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

    /* 
        Clones the player.
        If no return-type is given,
        it will use the parent-function as
        return-type.
    */
    clone;

    get_name[string];

    /* 
        Sets the players name and 
        returns the parent function.
    */
    set_name ( new_name[string] );
}


// Implementation of a real player
real_player[Player] (uuid[uid], name[string] ) = {

    clone = real_player(uuid, name);

    get_name[string] = name;

    set_name (new_name[string]) = {
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

    // You can think of ~ as being the parent functions name
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
