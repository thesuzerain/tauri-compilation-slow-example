# tauri-compilation-slow-example

A stripped down version of Modrinth's 'theseus' launcher using Tauri. This is a temporary repo to demonstrate a compilation slowness issue. This compilation time happens specifically with the tauri compilation procedure. In theseus, we have a 'theseus' backend API library, and 'theseus_gui' (the tauri app)- theseus compiles very quickly, whereas the gui library (which just contains #\[tauri::command\]s that call the API) compiles very slowly.

**ALL relevant code is in `theseus_gui/src_tauri/src`**

The compilation slowness seemss to have a couple interacting factors nested in the API:

- A OnceCell that is gettable from a function
- Many places in the code that call to access this oncecell
- Several #\[tauri::command\] functions that call the same pieces of code in the backend.

On my macbook pro (Apple M1, 8gb memory) this takes ~10 minutes to compile.  Reducing the number of commands here or even removing the Oncecell part reduces it to almost nothing.

It seems like when a #\[tauri::command\] is being compiled, it recompiles everything down the chain for each command. When the same nested code in the backened is reached from several command 'routes' (or multiple times per route in the smae command) it causes recompilation several or many times depending on the code structure.

In modrinth/theseus, we have modified it so that the nested functions called from #\[tauri::command\]'s don't share/repeat code, which has vastly improved compilation time (but has resulted in some ugly code in some cases).

Provided is an example of a easy-to-see case of the compilation taking some time. In this library, we have:

`do_task()` -  a simple getter/created of a oncecell

`do_task_n()` (1 through 15) - which exponentially calls `do_task_n()` for n-1, n-2, etc. 

`do_command_n()` (1 through 10) - which call all the tasks in do_task()

This is slightly exaggerated in this repo- you don't need to do 15 tasks or 10 commands to get this to work, but this will definitely cause it. I believe this happens because every time a command is compiled, it recompiles everything below it as well. (This could be wrong- but it seems consistent with the results here and also how we've changed the theseus library to avoid it). 

For tauri libraries with very discrete separate commands, this isn't an issue, but for ones that have a complex backend like theseus it builds up quickly.


