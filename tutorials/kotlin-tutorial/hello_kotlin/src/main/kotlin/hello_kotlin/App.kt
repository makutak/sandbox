/*
 * This Kotlin source file was generated by the Gradle 'init' task.
 */
package hello_kotlin

class App {
    val greeting: String
        get() {
            return "Hello world."
        }
}

fun start(): String = "OK";

fun main(args: Array<String>) {
    println(App().greeting)
    println(start())
}
