# pyfilon
Implementation of super-fast Filon quadrature in Python
via [`PyO3`](https://github.com/PyO3/pyo3) and [`maturin`](https://github.com/PyO3/maturin).

The Filon quadrature is a quadrature for highly oscillatory
integrals, such as $\int_a^b f(x) sin(mx)$ or $\int_a^b f(x) cos(mx)$.

This is a Python frontend to the Rust package [filon](https://github.com/alexhroom/filon), 
which ports [John Burkardt's implementation of Filon quadrature](https://people.math.sc.edu/Burkardt/cpp_src/filon/filon.html),
based on Chase and Fosdick's algorithm in the ACM, [available on Netlib as TOMS 353](https://netlib.org/toms/index.html).
