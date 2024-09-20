# TODO
 1. Change library to `no_std` and add a default `alloc` feature
 3. Add getting current user's language behind default `current` or `os`
   1. Windows
   2. Linux
 4. Add `m!` macro for getting the display of a key in the users current language or a specific language
   - Inputs:
     - Path to the key
     - List of arguments as `name => value` pairs
 4. Add fluent parser behind `fluent` feature
 5. Add `fluent!` procedural macro to load fluent bundles and build a module or set of modules full of keys from them
   - Individual files represent single languages. Directories become publically available submodules after macro expansion
 6. Add support for locale specific numbers
 7. Add support for locale specific currency
 8. Add support for locale specific dates and times
 9. Add "Accept-Language" header parser