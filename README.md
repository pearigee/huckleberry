## Huckleberry
Huckleberry is an expermental language trying combine the syntax of Lisp and Smalltalk. To acheive this, Huckleberry uses a flexible implementation of multimethods.

### Overview

Huckleberry has two expression types, S-expressions and method expressions.

For example, here we are using S-expressions to print 0 through 9:
```clojure
(for-each i (range 0 10)
    (println i))
```

The same can be done with method expressions:
```clojure
<0 to: 10 do: (fn [i] (println i))>
```

As you can see, method expressions are wrapped in angle brackets rather than parens. The two can forms can be combined at will. Notice the lambda function in the second example.

### Method creation
New methods can be created like so:
```clojure
(defm number? [to: max do: f] 
    (for-each i (range this max) (f i)))
```
Let's take this appart. `defm` is a special form that enables new methods to be defined.
```clojure
(defm [SELECTOR] [NAME] [...IMPLEMNTATION])
```
Let's break this down further:
 - **SELECTOR**: A selector is a function used to determine if an object is eligible to execute this method. In the example above, we pass the `number?` function which returns `true` when passed a number. This means any number can call this method. Learn more about method dispatch below.
 - **NAME**: The vector `[to: max do: f]` defines a method named `to do`. `max` and `f` are the variables the arguments will populate when called.
 - **IMPLEMENTATION**: This the body of the method.


### Method dispatch
Methods are stored in a map of `String` to `Vec<Method>`, where the key is the name of the method.

When a method is called, Huckleberry steps through the `Method` vector executing the selector functions until one returns a truthy value. This method will be executed.

If no method selectors return a truthy value than a "no method found" error is thrown.

Let's look at an example:
```clojure
(defm (fn [n] <n less-than: 18>)
      [is-under-18?]
      true)

(defm (fn [n] <n greater-than-eq: 18>)
      [is-under-18?]
      false)

(println <17 is-under-18?>) ;; Prints true
(println <18 is-under-18?>) ;; Prints false
```
> Note: In the real world you would also want to very that `n` is a number, but this is an example!

Here our selectors only accept numbers in a specific range.

### Function creation
Functions are created with a syntax similar to Clojure.

For example, here is an example with `defn` and `fn`:
```clojure
;; Computes n iterations of the fibonacci sequence.
(defn fib [n]
    (var a 0)
    (var b 1)
    <0 to: n 
       do: (fn [_] (var c (+ a b))
                   (set! a b)
                   (set! b c))>
    a)
```

### Equality
Unlike other languages, equality is computed by value (not by reference). For example:
```clojure
(var bob {:name "Bob" :age 22})
(var bob-twin {:name "Bob" :age 22})

;; S-expression form
(println (= bob bob-twin)) ;; Prints true

;; Method form
(println <bob = bob-twin>) ;; Prints true
```