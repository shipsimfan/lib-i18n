# TODO
 1. Add `m!` macro for getting the display of a key in the users current language or a specific language
   - Inputs:
     - Path to the key
     - List of arguments as `name => value` pairs
 2. Add fluent parser behind `fluent` feature
 3. Add `fluent!` procedural macro to load fluent bundles and build a module or set of modules full of keys from them
   - Individual files represent single languages. Directories become publically available submodules after macro expansion
   - Hide behind `proc_macro` default feature
 4. Add support for locale specific numbers
 5. Add support for locale specific currency
 6. Add support for locale specific dates and times
 7. Add "Accept-Language" header parser