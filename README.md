## Kobo To Anki Sync Tool
 Effortlessly create Anki decks from the words you save while reading books on your Kobo

### This tool provides you with simple, self explainatory gui interface
![This is an alt text.](https://raw.githubusercontent.com/SildCave/kobo-to-anki/refs/heads/main/screenshots/s1.png
 "GUI")

<details>
<summary>2nd screenshot -> click to expand</summary>
<IMG src="https://github.com/SildCave/kobo-to-anki/blob/main/screenshots/s2.png?raw=true"  alt="image.png"/>
</details>

### Example flashcard _currently there is no way to change the flashcard format_
![This is an alt text.](https://github.com/SildCave/kobo-to-anki/blob/main/screenshots/s4.png?raw=true
 "FLASHCARD")
 
<details>
<summary>front -> click to expand</summary>
<IMG src="https://github.com/SildCave/kobo-to-anki/blob/main/screenshots/s3.png?raw=true"  alt="image.png"/>
</details>

 
### Requirements
 - A computer running Windows 10/11 or Linux (Wayland and X11 are supported)
 - Patience

### How to install
 - Download the app through github release page

### How to use
- **IMPORTANT**: If the app freezes or becomes unresponsive, simply restart it.
- **IMPORTANT**: If the app shows an error stating it cannot connect to the server, please notify me on Discord (@sildcave) or on GitHub.

### How to compile (Linux only, on Windows it's probably just as easy)
 - Install [rust](https://www.rust-lang.org/)
 - `cargo build --release`

### How it works (in steps)
 - Find Kobo eReader (works on Linux, Windows might require manual selection)
 - Try to establish connection between anki and the app through [anki connect](https://ankiweb.net/shared/info/2055492159)
 - Compare words in the deck to the words in your reader
 - Add missing words to the deck (words come from my proxy, which uses the Cambridge Dictionary under the hood. The proxy is required to make the lookup times reasonable)

