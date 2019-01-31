# isildur

Not *stealing* the Ring, honest, just borrowing it...

This is a tool for downloading all versions of a crate from crates.io
and re-publishing it under a different name.  Why would I want to do
this?  Well...

Basically, the [`ring`](https://crates.io/crates/ring) crypto library
for Rust is great, but its [yanking policy](https://github.com/briansmith/ring/issues/774) is a real pain
in the ass.  So, this is a program that can be invoked via cron or
such to periodically watch the crates.io index, and if it sees a new
version of `ring` is published, will fetch it (even if it's yanked)
and re-publish it under a different crate name 
([`gnir`](https://crates.io/crates/gnir), 'cause, why not) which 
will never be yanked.

If you want to write a library using `ring` without potentially breaking heckin' everything forever
whenever one of your users tries to use `ring` as well, consider using `gnir` instead.

This tool republishes the crate with no changes, other than the name.  Be aware that 
using old versions of `ring` may expose you to security vulnerabilities, and that the original
maintainer of it does not provide any support for older versions except through paid contracting.


# Is this reliable?

I need this functionality, so I'm intending to just have this thing
running forever.  However, I won't be around forever, one way or
another, so I'm providing this software to whoever wants it to
implement their own such things.

If you don't trust that, feel free to deploy this software itself.

# Is this safe?

This software makes no modifications to the source crate besides the
name and a disclaimer in the readme.  Other people might also pretend
to mirror a crate but produce mimic crates that contain malware.
Because the crates.io checksum for a crate file includes the
`Cargo.toml` file when calculating its hash, the republished crates
created by this tool will have a different checksum than the original,
making it more difficult to detect this kind of attack.  Make sure you
trust your sources!
