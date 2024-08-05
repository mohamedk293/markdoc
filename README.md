# Markdoc: Annotated Markdown Files for Code Documentation
Markdoc is a tool used to create flexible code documentation, allowing users to easily create functions and grab parameters, return types, and the function's name using an easy-to-understand syntax. Therefore, when changing a variable name using Markdoc, the description under the code reflects the name change without having to manually change every instance where a function, variable, or return type appears in the documentation. Config files also allow users to swiftly change from one syntax to another. Some config files are already provided, such as Rust, C, and Python. Markdoc's interpreter was fully written in Rust and converts .mkdc files into .md files using the selected config. This was a quick project I managed to scrap together, so there are some inconsistencies or suboptimal code. However, the goal in mind was achieved.

## How to install
- Clone the repo using `git clone`
- Use the command `cargo run` to run Markdoc!

## How to Use
### A Superset of Markdown
Markdoc files are a superset of Markdown files, meaning that any Markdown syntax can also be used in Markdoc! Headers, formmating, lists, etc. can all be used in Markdoc files because the interpreter does not alter any of these elements, as the symbol used to note a Markdoc field (^) is not used in any Markdown syntax (as far as I'm aware) and is not used considerably within code, allowing for users to also include code eamples like a traditional Markdown document would! Even if the interpreter overrides something that shouldn't have been overriden, the Markdown document can be easily edited after translation.

### Field Syntax
Fields are defined using the `^` symbol and must be enclosed within two `^` symbols for proper field scope and generation. In fields, you can define a function or variable, as well as retrieve certain data from the __last__ function or variable that has been defined.
Here are some command explanations:

### Definitions
`^function name, return type, param name type, ...^`

Creates a function __header__ with the given name, return type, and any other given paramaters. Commands in fields must be separated by a comma while attributes must be separated by a space.

`^variable name type^`

Creates a variable with a designated return type.

`^get keyword^`

Returns the attribute that is given, for example:
- `^get function^` returns the last function name defined.
- `^get return^` returns the last defined function's return type.
- `^get param index^` returns the last defined function's nth parameter, when n starts at 1. __The paramater index must be given.__
- `^get variable^` returns the last varible name defined.
- `^get variable return^` returns the last defined variable's type.

### Example
Say we defined a function using `^function add, return int, param num1 int, param num2 int^`
- With the default config, the function description becomes `fn add(num1: int, num2: int) -> int`
- `^get function^` becomes `add`
- `^get return^` becomes `int`
- `^get param 0^` becomes `num1`
- `^get param 1^` becomes `num2`

Note: there is no support for getting the types of parameters yet. An alternative is to define variables after the function with their own description.

### Workflow
Create a .mkdc file under the mkdcfile folder. Once you've entered the desired content, run Markdoc and follow the prompts. If everything goes smoothly, a new .md file should appear under the same name of your .mkdc file.

### Config files walkthrough
Some sample config files have already been provided and can be used while running Markdoc as prompted (note: when Markdoc asks for a file, do __not__ include the file extension. Markdoc automatically handles file extensions). These config files allow for the user to customize the appearance of the returned value from Markdoc fields for functions.

#### Explanation
- `use_function_identifier` (true or false): if true, Markdoc will use the function identifier given in `function_identifier`. If not, it will use the return type of the function.
- `param_mode` (c-style or colon): c-style means that paramters in functions will be displayed as `type name`. Colon means parameters will be displayed as `name: type`.
- `return_format` (default, arrow, colon): default acts the same as if `use_function_identifier` was false. Colon means the function would display like this: `indentifier func(): type` while arrow displays like this: `indentifier func() -> type`

## License
This project is licensed under the MIT license.

## Acknowledgements
Me for putting up with my horrible code for multiple days. :) (Do yourself a favor and don't look ;-;)