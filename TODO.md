# TODO
 1. Add fluent parser behind `fluent` feature
 2. Add `fluent!` procedural macro to load fluent bundles and build a module or set of modules full of keys from them
   - Individual files represent single languages. Directories become publically available submodules after macro expansion
   - Hide behind `proc_macro` default feature
 3. Add support for locale specific numbers
 4. Add support for locale specific currency
 5. Add support for locale specific dates and times
 6. Add "Accept-Language" header parser