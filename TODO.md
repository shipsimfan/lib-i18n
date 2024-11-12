# TODO
 1. Re-implement `message!` macro
 2. Fix `message_key!` macro
 3. Add fluent parser behind `fluent` feature
   1. MessageReference
     1. AttributeAccessor
   2. StringLiteral
   3. NumberLiteral
   4. TermReference
     1. NameArgument
     2. Argument
     3. CallArguments
   5. FunctionReference
   6. SelectExpression
 4. Add `fluent!` procedural macro to load fluent bundles and build a module or set of modules full of keys from them
   1. Fix the macro with improvements to MessageKey
   2. String and number literals
   3. Message references
   4. Term references
   5. Function references
   6. Select expression
 5. Add support for locale specific numbers
   1. Add `NUMBER` function without currency support to `fluent!`
 6. Add support for locale specific currency
   1. Add currency support to `NUMBER` function in `fluent!`
 7. Add support for locale specific dates and times
   1. Add `DATETIME` function to `fluent!`
 8. Add "Accept-Language" header parser