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
Step 2: Put fonts inside "/fonts" and name them font.ttf and font_bold.ttf (Both have to be)
Step 3: Install Pdfium dynamic library and place it inside a pdfium folder
```
#### Features
- [✔] A Table that expands automatically with each service you add to your invoice(This can be useful in other projects rust alternative to jspdf-table(javascript) )
- [✔] GUI
- [✔] Invoice generator
- [✔] Fixed invoice template with table support
- [✔] Invoice Rendering through the GUI
- [✔] Invoice generation through GUI
- [✔] Deleting PDF's
- [✔] Only Slovenian language support on the invoice template(FOR NOW)
- [✔] Flexible data customization(Company,Partner,Invoice, Tax rate for the whole invoice)
- [✔] No tax template which removes the all tax parts from the template
#### Upcoming features.
- [❌] Custom user company , partner , services so you can select them instead of typing everything
- [❌] Converting everything fully to database
- [❌] PDF Sign feature
- [❌] Hopefully i can implement to convert PDF into JPG and save it to 
memory to show it in the gui instead of converting + saving it to the path of the invoice then displaying it(really inefficient)

# Showcase

![alt text](https://i.imgur.com/pwrZ4Xj.png "PDF Viewer")
![alt text](https://i.imgur.com/No2S5RC.png "Invoice Creation") 
![alt text](https://i.imgur.com/zxizbv3.png "Logo Title Text 1")

# Invoice template

![alt text](https://i.imgur.com/oSGMmMe.png "Logo Title Text 1")
