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
- [x] A Table that expands automatically with each service you add to your invoice(This can be useful in other projects rust alternative to jspdf-table(javascript) )
- [x] GUI
- [x] Invoice generator
- [x] Fixed invoice template with table support
- [x] Invoice Rendering through the GUI
- [x] Invoice generation through GUI
- [x] Deleting PDF's
- [x] Only Slovenian language support on the invoice template(FOR NOW)
- [x] Flexible data customization(Company,Partner,Invoice, Tax rate for the whole invoice)
- [x] No tax template which removes the all tax parts from the template
- [x] Custom user company , partner ,services (you cannot edit them for now)
- [ ] PDF Sign feature(It would work if the printpdf crate would support placing the image anywhere on the page(everything is implemented just the rendering is not))
#### Upcoming features.
- [ ] Converting everything fully to database
- [ ] PDF Sign feature
- [ ] Auto complete partner from Slovenian database (will probably use webscraping instead of an api)
- [ ] Hopefully i can implement to convert PDF into JPG and save it to memory to show it in the gui instead of converting + saving it to the path of the invoice then displaying it(really inefficient)

# Showcase

![alt text](https://i.imgur.com/pwrZ4Xj.png "PDF Viewer")
![alt text](https://i.imgur.com/No2S5RC.png "Invoice Creation") 
![alt text](https://i.imgur.com/2x5kPi6.png "Logo Title Text 1")

# Invoice template

![alt text](https://i.imgur.com/oSGMmMe.png "Logo Title Text 1")
