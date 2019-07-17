## Trial Division Example

This is a simple example of writing a Python module in Rust.

It can be used to illustrate the Python GIL limitations, since the module releases the GIL and therefore will allow 
multiple Python threads to run this code in parallel.

## Building

Ensure you have Rust `nightly` installed (the `pyo3` package requires it currently):

    $ rustup install nightly

Then, build as follows:

    $ ./build.sh

This will create the `target/release/primes.so` Python module.


## Simple example

    $ export PYTHONPATH=$PWD/target/release

    $ python                                                                                                 ~/rust/primes
    Python 3.7.3 (default, Mar 27 2019, 09:23:15)
    [Clang 10.0.1 (clang-1001.0.46.3)] on darwin
    Type "help", "copyright", "credits" or "license" for more information.
    >>> import primes
    >>> primes.trial_division
    <built-in function trial_division>
    >>> primes.trial_division(100)
    [2, 2, 5, 5]


## Full example

See [examples/threads1.py](examples/threads1.py) for a full Python program that compares the Python implementation of 
`trial_division` with the Rust one, with and witout the GIL.
