# Methods in Huckleberry

## Creation
Methods in Huckleberry are similar to multimethods in other languages.
To create one, use the function `method`.
```clojure
(defmethod target args & implementation)
```
## Examples
```clojure
;; Defining a method for numbers.
(defmethod Number [to: do:]
    (foreach (range self to:)
        (fn [i] (do: i)))

;; Methods are called with the $(...) form.
$(1 to: 5 do: (fn [i] (print i))) ;; Prints "1234"
```

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