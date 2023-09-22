# somasz-chess

# Itended use

The user first picks a coordinate.
That coordinate is used by select_piece to return an Option where the Some variant contains a Vector of all possible valid moves for the chess piece on that coordinate.

Then if the return value is a Some variant it should be unwrapped and then to vector of coordinates(represented by tuples) can be used to present the user with valid move options.

After the user has chosen a valid move coordinate it should be given to the set_piece function along with the coordinates of the picked piece.

After this the turn counter should be increased by using the increase_turn function

```
Side note: The select_piece function needs to be given a faction value which can be generated with the faction_decider function in order to make sure the user can only pick pieces from the color they are playing as.
```