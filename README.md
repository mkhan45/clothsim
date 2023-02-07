# clothsim

A cloth simulator written in Rust using macroquad

<https://cloth.mikail-khan.com>

___

I wanted to understand why I couldn't just use force-based spring constraints
for a cloth sim. Then, I wanted to reframe the position updates of spring constraints
as 'instant force application'. I got bored though so it's just a normal clothsim.
The general approach is taken from <https://github.com/johnBuffer/ClothSimulation>
