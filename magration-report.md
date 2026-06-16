# i am swatched old simple architecture to dom tree style architecture

when i first started building this browser, i had no idea how browsers worked internally. my goal was simple: fetch some html and show something on the screen.

at the beginning, i used a very simple architecture.

html
↓
elements
↓
renderer

this approach was perfect for learning. i converted html tags directly into rust elements and rendered them one by one.

it was easy to understand, easy to debug, and helped me build my first working browser features quickly. using this approach, i successfully implemented headings, paragraphs, links, images, lists, tables, bookmarks, tabs, and navigation history.

however, as the browser grew, i started noticing problems.

the biggest issue was that i was losing the structure of the original html document.

i could display the content, but i no longer understood how everything was connected together.

i no longer knew which elements belonged inside other elements. i lost the parent-child relationships that existed in the original html.

this made adding new features increasingly difficult.

every time i wanted to support a new html tag, i had to update multiple parts of the browser. the browser slowly became harder to maintain.

at that point, i realized that real browsers do not work this way.

chrome, firefox, and servo all use a dom tree.

a dom tree preserves the relationships between elements.

nothing is lost.

parent-child relationships remain intact.

once i understood this, switching to a dom tree became an obvious decision.

the migration was not because my original design was bad.

in fact, i think starting with the simpler architecture was one of the best decisions i made.

if i had started directly with a dom tree, i probably would not have understood what was happening.

the simple design taught me how parsing works, how rendering works, how browser features interact, how rust enums can represent html, how to debug compiler errors, and how to evolve software incrementally.

only after understanding these fundamentals was i ready to move to a more realistic architecture.

by introducing a dom tree, i gained several advantages.

the original html structure is preserved.

nested elements become easy to support.

css becomes possible.

forms become possible.

javascript becomes possible.

layout engines become possible.

future browser features become easier to implement.

i also learned an important software engineering lesson.

good architecture is not about choosing the most advanced solution from the beginning.

good architecture evolves.

the first version should solve today's problems.

when new limitations appear, the architecture can grow to meet new requirements.

that is exactly what happened in this project.

the original architecture helped me learn.

the dom tree architecture will help me grow.

this migration taught me that large systems are not built perfectly from the start.

they evolve step by step.

today, my browser is no longer just an html viewer.

it has become a small browser engine.

and i now understand a little better how real browsers work internally.
i am proud of both versions of this browser.
the first version taught me how to build.
the second version is teaching me how to design.