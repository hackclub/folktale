# Coding in Python Basics

Computers like following instructions. As such, this is the way programming languages like Python are designed: series of instructions.

In Python, each new line is a new instruction. We'll look at a few of these instructions; the ones that are needed to make your branching story game.

Let's break down the code that you typed out in the last section.
```python
print("Hello, world!")
```
There's only one instruction here - the `print` instruction. In Python, this prints out whatever text you feed in. But what are all these weird parentheses and quotes doing here?

The fancy word for these is *parameters*. When you call `print`, Python needs to know what to print. There are a ton of cases where one of these instructions takes in parameters, and the parentheses tell the Python interpreter that everything contained within them are parameters for that particular instruction.

If we had two of them:
```python
print("Line 1")
print("Line 2")
```
The parentheses of the first instruction here grabs "Line 1" for itself, and "Line 2" is its own thing for the next instruction. If you run this code, you'll see that two seperate lines get printed out, one for each of the instructions.

Quick vocabulary nibble, "instruction" is technically the wrong word to use here. I was using it since I think it conveys the message better, but I will be calling these types of lines of code by the more accurate term "function" from here on out.

Looping back to those `print` functions there, I never went over what the quotes mean. Whenever you see quotes (single and double quotes are equivalent in Python), that means everything inside them is treated as one big chunk of text. If you didn't have them, the Python interpreter might not be sure if this is all part of one argument or that it's not text at all.

That last part is important. Let's say we had this bit of code here:
```python
print(1 + 2)
print("1" + "2")
```
The first one treats the two as numbers, adds them like numbers, and prints out `3`.

The second one treats the two as text instead of numbers, "adds" them by sticking one on the end of the other, and will print out `12`.

So we've learned a fair bit about printing this out for the user, but to make a branching story game, you'll need to be able to ask the user for input. We use another function for this, fittingly named the `input` function.

For a simple example, let's print out a simple greeting for the user.

First, here's some code that will greet anyone named Bob.

```python
print("Hello, " + "Bob" + "!")
```

We're using the same adding text together trick as before, so we can separate the name of the person that we're greeting from everything else in our message. Now though, let's replace "Bob" with the input function.

```python
print("Hello, " + input("What's your name? ") + "!")
```

Alright, we've swapped out the hardcoded "Bob" name for the `input` function. This will print out the question inside its parameter, then it'll basically replace itself with whatever the user typed in. So if the user types in `Alice`, the code swaps out the `input` function and basically then becomes equivalent to:
```python
print("Hello, " + "Alice" + "!")
```

Awesome, we have input! However, putting the `input` statement directly in the middle of the `print` one is a bit messy. It would be cool if there were a way to store the output of the `input` function, then retrieve that later in the program when we want to print it out.

Well, we can do just that with variables.

A variable can be thought of as a little container with a label on it in which we can tuck away a value to be used later. Using the label on the front of the container, what we call the *name* of a variable, we can read, swap out, or modify the thing inside the variable at any time.

Variables are extremely simple to create in Python. To do it, we just write this:
```python
answer = input("What's your name? ")
```
We just write the name of the variable, write an equals sign, and write whatever we want it to set it to. We can set variables to a bunch of things:
```python
x = 4
y = x + 2
x = y - 3
a = int(input("Pick a number: "))
```
Side note: The `int` function converts the text from the `input` function to a number that we can work with instead of text that we can't. There is a difference between `2` and `"2"`, the latter is text and doing math on it won't work the way we think it will.

Alright, how to print out the name from the `input` function. We can just use the name of the variable in the place of where that function once was. It'll look like this:
```python
answer = input("What's your name? ")
print("Hello, " + answer + "!")
```
Try it out, run it! With variables, we've got almost all the tools that we need to make a branching story game. There's just one more thing we need: conditional logic.

