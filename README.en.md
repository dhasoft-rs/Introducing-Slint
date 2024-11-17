# TFRT 01 - Introducing Slint

There are a lot of libraries for creating GUIs with Rust (see here => https://areweguiyet.com/). After a short try with Egui, then Iced, my choice fell on Slint, whose main links are as follows:

 => https://slint.rs/

 => https://slint.dev/ 

 => https://docs.slint.dev

Equi and Iced are nice libraries, but Slint's approach was the clearest to me, and Iced is sorely lacking of documentation.
Slint's approach is quite similar to QML (Qt framework), providing many widgets that can be used directly.
Slint provides a very good separation between GUI and 'loqic' code.
I first created this project to share this new knowledge, as there is not so much documentation in French about the Slint library (yes, this one you're reading is the english version :-), you can read french version of this document is in the README.md file).

# The project

Here's a 1st example of code to create a simple GUI with Slint. It comes from the template provided by Slint to get things off to a good start. This application contains a label displaying the value of a counter, and 2 buttons for incrementing or decrementing the counter.

<p align="center">
  <img width="300" src="/01_Introducing_Slint.png">
</p>

This 'basic' project contains the minimum required to obtain something that works:
 - a Cargo.toml file containing the required dependencies.
 - a build.rs file which contains instructions for compiling the slint file in its main() function, so the slint UI will be included in the application binary.
 - the main.rs file contains the ```slint::include_modules!();``` line at the beginning, so it will instruct cargo to use the build.rs file to compile the GUI (GUI description files being in a separate file(s) from the source code).
 - a ui/ folder containing the GUI description file, app-window.slint.
 - a src/ folder containing the application's main source file, main.rs.
 - an assets/img/ folder containing the image to be used as the application icon (optional).

One point is I work with VSCode, using “rust-analyser” and “Slint” (Slint language support) plugins. This is not a prerequisite, but this configuration works quite well, and is simple and effective. 
The “Slint” plugin also provides a preview of the current GUI, very useful when you just want to create a new widget before even thinking of integrating it into an application. 


# File details / description 
## app-window.slint file :

 - the line ``import...`` contains the necessary imports, depending on the widgets used.
 - the export keyword in the line ``export component AppWindow inherits Window`` allows the rest of the application to access the AppWindow component (of type
   Window' type).
 - define min window size with ```min-width :``` and ```min-height :```. Please note that you must specify the unit to be used (usually px, logical pixels,
   but there are also physical pixels, noted phx).
 - For positioning and sizing, see => https://docs.slint.dev/latest/docs/slint/src/language/concepts/layouting).
 - define the application's title and image using 'title' and 'icon' properties.
 - the line ```in-out property <int> counter: 42;``` defines the counter variable as writable (in part, call ```set_counter()``` in the main.rs file).
   in main.rs) and readable (out part, call to ```get_counter()`` in main.rs) by the component user.
 - the ``callback...`` lines declare the existence of functions whose body must be entered in the user part of the component (here, in the
   main.rs file).
 - these 'callback' functions are called from actions on widgets in the '.slint' file. Here, they are called on the 'clicked' event of the 'Decrease value' and 'Increase value' buttons.

 - the 'Text' widget displays the value of the 'counter' variable updated by calls to the functions ```request-increase-value()``` and ```request-
   decrease-value()```
 - Slint automatically creates the necessary getters and setters for variables defined in the '.slint' file, as well as the 'on_XXXXXXXXX' functions
   declared in the 'callback' lines.
 - widgets are then defined in a 'VerticalBox' to be positioned from top to bottom in the window.

## buid.rs file :
 - This file is used to compile the .slint file(s). Compilation is performed according to the commands in the main() function of this file.
 - There are two basic approaches to compiling a Slint GUI:
   **a/ The first (and simplest) method uses the following command: **.
   ```
   slint_build::compile(“ui/app-window.slint”).expect(“Slint build failed”);
   ```
      This method seems to be the most suitable for generating a GUI without using a particular theme and for using custom widgets, which we'll see
      in another mini-project.
   **b/ The second method allows you to select a style to be applied to all widgets **.
   
      The styles available here are: https://docs.slint.dev/latest/docs/slint/src/advanced/style
      The commands to be applied are (for the 'fluent-dark' style) :
      ```
    let config = slint_build::CompilerConfiguration::new().with_style("fluent-dark".into());
    slint_build::compile_with_config("ui/app-window.slint", config).unwrap();
      ```
   
## main.rs file :
  - this file contains the application's main code.    
  - it contains the ``std::error::Error`` import needed to handle errors that may be returned at the output of the main() function ```(Box<dyn Error>)````
  - it also contains the macro
    ```
    slint::include_modules!();
    ```
    required to use the build.rs file
    
  - finally, it contains a single main() function which will begin by creating our application window:
    let ui = AppWindow::new()?;````
    
  - if the application is correctly created (no error output), we then define the contents of the callback methods that have been declared in 'callback' lines
    in the '.slint' file:
    ```
	ui.on_request_increase_value({...});
	ui.on_request_decrease_value({...});
    ```

  - then call the windowed application's run() method: ``ui.run()?   
  - finally, we return ```Ok(())```, as main() must return a Result<>.


So, that's all for this first Topic, hope it helps ...
