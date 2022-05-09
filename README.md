# Non-English Word Extractor

The motivation behind creating this was that a friend and my self wanted to come up with a unique name for some ideas that we had. While there are plenty of name generators out there on the internet they all seemed to be off. That's when I had the idea to start looking at old literature for inspiration. I quickly ran into the issue of when trying to find unique names (such as for characters and cities) that authors came up with, I would have to actually skim the text. I wanted a faster, more efficient and error-prone way and that's why I created this small program! This program works by taking every word in the provided file and seeing if it exists in the set of [466k english words](https://github.com/dwyl/english-words). If the word is contained in the set, then nothing happens and the program will move on to the next word. If it's not found within the set, the program will write the word on a new line to the output file that's name is the timestamp (to the second) of when the program began running. This process is repeated for all files until finished. 

## Required Software / Tech Stack

This program is written in rust and is meant to function as a command line tool, I didn't package and distribute it as an execute able **so you will need Rust installed to run it.** This also allows you to tweak any values to better suite your needs. *Note* - this is my first Rust code that I have ever written (I am yet to read any documentation or tutorials so their may be more idiomatic and efficient ways to things).

## How to Use

Once you have Rust installed the first step is to put the files that you want to have the non-english words extracted from in the the "input" folder of the root directory. The built in file types supported are txt, pdf and epub. All other file types will be skipped. After the input files are in the correct location, the next step is to run the command `cargo run` in the terminal of the root directory. This will fetch the dependencies and begin the program. The length of time until it finishes running is determined by the length of the files that are being used as input. The program will print status updates to the terminal as it processes each input file. Once complete you then can go to the output directory which will show the resulting output of non-english words in a file with the current date timestamp as the name. From here you can do as you please with the resulting output.

## Future Work

I encourage anyone to expand upon this by changing the special cases of cleaning special characters from text to prevent false positives for your specific input files or by adding additional file support (or in any other way you can think of)! You can even adapt the word set that the code checks to see if the word is in prior to writing it to the output file to be a different language/theme. Additionally, further organization and splitting the code into more separate files.

## Example Usage

I won't actually include the input files to be safe in terms of copyright but this example as the following input directory:

input:
- Edgar Allan Poe - Complete Works of Edgar Allan Poe .epubEdgar Allan Poe
- Robert W. Chambers - The King in Yellow.pdf
- test.notsupportedfileextension
- test.txt

then running the program:  
`cargo run`

```
running non-english word extractor
loading set...
finished loading 466546 items into set
creating output file...
reading input files...
processing: Edgar Allan Poe - Complete Works of Edgar Allan Poe.epub
processing: Robert W. Chambers - The King in Yellow.pdf
processing: test.notsupportedfileextension
unsupported file type: .notsupportedfileextension
supported file types are .txt and .pdf
skipping test.notsupportedfileextension
processing: test.txt
completed successfully! finished in 56 s
check the output directory for the generated files
```

output:  
[see here](./example-output/2022-05-09_16_54_34.txt)
