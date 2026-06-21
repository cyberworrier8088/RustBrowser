# Mini Browser

## introduction

Mini Browser is a web browser built from scratch in rust. the goal of this porject was not to create a production-ready browser, but to learn how browsers work internaly by building one from the ground up.

before starting this project, i used web browser every day without knowing what happened behind the scenes. through mini browser, i learned about networking, html parsing, DOM tree, layout engines, rendering piplines, downloads, history systems, bookmarks, and many other browser concepts

this project become one of the biggest rust projects i have ever worked on and taught me a huge amount about software architecture and debugging.

___

## why i staryed this project

i wanted to understand:
- how browser download web page
- how html becomes visible on screen
- how rendering engines work
- how tabs and navigation work
- etc

instead of only reading about these toppics, i decided to build my own browser.

___

## reserch phase

the first step was learning how real browsers work.

i discovered the browser engineering book and spent time learning:
- http requests
- html parsing
- dom tree
- rendering
- Layout systems
- browser archi

i also studied Servo, Mozila's expermental browser engine written in Rust.

Servo is an amzing project but it is extremely complex. looking at its source code helped me undestand how much much work goes into buildung a modern browser.

## technology stack
### lang
rust(Entire browser is written in rust)



___

## features implemented:
1. HTML Parsing

the browser can parse html documents and build an internal reperesention of the page

supported tags include:
- h1-h6
- p
- a
- img
- ul
- li
- table
- tr
- td
- div
- span
- strong
- b
- i
- em
- hr
- br

___

## dom tree
one of the biggest milestones was implementing a real DOM tree.

architecture:

html
|
dom parser
|
dom tree

each html element becomes a Node containing:

tag name
text
attributes
styles
children

this allows the browser to understand document structure.

## challenges

this project was not easy.

some major challenges included:

## Understanding Browser Architecture

at first i had no idea how browsers worked internally.

learning dom tree, layout engine, and rendering system took significant reserch

___

## rust ownership

many bugs came from:
- ownership issues
- borrow checker error
- lifetime probm

fixing these taught me a lot about rust.

___

