#!/usr/bin/env cargo

#![cargo(r#"
[dependencies]
foo = "1.2.3"
"#)]




#!/usr/bin/env cargo

/*!```cargo
[dependencies]
foo = "1.2.3"
*/



#!/usr/bin/env cargo

//! ```cargo
//! [dependencies]
//! foo = "1.2.3"
//! ```



#!/usr/bin/env cargo
---
[dependencies]
foo = "1.2.3"
---



#!/usr/bin/env cargo
#[cargo(version = "1.2.3")]
extern crate foo;






- Module level: nice to have at top
- Attribute has a lot of concepts if we have to do string
- 


In educational material, a comment for dependencies likely is already being used, so this is no different

custom class to hide the code block
"hidden" attribute in infostring

attributes are a "here be dragons"


"I don't event teach attributes until the second course"
- Nathan


Disable `mod foo;`?


doc-comment
- easy to understand when seen
- but concern from Nathan on teaching people to write it from scratch
