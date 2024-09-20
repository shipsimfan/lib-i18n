# TODO
 1. Add getting current user's language
   1. Windows
   2. Linux
 2. Add `m!` macro for getting the display of a key in the users current language or a specific language
   - Inputs:
     - Path to the key
     - List of arguments as `name => value` pairs
 3. Add fluent parser
 4. Add `fluent!` procedural macro to load fluent bundles and build a module or set of modules full of keys from them
   - Individual files represent single languages. Directories become publically available submodules after macro expansion
 3. Add support for locale specific numbers
 3. Add support for locale specific currency
 3. Add support for locale specific dates and times
 3. Add "Accept-Language" header parser