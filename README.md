Rust Ray Tracer
---------------

By Roger Allen

This is my implementation based on the "Ray Tracing in One Weekend" e-book by Peter Shirley. http://in1weekend.blogspot.com/2016/01/ray-tracing-in-one-weekend.html I wanted to implement this in Rust as a way to learn the language. Keep that in mind if you look at the code--I'm a beginner.  I hesitated publishing the code for that very reason, but what the heck, why not put it out there...

That said, I was pleasantly surprised that it ran about the same speed or maybe a bit faster (random scene so hard to compare precisely) than the C++ code Peter peter published at https://github.com/petershirley/raytracinginoneweekend

Then I adjusted the code to use the Rayon library and it sped up by the number of threaded cores I have in my CPU.  Nice!

I recommend you buy the e-book and implement this yourself.  It was a lot of fun.

The code is diverging from the book as I add my own personal features.

Usage
-----

cargo run --release > out.ppm

Some Notes
----------

Release vs. Debug is a dramatic difference.

Adding the #[inline(always)] pragmas in the vec3.rs code helps quite a bit.  E.g for a 1440x720 with 10 samples per pixel scene, I see:
* with    =   8.16s real 1m20.0 user
* without =  14.3s  real 2m25.4 user

You can also see the parallelism speedup in the user vs. real times above--about 10x.  I implemented this in two ways.  First with scoped_threadpool which worked, but ended up not being nearly as elegant as the Rayon code.

First announcement https://twitter.com/RogerAllen/status/974501138841051137

Followup https://twitter.com/RogerAllen/status/975616788108054528
