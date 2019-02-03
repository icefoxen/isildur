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
and attempt to re-publish it under a different crate name 
([`gnir`](https://crates.io/crates/gnir), 'cause, why not) which 
will never be yanked.

If you want to write a library using `ring` without potentially
breaking heckin' everything forever whenever one of your users tries
to use `ring` as well, consider using `gnir` instead.  Though you do so at your own risk, since 
this tool has to make a few irritating changes to `ring`'s build script and it does so very blindly. Also be aware that using old versions of
`ring` may expose you to security vulnerabilities, and that the
original maintainer of it does not provide any support for older
versions except through paid contracting.  And I sure as heck am not
going to be responsible for any other crate republished with this tool.


# Is this reliable?

I need this functionality, so I'm intending to just have this thing
running forever.  However, I won't be around forever, one way or
another, so I'm providing this software to whoever wants it to
implement their own such things.

If you don't trust that, feel free to deploy this software itself.
It isn't really designed to be usable for other people though.
I'm certainly not going to go out of my way to fix any bugs people
report that aren't accompanied by a pull request.

It's also possible that an old crate that currently exists, such as `ring` 0.3, can no
longer be published to crates.io. This can happen for a few different
reasons:

 * Ironically, it may depend on another crate that has been yanked
   (such as an old version of `untrusted`, [to pick an example
   completely at random](https://crates.io/crates/detsurtnu)), so you
   have to go down the dependency tree and mirror all of those as
   well, and patch the crates you want to mirror to point to those
   mirrors.  Fortunately, cargo has support for renaming crates, so 
   this tool has a list of crate names to patch and will rename them
   to the ones specified.
 * Rust has mutated a little over time, not *always* in a
   backwards-compatible way, so old packages may no longer build.



# Is this safe?

The goal of this software makes no modifications to the source crate besides the
name and a disclaimer in the readme.  Unfortunately `ring`'s build system does enough random STUFF that this program actually has to reach into it and tinker with it in horrible ways.  Besides the fact that the results of this process may actually be broken, other people might also pretend
to mirror a crate but produce mimic crates that contain malware.
Because the crates.io checksum for a crate file includes the
`Cargo.toml` file when calculating its hash, and this tool has to
modify the `Cargo.toml` to update the crate's name, the republished
crates created by this tool will have a different checksum than the
original.  That makes it more difficult (though still not impossible)
to detect this kind of attack.  Make sure you trust your sources!

# I wanna republish ring myself!

No you don't.

Ok, fine, but if you really do make sure you have `yasm` installed and symlinked to `yasm.exe` or else it won't work.  Versions of ring before 0.7.2 or so don't work anyway, but I don't care about those.