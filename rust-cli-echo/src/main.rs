use std::env; // Imports the 'env' module from Rust's standard library. This module provides functions for interacting with the environment, such as accessing command-line arguments. In Java, this is conceptually similar to `import java.lang.System`; in Python, `import sys`.

fn main() { // Defines the `main` function, which is the entry point for every executable Rust program. This is analogous to `public static void main(String[] args)` in Java or the code block under `if __name__ == "__main__":` in Python.
    let args: Vec<String> = env::args().collect(); // Declares an immutable variable named `args`.
                                                   // `env::args()` returns an iterator (a sequence you can loop over) of the command-line arguments, where each argument is a `String`.
                                                   // `.collect()` is a method used to consume an iterator and gather its items into a collection.
                                                   // `Vec<String>` is a type annotation specifying that `args` should be a `Vec` (vector), which is Rust's growable list type (similar to `ArrayList` in Java or `list` in Python), containing `String` elements.
                                                   // This line effectively collects all command-line arguments into a vector of strings.

    if args.len() > 1 { // Checks if the number of elements (length) in the `args` vector is greater than 1.
                        // The first argument (`args[0]`) is always the name of the program itself. So, `> 1` means at least one additional argument (the message) was provided by the user.
                        // `len()` is a method to get the number of elements, similar to `args.length` in Java or `len(args)` in Python.

        // Skip the first argument (program name) and join the rest with spaces
        let message = args[1..].join(" "); // Declares an immutable variable named `message`.
                                           // `args[1..]` creates a "slice" of the `args` vector, starting from index 1 (the second element) and going to the end. This effectively omits the program's name.
                                           // `.join(" ")` is a method available on slices of strings that concatenates all elements of the slice into a single `String`, using a space (" ") as a separator between them.
                                           // This is similar to `String.join(" ", Arrays.copyOfRange(args, 1, args.length))` in Java or `' '.join(args[1:])` in Python.
        println!("{}", message); // Prints the `message` string to the console, followed by a newline.
                                 // `println!` is a macro (Rust's powerful equivalent of a function that can do more at compile time, similar to preprocessor directives but much safer and more integrated) used for formatted output.
                                 // `{}` is a placeholder that will be replaced by the value of `message`.
                                 // This is analogous to `System.out.println(message)` in Java or `print(message)` in Python.

    } else { // This block executes if no message arguments were provided (i.e., only the program name or no arguments at all).

        println!("Usage: {} <message>", args[0]); // Prints a usage instruction to the console.
                                                  // `args[0]` is used here to dynamically insert the program's name into the usage message, making it more user-friendly.
        println!("Example: {} Hello World", args[0]); // Prints an example of how to use the program, again inserting the program's name.
    }
}