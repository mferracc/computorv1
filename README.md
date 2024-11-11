# **A Program To Solve Polynomial Equations**
What if a small rounding error could derail an entire computation?
This project dives into the subtle and surprisingly tricky task of solving polynomial equations, where precision, performance, and algorithmic elegance converge.

---

## **Table of Contents**
1. [Introduction](#1---introduction)
2. [Experience TDD By Building A Linear Solver](#2---experience-tdd-by-building-a-linear-solver)
3. [Solving Quadratic Equation Might Be More Complex Than You Imagine](#3---solving-quadratic-equation-might-be-more-complex-than-you-imagine)
4. [Solving cubic equations using Cardano's formula]()
5. [Ferrari's method for quartics]()
6. [Can we solve higher degrees polynomials? The Abel-Ruffini Theorem]()
7. [The guessing approach: Newton-Raphson Method]()
8. [Finding all the roots at the same time with the Durand-Kerner Method]()
9. [Last but not least: The Bairstow's Method]()
10. [Conclusion]()

---

## **1 - Introduction**

This project isn’t just about solving equations—it’s about tackling one of the trickiest and most essential challenges in math and programming.
Polynomials show up everywhere—from physics to computer graphics to machine learning—and having a way to solve them that’s both efficient and reliable is a game-changer.
But as you’ll see, it’s not just about plugging numbers into formulas.

Our goal is to build a **polynomial equation solver** that can handle everything from the humble linear equations to the more complex quartics—and who says we have to stop there?
By pushing the limits, we might just find ways to tackle even higher-degree equations.
And we’re doing it the right way, with a strong focus on **precision**, **performance**, and **Test-Driven Development (TDD)**.

We will be using Rust all along, because its memory safety, blazing speed, and powerful type system make it perfect for this challenge.
Plus, Rust’s testing tools make TDD not just possible, but actually enjoyable.

Each chapter will introduce a core programming concept that you will get to put directly into practice.
By the end, we will have built not only a powerful polynomial solver but also a toolkit of algorithms and programming techniques useful for future projects.

But enough talking. We've got plenty to build, so let’s get started!

---

## **2 - Experience TDD By Building A Linear Solver**
#### Concept: Test-Driven Development (TDD) and Unit Testing

In TDD we first define the expected behaviour of a function using tests before we even write the function itself.
This way, we have a clear set of criteria for what our code needs to achieve.

Let's start by handling the simpler cases, like [linear equations](https://en.wikipedia.org/wiki/Linear_equation).

Linear equations are of the form `ax + b = 0` represented as follows on a graph: 


<div align="center">
  <h3><i>Linear Equation Graph: y = ax + b</i></h3>
  <img src="assets/linear_equation.gif" alt="Linear Equation Graph Demo">
</div>

As shown in the graph above, there is only one case when a linear equation has zero solution: when `a` is equal to zero.
Otherwise, we can determine the value of `x` for which `y` will be `null` using the following formula:

<div align="center">
  <img src="https://latex.codecogs.com/svg.image?\inline&space;\LARGE&space;{\color{White}\mathbf{x=-\frac{b}{a}}}">
</div>

We will start now our [Test Driven Development](https://en.wikipedia.org/wiki/Test-driven_development) journey by writing tests for this function.
We are placing this tests in a module below the function definition.

Here’s a quick look at the main tools we’ll use to write tests:

- **#[cfg(test)]:** marks a test module, so it only gets compiled when running tests, but is ignored by the compiler when building the project.
- **#[test]:** marks a function as a test case.
- **assert_eq!:** macro used to compare what we expect our function to return with what it actually returns, and fails the test if they don’t match.

Since a linear equation of the form  `ax + b = 0` may or may not have a valid solution, it's a great place to use Rust's [`Option`](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values) type.
The `solve_linear` function will wrap the value `x` in `Some(x)` when a valid solution exists, or return `None` if there is no solution (for example when `a` is zero).

With that being said, we can write our first unit tests for the future `solve_linear` function.

Notice that we are writing these tests first before the actual function definition.

```rust
pub fn solve_linear(a: f64, b: f64) -> Option<f64> {
  unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_solution() {
        assert_eq!(solve_linear(2.0, 4.0), Some(-2.0));
    }

    #[test]
    fn test_negative_coefficient_a() {
        assert_eq!(solve_linear(-3.0, 9.0), Some(3.0));
    }

    #[test]
    fn test_negative_coefficient_b() {
        assert_eq!(solve_linear(4.0, -8.0), Some(2.0));
    }

    #[test]
    fn test_no_solution() {
        assert_eq!(solve_linear(0.0, 4.0), None);
    }

    #[test]
    fn test_zero_both_coefficient() {
        assert_eq!(solve_linear(0.0, 0.0), None);
    }

    #[test]
    fn test_edge_case_small_value() {
        assert_eq!(solve_linear(1e-10, 1e-10), Some(-1.0));
    }
}
```

Now that our tests are defined, we can finally implement the `solve_linear` function:

```rust
pub fn solve_linear(a: f64, b: f64) -> Option<f64> {
    if a == 0.0 {
        return None; // No solution
    }
    Some(-b / a)
}
```

Once you have implemented the function, try running the tests using `cargo test` in the terminal.
If everything is working as expected, all the tests will pass! If you see any failures, no problem - it's just a chance
to make your code even better! Tweak things until tests go green, and enjoy the process. You've got this!

---

## **3 - Solving Quadratic Equation Might Be more Complex Than You Imagine**
### Concept: Floating-Point Arithmetic and Precision

Now that we're done with linear equation, let's move on [Quadratic](https://en.wikipedia.org/wiki/Quadratic_equation).
Quadratic equations are of the form `ax^2 + bx + c = 0`, and their graphic representation is called a `parabola`.

<div align="center">
  <h3><i>Quadratic Equation Parabola: y = ax^2 + bx + c</i></h3>
  <img src="assets/quadratic_equation.gif" alt="Linear Equation Graph Demo">
</div>

We can solve this type of equation in two steps. First we calculate its `discriminant` ∆ (“delta”):

<div align="center">
  <img src="https://latex.codecogs.com/svg.image?\inline&space;\LARGE&space;{\color{White}\Delta=b^{2}-4ac}">
</div>

Then, if the discriminant `∆ ≥ 0`, the equation has one or two real solutions.
The solutions are calculated as:

<div align="center">
  <img src="https://latex.codecogs.com/svg.image?\inline&space;\LARGE&space;{\color{White}x_{1}=\frac{-b-\sqrt{\Delta}}{2a}}" style="margin-right: 20px;">
  <img src="https://latex.codecogs.com/svg.image?\inline&space;\LARGE&space;{\color{White}and}" style="margin-right: 20px;">
  <img src="https://latex.codecogs.com/svg.image?\inline&space;\LARGE&space;{\color{White}x_{2}=\frac{-b&plus;\sqrt{\Delta}}{2a}}">
</div>

Note that in the case of a single solution, the discriminant ∆ will be null and the unique solution can be expressed as:

<div align="center">
  <img src="https://latex.codecogs.com/svg.image?\inline&space;\LARGE&space;{\color{White}x_{0}=\frac{-b}{2a}}">
</div>

Now you might be thinking, “Great, let's just use this formula in our code and compute the values directly!”
But hold on-there's a tiny detail you're overlooking.
And this seemingly tiny detail can make a huge difference in your results.

When performing operations like `b^2 - 4ac` or `square_root(∆)`, we're dealing with [floating-point numbers](https://www.validlab.com/goldberg/paper.pdf).
And unfortunately, floating-point arithmetic can introduce small errors due how numbers are represented in computers.
These errors might accumulate and affect the accuracy of your final result.

In scientific computing, or any situation where precision is crucial, even the smallest inaccuracies can significantly affect the final result—leading to costly errors or even catastrophic failures, such as in aerospace or engineering systems.

////////////////////////////////////////////////////////////

-> solve the equation using the strict mathematical method for calculating the solution
-> test with crates
-> underline the unefficiency of the method (link to the [paper](http://i.stanford.edu/pub/cstr/reports/cs/tr/66/40/CS-TR-66-40.pdf)
-> find a better way to solve the equation in a cs way to avoid floating points errors due to approximation
-> danger of floating points arithmetic on the accuracy of solving a quadratic equation (rounding error)

``` text
We want a quadratic equation solver that will accept any floatingpoint 
numbers a, b, c, and compute any of the roots xl9 x2 that lie safely 
within the range of floating-point numbers.

Any computed root should have an error in the last decimal place not 
exceeding, say, 10 units.

If either x1 or x2 underflows, or overflows, there should be a message
about what happened.

Such an algorithm has been devised by William Kahan of the University
of Toronto.
```
[What every computer scientist should know about floating point arithmetic](https://www.validlab.com/goldberg/paper.pdf)

[Kahan summation algorithm](https://en.wikipedia.org/wiki/Kahan_summation_algorithm)

[Kahan algo in python](https://chrisjameswalker.com/2022/01/28/kahans-summation-algorithm/)


---


Solving Quadratic Equations Might Be More Complex Than You Imagine

Concept: Floating-Point Arithmetic and Precision
Highlight the challenges of floating-point arithmetic, especially in cases where slight inaccuracies can lead to incorrect roots. This chapter can also introduce handling numerical instability and discuss methods like Kahan summation to reduce errors.

Solving Cubic Equations Using Cardano's Formula
Concept: Recursive Functions
Introduce recursion, which can be particularly useful for polynomial decompositions or parts of Cardano's formula. Use this opportunity to discuss when recursion is efficient and when it can lead to stack overflow or performance issues.

Ferrari's Method for Quartics
Concept: Algorithm Complexity and Optimization
Ferrari's method is more complex, so this is a great place to introduce algorithm complexity. Explain Big O notation in practical terms and discuss why more efficient algorithms matter when computations get heavier.

Can We Solve Higher-Degree Polynomials? The Abel-Ruffini Theorem
Concept: Limits of Computability and Theoretical Constraints
Since the Abel-Ruffini theorem states that certain polynomials can’t be solved with radicals, introduce the idea of computational limits. Discuss how some problems have theoretical constraints that require alternative approaches like approximations or iterative methods.

The Guessing Approach: Newton-Raphson Method 
Concept: Iterative Methods and Convergence
Newton-Raphson is a classic iterative approach, so explain iterative methods and convergence criteria. This is a great place to discuss convergence speed, accuracy, and stopping conditions in iterative algorithms.

Finding All the Roots at the Same Time with the Durand-Kerner Method
Concept: Parallelism and Concurrent Computing
Durand-Kerner involves finding multiple roots simultaneously, which is a good context for introducing parallelism. Discuss how concurrent computing could be applied here and what it could mean for performance.

Last but Not Least: The Bairstow's Method
Concept: Error Handling and Robustness in Numerical Methods
Introduce robust error handling strategies and the importance of handling edge cases in numerical methods. This could also be a good place to introduce Rust’s Result type for error handling.
