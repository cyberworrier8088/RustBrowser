## building a browser from scratch :-D

i started this project becuse i want to understand how a web browser actually works internally. before this project, i used browser every day without knowing what happend behind the screen :). i thought  "why not try building one myself?"
the first thing i did was reserch how browsers are made. i looked for browser projects written in rust and discovered Servo, an experimental browser project by mozilla. Servo is an amzing project, but it is also vary complex and difficult to understand for a beginner like me. reading its code base was like hard task for me. 
even though i only built a simple version of a browser, this project taught me a lot, it is currently a vary veta version 0.0.0.0.1 beta but i hope that by keeping the code simple and readable, other beginners can learn from it and undestand how browsers work too.
to learn the fundamentals, i studied the book [Browser Engineering](https://browser.engineering/). before reading  it, i had almost no idea what happened inside a browser. After completing it, i understood concept  such as networking, HTML parsing, rendering, and how all the pieces work together. it completly changed the why i look at browser. 
for i chose the rust crate reqwest. i selected it becuse it is powerfull, well good and easy to use . i spent several hours learning how http requests work, how browsers download pages, and how to fetch content from the internet using reqwest.
for window system and rendering, i used Winit and Pixels. Winit handles window creation an user events, while Pixels provides a simple framebuffer for software rendering. learning these crates was one of the hardest parts of the project, but it helped me undestand event loops, rendering piplines, and how to applictions draw things on the screen.
when i finally started coding, i wasn't sure where to begin. i started with networking becuse every browser first needs to download web pages. from there, i build a simple HTML parser and slowly improved it over time. after that, i created a render using pixels and winit. step by step, i added more fetures and learnedby experimentinng.
some of the features include :- 
* html parsing
* text rendering
* headings and paragraphs
* links and nav
* list
* img
* network img loading
* img caching
* multiple tabs
* back and forward history
* reload support
* bookmarks
* a good interface
* basic table
* h1-h6
* strong and italic text

along the way, i encountered many bugs and compiler errror. sometimes i spent horse trying to undestand rust ownership system and error msg. i also used chatgpt as a learning assistant to help me  debug prbm, explain concepts, and guide me through difficult parts of the project. instead of simple giving me answers, it helped me undestand why things worked and how to improve my code.

this project taught me much more than just browser developemnt. learneed how to:-
* reserch unfamiliar topics
* read documentation
* write cleaner code
* git skill improved
* problem solving
* and many more...
* added google search

this browser is not meant to compete with chrome or firefox or servo. it was build for learning. through this project, i improved my rust skills and gained a much deeper undestanding of how browsers work internally.
there is still a long way to go. In the future, i want to add better HTML support, CSS, forms, improved layout, and more advanced rendering techniques.
most importantly, this project taught me that even very complicated software can be understood if you break it down into small steps and keep learning every day. What once looked impossible became manageable, one feature at a time.


demo come finaly:
see you soon

learnining and reserch time 7+ hours
most consumed my time read and learning this [Browser Engineering](https://browser.engineering/)