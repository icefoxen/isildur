# isildur

Not *stealing* the Ring, honest, just borrowing it...

Basically, the [`ring`](https://crates.io/crates/ring) crypto library for Rust is great,
but its [yanking policy](https://github.com/briansmith/ring/issues/774) is a real pain in the ass.
So, this is a program that will watch the crates.io index once a day, and if it sees a new
version of `ring` is published, will fetch it (even if it's yanked) and re-publish it under
a different crate name (`gnir`, 'cause, why not) which will never be yanked.

If you want to write a library using `ring` without potentially breaking heckin' everything forever
whenever one of your users tries to use `ring` as well, use `gnir` instead.  

This tool republishes `ring` with no changes, other than the name.  Be aware that 
using old versions of `ring` may expose you to security vulnerabilities, and that the original
maintainer of it does not provide any support for older versions except through paid contracting.


# Is this reliable?

I need this functionality, so I'm intending to just have this thing running forever.  However, I
won't be around forever, one way or another, so I'm providing this software to whoever wants
it to implement their own set.

If you don't trust that, feel free to deploy this software itself.
