# INTRODUCTION

#### The making of this app is inspired by my will to learn Rust and become better at it :).

### Written 100% in rust with [pdfium](https://github.com/bblanchon/pdfium-binaries/releases) library bindings

# Use

### This app is made for making,viewing,deleting custom invoice's through the gui.

### For now its pretty slow especially on rendering the image from path even though the jpg's are like 200kb

### Use 
If you actually want to use it there are some steps 
```
Step 1: Install Fonts
Step 2: Put fonts inside "/fonts" and name them font.ttf and font_bold.ttf (Both have to be there)
Step 3: Install Pdfium dynamic library and place it inside a pdfium folder
```
#### Features

- [✔] GUI
- [✔] Invoice generator
- [✔] Fixed invoice template with table support
- [✔] Invoice Rendering through the GUI
- [✔] Invoice generation through GUI
- [✔] Deleting PDF's
- [✔] Only Slovenian language support(FOR NOW)
- [✔] Customizable data in the invoice (company , partner, services etc..)

#### Upcoming features.
- [❌] Converting everything fully to database
- [❌] PDF Sign feature
- [❌] Hopefully i can implement to convert PDF into JPG and save it to memory to show it in the gui instead of converting + saving it to the path of the invoice then displaying it(really inefficient)

# Showcase
## Outdated pictures
![alt text](https://i.imgur.com/pwrZ4Xj.png "PDF Viewer")

![alt text](https://i.imgur.com/zxizbv3.png "Logo Title Text 1")

# Invoice template

![alt text](https://i.imgur.com/oSGMmMe.png "Logo Title Text 1")
