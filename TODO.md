# TODO
 1. Add fluent parser behind `fluent` feature
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
 2. Add `fluent!` procedural macro to load fluent bundles and build a module or set of modules full of keys from them
   1. String and number literals
   2. Message references
   3. Term references
   4. Function references
   5. Select expression
 3. Add support for locale specific numbers
   1. Add `NUMBER` function without currency support to `fluent!`
 4. Add support for locale specific currency
   1. Add currency support to `NUMBER` function in `fluent!`
 5. Add support for locale specific dates and times
   1. Add `DATETIME` function to `fluent!`
 6. Add "Accept-Language" header parser