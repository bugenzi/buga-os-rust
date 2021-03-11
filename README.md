## Rust os
Writing %%OS%%  ``Step by step guide``

### Bare bones of the os 
        
- Freestanding rust binary ( First step of building os is to build kernel , rust executable that dose not link the standard library ).

- Minimal rust Kenral (creating a minimal 64-bit Rust kernel for the x84 arhitecutre)

- VGA Text Mode ( Creating a simple way to print text on the screen )


### Free standing kernel 

First we disabled standard library (we cannot use any os dependet libraries )

Panic implementation it called when ever somthing fucks up so we save memory and time by aborting the compile and alocaiton of the 
memory for the failed or buggy funnction

#### FUN FACT 
main isnt the first thing that needs to be run in rust compile , fist thing that runs is the runtime 
--runtime ? helps us start stack, hash or even memory /garbage collector 
* * *
Speaking of compiler and runtime we also need to be carefull with compilation erros
            There are 3 types of erros:

- compiler warnings++;(Dont ignore but tbh it isnt somthing severe enough to keep your code from compiling, unles you step
                your compiler to treat them as errors )

- compiler errors++;(on the other hand we have errors they are conditions that prevent the compiler from 
                compilation of the file or code )

- linker errors++;( Dont care for syntax care for finding defitinion offuctions ,classes etc  e.g could not find definiton x)


Why we need to think of linker erros , its becouse of different os like linux , win etc... we could use rust Bare metal target 
it automatiaclly tries to detect what type of executable is it ".exe" or somthing else 

* * *
## Bare metal target  will help us run the correct enviroment (rustc --version --verbose e.g "it shows our env and os ")

ABI(application binary interface )

LLVM ?
- Used tools for developing front end of any programing language and back end for any set of arhitecutre
    LLD ? is the LLVM linker benefit is the cross platfrom integration 

Linker ?
- In computing linker is computer system program that takes one or more objects and combines them together into a single executable,
    or another object 


Pointer analysis ?
- Is static code analysis that establishes witch pointer or heap refrence can point to witch variable or storage location

What is static code analysis ? 
- Is a concept of describing analyzing program wihtout actually executing them
    
    What is dynamic code analysis then ?
        Is concerned with analyzing programs during execution time 

System V ABI (System v application binary interface)?
-----------------------------------------------------
    is set of specification that describes calling convention, object file formats and executable file mormat , dynamic linking and much more
    for the system that compiles with the x/Opne common application Environment specification and system v interface definition .
        + It is today the standard ABI used by the major Unix operating systems such as Linux, the BSD systems, and many others. 
        +The Executable and Linkable Format (ELF) is part of the System V ABI.


### RED ZONE 
Red zone is optimization for the system V ABI that allow function to temporarly use the 128 byte below its stack frame 
            
#### +++Purpose+++ 
it is purely and simply and optimization that can save instructions .Thats why we need to disable red zone becouse it will corupt otherwise 

``` rs 
#![no_std]// dont lin the rust lib 
#![no_main] // disable all Rust-level entry points 

use core::panic::PanicInfo;

/// This function is called on panic.

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

#[panic_handler] 
fn panic    (_info: &PanicInfo) -> ! {
    
    loop {}
}
```

   
   
Using Static to write our byte string of "hello world " and we used addres 0xb8000


Rust unsafe was used becouse rust compiler dosent know if this is a valid pointer , they can point anywhere and lead to data coruption
    NOTE 
        Unsafe dosent remove safety checks 

                Unsafe 5 things it lets me do :

- Derefrence a raw pointer 
- Call an unsafe function or method 
- Access or modify a mutable static variable 
- IMplement an unsafe trait
- Access field of unions 



***
## Running kernal (QEMU)

Our compiled kernel is now saved at location ***"target/x86_64-buga_os/debug/bootimage-buga_os.bin"*** . For running our basic kernal we need a vm (virtual machine ),there are lots of good vms but for the simple os and for the sake of conistency of the course I used QEMU

### Bootloader ?
To run our kernerl we also need to turn it into a bootable disk image ,for that we are gonna use bootloader crate .

#### What dose bootloader do ?
Bootloader is super conviniet and usefull insted of writing our own bootloader and creating our own bios it dose it for us without any C dependecies (rust and asebly )


***

# VGA text mode 
VGA text mode was built in 1987 by IBM , part of a vga standard that was widespread trought 90s and today for some modern pcs.

Main purpose of vga is to display colors (programable 16 color palette ) characters and ther background ,aslo added different shapes of cursor blinking and different loadable fronts 


**FUN FACT**
Linux conosle traditionally uses hardware VGA text modes 


## **Text buffer**

Each character represents 2 bytes or 16 bits .
8 bits are used to represent atribute of characters(color,background color etc) and remaining 8 represent code point of character

VGA text buffer is usually 2 dimensial array that has 25 to 80 colums witch is directly renderd to screen , each array describers a single character .

First btyes represent character that should be printed in ASCII encoding.Its not ASCII but code **page 407**

We are gona use dead code(lint ) to warn us about some functin we are not using ,
derive for basic implementations for some traits(**they can be manually implemented**)


##### Important  Lint is a static code analysis tool used to flag programming errors,bugs , stylistic error and suspiciues constructors	

`#[allow(dead_code)] is an atribute that disables the dead code lint`

Derive has following traits:
- Comparison trait (**eq,PartialEq,Ord**).
- Clone, to create T from &T via copy.
- Copy,give **copy scemantics** not instead of **move scemantics**("**will iterate on next section**").
- Hash,to compute the hash from T.
- Default, to create empty istance of a data type.
- Debug	, to format a value using {:?}formatter.


#### Copy vs Move
The difference is that the copy duplicates all the resources unlike move witch moves the intier resource from one place to another .Thats why **copying is expensive**  and **moving is cheap**

#### Copying
![img](https://www.modernescpp.com/images/blog/EmbeddedProgrammierung/CopyVersusMove/copy.jpg)
Both str 1 and str 2 have resource after copying ABCDEF it basically mirrors the data from str 1 

#### Moving 

![img](https://www.modernescpp.com/images/blog/EmbeddedProgrammierung/CopyVersusMove/move.jpg)
Unlike copying moving will move resourse from str 1 to str 3 witch in return leaves str 1 ""empty and str 3 the new host of the resourse ABCDEF




By following the course we are gonna use C like enum now what is C Like enum ? and why is it different compared to Rust enum ?????? Dunno we will ask google 


**Google** : " C++ has two methods to represent an enum. Using enum class should be favored as it provides scoping.

Rust's enums provides equality testing by adding #[derive(PartialEq, Eq)] to the enum. The use of match is helpful because it forces the checking of all options of the enum. Rust also provides a mechanism to capture an instance of a type for an option of an enum. "


`*AHA Becouse it has scoping it will help us with the color enums plus rust enums more better for equality testing matching and force check .Also rust dosent have u4 unlike c witch is sufficent for colors (this part is the biggers reason we are using c enums)*`



**VGA code structure** 

To make this all work fist we need to implement colors like we wrote beforehand , afther that we need colorcode for giving foreground and background colors  then we need to build screenchar (screen charaters) forthat structure we need c so the strucuture is layed out the same. And after that we need buffer so we can add the height and width of the screen, after all that is finished we need to code a writter that will fuse all these instructions into readable and colored text

- Colors (dead_code ,derive ,u8)
- ColorCode(derive and transparent)
- ScreenChar(derive and c)
- Buffer(transparent)

This part is bit scruffed but I hope you will understand when reading the code 

Essetnial these are the building blocks of a vga
