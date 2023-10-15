gcc ./hello.c
./a.out > loop.kt
kotlinc ./loop.kt
kotlin LoopKt.class > loop.rs
rustc ./loop.rs
./loop > loop.c
diff ./loop.c ./hello.c # output should be empty
