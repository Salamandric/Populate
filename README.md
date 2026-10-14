# Populate

A self-run population sim project, where virtual people ("Dolls") live and die, interact with each other, and have children.

Right now dolls can have a first and last name chosen at random, and a set of "needs" values for hunger, mood, and energy (currently nonfunctional).

# The current version of the program:
- Creates a page that lets you view a table of all dolls from the database.
- Has a button that creates between 1-100 randomly generated dolls, amount changeable by a slider.

# Currently planned features:
- "Breeding" page which allows you to choose two dolls and make a child that inherits their traits.
- "Relationship" page which allows you to create different relationship types (eg. enemy, friend, lover) between dolls.
- Simulation which ties the above together.
  - Dolls will make an action each tick, which may change their needs values or relationships.
  - Actions will be randomly chosen with weight depending on needs (ie. more likely to eat when hungry).
  - When dolls have a compatible lover or partner they might breed and make a new doll.
- Add new stats to represent skills
  - stats will change a doll's needs (a doll with more stamina will have energy fall less each tick)
  - when a new doll is made stats are averaged from the parents.
