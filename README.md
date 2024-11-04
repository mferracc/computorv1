# **ComputorV1: A Polynomial Equation Solver**

---

## **Table of Contents**
1. [Introduction](#1---introduction)
2. [Solving linear and quadratic equations](#2---solving-linear-and-quadratic-equations)
3. [Existing Algorithms for Higher Degree Polynomials](#5---existing-algorithms-for-higher-degree-polynomials)
4. [Crafting a Robust Program Design](#6---crafting-a-robust-program-design)
5. [Conquer the Chaos: Mastering Error Handling](#7---conquer-the-chaos-mastering-error-handling)
6. [Next Steps: Where to Go From Here](#8---next-steps-where-to-go-from-here)

---

## **1 - Introduction**

In this project, we will build a **polynomial equation solver** that can handle equations of varying degrees (from linear to quartic). Polynomials are fundamental structures in both mathematics and programming, and solving them efficiently can serve as a building block for more advanced algorithms.

- **Language:** We chose Rust for its memory safety, performance, and expressive type system, which is perfect for building reliable and efficient solvers.
- **Approach:** We'll adopt a **Test-Driven Development (TDD)** approach, where we write tests before implementing the solver functions.


## **2 - Solving linear and quadratic equations**

Let's start by handling the simpler cases. Linear equations are of the form `ax + b = 0` represented as follows on a graph: 


<div align="center">
  <h3><i>Linear Equation Graph: y = ax + b</i></h3>
  <img src="assets/linear_equation.gif" alt="Linear Equation Graph Demo">
</div>

As shown in the graph above, there is only one case when a linear equation has zero solution, and it is when 'a' is equal to zero.
Otherwise, we can determine the value of 'x' for which y will be null using the following logic:



In this section, you’ll build your first polynomial solver for **linear** and **quadratic** equations:
- **Linear Solver:** Implementing a solver for equations of the form `ax + b = 0`.
- **Quadratic Solver:** Using the **quadratic formula** to solve equations of the form `ax^2 + bx + c = 0`.

Each step will include explanations of the underlying mathematical principles, accompanied by Rust code examples.

---

## **5 - Existing Algorithms for Higher Degree Polynomials**

Once the basics are covered, we’ll move to more advanced algorithms for solving **cubic and quartic** equations:
- **Analytical Approach**:
    - **Derivatives**: By finding the derivative of a polynomial, we can identify critical points, inflection points, and make predictions about the behavior of the function. While this doesn’t solve the polynomial directly, it provides insights for more efficient numerical methods.
    - **Root Refinement**: Derivatives can be used alongside root-finding algorithms to refine the accuracy of potential solutions. We’ll show how to calculate derivatives programmatically in Rust to support root-finding algorithms.

- **Numerical Methods**:
    - **Newton’s Method**: Iterative method to approximate roots.
    - **Durand-Kerner Method**: A complex, but effective algorithm for solving polynomials of higher degrees.
    - **Newton-Raphson Algorithm**: Commonly used for finding successively better approximations to the roots.

This section dives into how these algorithms work and provides Rust implementations.

---

## **6 - Crafting a Robust Program Design**

This section focuses on creating a robust, scalable design for the solver:
- **Data Structures:** How to represent polynomials and their coefficients effectively in Rust.
- **Modular Design:** Break down the solver into reusable, maintainable components.
- **Performance Considerations:** Optimizing memory usage and execution speed to handle large polynomials efficiently.

---

## **7 - Conquer the Chaos: Mastering Error Handling**

A good solver must handle all potential errors gracefully. In this section, we’ll tackle:
- **Parsing Errors:** How to manage invalid input or malformed equations.
- **No Solution or Complex Solutions:** Handling cases where no real solution exists (e.g., complex roots) by leveraging Rust’s `Result` and `Option` types.

This section will walk you through best practices for error handling, making your program more reliable and user-friendly.

---

## **8 - Next Steps: Where to Go From Here**

Once you’ve mastered solving polynomial equations, you can take this project even further:
- **Polynomial Operations:** Implement support for operations like addition, subtraction, multiplication, and division of polynomials.
- **Graphing Polynomial Functions:** Visualize solutions graphically using Rust libraries.
- **Extending to Systems of Equations:** Tackle more complex mathematical problems by extending your solver to handle systems of polynomial equations.

Explore additional resources and continue growing your knowledge!

---

### **Project Status**
- [x] Naming and structuring the documentation
- [ ] Rewrite the first text because I will not push my source code to the public repository but only my unit tests
- [ ] Implementing a linear polynomials solver
- [ ] Implementing quadratic polynomials (via the quadratic formula)
- [ ] Solving cubic and quartic polynomials (advanced algorithms)
- [ ] Lint the code with clippy
- [ ] use Rustfmt to properly format your rust code