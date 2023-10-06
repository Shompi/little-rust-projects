# Simple Calculator

An extremely simple calculator program with just 4 operations:

* Add
* Substract
* Multiply
* Division

The program prompts the user with a menu to select the operation
It only takes 2 numbers as inputs (Integers) and shows a result.

This is literally my first rust program, feedback is pretty much welcome

One of the things I didn't really like was the match pattern, my function `parse_number()` can return two results, an integer or an error, and with just two numbers you can see how the nesting can get very deep and messy. So for now I'll try and see if I can use another pattern for that, I read about the use of unwrap but that can `panic` which is not really ideal since i dont want the program to crash everytime something goes wrong. 