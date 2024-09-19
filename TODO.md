# TODO
 1. Implement `argument`
 3. Add `message_key!` macro for making key creation simpler and standardized for the `m!` macro
   - Inputs:
     - Key constant name
     - Optional argument structure shape or type name
     - Messages as `tag => |args, f| { body }`, `tag => literal`, or `tag => function_name` pairs
   - Create a `const` holding the message key
   - Create a `struct` with the same name for the arguments
 4. Add getting current user's language
   1. Windows
   2. Linux
 5. Add `m!` macro for getting the display of a key in the users current language or a specific language
   - Inputs:
     - Path to the key
     - List of arguments as `name => value` pairs