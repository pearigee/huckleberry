# Methods in Huckleberry

# Example
```clojure
;; Defining a method for numbers.
(defmethod Number [to: do:]
    (foreach (range self to:)
        (fn [i] (do: i)))

;; Methods are called with the $(...) form.
$(1 to: 5 do: (fn [i] (print i))) ;; Prints "1234"
```

# Creation
Methods in Huckleberry are similar to multimethods in other languages.
To create one, use the function `method`.
```clojure
(defmethod target args & implementation)
```

## Method resolution by target
The `target` can be a class or function. Depending on the target, method resolution is preformed differently:

| Target | Can be overwritten? | Resolution |
| ------ | ------- | ---------- |
| class  | **Yes**, will overwrite previous method | Via map lookup |
| function | **No**, will be appened to evaluation list | First passing method wins |

**Class** methods are stored in a map with a tuple key. The tuple is the class followed by the argument names. Defining another methods for the class with the same argument names will overwrite any previous values. Method resolution is fast.

**Function** methods are resolved by evaluating `target` on `self` until one returns a truthy value. As a result, _be careful that the resoltion functions are small, pure, and lightweight_. When a function method is resolved, the winning method is moved to the front of the evaluation list. Overtime this meands that the most commonly used functions are bubbled up to the top reducing resolution time.

## Example in code
```clojure
;; Creates an constructor with the defaults provided.
;; The map returned will set its type metadata to "Person".
(class Person {
    :age 10
    :name "default"})

;; When initializing a Person, the defaults can be overwritten.
(def larry (Person {:name "Larry"}))
(def bob (Person {:name "Bob" :age 28}))

;; Methods without arguments are defined a as symbol.
(defmethod Person say-hi
    (println (str $(self get: :name) " says hi!")))

;; This would be called like so:
$(bob say-hi) ;; -> "Bob says hi!"

;; Methods with multiple arguments are defined as a vector
(defmethod Person [say to]
    (println 
        (str #(self get: :name) " says " say " to " $(to get: name) ".")))

;; Now lets call it. Note that arguments that expect a value end in colon.
$(bob say: "hello" to: larry) ;; -> "Bob says hello to Larry."

;; Methods can be defined for things other than classes. If instead
;; we pass a function as the target, the first method to return `true`
;; when passed self will be executed.
(defmethod 
    (fn [self] (and $(self is-type: Person)
                    (< $(self get: :age) 18)) 
    is-old-enough
    (println "TO YOUNG!")))

(defmethod 
    (fn [a] (and $(p is-type: Person)
                 (>= $(p get: :age) 18)) 
    is-old-enough
    (println "Old enough")))

$(bob is-old-enough)   ;; -> "Old enough"
$(larry is-old-enough) ;; -> "TO YOUNG!"
```